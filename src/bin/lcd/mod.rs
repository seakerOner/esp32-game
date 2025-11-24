use alloc::vec::Vec;
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_hal::digital::OutputPin;
use esp_hal::delay::Delay;
use log::error;
use mipidsi::interface::SpiInterface;
use mipidsi::options::Orientation;
use mipidsi::{Builder, models::ILI9341Rgb565};
use mipidsi::{Display, NoResetPin};

use crate::{MONITOR_COLLUMNS, MONITOR_HEIGHT, MONITOR_ROWS, MONITOR_WIDTH, vec_into_psram};

pub struct LcdMonitor;

impl LcdMonitor {
    pub fn init_display_raw<'d, SPI, DC>(
        di: SpiInterface<'d, SPI, DC>,
        delay: &mut Delay,
        rst_pin: &mut impl OutputPin,
    ) -> Option<Display<SpiInterface<'d, SPI, DC>, ILI9341Rgb565, NoResetPin>>
    where
        SPI: embedded_hal::spi::SpiDevice,
        DC: embedded_hal::digital::OutputPin,
    {
        rst_pin.set_low().ok();
        delay.delay_millis(20u32);

        rst_pin.set_high().ok();
        delay.delay_millis(200u32);

        if let Ok(b) = Builder::new(ILI9341Rgb565, di)
            .orientation(Orientation::new().flip_horizontal())
            .display_size(MONITOR_HEIGHT as u16, MONITOR_WIDTH as u16)
            .color_order(mipidsi::options::ColorOrder::Bgr)
            .init(delay)
        {
            Some(b)
        } else {
            error!("Could not create Spi Display");
            None
        }
    }

    pub fn fill_monitor<'d, SPI, DC>(
        display: &mut Display<SpiInterface<'d, SPI, DC>, ILI9341Rgb565, NoResetPin>,
        color: Rgb565,
    ) where
        SPI: embedded_hal::spi::SpiDevice,
        DC: embedded_hal::digital::OutputPin,
    {
        let square_height = MONITOR_HEIGHT;
        let square_width = MONITOR_WIDTH;

        let mut color_pixels = vec_into_psram::<Rgb565>(square_width * square_height).unwrap();
        for _ in 0..(square_height * square_width) {
            if let Err(color) = color_pixels.push_within_capacity(color) {
                color_pixels.reserve_exact(1);

                color_pixels.push_within_capacity(color).unwrap();
            };
        }

        if let Err(_) = display.set_pixels(
            0,
            0,
            square_height as u16,
            square_width as u16,
            color_pixels,
        ) {
            error!("Could not draw to monitor");
        }
    }
}
