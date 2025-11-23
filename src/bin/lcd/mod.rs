use embedded_hal::digital::OutputPin;
use esp_hal::delay::Delay;
use log::error;
use mipidsi::interface::SpiInterface;
use mipidsi::options::Orientation;
use mipidsi::{Builder, models::ILI9341Rgb565};
use mipidsi::{Display, NoResetPin};

pub struct LcdMonitor;

impl LcdMonitor {
    pub fn init_display<'d, SPI, DC>(
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
            .display_size(240, 320)
            .color_order(mipidsi::options::ColorOrder::Bgr)
            .init(delay)
        {
            Some(b)
        } else {
            error!("Could not create Spi Display");
            None
        }
    }
}
