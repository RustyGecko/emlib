#![no_std]
#![no_main]
#![feature(lang_items, core, no_std, asm)]

extern crate core;
extern crate emlib;

use emlib::stk::io::{PB0, PB1, LED0, LED1};
use emlib::modules::{Button, Led};

fn led0(_pin: u8) {
    LED0.toggle();
}

fn led1(_pin: u8) {
    LED1.toggle();
}

#[no_mangle]
pub extern fn main() {
    PB0.init();
    PB1.init();
    LED0.init();
    LED1.init();

    PB0.on_click(led0);
    PB1.on_click(led1);

    loop {}
}
