#![no_std]
#![no_main]
#![feature(core, lang_items, no_std)]

extern crate core;
extern crate emlib;

use emlib::chip;
use emlib::cmu;
use emlib::gpio;
use emlib::emu;
use emlib::emdrv::gpioint;

const PB0: u32 = 9;
const PB1: u32 = 10;

static mut mode: u32 = 0;

extern fn button_callback(pin: u8) {
    if pin == 9 {
        unsafe { mode += 1; }
    } else {
        unsafe { mode -= 1; }
    }
}

fn gpio_setup() {
    cmu::clock_enable(cmu::Clock::GPIO, true);

    gpio::pin_mode_set(gpio::Port::B, PB0, gpio::Mode::Input, 0);
    gpio::pin_mode_set(gpio::Port::B, PB1, gpio::Mode::Input, 0);

    gpioint::init();

    gpioint::callback_register(PB0 as u8, button_callback);
    gpioint::callback_register(PB1 as u8, button_callback);

    gpio::int_config(gpio::Port::B, PB0, false, true, true);
    gpio::int_config(gpio::Port::B, PB1, false, true, true);

}

#[no_mangle]
pub extern fn main() {
    chip::init();

    gpio_setup();

    loop {
        match unsafe { mode } {
            1 => emu::enter_em1(),
            2 => emu::enter_em2(true),
            3 => emu::enter_em3(true),
            4 => emu::enter_em4(),
            _ => ()
        }
    }
}

#[lang = "stack_exhausted"]
pub extern fn stack_exhausted() {}

#[lang = "eh_personality"]
pub extern fn eh_personality() {}

#[lang = "panic_fmt"]
#[allow(unused_variables)]
pub extern fn rust_begin_unwind(msg: core::fmt::Arguments, file: &'static str, line: usize) -> ! {
    loop { }
}
