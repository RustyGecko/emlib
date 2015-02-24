#![no_std]
#![no_main]
#![feature(lang_items, core, no_std)]

extern crate core;
extern crate emlib;

use core::default::Default;
use emlib::modules::Usart;

#[no_mangle]
pub extern fn main() {
    let usart: Usart = Default::default();
    usart.init();

    loop {
        usart.write(usart.read());
        usart.write_str("Heisann\n\r");

    }
}

#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}

#[lang = "panic_fmt"]
#[allow(unused_variables)]
pub extern fn rust_begin_unwind(msg: core::fmt::Arguments, file: &'static str, line: usize) -> ! {
    loop { }
}
