#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![feature(vec_push_within_capacity)]
#![feature(slice_as_array)]

use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::RgbColor;
use esp_alloc::HEAP;
use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_hal::gpio::{Input, InputConfig, Level, Output, OutputConfig, Pull};
use esp_hal::spi::master::{Config, Spi};
use esp_hal::time::{Duration, Instant};
use esp_hal::{Blocking, main, spi};

mod utils;
use mipidsi::interface::SpiInterface;
use utils::{buffer_into_iram, buffer_into_psram, vec_into_iram, vec_into_psram};
mod inputs;
use inputs::I2cInputs;
mod lcd;
use lcd::LcdMonitor;
use log::{error, info};

extern crate alloc;

const INTERNAL_HEAP_SIZE: usize = 98768;
const FPS_WANTED: u64 = 60;

const MONITOR_WIDTH: usize = 320;
const MONITOR_HEIGHT: usize = 240;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    // generator version: 1.0.1

    esp_println::logger::init_logger_from_env();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: INTERNAL_HEAP_SIZE);
    esp_alloc::psram_allocator!(peripherals.PSRAM, esp_hal::psram);

    let left_bump = Input::new(
        peripherals.GPIO36,
        InputConfig::default().with_pull(Pull::None),
    );
    let menu = Input::new(
        peripherals.GPIO35,
        InputConfig::default().with_pull(Pull::None),
    );
    let right_bump = Input::new(
        peripherals.GPIO34,
        InputConfig::default().with_pull(Pull::Up),
    );

    let mut inputs = I2cInputs::new(peripherals.I2C0, peripherals.GPIO21, peripherals.GPIO22)
        .with_ext_inputs(left_bump, right_bump, menu);

    let dc = Output::new(peripherals.GPIO12, Level::High, OutputConfig::default());
    let cs = Output::new(peripherals.GPIO5, Level::High, OutputConfig::default());
    let mut bl = Output::new(peripherals.GPIO27, Level::High, OutputConfig::default());
    bl.set_high();

    let spi_bus: Spi<'_, Blocking> = spi::master::Spi::new(peripherals.SPI2, Config::default())
        .expect("Could not create spi bus")
        .with_sck(peripherals.GPIO18)
        .with_mosi(peripherals.GPIO23);

    let spi_device = embedded_hal_bus::spi::ExclusiveDevice::new_no_delay(spi_bus, cs)
        .expect("Could not create spi device");

    let buf = buffer_into_iram::<u8>(512).unwrap();
    let buf = unsafe { &mut *buf };

    let spi_iface = SpiInterface::new(spi_device, dc, buf);

    let mut rst = Output::new(peripherals.GPIO2, Level::Low, OutputConfig::default());

    let mut delay = Delay::new();
    let mut monitor = LcdMonitor::init_display(spi_iface, &mut delay, &mut rst).unwrap();

    let color = Rgb565::RED;
    let square_height = MONITOR_HEIGHT;
    let square_width = MONITOR_WIDTH;
    let mut color_pixels = vec_into_psram::<Rgb565>(square_width * square_height).unwrap();

    for _ in 0..(square_height * square_width) {
        if let Err(color) = color_pixels.push_within_capacity(color) {
            color_pixels.reserve(1);

            color_pixels.push_within_capacity(color).unwrap();
        };
    }

    if let Err(_) = monitor.set_pixels(
        0,
        0,
        square_height as u16,
        square_width as u16,
        color_pixels,
    ) {
        error!("Could not draw to monitor");
    }

    let mut buf = [0u8; 1];
    let mut running_fps: u32 = 0;

    loop {
        let delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_micros(1000_000 / (FPS_WANTED * 100)) {}
        running_fps = running_fps + 1;

        if running_fps == FPS_WANTED as u32 {
            info!("HEAP STATS: {}", HEAP.stats());
            //info!("FPS: {}", running_fps);
            inputs.read_inputs(&mut buf);
            running_fps = 0;
        }
    }
}
