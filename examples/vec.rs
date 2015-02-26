#![no_std]
#![no_main]
#![feature(start, core, no_std, alloc, collections)]

// for lang_items
extern crate emlib;
extern crate core;
extern crate alloc;
extern crate libc;
#[macro_use]
extern crate collections;
extern crate rlibc;

use collections::vec::Vec;
use collections::string::String;
use libc::{c_void, size_t, c_int};

#[no_mangle]
pub extern fn main() {

    let _x: Vec<u8> = vec![1, 2, 3];
    let _y: String = String::from_str("hei");

    loop {}

}

#[no_mangle]
pub extern fn posix_memalign(memptr: *mut *mut c_void, alignment: size_t, size: size_t) -> c_int {
    unsafe { memalign(memptr, alignment, size) }
}

extern {
    fn memalign(memptr: *mut *mut c_void, alignment: size_t, size: size_t) -> c_int;
}

#[no_mangle]
pub unsafe fn __aeabi_memmove(dest: *mut u8, src: *const u8,
                             n: usize) -> *mut u8 {
    rlibc::memmove(dest, src, n)
}
