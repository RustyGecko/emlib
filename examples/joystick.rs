#![no_std]
#![feature(lang_items, start, core, no_std)]

extern crate core;
extern crate emlib;

use emlib::{chip, cmu, adc};
use core::default::Default;

fn main() {

    chip::init();

    setup_adc();

    let adc_y = adc::Adc::adc0();

    loop {

        let _y = get_sample(adc_y);

    }
}

fn get_sample(adc: &adc::Adc) -> u32 {

    adc.start(adc::Start::Single);
    while adc.STATUS & adc::STATUS_SINGLEACT != 0 {}

    adc.data_single_get()
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
        reference: adc::Ref::RefVDD,
        input: adc::SingleInput::Ch0,
        resolution: adc::Res::Res12Bit,
        acq_time: adc::AcqTime::Time32,
        ..Default::default()
    });

}
