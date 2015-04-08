#![no_std]
#![no_main]
#![feature(core, no_std, collections)]

extern crate core;
extern crate emlib;
#[macro_use]
extern crate collections;

use core::prelude::*;
use core::str::from_utf8;
use core::default::Default;
use core::iter::range_step_inclusive;

use emlib::cmu;
use emlib::emdrv::flash;
use emlib::stk::bsp;
use emlib::utils::cmdparse::{get_command, Cmd};

use emlib::modules::Usart;

const PAGE_SIZE: usize = 512;
const LINE_WIDTH: usize = 16;

static mut DATA: [u8; PAGE_SIZE] = [0; PAGE_SIZE];

#[no_mangle]
pub extern fn main() {

    emlib::chip::init();
    cmu::clock_select_set(cmu::Clock::HF, cmu::Select::HFXO);

    bsp::ebi_init();
    flash::init(-1);

    let mut usart: Usart = Default::default();
    usart.init_async();

    println("Hello");
    println(" to read send 'r <n>'");
    println(" to write send 'w <n>' followed by a line");

    loop {

        match get_command() {
            Cmd::Read(n) => read(n),
            Cmd::Write(n) => write(n),
            _ => ()
        }
    }
}

fn println(line: &str) {
    let usart: Usart = Default::default();

    let s = format!("{}\n\r", line);
    usart.write_line(&s);
}

fn print_page(page: &[u8]) {

    let usart: Usart = Default::default();

    for i in range_step_inclusive(0, PAGE_SIZE, LINE_WIDTH) {

        let line = &unsafe { DATA }[i .. i+LINE_WIDTH];

        for &num in line {
            let s = format!("{:02x} ", num);
            usart.write_line(&s);
        }

        usart.write_line(" ");

        for &sym in line {
            let ch = if is_readable(sym) {
                sym as char
            } else {
                ' '
            };

            let s = format!("{}", ch);
            usart.write_line(&s);

        }

        usart.write_line("\n\r");
    }


}

fn read(page_num: u32) {

    let addr = page_num_to_addr(page_num);
    flash::read(addr, unsafe { &mut DATA });


    print_page(unsafe { DATA }.as_slice());


}

fn write(page_num: u32) {
    let usart: Usart = Default::default();

    let string = usart.read_line();

    let mut i = 0;
    for ch in string.chars() {
        unsafe { DATA[i] = ch as u8; }
        i += 1;
    }

    let addr = page_num_to_addr(page_num);
    flash::write(addr, unsafe { &mut DATA });
}

fn page_num_to_addr(page_num: u32) -> u32 {

    let flash_info = flash::device_info();

    let page_size = flash_info.page_size;
    let base_addr = flash_info.base_address;

    page_num * page_size + base_addr
}

fn is_readable(sym: u8) -> bool {
    (sym >= ' ' as u8 && sym <= '~' as u8) || (sym >= 161 && sym <= 255)
}
