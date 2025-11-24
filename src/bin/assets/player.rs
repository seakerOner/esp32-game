use alloc::vec::Vec;
use embedded_graphics::pixelcolor::Rgb565;
use log::error;

use super::Mob;

pub struct Player {
    hp: u8,
    max_hp: u8,
    velocity: u8,
    texture_map: Vec<Rgb565>,
    width: u16,
}

impl Mob for Player {
    fn new(texture_map: Vec<Rgb565>) -> Self {
        Player {
            hp: 100,
            max_hp: 100,
            velocity: 0,
            texture_map,
            width: 32,
        }
    }

    fn draw<'d, SPI, DC>(
        &self,
        x: u16,
        y: u16,
        display: &mut mipidsi::Display<
            mipidsi::interface::SpiInterface<'d, SPI, DC>,
            mipidsi::models::ILI9341Rgb565,
            mipidsi::NoResetPin,
        >,
    ) where
        SPI: embedded_hal::spi::SpiDevice,
        DC: embedded_hal::digital::OutputPin,
    {
        let offset: u16 = self.width / 2;
        if let Err(_) = display.set_pixels(
            x - offset,
            y - offset,
            (x + self.width) - offset,
            (y + self.width) - offset,
            self.texture_map.clone(),
        ) {
            error!("Could not draw player");
        }
    }
}
