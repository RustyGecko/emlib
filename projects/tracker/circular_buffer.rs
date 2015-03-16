use core::prelude::*;

const PAGE_SIZE: usize = 512;
//const BUFFER_SIZE: usize = PAGE_SIZE * 2 * 30;
const BUFFER_SIZE: usize = PAGE_SIZE;

type WraparoundFn = fn(&'static [u8]);

pub static mut I: u16 = 0;
pub static mut BUFFER: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];

static mut OnWraparound: Option<WraparoundFn> = None;

pub fn get() -> &'static [u8] {
    unsafe { &BUFFER }
}

pub fn on_wraparound(func: WraparoundFn) {
    unsafe {
        OnWraparound = Some(func);
    }
}

pub fn push(val: u8) {
    unsafe {
        BUFFER[I as usize] = val;
        if I < (BUFFER_SIZE-1) as u16 {
            I += 1;
        } else {
            I = 0;

            match OnWraparound {
                Some(func) => func(&BUFFER),
                _ => ()
            }

        }
    }
}
