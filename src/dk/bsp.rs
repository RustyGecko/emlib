#![allow(non_snake_case)]
pub const SPI: u32 = 1;
pub const EBI: u32 = 2;

pub fn init(flags: u32) -> i32 {
    unsafe {
        BSP_Init(flags)
    }
}

pub fn leds_set(leds: u32) -> i32 {
    unsafe {
        BSP_LedsSet(leds)
    }
}

pub fn register_read(addr: &u16) -> u16 {
    unsafe {
        BSP_RegisterRead(addr)
    }
}

extern {
    fn BSP_Init(flags: u32) -> i32;
    fn BSP_LedsSet(lefs: u32) -> i32;
    fn BSP_RegisterRead(addr: &u16) -> u16;
}
