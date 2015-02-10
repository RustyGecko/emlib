pub type IrqCallback = extern fn (pin: u8);

extern {
    pub fn GPIOINT_Init();
    pub fn GPIOINT_CallbackRegister(pin: u8, callback: IrqCallback);
    pub fn STATIC_INLINE_GPIOINT_CallbackUnRegister(pin: u8);
}

pub fn init() {
    unsafe { GPIOINT_Init() }
}

pub fn callback_register(pin: u8, callback: IrqCallback) {
    unsafe { GPIOINT_CallbackRegister(pin, callback) }
}

pub fn callback_un_register(pin: u8) {
    unsafe { STATIC_INLINE_GPIOINT_CallbackUnRegister(pin) }
}
