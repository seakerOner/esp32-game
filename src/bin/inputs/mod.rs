use esp_hal::{
    Blocking,
    gpio::interconnect::PeripheralOutput,
    i2c::master::{I2c, Instance},
    peripherals::Peripherals,
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

pub struct Inputs<'a> {
    i2c: I2c<'a, Blocking>,
}

impl<'a> Inputs<'a> {
    pub fn new<'d: 'a>(
        i2c: impl Instance + 'd,
        sda: impl PeripheralOutput<'d>,
        scl: impl PeripheralOutput<'d>,
    ) -> Self {
        Inputs {
            i2c: {
                I2c::new(i2c, esp_hal::i2c::master::Config::default())
                    .unwrap_or_else(|e| {
                        error!("IC2 instance failed to create; ERROR: {}", e);
                        panic!();
                    })
                    .with_sda(sda)
                    .with_scl(scl)
            },
        }
    }

    pub fn read_inputs(mut self, buf: &mut [u8]) {
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
    }
}
