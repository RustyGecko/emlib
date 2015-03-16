use emlib::{cmu, adc, timer, prs};
use emlib::cmsis::nvic;
use core::default::Default;

const TOP: u32 = 64; // 27342;

pub fn get() -> u32 {

    let adc = adc::Adc::adc0();
    adc.start(adc::Start::Single);
    while adc.STATUS & adc::STATUS_SINGLEACT != 0 {}
    adc.data_single_get()
}

pub fn init() {

    cmu::clock_enable(cmu::Clock::HFPER, true);
    cmu::clock_enable(cmu::Clock::ADC0, true);
    cmu::clock_enable(cmu::Clock::PRS, true);
    cmu::clock_enable(cmu::Clock::TIMER0, true);

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

    let timer = timer::Timer::timer0();
    timer.top_set(TOP);
    timer.init(&timer::Init {
        debug_run: true,
        prescale: timer::Prescale::Prescale1024,
        ..Default::default()
    });

    prs::source_signal_set(0, prs::CH_CTRL_SOURCESEL_TIMER0, prs::CH_CTRL_SIGSEL_TIMER0OF, prs::Edge::Pos);

}
