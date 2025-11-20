#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use core::alloc::Layout;

use alloc::vec::Vec;
use esp_alloc::HEAP;
use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::time::{Duration, Instant};
use esp_hal::{main, psram};
use log::info;

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

    let psram_ptr = unsafe {
        HEAP.alloc_caps(
            esp_alloc::MemoryCapability::External.into(),
            Layout::array::<u32>(100).unwrap(),
        )
    };

    let ram_ptr = unsafe {
        HEAP.alloc_caps(
            esp_alloc::MemoryCapability::Internal.into(),
            Layout::array::<u32>(100).unwrap(),
        )
    };

    let mut v_in_ram = unsafe { Vec::from_raw_parts(ram_ptr as *mut u32, 0, 10) };
    let mut v_in_psram = unsafe { Vec::from_raw_parts(psram_ptr as *mut u32, 0, 10) };

    for _ in 0..3 {
        v_in_ram.push(2);
        v_in_psram.push(2);
    }

    loop {
        info!("----------------------------");
        info!("HEAP STATS: {}", HEAP.stats());
        info!("----------------------------");

        let delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_millis(500) {}
    }
}
