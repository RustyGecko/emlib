use emlib;
use emlib::{cmu, adc};
use core::prelude::*;
use core::default::Default;

use ram_store as store;
use circular_buffer::CircularBuffer4;

static mut BUFFER: CircularBuffer4<u8> = CircularBuffer4 {
    tail_index: 0,
    head_index: 0,
    data: [0; 4]
};

pub fn init() {
    setup_adc();
}

fn setup_adc() {
    cmu::clock_enable(cmu::Clock::ADC0, true);

    let adc = adc::Adc::adc0();
    adc.init(&adc::Init {
        ..Default::default()
    });

    adc.init_single(&adc::InitSingle {
        reference: adc::Ref::Ref2V5,
        input: adc::SingleInput::Temp,
        resolution: adc::Res::Res8Bit,
        ..Default::default()
    });
}

pub fn perform_measurement() {

    let adc = adc::Adc::adc0();
    adc.start(adc::Start::Single);

    while adc.STATUS & adc::STATUS_SINGLEACT != 0 {}

    let data = adc.data_single_get();


    match unsafe { BUFFER.push(data as u8) } {
        Err(msg) => panic!("{}", msg),
        Ok(()) => (),
    }
}

pub fn pop() -> Result<u8, &'static str> {
    unsafe { BUFFER.pop() }
}
