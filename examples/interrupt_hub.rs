#![no_std]
#![no_main]
#![feature(lang_items, start, alloc, core, no_std, unboxed_closures)]

extern crate core;
extern crate alloc;
extern crate emlib;

use core::default::Default;

use emlib::cmsis::nvic;
use emlib::{cmu, rtc, gpio};
use emlib::modules::irq;
use emlib::modules::irq::{Hub, Event};

use alloc::boxed::Box;

#[no_mangle]
pub extern fn main() {
    let mut hub = Hub::new();
    unsafe { irq::set_interrupt_hub(&mut hub); }

    init();

    run(&mut hub);

    loop {}

}

pub fn init() {
    rtc_setup();
    gpio_setup();
}

pub fn run(hub: &mut Hub) {

    hub.on(Event::RTC, Box::new(|| {
        rtc::int_clear(rtc::RTC_IEN_COMP0);

        gpio::pin_out_toggle(gpio::Port::E, 2);

    }));

}

const LFXO_FREQ: u32 = 32768;
const RTC_TIMEOUT_S: u32 = 2;

fn gpio_setup() {
    cmu::clock_enable(cmu::Clock::GPIO, true);

    gpio::pin_mode_set(gpio::Port::E, 2, gpio::Mode::PushPullDrive, 0);
    gpio::pin_out_clear(gpio::Port::E, 2);
}

fn rtc_setup() {
    let rtc_init = rtc::Init { enable: false, .. Default::default() };

    /* Enable LE domain registers */
    cmu::clock_enable(cmu::Clock::CORELE, true);

    /* Enable LFXO as LFACLK in CMU. This will also start LFXO */
    cmu::clock_select_set(cmu::Clock::LFA, cmu::Select::LFXO);

    /* Enable RTC clock */
    cmu::clock_enable(cmu::Clock::RTC, true);

    rtc::init(&rtc_init);

    /* Interrupt every second */
    rtc::compare_set(0, (LFXO_FREQ * RTC_TIMEOUT_S) / 2);

    /* Enable interrupt */
    nvic::enable_irq(nvic::IRQn::RTC);
    rtc::int_enable(rtc::RTC_IEN_COMP0);

    /* Start Counter */
    rtc::enable(true);
}
