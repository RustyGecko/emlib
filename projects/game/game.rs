#![allow(warnings)]

use core::default::Default;

use emlib::ebi;
use emlib::ebi::{TFTInit};
use emlib::cmu;
use emlib::gpio;
use emlib::cmsis;
use emlib::cmsis::nvic;
use emlib::emdrv::tft;

use emlib::modules::{Usart, Config, Location};
use emlib::modules::{Button, GpioPin};
use emlib::dk::{bc, bsp};

pub mod gamepad;
pub mod utils;
pub mod display;

pub static mut fb_p: u16 = 0;

#[no_mangle]
pub extern fn EBI_IRQHandler() {
    let flags = ebi::int_get();
    ebi::int_clear(flags);
}

pub fn run() {
    unsafe {
        fb_p = ebi::bank_address(ebi::BANK2) as u16;
    }
    let mut redraw: bool = false;

    // Configure for 48MHz HFXO operation of core clock
    cmu::clock_select_set(cmu::Clock::HF, cmu::Select::HFXO);

    // Setup SysTick Timer for 1 msec interrupts
    if (cmsis::sys_tick::config(cmu::clock_freq_get(cmu::Clock::CORE) / 1000) != 0) {
        loop {}
    }

    // AEM has to be exited in order for the MCU to control the screen
    let bcreg = bc::BC::bc_register();
    while (bcreg.UIF_AEM != bc::UIF_AEM_EFM) {
        utils::blink(1)
    }

    redraw = display::init();
    bsp::leds_set(0x8001);

    ebi::tfth_stride_set((display::V_WIDTH - display::D_WIDTH) * 2);
    display::irq_enable(ebi::IF_VFPORCH | ebi::IF_HSYNC);
    display::clear();

    gamepad::init();

    loop {
        bsp::leds_set(0xf731);
    }
}
