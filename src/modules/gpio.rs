use emdrv::gpioint;
use {cmu, gpio};

pub struct GpioPin {
    pub port: gpio::Port,
    pub pin: u8,
}

impl GpioPin {
    pub fn new(port: gpio::Port, pin: u8) -> GpioPin {
        GpioPin {
            port: port,
            pin: pin,
        }
    }
}

pub trait Button {
    fn init(&self);

    fn on_click(&self, func: gpioint::IrqCallback);
}

pub trait Led {
    fn init(&self);

    fn on(&self);

    fn off(&self);

    fn toggle(&self);
}

impl Button for GpioPin {
    fn init(&self) {
        cmu::clock_enable(cmu::Clock::GPIO, true);
        gpio::pin_mode_set(self.port, self.pin as u32, gpio::Mode::Input, 0);
        gpio::int_config(self.port, self.pin as u32, false, true, true);

        gpioint::init();
    }

    fn on_click(&self, func: gpioint::IrqCallback) {
        gpioint::register(self.pin, func)
    }
}

impl Led for GpioPin {
    fn init(&self) {
        cmu::clock_enable(cmu::Clock::GPIO, true);
        gpio::pin_mode_set(self.port, self.pin as u32, gpio::Mode::PushPull, 0);
    }

    fn on(&self) {
        gpio::pin_out_set(self.port, self.pin as u32);
    }

    fn off(&self) {
        gpio::pin_out_clear(self.port, self.pin as u32);
    }

    fn toggle(&self) {
        gpio::pin_out_toggle(self.port, self.pin as u32);
    }
}
