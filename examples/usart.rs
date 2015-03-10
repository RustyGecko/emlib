#![no_std]
#![no_main]
#![feature(lang_items, core, collections, no_std)]

extern crate core;
extern crate emlib;
#[macro_use]
extern crate collections;

use core::default::Default;
use emlib::modules::Usart;

#[no_mangle]
pub extern fn main() {
    let mut usart: Usart = Default::default();
    usart.init_async();

    loop {
        let s = format!("Received: {}\n\r", usart.read_line());
        usart.write_line(&s);
    }
}
