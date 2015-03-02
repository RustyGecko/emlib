use emdrv::gpioint;
use {cmu, gpio};

pub struct Button {
    port: gpio::Port,
    pin: u8,
}

impl Button {

    fn new(port: gpio::Port, pin: u8) -> Button {
        Button {
            port: port,
            pin: pin,
        }
    }

    pub fn init(&self) {
        cmu::clock_enable(cmu::Clock::GPIO, true);
        gpio::pin_mode_set(self.port, self.pin as u32, gpio::Mode::Input, 0);
        gpio::int_config(self.port, self.pin as u32, false, true, true);

        gpioint::init();
    }

    pub fn pb0() -> Button {
        Button::new(gpio::Port::B, 9)
    }

    pub fn pb1() -> Button {
        Button::new(gpio::Port::B, 10)
    }

    pub fn click(&self, func: gpioint::IrqCallback) {
        gpioint::register(self.pin, func)
    }

}

pub struct Led {
    port: gpio::Port,
    pin: u32
}

impl Led {

    fn new(port: gpio::Port, pin: u32) -> Led {
        Led {
            port: port,
            pin: pin
        }
    }

    pub fn init(&self) {
        cmu::clock_enable(cmu::Clock::GPIO, true);
        gpio::pin_mode_set(self.port, self.pin, gpio::Mode::PushPull, 0);
    }

    pub fn led0() -> Led {
        Led::new(gpio::Port::E, 2)
    }

    pub fn led1() -> Led {
        Led::new(gpio::Port::E, 3)
    }

    pub fn toggle(&self) {
        gpio::pin_out_toggle(self.port, self.pin);
    }

}
