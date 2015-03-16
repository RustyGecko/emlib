use emlib::{cmu, adc, timer, prs, dma};
use emlib::cmsis::nvic;
use core::prelude::*;
use core::intrinsics::transmute;
use core::default::Default;
use core::ptr;

use circular_buffer;
use ram_store as store;

const TOP: u32 = 27342;


static CB: dma::CB = dma::CB {
    cb_func: transfer_complete,
    user_ptr: 0,
    primary: 0
};

pub fn get() -> u32 {

    let adc = adc::Adc::adc0();
    adc.start(adc::Start::Single);
    while adc.STATUS & adc::STATUS_SINGLEACT != 0 {}
    adc.data_single_get()
}

pub fn init(buffer: &'static mut [u8]) {
    cmu::clock_enable(cmu::Clock::HFPER, true);

    setup_timer();
    setup_adc();
    setup_dma(buffer);

}

extern fn transfer_complete(channel: u32, _primary: bool, _user: u32) {

    let dma = dma::DMA::channel0();
    dma.activate_basic::<u8>(
        true,
        false,
        unsafe { transmute(ptr::null::<u8>()) },
        unsafe { transmute(ptr::null::<u8>()) },
        1024 - 1
    );

    store::write(circular_buffer::get());

}

fn setup_timer() {
    cmu::clock_enable(cmu::Clock::PRS, true);
    cmu::clock_enable(cmu::Clock::TIMER0, true);

    let timer = timer::Timer::timer0();
    timer.top_set(TOP);
    timer.init(&timer::Init {
        debug_run: true,
        prescale: timer::Prescale::Prescale1024,
        ..Default::default()
    });

    prs::source_signal_set(0, prs::CH_CTRL_SOURCESEL_TIMER0, prs::CH_CTRL_SIGSEL_TIMER0OF, prs::Edge::Pos);

}

fn setup_adc() {
    cmu::clock_enable(cmu::Clock::ADC0, true);

    let adc = adc::Adc::adc0();
    adc.init(&adc::Init {
        timebase: adc::timebase_calc(0),
        prescale: adc::prescale_calc(7_000_000, 0),
        ..Default::default()
    });

    adc.init_single(&adc::InitSingle {
        prs_sel: adc::PRSSEL::Ch0,
        prs_enable: true,
        reference: adc::Ref::Ref1V25,
        input: adc::SingleInput::Temp,
        resolution: adc::Res::Res12Bit,
        acq_time: adc::AcqTime::Time32,
        ..Default::default()
    });

    adc.IEN = adc::IEN_SINGLE;

    nvic::enable_irq(nvic::IRQn::ADC0);

}

fn setup_dma(buffer: &'static mut [u8]) {

    dma::init(&dma::Init {
        hprot: 0,
        control_block: dma::dma_control_block(),
    });

    let dma = dma::DMA::channel0();
    dma.configure_channel(&dma::CfgChannel {
        high_pri: true,
        enable_int: true,
        select: dma::REQ_ADC0_SINGLE,
        cb: &CB
    });

    dma.configure_descriptor(true, &dma::CfgDescriptor {
        dst_inc: dma::DataInc::Inc1,
        src_inc: dma::DataInc::IncNone,
        size: dma::DataSize::Size2,
        arb_rate: dma::ArbiterConfig::Arbitrate1,
        hprot: 0
    });

    let adc = adc::Adc::adc0();
    dma.activate_basic::<u8>(
        true,
        false,
        unsafe { transmute(buffer.as_ptr()) },
        unsafe { transmute(&adc.SCANDATA) },
        512 - 1
    );

}
