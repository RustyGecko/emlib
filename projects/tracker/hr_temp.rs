use core::default::Default;
use core::intrinsics::transmute;

use emlib::emdrv;
use emlib::sensors::si7013;
use emlib::i2c;

use buffer::FixedSizeBuffer128;
use ram_store as store;

static mut RELATIVE_HUMIDITY_BUFFER: FixedSizeBuffer128<u32> = FixedSizeBuffer128 {
    index: 0,
    data: [0; 128]
};

static mut TEMPERATURE_BUFFER: FixedSizeBuffer128<i32> = FixedSizeBuffer128 {
    index: 0,
    data: [0; 128]
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

    unsafe {
        if RELATIVE_HUMIDITY_BUFFER.push(relative_humidity) {
            store::write(store::Kind::HumidityRelative, &RELATIVE_HUMIDITY_BUFFER.data);
        }
    }

    unsafe {
        if TEMPERATURE_BUFFER.push(temperature) {
            store::write(store::Kind::Temperature, &TEMPERATURE_BUFFER.data);
        }
    }

}
