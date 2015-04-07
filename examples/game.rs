#![no_std]
#![no_main]
#![feature(lang_items, core, no_std, asm)]
#![feature(collections)]
#![feature(negate_unsigned)]

#[macro_use(assert, panic)] extern crate core;
#[macro_use(format)] extern crate collections;

extern crate emlib;

use emlib::dk::bsp;

#[path="../projects/game/game.rs"]
pub mod game;

#[no_mangle]
pub extern fn main() {
    bsp::init(bsp::EBI);
    game::run();
}
