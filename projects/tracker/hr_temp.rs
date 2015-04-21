use core::default::Default;
use core::intrinsics::transmute;
use core::prelude::*;

use emlib::emdrv;
use emlib::sensors::si7013;
use emlib::i2c;

use circular_buffer::CircularBuffer4;
use buffer::FixedSizeBuffer128;
use ram_store as store;

use sensor::Sensor;

pub struct HumidityRelativeAndTemperatureSensor {
    i2c: &'static i2c::I2C
}

impl HumidityRelativeAndTemperatureSensor {

    pub fn new(i2c: &'static i2c::I2C) -> HumidityRelativeAndTemperatureSensor {
        HumidityRelativeAndTemperatureSensor {
            i2c: i2c
        }
    }

    pub fn init(&self) {
        emdrv::i2c::init(&Default::default());
    }

    pub fn detect(&self) -> bool {
        si7013::detect(self.i2c, si7013::ADDR_0)
    }

}

impl Sensor<(u32, i32)> for HumidityRelativeAndTemperatureSensor {

    fn measure(&mut self) -> (u32, i32) {

        let mut relative_humidity: u32 = 0;
        let mut temperature: i32 = 0;
        si7013::measure_rh_and_temp(self.i2c, si7013::ADDR_0, &mut relative_humidity, &mut temperature);

        (relative_humidity, temperature)
    }

}
