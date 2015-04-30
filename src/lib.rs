#![no_std]
#![crate_type="lib"]
#![crate_name="emlib"]
#![allow(warnings)]
#![feature(no_std)]
#![feature(core, collections, convert)]
#![feature(slice_patterns)]

#[macro_use]
extern crate core;
extern crate startup;

extern crate rlibc;
extern crate libc;

extern crate collections;

extern crate cmsis;

// emlib bindings
pub mod acmp;
pub mod adc;
pub mod chip;
pub mod cmu;
pub mod ebi;
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

// higher-level modules depending on the bindings
pub mod modules;

// drivers for Silabs sensors
pub mod sensors;

// About to remove this as it is dependent on emdrv
#[cfg(feature = "dk3750")]
pub mod dk;

#[cfg(feature = "stk3700")]
pub mod stk;

mod std {
    pub use core::*;
}
