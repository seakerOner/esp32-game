use esp_hal::{
    Blocking,
    gpio::{Input, InputConfig, interconnect::PeripheralOutput},
    i2c::master::{I2c, Instance},
};
use log::{error, info};

static PCF8574_ADDRESS: u8 = 0x20;

const NUMPAD_UP: u8 = 0b1111_1011;
const NUMPAD_DOWN: u8 = 0b1111_0111;
const NUMPAD_LEFT: u8 = 0b1110_1111;
const NUMPAD_RIGHT: u8 = 0b1101_1111;

const NUMPAD_SELECT: u8 = 0b1111_1101;
const NUMPAD_START: u8 = 0b1111_1110;

const BUTTON_A: u8 = 0b1011_1111;
const BUTTON_B: u8 = 0b0111_1111;

const IDLE: u8 = 0b1111_1111;

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

    pub fn read_inputs(&mut self, buf: &mut [u8]) {
        match self.i2c.read(PCF8574_ADDRESS, buf) {
            Ok(_) => match buf[0] {
                NUMPAD_UP => info!("NUMPAD_UP"),
                NUMPAD_DOWN => info!("NUMPAD_DOWN"),
                NUMPAD_LEFT => info!("NUMPAD_LEFT"),
                NUMPAD_RIGHT => info!("NUMPAD_RIGHT"),
                NUMPAD_START => info!("START"),
                NUMPAD_SELECT => info!("SELECT"),
                BUTTON_A => info!("A"),
                BUTTON_B => info!("B"),
                _ => {}
            },
            Err(e) => error!("No device at {:X}, error: {:}", PCF8574_ADDRESS, e),
        }
        if let Some(i) = self.left_bump.as_mut() {
            if i.is_low() {
                info!("LEFT_BUMP");
            }
        }
        if let Some(i) = self.right_bump.as_mut() {
            if i.is_low() {
                info!("RIGHT_BUMP");
            }
        }
        if let Some(i) = self.menu.as_mut() {
            if i.is_low() {
                info!("MENU");
            }
        }
    }
}
