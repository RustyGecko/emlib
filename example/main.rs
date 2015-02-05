#![no_std]
#![no_main]
#![allow(warnings)]
#![allow(unstable)]
#![feature(lang_items)]


extern crate core;
extern crate emlib;

use emlib::chip;
use emlib::cmu;
use emlib::gpio;
use emlib::emdrv::gpioint;

extern {
    pub fn BSP_TraceSwoSetup();
}

const LED0: u32 = 2;
const LED1: u32 = 3;

const PB0: u32 = 9;
const PB1: u32 = 10;

extern fn button_callback(pin: u8) {

    if pin == 9 {
        gpio::pin_out_toggle(gpio::Port::E, LED0);
    } else {
        gpio::pin_out_toggle(gpio::Port::E, LED1);
    }
}

fn gpio_setup() {
    cmu::clock_enable(cmu::Clock::GPIO, true);

    gpio::pin_mode_set(gpio::Port::B, PB0, gpio::Mode::Input, 0);
    gpio::pin_mode_set(gpio::Port::B, PB1, gpio::Mode::Input, 0);

    gpioint::init();

    gpioint::callback_register(PB0 as u8, button_callback);
    gpioint::callback_register(PB1 as u8, button_callback);

    gpio::int_config(gpio::Port::B, PB0, false, true, true);
    gpio::int_config(gpio::Port::B, PB1, false, true, true);
}

#[no_mangle]
pub extern fn main() {

    chip::init();

    gpio_setup();

    gpio::pin_mode_set(gpio::Port::E, LED0, gpio::Mode::PushPull, 0);
    gpio::pin_mode_set(gpio::Port::E, LED1, gpio::Mode::PushPull, 0);

    loop {}
}

#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop {} }
