#![allow(warnings)]

use emlib::ebi;
use emlib::cmu;

pub static mut fb_p: u16 = 0;

pub fn start() {
    unsafe {
        fb_p = ebi::bank_address(ebi::BANK2) as u16;
    }

    cmu::clock_select_set(cmu::Clock::HF, cmu::Select::HFXO);

}
