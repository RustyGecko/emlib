#![no_std]
#![no_main]
#![feature(lang_items, start, core, no_std, alloc)]

extern crate core;
extern crate alloc;
extern crate libc;

use libc::{c_void, size_t, c_int};
use alloc::boxed::Box;

#[no_mangle]
pub extern fn main() {

    let _a = 1;
    let _b = &2;
    let _c = &3;

    let _x = Box::new(1);
    {
        let _y = Box::new(2);
    }
    let _z = Box::new(3);

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
