#![no_std]
#![no_main]
#![feature(start, core, no_std, alloc)]

// for lang_items
extern crate emlib;
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

#[no_mangle]
pub extern fn posix_memalign(memptr: *mut *mut c_void, alignment: size_t, size: size_t) -> c_int {
    unsafe { memalign(memptr, alignment, size) }
}

extern {
    fn memalign(memptr: *mut *mut c_void, alignment: size_t, size: size_t) -> c_int;
}
