use core::prelude::*;
use core::default::Default;

use collections::vec::Vec;
use collections::string::String;

use cmsis::nvic;
use {cmu, gpio, usart};

#[derive(Copy)]
pub enum Location {
    Loc0,
    Loc1,
}

#[allow(dead_code)]
pub struct Usart {
    location: Location,
    baudrate: u32,
    usart: &'static mut usart::Usart,
}

impl Default for Usart {
    fn default() -> Usart {
        let mut usart = Usart {
            location: Location::Loc1,
            baudrate: 9600,
            usart: usart::Usart::usart1()
        };

        usart.init();
        usart
    }
}

impl Usart {
    pub fn new(loc: Location, baudrate: u32) -> Usart {
        Usart {
            location: loc,
            baudrate: baudrate,
            usart: match loc {
                Location::Loc0 => usart::Usart::usart1(),
                Location::Loc1 => usart::Usart::usart1(),
            }
        }
    }

    // TODO: Add support for Usart0
    pub fn init(&mut self) {
        // Enable clock for HF peripherals
        cmu::clock_enable(cmu::Clock::HFPER, true);
        // Enable clock for USART module
        cmu::clock_enable(cmu::Clock::USART1, true);
        // Enable clock for GPIO module (required for pin configuration)
        cmu::clock_enable(cmu::Clock::GPIO, true);

        // Configure GPIO pins for tx (D0) and rx (D1) for Usart1
        gpio::pin_mode_set(gpio::Port::D, 0, gpio::Mode::PushPull, 1);
        gpio::pin_mode_set(gpio::Port::D, 1, gpio::Mode::Input, 0);

        self.usart.init_async(&usart::InitAsync {
            enable: usart::Enable::Disable,
            baudrate: self.baudrate,
            .. Default::default()
        });

        // Clear and setup the interrupt vector
        self.usart.int_clear(usart::IF_MASK);
        self.usart.int_enable(usart::IF_RXDATAV);
        nvic::clear_pending_irq(nvic::IRQn::UART1_RX);
        nvic::clear_pending_irq(nvic::IRQn::UART1_TX);
        nvic::enable_irq(nvic::IRQn::UART1_RX);
        nvic::enable_irq(nvic::IRQn::UART1_TX);

        // Enable tx and rx for location 1
        self.usart.ROUTE = usart::ROUTE_RXPEN | usart::ROUTE_TXPEN | usart::ROUTE_LOCATION_LOC1;
        // Enable Usart
        self.usart.enable(usart::Enable::Enable);
    }

    pub fn putc(&self, data: u8) {
        self.usart.tx(data);
    }

    pub fn getc(&self) -> u8 {
        self.usart.rx()
    }

    pub fn write_str(&self, string: &str) {
        for c in string.chars() {
            self.putc(c as u8);
        }
    }

    pub fn read_string(&self) -> String {
        let mut bytes = Vec::new();

        loop {
            match self.getc() as char {
                '\0' | '\r' => { break; }
                b => { bytes.push(b as u8); }
            }
        }

        String::from_utf8(bytes).unwrap_or_else(|e| {
            panic!("{}", e)
        })
    }

    pub fn readable(&self) -> bool {
        self.usart.IF & usart::USART_STATUS_RXDATAV > 0
    }

    pub fn writeable(&self) -> bool {
        self.usart.IF & usart::USART_STATUS_TXBL > 0
    }
}
