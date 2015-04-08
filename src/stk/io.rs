use gpio;
use modules::{GpioPin, Button, Led};

pub static PB0: &'static Button = &GpioPin { port: gpio::Port::B, pin: 9 };
pub static PB1: &'static Button = &GpioPin { port: gpio::Port::B, pin: 10 };

pub static LED0: &'static Led = &GpioPin { port: gpio::Port::E, pin: 2 };
pub static LED1: &'static Led = &GpioPin { port: gpio::Port::E, pin: 3 };
