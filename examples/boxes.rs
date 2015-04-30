#![no_std]
#![feature(no_std, alloc)]

// for lang_items, memmove and memalign
extern crate alloc;
extern crate emlib;

use alloc::boxed::Box;

fn main() {

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
