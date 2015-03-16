#![no_std]
#![no_main]
#![feature(lang_items, start, core, no_std)]

extern crate core;
extern crate emlib;
#[macro_use]
extern crate collections;

use core::prelude::*;
use core::default::Default;

use emlib::{chip, adc, emu};
use emlib::modules::Usart;

use ram_store as store;

mod temperature;
mod circular_buffer;
mod ram_store;

static mut DATA: [u8; 512] = [0; 512];

#[no_mangle]
pub extern fn main() {

    chip::init();


    let buffer = circular_buffer::get();

    temperature::init(buffer);
    store::init();

    let mut uart: Usart = Default::default();
    uart.init_async();


    let mut page: usize = 0;
    loop {


        let uart: Usart = Default::default();

        let s = format!("Printing data starting at {}\n\r", page);
        uart.write_line(&s);

        store::read(page, unsafe { &mut DATA });

        for &ch in unsafe { &DATA }.iter() {
            let s = format!("{:02x} ", ch);
            uart.write_line(&s);
        }

        uart.putc('\n' as u8);
        uart.putc('\r' as u8);

        page += 1;
    }
}

fn on_dma_finished() {

    let buffer = circular_buffer::get();

    let mut page: usize = 0;
    let uart: Usart = Default::default();

    store::write(buffer);

    let s = format!("Printing data starting at {}\n\r", page);
    uart.write_line(&s);

    store::read(page, unsafe { &mut DATA });

    for &ch in unsafe { &DATA }.iter() {
        let s = format!("{:02x} ", ch);
        uart.write_line(&s);;
    }

    uart.putc('\n' as u8);
    uart.putc('\r' as u8);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern fn ADC0_IRQHandler() {

    let adc = adc::Adc::adc0();
    adc.IFC = 1;

    let temperature = adc.data_single_get();
    circular_buffer::push(temperature as u8);

}
