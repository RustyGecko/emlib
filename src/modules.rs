use core::default::Default;
use core::prelude::*;

use cmsis::nvic;
use {cmu, gpio, usart};

pub struct Location(u32);

pub struct Usart {
    pub location: Location,
    pub baudrate: u32,
    usart: &'static mut usart::Usart,
}

impl Default for Usart {
    fn default() -> Usart {
        Usart {
            location: Location(1),
            baudrate: 115200,
            usart: usart::Usart::usart1()
        }
    }
}

impl Usart {
    // TODO: Add support for Usart0
    pub fn init(&self) {
        // Enable clock for HF peripherals
        cmu::clock_enable(cmu::Clock::HFPER, true);
        // Enable clock for USART module
        cmu::clock_enable(cmu::Clock::USART1, true);
        // Enable clock for GPIO module (required for pin configuration)
        cmu::clock_enable(cmu::Clock::GPIO, true);

        // Configure GPIO pins for tx (D0) and rx (D1) for Usart1
        gpio::pin_mode_set(gpio::Port::D, 0, gpio::Mode::PushPull, 1);
        gpio::pin_mode_set(gpio::Port::D, 1, gpio::Mode::Input, 0);

        let usart1 = usart::Usart::usart1();
        usart1.init_async(&usart::InitAsync {
            enable: usart::Enable::Disable,
            baudrate: self.baudrate,
            .. Default::default()
        });

        // Clear and setup the interrupt vector
        usart1.int_clear(usart::IF_MASK);
        usart1.int_enable(usart::IF_RXDATAV);
        nvic::clear_pending_irq(nvic::IRQn::UART1_RX);
        nvic::clear_pending_irq(nvic::IRQn::UART1_TX);
        nvic::enable_irq(nvic::IRQn::UART1_RX);
        nvic::enable_irq(nvic::IRQn::UART1_TX);

        // Enable tx and rx for location 1
        usart1.ROUTE = usart::ROUTE_RXPEN | usart::ROUTE_TXPEN | usart::ROUTE_LOCATION_LOC1;
        // Enable Usart
        usart1.enable(usart::Enable::Enable);
    }

    pub fn write_str(&self, string: &str) {
        for c in string.chars() {
            self.usart.tx(c as u8);
        }
    }

    pub fn write(&self, data: u8) {
        self.usart.tx(data);
    }

    pub fn read(&self) -> u8 {
        self.usart.rx()
    }
}
