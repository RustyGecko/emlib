use emlib::gpio;
use emlib::modules::{Button, GpioPin};

pub static SW1: &'static Button = &GpioPin { port: gpio::Port::C, pin: 0 };
pub static SW2: &'static Button = &GpioPin { port: gpio::Port::C, pin: 1 };
pub static SW3: &'static Button = &GpioPin { port: gpio::Port::C, pin: 2 };
pub static SW4: &'static Button = &GpioPin { port: gpio::Port::C, pin: 3 };
pub static SW5: &'static Button = &GpioPin { port: gpio::Port::C, pin: 4 };
pub static SW6: &'static Button = &GpioPin { port: gpio::Port::C, pin: 5 };
pub static SW7: &'static Button = &GpioPin { port: gpio::Port::C, pin: 6 };
pub static SW8: &'static Button = &GpioPin { port: gpio::Port::C, pin: 7 };

pub fn init() {
    SW1.init();
    SW2.init();
    SW3.init();
    SW4.init();
    SW5.init();
    SW6.init();
    SW7.init();
    SW8.init();
}
