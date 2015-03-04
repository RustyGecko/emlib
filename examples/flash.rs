#![no_std]
#![no_main]
#![feature(lang_items, core, no_std)]

extern crate core;
extern crate emlib;

use emlib::emdrv::flash;
use emlib::stk::bsp;
use emlib::stk::io::{Button, Led};

const PAGE_SIZE: usize = 512;
static PAGE_NUM: u32 = 0;

static mut SRC: [u8; PAGE_SIZE] = [0; PAGE_SIZE];
static mut DST: [u8; PAGE_SIZE] = [0; PAGE_SIZE];

#[no_mangle]
pub extern fn main() {

    emlib::chip::init();
    bsp::ebi_init();
    flash::init(1);

    let btn0 = Button::init_pb0();
    let btn1 = Button::init_pb1();

    unsafe {
        SRC[0] = 0xB;
        SRC[1] = 0xE;
        SRC[2] = 0xE;
        SRC[3] = 0xF;
    }

    btn0.on_click(write_data);
    btn1.on_click(verify_data);

    Led::init_led0();
    Led::init_led1();

    loop {}
}

fn write_data(_pin: u8) {

    let addr = page_num_to_addr(PAGE_NUM);

    flash::write(addr, unsafe { &mut SRC });
    Button::pb0().on_click(read_data);
}

fn read_data(_pin: u8) {

    let addr = page_num_to_addr(PAGE_NUM);

    flash::read(addr, unsafe { &mut DST });
    Button::pb0().on_click(write_data);
}

fn verify_data(_pin: u8) {

    if unsafe { DST[0] == 0xB && DST[1] == 0xE && DST[2] == 0xE && DST[3] == 0xF } {
        Led::led0().on();
        Led::led1().off();
    } else {
        Led::led0().off();
        Led::led1().on();
    }

}

fn page_num_to_addr(page_num: u32) -> u32 {

    let flash_info = flash::device_info();

    let page_size = flash_info.page_size;
    let base_addr = flash_info.base_address;

    page_num * page_size + base_addr
}
