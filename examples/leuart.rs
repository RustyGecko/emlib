#![no_std]
#![no_main]
#![feature(core, no_std)]

extern crate core;
extern crate emlib;
extern crate cmsis;

use emlib::{chip, cmu, emu, gpio, leuart};
use cmsis::nvic;
use core::default::Default;

#[no_mangle]
pub extern fn main() {

    setup_leuart();

    loop {
        emu::enter_em2(true);
    }
}

#[no_mangle]
#[allow(dead_code, non_snake_case)]
pub extern fn LEUART1_IRQHandler() {

    let uart = leuart::Leuart::leuart1();

    let luartif = uart.int_get();
    uart.int_clear(luartif);

    if luartif & leuart::IF_RXDATAV != 0 {
        uart.tx(uart.rx());
    }
}

fn setup_leuart() {
    chip::init();

    cmu::clock_select_set(cmu::Clock::LFA, cmu::Select::LFXO);
    cmu::clock_select_set(cmu::Clock::LFB, cmu::Select::LFXO);

    cmu::clock_enable(cmu::Clock::CORELE, true);
    cmu::clock_enable(cmu::Clock::GPIO, true);

    gpio::pin_mode_set(gpio::Port::C, 6, gpio::Mode::PushPull, 1);
    gpio::pin_mode_set(gpio::Port::C, 7, gpio::Mode::InputPull, 1);

    cmu::clock_enable(cmu::Clock::LEUART1, true);

    let uart = leuart::Leuart::leuart1();
    uart.reset();
    uart.init(&leuart::Init {
        enable: leuart::Enable::Enable,
        ..Default::default()
    });
    uart.ROUTE = leuart::ROUTE_RXPEN
        | leuart::ROUTE_TXPEN
        | leuart::ROUTE_LOCATION_LOC0;

    uart.int_enable(leuart::IEN_RXDATAV);
    nvic::enable_irq(nvic::IRQn::LEUART1);

}
