#![no_std]
#![no_main]
#![feature(core, no_std)]

extern crate core;
extern crate emlib;
extern crate cmsis;

use emlib::{ chip, cmu, timer, gpio };
use cmsis::nvic;
use core::default::Default;

const TOP: u32 = 27342;

#[no_mangle]
#[allow(non_snake_case)]
pub extern fn TIMER0_IRQHandler() {
    let timer0 = timer::Timer::timer0();
    timer0.int_clear(timer::TIMER_IF_OF);

    gpio::pin_out_toggle(gpio::Port::E, 2);
}

#[no_mangle]
pub extern fn main() {
    chip::init();

    cmu::clock_enable(cmu::Clock::HFPER, true);
    cmu::clock_enable(cmu::Clock::GPIO, true);
    cmu::clock_enable(cmu::Clock::TIMER0, true);

    gpio::pin_mode_set(gpio::Port::E, 2, gpio::Mode::PushPullDrive, 0);
    gpio::pin_out_clear(gpio::Port::E, 2);

    let timer_init = timer::Init {
        debug_run: true,
        prescale: timer::Prescale::Prescale1024,
        ..Default::default()
    };

    let timer0 = timer::Timer::timer0();

    timer0.int_enable(timer::TIMER_IF_OF);
    nvic::enable_irq(nvic::IRQn::TIMER0);
    timer0.top_set(TOP);
    timer0.init(&timer_init);

    loop {}
}
