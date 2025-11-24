use alloc::vec::Vec;
use embedded_graphics::pixelcolor::Rgb565;

mod player;
use mipidsi::{Display, NoResetPin, interface::SpiInterface, models::ILI9341Rgb565};
pub use player::Player;

pub trait Mob {
    fn new(texture_map: Vec<Rgb565>) -> Self;

    fn draw<'d, SPI, DC>(
        &self,
        x: u16,
        y: u16,
        display: &mut Display<SpiInterface<'d, SPI, DC>, ILI9341Rgb565, NoResetPin>,
    ) where
        SPI: embedded_hal::spi::SpiDevice,
        DC: embedded_hal::digital::OutputPin;
}
