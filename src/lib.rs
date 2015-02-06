#![no_std]
#![allow(unstable)]
#![feature(lang_items)]

#![crate_type="lib"]
#![crate_name="emlib"]

extern crate core;

pub mod chip;
pub mod cmu;
pub mod gpio;

pub mod emdrv;

pub mod std {
  pub use core::cmp;
  pub use core::option;
  pub use core::num;
  pub use core::marker;
}
