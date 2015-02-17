#![no_std]
#![no_main]
#![feature(lang_items, core, no_std)]

extern crate core;
extern crate emlib;

use emlib::{cmu, gpio, usart};
use core::default::Default;

#[no_mangle]
pub extern fn main() {

    let uart1 = setup_uart1();
    
    loop {
        uart1.tx(uart1.rx());
    }
}

fn setup_uart1() -> &'static usart::Usart {
    cmu::clock_select_set(cmu::Clock::HF, cmu::Select::HFRCO);
    
    cmu::clock_enable(cmu::Clock::GPIO, true);
    cmu::clock_enable(cmu::Clock::USART1, true);

    let uart1 = usart::Usart::uart1();

    gpio::pin_mode_set(gpio::Port::D, 0, gpio::Mode::PushPull, 1);
    gpio::pin_mode_set(gpio::Port::D, 1, gpio::Mode::Input, 0);
    gpio::pin_mode_set(gpio::Port::D, 2, gpio::Mode::PushPull, 1);
    gpio::pin_mode_set(gpio::Port::D, 3, gpio::Mode::PushPull, 1);

    uart1.init_async(&usart::InitAsync {
        baudrate: 9600,
        ..Default::default()
    });

    uart1.int_clear(0x00001FF9);
    uart1.int_enable(uart::IEN_RXDATAV);
    nvic::clear_pending_irq(nvic::IRQn::USART1_RX);
    nvic::clear_pending_irq(nvic::IRQn::USART1_RX);
    nvic::enable_irq(nvic::IRQn::USART1_RX);
    nvic::enable_irq(nvic::IRQn::USART1_TX);
    
    uart1.ROUTE = usart::ROUTE_RXPEN | usart::ROUTE_TXPEN | usart::ROUTE_LOCATION_LOC1;

    return uart1;
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern fn UART1_RX_IRQHandler() {}

#[no_mangle]
#[allow(non_snake_case)]
pub extern fn UART1_TX_IRQHandler() {}

#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop {} }
