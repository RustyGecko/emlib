use core::default::Default;
use core::intrinsics::transmute;
use core::prelude::*;

use emlib::emdrv;
use emlib::sensors::si7013;
use emlib::i2c;

use circular_buffer::CircularBuffer4;
use buffer::FixedSizeBuffer128;
use ram_store as store;

static mut RELATIVE_HUMIDITY_BUFFER: CircularBuffer4<u32> = CircularBuffer4 {
    tail_index: 0,
    head_index: 0,
    data: [0; 4]
};

static mut TEMPERATURE_BUFFER: CircularBuffer4<i32> = CircularBuffer4 {
    tail_index: 0,
    head_index: 0,
    data: [0; 4]
};

//let sensor = si7013::init(i2c::I2C::i2c1(), si7013::ADDR_0);
//let status = sensor.detect();
//
//let (relative_humidity, temperature) = sensor.measure();

pub fn init() {
    let i2c = i2c::I2C::i2c1();
    emdrv::i2c::init(&Default::default());
    let status = si7013::detect(i2c, si7013::ADDR_0);
}

pub fn measure() -> (u32, i32) {
    let i2c = i2c::I2C::i2c1();
    let mut relative_humidity: u32 = 0;
    let mut temperature: i32 = 0;
    si7013::measure_rh_and_temp(i2c, si7013::ADDR_0, &mut relative_humidity, &mut temperature);

    (relative_humidity, temperature)
}

pub fn on_rtc() {

    let (relative_humidity, temperature) = measure();

    unsafe { RELATIVE_HUMIDITY_BUFFER.push(relative_humidity); }
    unsafe { TEMPERATURE_BUFFER.push(temperature); }
}

pub fn pop_hr() -> Result<u32, &'static str> {
    unsafe { RELATIVE_HUMIDITY_BUFFER.pop() }
}

pub fn pop_temp() -> Result<i32, &'static str> {
    unsafe { TEMPERATURE_BUFFER.pop() }
}
