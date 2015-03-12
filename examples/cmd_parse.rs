#![no_std]
#![no_main]
#![feature(lang_items, core, collections, no_std)]

extern crate core;
extern crate emlib;
#[macro_use]
extern crate collections;

use core::default::Default;

use emlib::utils::cmdparse;
use emlib::modules::Usart;


#[no_mangle]
pub extern fn main() {
    let mut usart: Usart = Default::default();
    usart.init_async();

    loop {

        let cmd = cmdparse::get_command();
        let s = format!("Received: {:?}\n\r", cmd);
        usart.write_line(&s);
    }
}
