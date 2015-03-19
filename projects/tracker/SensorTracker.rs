#![no_std]
#![no_main]
#![feature(lang_items, start, core, no_std)]

extern crate core;
extern crate emlib;
#[macro_use]
extern crate collections;

use core::prelude::*;
use core::default::Default;

use emlib::{chip, adc, emu, timer};
use emlib::modules::Usart;
use emlib::utils::cmdparse::{get_command, Cmd};

use ram_store as store;

mod temperature;
mod circular_buffer;
mod ram_store;

static mut PRIM: [u8; 512] = [0; 512];

#[no_mangle]
pub extern fn main() {

    chip::init();

    temperature::init();
    store::init();

    let mut uart: Usart = Default::default();
    uart.init_async();

    loop {
        match get_command() {
            Cmd::Read(page) => read(page as usize),
            _ => ()
        }
    }
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

#[no_mangle]
#[allow(non_snake_case)]
pub extern fn TIMER0_IRQHandler() {
    let timer = timer::Timer::timer0();
    timer.int_clear(timer::TIMER_IF_OF);

}

#[no_mangle]
#[allow(non_snake_case)]
pub extern fn ADC0_IRQHandler() {
    let adc = adc::Adc::adc0();
    adc.int_clear(adc::IF_SINGLE);


}
