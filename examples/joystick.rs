#![no_std]
#![no_main]
#![feature(lang_items, start, core, no_std)]

extern crate core;
extern crate emlib;

use emlib::{chip, cmu, adc};
use core::default::Default;

#[no_mangle]
pub extern fn main() {


    chip::init();

    setup_adc();



    loop {
        let adc0 = adc::Adc::adc0();
        adc0.start(adc::Start::Single);

        let sample = adc0.data_single_get();
        let voltage = (sample * 1250 * 3) / 4096;

    }
}

fn setup_adc() {

    cmu::clock_enable(cmu::Clock::HFPER, true);
    cmu::clock_enable(cmu::Clock::ADC0, true);

    let adc0 = adc::Adc::adc0();
    adc0.init(&adc::Init {
        timebase: adc::timebase_calc(0),
        prescale: adc::prescale_calc(7_000_000, 0),
        ..Default::default()
    });

    adc0.init_single(&adc::InitSingle {
        reference: adc::Ref::Ref1V25,
        input: adc::SingleInput::VDDDiv3,
        resolution: adc::Res::Res12Bit,
        acq_time: adc::AcqTime::Time32,
        ..Default::default()
    });

}
