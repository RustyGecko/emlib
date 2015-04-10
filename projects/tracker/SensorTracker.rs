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
use core::fmt::Debug;

use collections::vec::Vec;

use emlib::{chip, emu, rtc};
use emlib::modules::{Usart};
use emlib::utils::cmdparse::{get_command, Cmd};
use emlib::stk::io::{PB0, PB1};

use ram_store as store;

mod hr_temp;
mod internal_temperature;
mod ram_store;
mod buffer;
mod circular_buffer;

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

    let mut it_store = Vec::new();
    let mut hr_store = Vec::new();
    let mut t_store = Vec::new();


    hr_temp::init();
    internal_temperature::init(100, false);

    let mut uart: Usart = Default::default();
    uart.init_async();

    loop {
        match unsafe { &MODE } {
            &State::Connected => match get_command() {
                Cmd::Read(page) => {
                    match page % 3 {
                        0 => read(&it_store[..]),
                        1 => read(&hr_store[..]),
                        2 => read(&t_store[..]),
                        _ => ()
                    }
                },
                _ => ()
            },
            _ => {
                empty_queues(
                    &mut it_store,
                    &mut hr_store,
                    &mut t_store
                );
                emu::enter_em2(true);
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

fn empty_queues(it_store: &mut Vec<u8>, hr_store: &mut Vec<u32>, t_store: &mut Vec<i32>) {

    loop {
        match internal_temperature::pop() {
            Ok(val) => it_store.push(val),
            Err(_) => break,
        }
    }

    loop {
        match hr_temp::pop_hr() {
            Ok(val) => hr_store.push(val),
            Err(_) => break,
        }
    }

    loop {
        match hr_temp::pop_temp() {
            Ok(val) => t_store.push(val),
            Err(_) => break,
        }
    }

}

fn btn0_cb(_pin: u8) {
    unsafe { MODE = State::Connected; }
}

fn btn1_cb(_pin: u8) {
    unsafe { MODE = State::Unconnected; }
}

fn read<T: Debug>(samples: &[T])  {
    let uart: Usart = Default::default();

    for sample in samples {
        let s = format!("{:?} ", sample);
        uart.write_line(&s);
    }

    uart.putc('\n' as u8);
    uart.putc('\r' as u8);

}
