#![no_std]
#![no_main]
#![feature(lang_items, core, no_std, asm)]
#![feature(collections)]

extern crate core;
extern crate emlib;
#[macro_use(format)] extern crate collections;

use emlib::dk::bsp;

#[path="../projects/game/game.rs"]
pub mod game;

#[no_mangle]
pub extern fn main() {
    bsp::init(bsp::EBI);
    game::run();
}
