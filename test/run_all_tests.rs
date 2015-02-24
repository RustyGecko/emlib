#![no_std]
#![no_main]
#![feature(lang_items, start, core, no_std)]

extern crate core;
extern crate emlib;

use emlib::{cmu, gpio, usart};
use core::default::Default;

mod tests {
    pub mod timer;
}

#[no_mangle]
pub extern fn main() {

    setup_leds();
    setup_usart1();

    start();

    tests::timer::run_tests();

    success();
    loop {}
}

fn start() { gpio::pin_out_set(gpio::Port::E, 2); }
fn success() { gpio::pin_out_set(gpio::Port::E, 3); }

fn setup_leds() {

    cmu::clock_enable(cmu::Clock::GPIO, true);

    gpio::pin_mode_set(gpio::Port::E, 2, gpio::Mode::PushPull, 0);
    gpio::pin_mode_set(gpio::Port::E, 3, gpio::Mode::PushPull, 0);
    
}

fn setup_usart1() {
    cmu::clock_select_set(cmu::Clock::HF, cmu::Select::HFRCO);

    cmu::clock_enable(cmu::Clock::GPIO, true);

    let usart1 = usart::Usart::usart1();

    gpio::pin_mode_set(gpio::Port::D, 0, gpio::Mode::PushPull, 1);
    gpio::pin_mode_set(gpio::Port::D, 1, gpio::Mode::Input, 0);

    cmu::clock_enable(cmu::Clock::USART1, true);

    usart1.init_async(&usart::InitAsync {
        baudrate: 9600,
        ..Default::default()
    });

    usart1.ROUTE = usart::ROUTE_RXPEN
        | usart::ROUTE_TXPEN
        | usart::ROUTE_LOCATION_LOC1;

}

#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"]
pub extern fn rust_begin_unwind(_msg: core::fmt::Arguments, _file: &'static str, _line: usize) -> ! {
    loop { }
}
