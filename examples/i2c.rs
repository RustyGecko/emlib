#![no_std]
#![no_main]
#![feature(lang_items, core)]

extern crate core;
extern crate emlib;

use emlib::{chip, i2c, cmu, gpio};
use core::default::Default;

const I2C_ADDRESS: u16 = 0x8;
static TX_BUFFER: &'static str = "RustyGecko";
static mut RX_BUFFER: &'static str = "";

#[no_mangle]
pub extern fn main() {

    chip::init();

    setup_oscillators();
    setup_i2c();

    loop {
        perform_i2c_transfer();
    }    
}

fn setup_oscillators() {
    
    cmu::clock_enable(cmu::Clock::I2C0, true);
    cmu::clock_enable(cmu::Clock::GPIO, true);
    cmu::clock_enable(cmu::Clock::CORELE, true);
    
    cmu::clock_enable(cmu::Clock::USART0, true);

    cmu::oscillator_enable(cmu::Osc::LFXO, true, true);

    cmu::clock_select_set(cmu::Clock::LFA, cmu::Select::LFXO);
}

fn perform_i2c_transfer() {
    gpio::pin_out_set(gpio::Port::C, 0);

    let i2c0 = i2c::I2C::i2c0();
    i2c0.transfer_init(&i2c::TransferSeq {
        addr: I2C_ADDRESS,
        flags: i2c::FLAG_WRITE,
        buf0: &TX_BUFFER,
        buf1: unsafe { &RX_BUFFER }
    });

    loop {
        match i2c0.transfer() {
            i2c::TransferReturn::InProgress => (),
            _ => break
        }
    }

    gpio::pin_out_clear(gpio::Port::C, 0);
}

fn setup_i2c() {

    gpio::pin_mode_set(gpio::Port::D, 7, gpio::Mode::WiredAndPullUpFilter, 1);
    gpio::pin_mode_set(gpio::Port::D, 6, gpio::Mode::WiredAndPullUpFilter, 1);
    gpio::pin_mode_set(gpio::Port::C, 0, gpio::Mode::PushPull, 0);
    
    let i2c0 = i2c::I2C::i2c0();
    i2c0.ROUTE = i2c::ROUTE_SDAPEN | i2c::ROUTE_SCLPEN | (1 << 8);

    i2c0.init(&Default::default());
}

#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop {} }
