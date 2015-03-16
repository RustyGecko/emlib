use core::prelude::*;

static mut STORE: [[u8; 512]; 8] = [[0; 512]; 8];
static mut INDEX: usize = 0;

pub fn init() {}

pub fn write(buffer: &[u8]) {

    for i in range(0, 512) {
        unsafe{ STORE[INDEX][i] = buffer[i]; }
    }

    unsafe { INDEX = (INDEX + 1) % 8; }

}

pub fn read(page: usize, buffer: &'static mut[u8]) {

    for i in range(0, 512) {
        buffer[i] = unsafe { STORE[page % 8][i] };
    }

}
