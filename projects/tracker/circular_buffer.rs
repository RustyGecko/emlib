use core::prelude::*;

const PAGE_SIZE: usize = 512;
//const BUFFER_SIZE: usize = PAGE_SIZE * 2 * 30;
const BUFFER_SIZE: usize = PAGE_SIZE;


pub static mut I: usize = 0;
pub static mut BUFFER: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];


pub fn get() -> &'static mut [u8] {
    unsafe { &mut BUFFER }
}


pub fn push(val: u8) {
    unsafe {
        BUFFER[I] = val;
        I = (I + 1) % BUFFER_SIZE;
    }
}
