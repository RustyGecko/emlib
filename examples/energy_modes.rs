#![no_std]
#![no_main]
#![feature(core, no_std)]

extern crate core;
extern crate cmsis;
extern crate emlib;

use cmsis::nvic;
use emlib::chip;
use emlib::cmu;
use emlib::gpio;
use emlib::emu;

const PB0: u32 = 9;
const PB1: u32 = 10;

static mut mode: u32 = 0;


#[no_mangle]
#[allow(non_snake_case)]
pub extern fn GPIO_ODD_IRQHandler() {

    gpio::int_clear(gpio::int_get_enabled());
    unsafe { mode += 1; }
 }

#[no_mangle]
#[allow(non_snake_case)]
pub extern fn GPIO_EVEN_IRQHandler() {

    gpio::int_clear(gpio::int_get_enabled());
    unsafe { mode -= 1; }
}



fn gpio_setup() {
    cmu::clock_enable(cmu::Clock::GPIO, true);

    gpio::pin_mode_set(gpio::Port::B, PB0, gpio::Mode::Input, 0);
    gpio::pin_mode_set(gpio::Port::B, PB1, gpio::Mode::Input, 0);

    nvic::clear_pending_irq(nvic::IRQn::GPIO_ODD);
    nvic::enable_irq(nvic::IRQn::GPIO_ODD);
    nvic::clear_pending_irq(nvic::IRQn::GPIO_EVEN);
    nvic::enable_irq(nvic::IRQn::GPIO_EVEN);

    gpio::int_config(gpio::Port::B, PB0, false, true, true);
    gpio::int_config(gpio::Port::B, PB1, false, true, true);

}

#[no_mangle]
pub extern fn main() {
    chip::init();

    gpio_setup();

    loop {
        match unsafe { mode } {
            1 => emu::enter_em1(),
            2 => emu::enter_em2(true),
            3 => emu::enter_em3(true),
            4 => emu::enter_em4(),
            _ => ()
        }
    }
}
