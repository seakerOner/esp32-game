use alloc::vec::Vec;
use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::{RgbColor, WebColors},
};
use log::error;

use crate::{
    inputs::{
        NUMPAD_BUTTON_A, NUMPAD_BUTTON_B, NUMPAD_DOWN, NUMPAD_IDLE, NUMPAD_LEFT, NUMPAD_RIGHT,
        NUMPAD_SELECT, NUMPAD_START, NUMPAD_UP,
    },
    utils::{vec_into_iram, vec_into_psram},
};

use super::Mob;

pub struct Player {
    state: PlayerState,
    direction: Direction,
    hp: u8,
    max_hp: u8,
    velocity: u8,
    pos: PlayerPos,
    texture_map: Vec<Rgb565>,
    width: u16,
}

#[derive(Clone, Copy)]
pub struct PlayerPos {
    x: Option<u16>,
    y: Option<u16>,
}

pub enum PlayerState {
    Idle,
    Moving,
}

pub enum Direction {
    None,
    Up,
    Left,
    Right,
    Down,
}

impl Mob for Player {
    fn new(texture_map: Vec<Rgb565>) -> Self {
        Player {
            state: PlayerState::Idle,
            direction: Direction::None,
            hp: 100,
            max_hp: 100,
            velocity: 0,
            pos: PlayerPos { x: None, y: None },
            texture_map,
            width: 32,
        }
    }

    fn draw<'d, SPI, DC>(
        &mut self,
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
        self.pos.x.replace(x);
        self.pos.y.replace(y);
    }

    fn update_state<'d, SPI, DC>(
        &mut self,
        input: (u8, alloc::string::String),
        display: &mut mipidsi::Display<
            mipidsi::interface::SpiInterface<'d, SPI, DC>,
            mipidsi::models::ILI9341Rgb565,
            mipidsi::NoResetPin,
        >,
    ) where
        SPI: embedded_hal::spi::SpiDevice,
        DC: embedded_hal::digital::OutputPin,
    {
        if input.0 != NUMPAD_IDLE
            || input.0 != NUMPAD_SELECT
            || input.0 != NUMPAD_START
            || input.0 != NUMPAD_BUTTON_A
            || input.0 != NUMPAD_BUTTON_B
        {
            self.state = PlayerState::Moving;
        }
        if input.0 == NUMPAD_IDLE {
            self.state = PlayerState::Idle;
        }

        match self.state {
            PlayerState::Idle => {
                self.velocity = 0;
            }
            PlayerState::Moving => {
                self.velocity = 1;
                let old_pos = self.pos.clone();

                match input.0 {
                    NUMPAD_UP => {
                        self.direction = Direction::Up;
                        self.pos
                            .x
                            .replace(self.pos.x.unwrap() + self.velocity as u16);
                    }
                    NUMPAD_DOWN => {
                        self.direction = Direction::Down;
                        self.pos
                            .x
                            .replace(self.pos.x.unwrap() - self.velocity as u16);
                    }
                    NUMPAD_LEFT => {
                        self.direction = Direction::Left;
                        self.pos
                            .y
                            .replace(self.pos.y.unwrap() - self.velocity as u16);
                    }
                    NUMPAD_RIGHT => {
                        self.direction = Direction::Right;
                        self.pos
                            .y
                            .replace(self.pos.y.unwrap() + self.velocity as u16);
                    }
                    _ => {}
                }

                self.draw_and_clean_dirty_pixels(
                    old_pos,
                    self.pos.x.unwrap() as u16,
                    self.pos.y.unwrap() as u16,
                    display,
                );
            }
        }
    }

    fn draw_and_clean_dirty_pixels<'d, SPI, DC>(
        &mut self,
        old_pos: PlayerPos,
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

        let mut colors_to_replace_dirty_pixels =
            vec_into_psram::<Rgb565>(self.width as usize).unwrap();
        for _ in 0..self.width {
            if let Err(color) =
                colors_to_replace_dirty_pixels.push_within_capacity(Rgb565::CSS_LIGHT_GREEN)
            {
                colors_to_replace_dirty_pixels.reserve_exact(1);

                colors_to_replace_dirty_pixels
                    .push_within_capacity(color)
                    .unwrap();
            }
        }

        match self.direction {
            Direction::Up => {
                if let Err(_) = display.set_pixels(
                    old_pos.x.unwrap(),
                    old_pos.y.unwrap() - offset,
                    old_pos.x.unwrap() - offset,
                    old_pos.y.unwrap() + self.width,
                    colors_to_replace_dirty_pixels,
                ) {
                    error!("Could clean dirty pixels (UP)");
                }
            }
            Direction::Left => {
                if let Err(_) = display.set_pixels(
                    old_pos.x.unwrap() - offset,
                    old_pos.y.unwrap() + self.width - offset,
                    old_pos.x.unwrap() + offset,
                    old_pos.y.unwrap() + self.width - offset,
                    colors_to_replace_dirty_pixels,
                ) {
                    error!("Could clean dirty pixels (LEFT)");
                }
            }
            Direction::Right => {
                if let Err(_) = display.set_pixels(
                    old_pos.x.unwrap() - offset,
                    old_pos.y.unwrap() - offset,
                    old_pos.x.unwrap() + self.width,
                    old_pos.y.unwrap(),
                    colors_to_replace_dirty_pixels,
                ) {
                    error!("Could clean dirty pixels (RIGHT)");
                }
            }
            Direction::Down => {
                if let Err(_) = display.set_pixels(
                    old_pos.x.unwrap() + self.width - offset,
                    old_pos.y.unwrap() - offset,
                    old_pos.x.unwrap() + self.width - offset,
                    old_pos.y.unwrap() + self.width - offset,
                    colors_to_replace_dirty_pixels,
                ) {
                    error!("Could clean dirty pixels (DOWN)");
                }
            }
            Direction::None => {}
        }

        self.pos.x.replace(x);
        self.pos.y.replace(y);
    }
}
