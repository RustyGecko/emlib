#![no_std]
#![no_main]
#![feature(lang_items, core, no_std, asm)]

extern crate core;
extern crate emlib;

use emlib::stk::io::{Button, Led};

fn led0(_pin: u8) {
    Led::led0().toggle();
}

fn led1(_pin: u8) {
    Led::led1().toggle();
}

#[no_mangle]
pub extern fn main() {

    let btn0 = Button::pb0();
    let btn1 = Button::pb1();

    btn0.init();
    btn1.init();

    Led::led0().init();
    Led::led1().init();

    btn0.on_click(led0);
    btn1.on_click(led1);

    loop {}
}
