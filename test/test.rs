#![no_std]
#![no_main]
#![feature(lang_items, start, core, no_std)]

extern crate core;
extern crate emlib;

use emlib::{cmu, gpio};

mod timer;

mod led_test {

    use emlib::gpio;

    pub fn success() {
        assert(true);
    }

    pub fn fail() {
        assert(false);
    }

    pub fn refute(result: bool) {
        if !result {
            gpio::pin_out_set(gpio::Port::E, 2);
        } else {
            gpio::pin_out_set(gpio::Port::E, 3);
        }
    }

    pub fn assert(result: bool) {
        if result {
            gpio::pin_out_set(gpio::Port::E, 2);
        } else {
            gpio::pin_out_set(gpio::Port::E, 3);
        }
    }
}

#[no_mangle]
pub extern fn main() {

    cmu::clock_enable(cmu::Clock::GPIO, true);
    gpio::pin_mode_set(gpio::Port::E, 2, gpio::Mode::PushPull, 0);
    gpio::pin_mode_set(gpio::Port::E, 3, gpio::Mode::PushPull, 0);

    timer::tests();

    loop {}
}

#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"]
pub extern fn rust_begin_unwind(_msg: core::fmt::Arguments, _file: &'static str, _line: usize) -> ! {
    loop { }
}
