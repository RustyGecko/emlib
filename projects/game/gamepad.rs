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
    SW1.on_click(callback);
    SW2.init();
    SW2.on_click(callback);
    SW3.init();
    SW3.on_click(callback);
    SW4.init();
    SW4.on_click(callback);
    SW5.init();
    SW5.on_click(callback);
    SW6.init();
    SW6.on_click(callback);
    SW7.init();
    SW7.on_click(callback);
    SW8.init();
    SW8.on_click(callback);
}

fn callback(pin: u8) {

}
