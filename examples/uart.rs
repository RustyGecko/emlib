#![no_std]
#![no_main]
#![feature(lang_items, core, no_std)]

extern crate core;
extern crate emlib;

use emlib::{cmu, gpio, usart};
use emlib::cmsis::nvic;
use core::default::Default;

#[no_mangle]
pub extern fn main() {

    setup_usart1();

    let usart1 = usart::Usart::usart1();
    
    loop {
        usart1.tx(usart1.rx());
    }
}

fn setup_usart1() {
    cmu::clock_select_set(cmu::Clock::HF, cmu::Select::HFRCO);

    cmu::clock_enable(cmu::Clock::GPIO, true);

    let usart1 = usart::Usart::usart1();

    gpio::pin_mode_set(gpio::Port::D, 0, gpio::Mode::PushPull, 1);
    gpio::pin_mode_set(gpio::Port::D, 1, gpio::Mode::Input, 0);
    gpio::pin_mode_set(gpio::Port::D, 2, gpio::Mode::PushPull, 1);
    gpio::pin_mode_set(gpio::Port::D, 3, gpio::Mode::PushPull, 1);

    cmu::clock_enable(cmu::Clock::USART1, true);
    
    usart1.init_async(&usart::InitAsync {
        enable: usart::Enable::Enable,
        baudrate: 9600,
        ..Default::default()
    });

    usart1.ROUTE = usart::ROUTE_RXPEN
        | usart::ROUTE_TXPEN
        | usart::ROUTE_CLKPEN
        | usart::ROUTE_CSPEN
        | usart::ROUTE_LOCATION_LOC1;

}


#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop {} }
