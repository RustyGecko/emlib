#![no_std]
#![feature(start, no_std, collections)]

// for lang_items, memmove and memalign
extern crate emlib;

#[macro_use]
extern crate collections;

use collections::vec::Vec;
use collections::string::String;

fn main() {

    let _x: Vec<u8> = vec![1, 2, 3];
    let _y: String = String::from_str("hei");

    loop {}

}
