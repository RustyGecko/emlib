#![no_std]
#![no_main]
#![feature(lang_items, start, core, no_std, alloc, collections)]

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
pub unsafe fn __aeabi_memmove(dest: *mut u8, src: *const u8,
                             n: usize) -> *mut u8 {
    rlibc::memmove(dest, src, n)
}
