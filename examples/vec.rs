#![no_std]
#![no_main]
#![feature(lang_items, start, core, no_std)]

extern crate core;
extern crate alloc;
extern crate libc;
#[macro_use]
extern crate collections;

use core::ptr::PtrExt;
use collections::vec::Vec;
use collections::string::String;
use libc::{c_void, size_t, c_int};

#[no_mangle]
pub extern fn main() {

    let mut x: Vec<u8> = vec![1, 2, 3];
    let mut y: String = String::from_str("hei");

    loop {}

}

#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"]
pub extern fn rust_begin_unwind(_msg: core::fmt::Arguments, _file: &'static str, _line: usize) -> ! {
    loop { }
}

#[no_mangle]
pub extern fn posix_memalign(memptr: *mut *mut c_void, alignment: size_t, size: size_t) -> c_int {
    unsafe { memalign(memptr, alignment, size) }
}

extern {
    fn memalign(memptr: *mut *mut c_void, alignment: size_t, size: size_t) -> c_int;
}

#[no_mangle]
pub unsafe extern fn __aeabi_memmove(dest: *mut u8, src: *const u8,
                             n: usize) -> *mut u8 {
    if src < dest as *const u8 { // copy from end
        let mut i = n;
        while i != 0 {
            i -= 1;
            *dest.offset(i as isize) = *src.offset(i as isize);
        }
    } else { // copy from beginning
        let mut i = 0;
        while i < n {
            *dest.offset(i as isize) = *src.offset(i as isize);
            i += 1;
        }
    }
    return dest;
}
