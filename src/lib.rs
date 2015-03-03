#![no_std]
#![crate_type="lib"]
#![crate_name="emlib"]
#![deny(warnings)]
#![feature(core, lang_items, no_std, asm)]

extern crate core;

extern crate rlibc;
extern crate libc;

// emlib library utilities
pub mod utils;

// emlib bindings
pub mod chip;
pub mod cmu;
pub mod emu;
pub mod dma;
pub mod gpio;
pub mod irq;
pub mod rtc;
pub mod timer;
pub mod usart;

// emlib driver bindings
pub mod emdrv;

// cmsis bindings
pub mod cmsis;

// stk utils
pub mod stk;

mod std {
    pub use core::*;
}
