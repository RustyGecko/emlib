#![no_std]
#![no_main]
#![feature(lang_items, core, no_std, asm)]

extern crate core;
extern crate emlib;

#[path="../projects/game/game.rs"] mod game;

#[no_mangle]
pub extern fn main() {
    game::start();

    loop {}
}
