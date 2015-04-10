use emlib;
use emlib::{cmu, timer, prs, rtc};
use emlib::cmsis::nvic;
use emlib::modules::{dma, adc};
use core::prelude::*;
use core::default::Default;

use ram_store as store;
use circular_buffer::CircularBuffer4;

static mut BUFFER: CircularBuffer4<u8> = CircularBuffer4 {
    tail_index: 0,
    head_index: 0,
    data: [0; 4]
};

pub fn init(interval: u32, force_dma: bool) {
    cmu::clock_enable(cmu::Clock::HFPER, true);

    if force_dma {
        setup_timer(interval);
        setup_adc();
    } else {
        setup_rtc(interval);
        setup_adc();
    }
}

fn setup_timer(interval: u32) {
    cmu::clock_enable(cmu::Clock::PRS, true);
    cmu::clock_enable(cmu::Clock::TIMER0, true);
    cmu::clock_div_set(cmu::Clock::HF, 512);

    prs::source_signal_set(0, prs::CH_CTRL_SOURCESEL_TIMER0, prs::CH_CTRL_SIGSEL_TIMER0OF, prs::Edge::Pos);

    let timer = timer::Timer::timer0();
    timer.init(&timer::Init {
        enable: false,
        ..Default::default()
    });
    let freq = cmu::clock_freq_get(cmu::Clock::HFPER);
    let top = (freq / 1000) * interval;
    timer.top_set(top);

    timer.enable(true);
}

fn setup_rtc(interval: u32) {

    const LFXO_FREQ: u32 = 32768;
    const RTC_TIMEOUT_S: u32 = 2;

    cmu::clock_enable(cmu::Clock::CORELE, true);
    cmu::clock_enable(cmu::Clock::RTC, true);
    cmu::clock_select_set(cmu::Clock::LFA, cmu::Select::LFXO);

    rtc::init(&rtc::Init {
        enable: false,
        ..Default::default()
    });

    let freq = (LFXO_FREQ * RTC_TIMEOUT_S) / 2;
    rtc::compare_set(0, (freq / 1000) * interval);

    nvic::enable_irq(nvic::IRQn::RTC);
    rtc::int_enable(rtc::RTC_IEN_COMP0);

    rtc::enable(true);

}

fn setup_adc() {
    cmu::clock_enable(cmu::Clock::ADC0, true);

    let adc = emlib::adc::Adc::adc0();
    adc.init(&emlib::adc::Init {
        ..Default::default()
    });

    adc.init_single(&emlib::adc::InitSingle {
        prs_sel: emlib::adc::PRSSEL::Ch0,
        prs_enable: true,
        reference: emlib::adc::Ref::Ref2V5,
        input: emlib::adc::SingleInput::Temp,
        resolution: emlib::adc::Res::Res8Bit,
        ..Default::default()
    });
}

pub fn on_rtc() {

    let adc = emlib::adc::Adc::adc0();
    adc.start(emlib::adc::Start::Single);

    while adc.STATUS & emlib::adc::STATUS_SINGLEACT != 0 {}

    let data = adc.data_single_get();


    match unsafe { BUFFER.push(data as u8) } {
        Err(msg) => panic!("{}", msg),
        Ok(()) => (),
    }
}

pub fn pop() -> Result<u8, &'static str> {
    unsafe { BUFFER.pop() }
}
/*
fn cb(dma: &mut dma::Dma) {

    dma.refresh().then(cb);
    store::write(0, unsafe { &BUFFER.data });
}
*/
fn setup_dma() {

    /*dma::init();

    let dma: &mut dma::Dma = unsafe { &mut dma::dma0 };

    dma.start_basic(
        &adc::Adc { device: emlib::adc::Adc::adc0() },
        &dma::Buffer { buffer: unsafe {&BUFFER.data} },
        dma::Signal::AdcSingle
    ).then(cb);*/

}
