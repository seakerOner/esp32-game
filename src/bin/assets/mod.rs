use alloc::string::String;
use alloc::vec::Vec;
use embedded_graphics::pixelcolor::Rgb565;

mod player;
use mipidsi::{Display, NoResetPin, interface::SpiInterface, models::ILI9341Rgb565};
pub use player::Player;

use crate::assets::player::PlayerPos;

pub trait Mob {
    fn new(texture_map: Vec<Rgb565>) -> Self;

    fn draw<'d, SPI, DC>(
        &mut self,
        x: u16,
        y: u16,
        display: &mut Display<SpiInterface<'d, SPI, DC>, ILI9341Rgb565, NoResetPin>,
    ) where
        SPI: embedded_hal::spi::SpiDevice,
        DC: embedded_hal::digital::OutputPin;

    fn update_state<'d, SPI, DC>(
        &mut self,
        input: (u8, String),
        display: &mut Display<SpiInterface<'d, SPI, DC>, ILI9341Rgb565, NoResetPin>,
    ) where
        SPI: embedded_hal::spi::SpiDevice,
        DC: embedded_hal::digital::OutputPin;

    fn draw_and_clean_dirty_pixels<'d, SPI, DC>(
        &mut self,
        old_pos: PlayerPos,
        x: u16,
        y: u16,
        display: &mut Display<SpiInterface<'d, SPI, DC>, ILI9341Rgb565, NoResetPin>,
    ) where
        SPI: embedded_hal::spi::SpiDevice,
        DC: embedded_hal::digital::OutputPin;
}
