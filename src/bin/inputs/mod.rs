use alloc::{string::String, vec::Vec};
use esp_hal::{
    Blocking,
    gpio::{Input, InputConfig, interconnect::PeripheralOutput},
    i2c::master::{I2c, Instance},
};
use log::{error, info};

use crate::utils::vec_into_iram;

static PCF8574_ADDRESS: u8 = 0x20;

pub const NUMPAD_UP: u8 = 0b1111_1011;
pub const NUMPAD_DOWN: u8 = 0b1111_0111;
pub const NUMPAD_LEFT: u8 = 0b1110_1111;
pub const NUMPAD_RIGHT: u8 = 0b1101_1111;

pub const NUMPAD_SELECT: u8 = 0b1111_1101;
pub const NUMPAD_START: u8 = 0b1111_1110;

pub const NUMPAD_BUTTON_A: u8 = 0b1011_1111;
pub const NUMPAD_BUTTON_B: u8 = 0b0111_1111;

pub const NUMPAD_IDLE: u8 = 0b1111_1111;

pub struct I2cInputs<'a> {
    i2c: I2c<'a, Blocking>,
    left_bump: Option<Input<'a>>,
    right_bump: Option<Input<'a>>,
    menu: Option<Input<'a>>,
}

impl<'a> I2cInputs<'a> {
    pub fn new<'d: 'a>(
        i2c: impl Instance + 'd,
        sda: impl PeripheralOutput<'d>,
        scl: impl PeripheralOutput<'d>,
    ) -> Self {
        I2cInputs {
            i2c: {
                I2c::new(i2c, esp_hal::i2c::master::Config::default())
                    .unwrap_or_else(|e| {
                        error!("IC2 instance failed to create; ERROR: {}", e);
                        panic!();
                    })
                    .with_sda(sda)
                    .with_scl(scl)
            },
            left_bump: None,
            right_bump: None,
            menu: None,
        }
    }

    pub fn with_ext_inputs<'d: 'a>(
        mut self,
        left_bump: Input<'d>,
        right_bump: Input<'d>,
        menu: Input<'a>,
    ) -> Self {
        self.left_bump = Some(left_bump);
        self.right_bump = Some(right_bump);
        self.menu = Some(menu);
        self
    }

    pub fn read_inputs(&mut self, buf: &mut [u8]) -> (u8, String) {
        let mut i2c_input: u8 = 0;
        let mut ext_input: String = String::new();

        match self.i2c.read(PCF8574_ADDRESS, buf) {
            Ok(_) => match buf[0] {
                NUMPAD_UP => {
                    //info!("NUMPAD_UP");
                    i2c_input = NUMPAD_UP;
                }
                NUMPAD_DOWN => {
                    //info!("NUMPAD_DOWN");
                    i2c_input = NUMPAD_DOWN;
                }
                NUMPAD_LEFT => {
                    //info!("NUMPAD_LEFT");
                    i2c_input = NUMPAD_LEFT;
                }
                NUMPAD_RIGHT => {
                    //info!("NUMPAD_RIGHT");
                    i2c_input = NUMPAD_RIGHT;
                }
                NUMPAD_START => {
                    //info!("START");
                    i2c_input = NUMPAD_START;
                }
                NUMPAD_SELECT => {
                    //info!("SELECT");
                    i2c_input = NUMPAD_SELECT;
                }
                NUMPAD_BUTTON_A => {
                    //info!("A");
                    i2c_input = NUMPAD_BUTTON_A;
                }
                NUMPAD_BUTTON_B => {
                    //info!("B");
                    i2c_input = NUMPAD_BUTTON_B;
                }
                NUMPAD_IDLE => {
                    //info!("IDLE");
                    i2c_input = NUMPAD_IDLE;
                }
                _ => {}
            },
            Err(e) => error!("No device at {:X}, error: {:}", PCF8574_ADDRESS, e),
        }
        if let Some(i) = self.left_bump.as_mut() {
            if i.is_low() {
                //info!("LEFT_BUMP");
                ext_input = String::from("LEFT_BUMP");
            }
        }
        if let Some(i) = self.right_bump.as_mut() {
            if i.is_low() {
                //info!("RIGHT_BUMP");
                ext_input = String::from("RIGHT_BUMP");
            }
        }
        if let Some(i) = self.menu.as_mut() {
            if i.is_low() {
                //info!("MENU");
                ext_input = String::from("BUMP");
            }
        }

        (i2c_input, ext_input)
    }
}
