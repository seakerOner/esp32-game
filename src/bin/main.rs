#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![feature(vec_push_within_capacity)]

use esp_alloc::HEAP;
use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::i2c::master::{Event, I2c};
use esp_hal::time::{Duration, Instant};
use esp_hal::{main, psram};
use log::error;
use log::info;

mod utils;
use utils::{vec_into_iram, vec_into_psram};
mod inputs;
use inputs::Inputs;

extern crate alloc;

const INTERNAL_HEAP_SIZE: usize = 98768;

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

    let inputs = Inputs::new(peripherals.I2C0, peripherals.GPIO21, peripherals.GPIO22);
    let mut test = vec_into_iram::<u32>(200).unwrap();
    let mut test2 = vec_into_psram::<u32>(200).unwrap();

    let mut buf = [0u8; 1];
    inputs.read_inputs(&mut buf);
    loop {
        //info!("HEAP STATS: {}", HEAP.stats());

        let delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_millis(500) {}
    }
}
