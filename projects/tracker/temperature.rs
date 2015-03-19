use emlib::{cmu, adc, timer, prs, dma};
use emlib::cmsis::nvic;
use core::prelude::*;
use core::intrinsics::transmute;
use core::default::Default;
use core::ptr;

static mut DATA: [u8; 512] = [0; 512];

use circular_buffer;
use ram_store as store;

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

pub fn init() {
    cmu::clock_enable(cmu::Clock::HFPER, true);

    setup_timer();
    setup_dma();
    setup_adc();

}

extern fn transfer_complete(_channel: u32, primary: bool, _user: u32) {

    let dma = dma::DMA::channel0();

    dma.activate_basic::<u8>(
        true,
        false,
        unsafe { transmute(ptr::null::<u8>()) },
        unsafe { transmute(ptr::null::<u8>()) },
        512 - 1
    );

    store::write(unsafe { &DATA });

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

    timer.int_enable(timer::TIMER_IF_OF);
    nvic::enable_irq(nvic::IRQn::TIMER0);

    timer.enable(true);

}

fn setup_adc() {
    cmu::clock_enable(cmu::Clock::ADC0, true);

    let adc = adc::Adc::adc0();
    adc.init(&adc::Init {
        warm_up_mode: adc::Warmup::KeepADCWarm,
        timebase: adc::timebase_calc(0),
        prescale: adc::prescale_calc(400_000, 0),
        ..Default::default()
    });

    adc.init_single(&adc::InitSingle {
        prs_sel: adc::PRSSEL::Ch0,
        prs_enable: true,
        reference: adc::Ref::Ref1V25,
        input: adc::SingleInput::Temp,
        resolution: adc::Res::Res12Bit,
        ..Default::default()
    });

    adc.int_enable(adc::IEN_SINGLE);
    nvic::enable_irq(nvic::IRQn::ADC0);
}

fn setup_dma() {
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
        size: dma::DataSize::Size1,
        arb_rate: dma::ArbiterConfig::Arbitrate1,
        hprot: 0
    });

    let adc = adc::Adc::adc0();
    dma.activate_basic::<u8>(
        true,
        false,
        unsafe { transmute((&DATA).as_ptr()) },
        unsafe { transmute(&adc.SINGLEDATA) },
        512 - 1
    );

}
