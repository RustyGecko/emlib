#![no_std]
#![crate_type="lib"]
#![crate_name="emlib"]
#![allow(warnings)]
#![feature(lang_items, no_std, asm)]
#![feature(core, collections, convert)]
#![feature(slice_patterns)]

#[macro_use]
extern crate core;

extern crate rlibc;
extern crate libc;

extern crate collections;

// emlib library utilities
pub mod utils;

// emlib bindings
pub mod acmp;
pub mod adc;
pub mod chip;
pub mod cmu;
pub mod emu;
pub mod dma;
pub mod gpio;
pub mod i2c;
pub mod irq;
pub mod leuart;
pub mod lesense;
pub mod prs;
pub mod rtc;
pub mod timer;
pub mod usart;

// emlib driver bindings
pub mod emdrv;

// cmsis bindings
pub mod cmsis;

// higher-level modules depending on the bindings
pub mod modules;

// drivers for Silabs sensors
pub mod sensors;

// Temporary kit dependencies
#[cfg(feature = "dk3750")]
pub mod dk;

#[cfg(feature = "stk3700")]
pub mod stk;

mod std {
    pub use core::*;
}
