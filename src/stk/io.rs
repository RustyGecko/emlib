use {cmu, gpio};

use modules::GpioPin;

pub const PB0: GpioPin = GpioPin { port: gpio::Port::B, pin: 9 };
pub const PB1: GpioPin = GpioPin { port: gpio::Port::B, pin: 10 };

pub const LED0: GpioPin = GpioPin { port: gpio::Port::E, pin: 2 };
pub const LED1: GpioPin = GpioPin { port: gpio::Port::E, pin: 3 };
