use emlib;
use emlib::{cmu, timer, prs};
use emlib::modules::{dma, adc};
use emlib::cmsis::nvic;
use core::prelude::*;
use core::intrinsics::transmute;
use core::default::Default;
use libc::c_void;
use alloc::boxed::Box;

pub static mut DATA: [u8; 512] = [0; 512];

use ram_store as store;

pub fn get() -> u32 {

    let adc = emlib::adc::Adc::adc0();
    adc.start(emlib::adc::Start::Single);
    while adc.STATUS & emlib::adc::STATUS_SINGLEACT != 0 {}
    adc.data_single_get()
}

pub fn init() {
    cmu::clock_enable(cmu::Clock::HFPER, true);

    setup_timer();
    setup_dma();
    setup_adc();

}

fn setup_timer() {
    cmu::clock_enable(cmu::Clock::PRS, true);
    cmu::clock_enable(cmu::Clock::TIMER0, true);
    cmu::clock_div_set(cmu::Clock::HF, 512);

    prs::source_signal_set(0, prs::CH_CTRL_SOURCESEL_TIMER0, prs::CH_CTRL_SIGSEL_TIMER0OF, prs::Edge::Pos);

    let timer = timer::Timer::timer0();
    timer.init(&Default::default());

    let freq = cmu::clock_freq_get(cmu::Clock::HFPER);
    let top = freq / 4;
    timer.top_set(top);

    timer.enable(true);

}

fn setup_adc() {
    cmu::clock_enable(cmu::Clock::ADC0, true);

    let adc = emlib::adc::Adc::adc0();
    adc.init(&emlib::adc::Init {
        warm_up_mode: emlib::adc::Warmup::KeepADCWarm,
        timebase: emlib::adc::timebase_calc(0),
        prescale: emlib::adc::prescale_calc(400_000, 0),
        ..Default::default()
    });

    adc.init_single(&emlib::adc::InitSingle {
        prs_sel: emlib::adc::PRSSEL::Ch0,
        prs_enable: true,
        reference: emlib::adc::Ref::Ref1V25,
        input: emlib::adc::SingleInput::Temp,
        resolution: emlib::adc::Res::Res12Bit,
        ..Default::default()
    });

}

fn cb(dma: &mut dma::Dma) {

    dma.refresh().then(cb);
    store::write(unsafe { &DATA });
}

fn setup_dma() {

    dma::init();

    let dma: &mut dma::Dma = unsafe {&mut dma::dma0};

    dma.start_basic(
        &adc::Adc { device: emlib::adc::Adc::adc0() },
        &dma::Buffer { buffer: unsafe {&DATA} },
        dma::Signal::AdcSingle
    ).then(cb);

}
