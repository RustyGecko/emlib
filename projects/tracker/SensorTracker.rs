#![no_std]
#![no_main]
#![feature(lang_items, start, core, no_std)]
#![feature(collections, alloc)]

#[macro_use]
extern crate core;
extern crate emlib;
extern crate libc;
extern crate alloc;

#[macro_use]
extern crate collections;

use core::prelude::*;
use core::default::Default;

use emlib::{chip, emu, rtc};
use emlib::modules::{Usart};
use emlib::utils::cmdparse::{get_command, Cmd};
use emlib::stk::io::{PB0, PB1};


use ram_store as store;

mod hr_temp;
mod internal_temperature;
mod ram_store;
mod buffer;

enum State {
    Connected,
    Unconnected
}

static mut MODE: State = State::Unconnected;

#[no_mangle]
pub extern fn main() {

    chip::init();

    PB0.init(); PB0.on_click(btn0_cb);

    PB1.init(); PB1.on_click(btn1_cb);
    store::init();

    hr_temp::init();
    internal_temperature::init(100, false);

    let mut uart: Usart = Default::default();
    uart.init_async();

    loop {
        match unsafe { &MODE } {
            &State::Connected => match get_command() {
                Cmd::Read(page) => read(page as usize),
                _ => ()
            },
            _ => {
                empty_queues();
            },
        }
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern fn RTC_IRQHandler() {

    rtc::int_clear(rtc::RTC_IEN_COMP0);
    internal_temperature::on_rtc();
    hr_temp::on_rtc();

}

fn btn0_cb(_pin: u8) {
    unsafe { MODE = State::Connected; }
}

fn btn1_cb(_pin: u8) {
    unsafe { MODE = State::Unconnected; }
}

fn read(page_num: usize)  {
    let uart: Usart = Default::default();

    let s = format!("Printing data starting at {}\n\r", page_num);
    uart.write_line(&s);

    let mut page: [u8; 512] = [0; 512];

    store::read(page_num, &mut page);

    for ch in page.iter() {
        let s = format!("{:02x} ", ch);
        uart.write_line(&s);
    }

    uart.putc('\n' as u8);
    uart.putc('\r' as u8);

}
