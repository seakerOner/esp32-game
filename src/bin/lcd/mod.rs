use embedded_graphics::{pixelcolor::Rgb666, prelude::*};
use embedded_hal::digital::OutputPin;
use embedded_hal::spi::ErrorKind;
use esp_hal::delay::Delay;
use esp_hal::gpio::Output;
use log::error;
use mipidsi::interface::{Interface, SpiInterface};
use mipidsi::options::Orientation;
use mipidsi::{Builder, models::ILI9341Rgb666};
use mipidsi::{Display, NoResetPin};

pub struct LcdMonitor;

impl LcdMonitor {
    pub fn init_display<'d, SPI, DC>(
        di: SpiInterface<'d, SPI, DC>,
        delay: &mut Delay,
        rst_pin: &mut impl OutputPin,
    ) -> Option<Display<SpiInterface<'d, SPI, DC>, ILI9341Rgb666, NoResetPin>>
    where
        SPI: embedded_hal::spi::SpiDevice,
        DC: embedded_hal::digital::OutputPin,
    {
        rst_pin.set_low().ok();
        delay.delay_millis(20);

        rst_pin.set_high().ok();
        delay.delay_millis(20);

        if let Ok(b) = Builder::new(ILI9341Rgb666, di)
            .orientation(Orientation::new().flip_horizontal())
            .display_size(240, 320)
            .color_order(mipidsi::options::ColorOrder::Rgb)
            .init(delay)
        {
            Some(b)
        } else {
            error!("Could not create Spi Display");
            None
        }
    }
}

