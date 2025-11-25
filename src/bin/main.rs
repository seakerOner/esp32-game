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
use embedded_graphics::prelude::{RgbColor, WebColors};
use esp_alloc::HEAP;
use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_hal::gpio::{Input, InputConfig, Level, Output, OutputConfig, Pull};
use esp_hal::spi::master::{Config, Spi};
use esp_hal::time::{Duration, Instant};
use esp_hal::{Blocking, main, spi};
use log::{error, info};

mod utils;
use mipidsi::interface::SpiInterface;
use utils::{buffer_into_iram, buffer_into_psram, vec_into_iram, vec_into_psram};
mod inputs;
use inputs::I2cInputs;
mod lcd;
use lcd::LcdMonitor;
mod assets;
use assets::Player;

use crate::assets::Mob;

extern crate alloc;

const INTERNAL_HEAP_SIZE: usize = 98768;

const MONITOR_WIDTH: usize = 320;
const MONITOR_HEIGHT: usize = 240;

const MONITOR_COLLUMNS: usize = MONITOR_WIDTH / 32;
const MONITOR_ROWS: usize = MONITOR_HEIGHT / 32;

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
    let mut monitor = LcdMonitor::init_display_raw(spi_iface, &mut delay, &mut rst).unwrap();

    LcdMonitor::fill_monitor(&mut monitor, Rgb565::CSS_LIGHT_GREEN);

    let mut player_texture = vec_into_psram::<Rgb565>(32 * 32).unwrap();

    for _ in 0..(32 * 32) {
        if let Err(color) = player_texture.push_within_capacity(Rgb565::RED) {
            player_texture.reserve_exact(1);

            player_texture.push_within_capacity(color).unwrap();
        }
    }
    let mut player = Player::new(player_texture);

    player.draw(
        (MONITOR_HEIGHT / 2) as u16,
        (MONITOR_WIDTH / 2) as u16,
        &mut monitor,
    );

    let mut running_fps: u32 = 0;
    let mut buf = [0u8; 1];

    loop {
        let delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_micros(1000_000) {
            running_fps = running_fps + 1;
            let (i2c_input, ext_input) = inputs.read_inputs(&mut buf);

            player.update_state((i2c_input, ext_input), &mut monitor);
        }
        //info!("HEAP STATS: {}", HEAP.stats());
        info!("FPS: {}", running_fps);
        running_fps = 0;
    }
}
