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

extern {
    pub fn BSP_TraceSwoSetup();
}

const LED0: u32 = 2;
const LED1: u32 = 3;

const PB0: u32 = 9;
const PB1: u32 = 10;

#[no_mangle]
#[allow(non_snake_case)]
pub extern fn GPIO_ODD_IRQHandler() {

    gpio::int_clear(gpio::int_get_enabled());
    gpio::pin_out_toggle(gpio::Port::E, LED0);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern fn GPIO_EVEN_IRQHandler() {

    gpio::int_clear(gpio::int_get_enabled());
    gpio::pin_out_toggle(gpio::Port::E, LED1);
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

    gpio::pin_mode_set(gpio::Port::E, LED0, gpio::Mode::PushPull, 0);
    gpio::pin_mode_set(gpio::Port::E, LED1, gpio::Mode::PushPull, 0);

    loop {}
}
