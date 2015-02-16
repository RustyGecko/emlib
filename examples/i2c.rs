#![no_std]
#![no_main]
#![feature(lang_items, core, no_std)]

extern crate core;
extern crate emlib;

use emlib::{chip, cmu, gpio, i2c};
use emlib::cmsis::nvic;
use emlib::emdrv::gpioint;

use core::default::Default;
use core::intrinsics::{volatile_load};

pub mod std { pub use core::marker; }

const ARDUINO_ADDR: u16 = 0x8;
const GECKO_ADDR: u16 = 0x2;
static TX_BUFFER: &'static str = "RustyGecko";
static mut RX_BUFFER: &'static str = "";

static mut RX: u32 = 0;

const LED0: u32 = 2;
const LED1: u32 = 3;

const PB0: u32 = 9;
const PB1: u32 = 10;

#[derive(Copy)]
enum Mode {
    Receive,
    Send,
    Idle
}

static mut MODE: Mode = Mode::Idle;

#[no_mangle]
pub extern fn main() {

    chip::init();

    setup_oscillators();
    setup_i2c();
    setup_gpio();

    loop {
        match unsafe { MODE } {

            Mode::Receive => receive_i2c_data(),

            Mode::Send => perform_i2c_transfer(),

            Mode::Idle => ()

        }
    }
}

extern fn button_callback(_: u8) {

    match unsafe { MODE } {
        
        Mode::Idle => {
            disable_i2c_slave_interrupts();
            unsafe { MODE = Mode::Send; }
        },

        _ => ()
                
    }

}

fn receive_i2c_data() {}

fn perform_i2c_transfer() {
    
    gpio::pin_out_set(gpio::Port::C, 0);
    gpio::pin_out_set(gpio::Port::E, LED0);

    let i2c0 = i2c::I2C::i2c0();
    i2c0.transfer_init(&i2c::TransferSeq {
        addr: ARDUINO_ADDR,
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
    gpio::pin_out_clear(gpio::Port::E, LED0);
    
    unsafe { MODE = Mode::Idle; }

    enable_i2c_slave_interrupts();

}

fn setup_oscillators() {
    
    cmu::clock_enable(cmu::Clock::I2C0, true);
    cmu::clock_enable(cmu::Clock::GPIO, true);
    cmu::clock_enable(cmu::Clock::CORELE, true);
    
    cmu::clock_enable(cmu::Clock::USART0, true);

    cmu::oscillator_enable(cmu::Osc::LFXO, true, true);

    cmu::clock_select_set(cmu::Clock::LFA, cmu::Select::LFXO);
}

fn setup_i2c() {

      /* Using PD6 (SDA) and PD7 (SCL) */
    gpio::pin_mode_set(gpio::Port::D, 7, gpio::Mode::WiredAndPullUpFilter, 1);
    gpio::pin_mode_set(gpio::Port::D, 6, gpio::Mode::WiredAndPullUpFilter, 1);
    gpio::pin_mode_set(gpio::Port::C, 0, gpio::Mode::PushPull, 0);
    
    let i2c0 = i2c::I2C::i2c0();
    i2c0.ROUTE = i2c::ROUTE_SDAPEN | i2c::ROUTE_SCLPEN | (1 << 8);

    i2c0.init(&Default::default());

    i2c0.SADDR = GECKO_ADDR as u32;
    i2c0.CTRL |= i2c::CTRL_SLAVE | i2c::CTRL_AUTOACK | i2c::CTRL_AUTOSN;
    enable_i2c_slave_interrupts();
}

fn enable_i2c_slave_interrupts() {
    let i2c0 = i2c::I2C::i2c0();
    i2c0.int_clear(i2c::IEN_ADDR | i2c::IEN_RXDATAV | i2c::IEN_SSTOP);
    i2c0.int_enable(i2c::IEN_ADDR | i2c::IEN_RXDATAV | i2c::IEN_SSTOP);
    nvic::enable_irq(nvic::IRQn::I2C0);
}

fn disable_i2c_slave_interrupts() {
    let i2c0 = i2c::I2C::i2c0();
    nvic::disable_irq(nvic::IRQn::I2C0);
    i2c0.int_disable(i2c::IEN_ADDR | i2c::IEN_RXDATAV | i2c::IEN_SSTOP);
    i2c0.int_clear(i2c::IEN_ADDR | i2c::IEN_RXDATAV | i2c::IEN_SSTOP);
}

fn setup_gpio() {

    gpio::pin_mode_set(gpio::Port::B, PB0, gpio::Mode::Input, 0);
    gpio::pin_mode_set(gpio::Port::B, PB1, gpio::Mode::Input, 0);

    gpioint::init();

    gpioint::callback_register(PB0 as u8, button_callback);
    gpioint::callback_register(PB1 as u8, button_callback);

    gpio::int_config(gpio::Port::B, PB0, false, true, true);
    gpio::int_config(gpio::Port::B, PB1, false, true, true);

    gpio::pin_mode_set(gpio::Port::E, LED0, gpio::Mode::PushPull, 0);
    gpio::pin_mode_set(gpio::Port::E, LED1, gpio::Mode::PushPull, 0);
    
}

#[no_mangle]
#[allow(dead_code, non_snake_case)]
pub extern fn I2C0_IRQHandler() {

    gpio::pin_out_set(gpio::Port::E, LED1);
    let i2c0 = i2c::I2C::i2c0();

    unsafe {

        let status = volatile_load(&i2c0.IF as *const u32);
        
        if (status & i2c::IF_ADDR) != 0 {

            MODE = Mode::Receive;
            RX = volatile_load(&i2c0.RXDATA as *const u32);
            i2c0.int_clear(i2c::IFC_ADDR);

        } else if (status & i2c::IEN_SSTOP) != 0 {
            
            i2c0.int_clear(i2c::IEN_SSTOP);
            MODE = Mode::Idle;
            
        } else if (status & i2c::IF_RXDATAV) != 0 {

            RX = volatile_load(&i2c0.RXDATA as *const u32);

        }
    }

    gpio::pin_out_clear(gpio::Port::E, LED1);
}

#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop {} }
