#![no_std]
#![no_main]
#![feature(lang_items, core, collections, no_std)]

extern crate core;
extern crate emlib;
#[macro_use]
extern crate collections;

use core::default::Default;
use emlib::kits::dk3750::bsp;
use emlib::modules::Usart;

#[no_mangle]
pub extern fn main() {
    bsp::init(bsp::SPI);

    let usart: Usart = Default::default();

    loop {
        let s = format!("Received: {}\n\r", usart.read_string());
        usart.write_str(&s[..]);
    }
}
