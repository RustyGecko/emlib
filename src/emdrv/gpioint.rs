use cmsis::nvic;
use {gpio, irq};
use core::prelude::*;

pub type IrqCallback = fn (u8);

static mut CALLBACKS: [Option<IrqCallback>; 16] = [None; 16];

pub fn init() {
    nvic::clear_pending_irq(nvic::IRQn::GPIO_ODD);
    nvic::enable_irq(nvic::IRQn::GPIO_ODD);
    nvic::clear_pending_irq(nvic::IRQn::GPIO_EVEN);
    nvic::enable_irq(nvic::IRQn::GPIO_EVEN);
}

fn set(pin: u8, callback: Option<IrqCallback>) {

    irq::disable();
    unsafe { CALLBACKS[pin as usize] = callback; }
    irq::enable();
}

fn get(pin: u32) -> Option<IrqCallback> {

    irq::disable();
    let callback = unsafe { CALLBACKS[pin as usize] };
    irq::enable();

    callback
}

pub fn register(pin: u8, callback: IrqCallback) {
    set(pin, Some(callback));
}

pub fn un_register(pin: u8) {
    set(pin, None);
}

fn mask_to_index(iflags: u32) -> u32 {
    __CLZ(__RBIT(iflags))
}

fn dispatch(mut iflags: u32) {

    while iflags != 0 {

        let irq_idx = mask_to_index(iflags);

        iflags &= !(1 << irq_idx);

        match get(irq_idx) {
            Some(func) => func(irq_idx as u8),
            None => ()
        }
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern fn GPIO_ODD_IRQHandler() {

    let iflags = gpio::int_get_enabled() & 0x0000AAAA;

    gpio::int_clear(iflags);

    dispatch(iflags);
 }

#[no_mangle]
#[allow(non_snake_case)]
pub extern fn GPIO_EVEN_IRQHandler() {

    let iflags = gpio::int_get_enabled() & 0x00005555;

    gpio::int_clear(iflags);

    dispatch(iflags);
}



#[inline]
#[allow(non_snake_case)]
fn __RBIT(value: u32) -> u32 {

    let mut result: u32;
    unsafe { asm!("rbit $0, $1" : "=r"(result) : "r"(value)); }
    result
}

#[inline]
#[allow(non_snake_case)]
fn __CLZ(data: u32) -> u32 {

    let mut count: u32 = 0;
    let mut mask: u32 = 0x80000000;

    while (data & mask) == 0 {
        count += 1;
        mask = mask >> 1;
    }

    count
}
