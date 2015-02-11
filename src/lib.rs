#![no_std]
#![crate_type="lib"]
#![crate_name="emlib"]
#![feature(core, lang_items, no_std)]

extern crate core;

pub mod chip;
pub mod cmu;
pub mod dma;
pub mod gpio;
pub mod timer;

pub mod emdrv;
pub mod cmsis;

mod std {
    pub use core::*;
}
