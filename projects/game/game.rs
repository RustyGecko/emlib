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

const D_WIDTH: u32 = 320;
const D_HEIGHT: u32 = 240;

const V_WIDTH: u32 = 672;
const V_HEIGHT: u32 = 240;

static tft_init: TFTInit = TFTInit {
    bank:            ebi::TFTBank::_2,
    width:           ebi::TFTWidth::HalfWord,
    colsrc:          ebi::TFTColorSrc::Mem,
    interleave:      ebi::TFTInterleave::Unlimited,
    fb_trigger:      ebi::TFTFrameBufTrigger::HSync,
    shift_dclk:      false,
    mask_blend:      ebi::TFTMaskBlend::Disabled,
    drive_mode:      ebi::TFTDDMode::External,
    cs_polarity:     ebi::Polarity::ActiveLow,
    dclk_polarity:   ebi::Polarity::ActiveHigh,
    dataen_polarity: ebi::Polarity::ActiveLow,
    hsync_polarity:  ebi::Polarity::ActiveLow,
    vsync_polarity:  ebi::Polarity::ActiveLow,
    hsize:           320,
    h_porch_front:   1,
    h_porch_back:    30,
    h_pulse_width:   2,
    vsize:           240,
    v_porch_front:   1,
    v_porch_back:    4,
    v_pulse_width:   2,
    address_offset:  0x0000,
    dclk_period:     8,
    start_position:  0,
    setup_cycles:    0,
    hold_cycles:     0,
};

pub static mut fb_p: u16 = 0;

fn tft_irq_enable(flags: u32) {
    ebi::int_disable(ebi::IF_MASK);
    ebi::int_clear(ebi::IF_MASK);
    ebi::int_enable(flags);

    nvic::clear_pending_irq(nvic::IRQn::EBI);
    nvic::enable_irq(nvic::IRQn::EBI);
}

fn tft_tisplay_clear() {
    // Clear entire display using 32-bit write operations.
    // TODO: Verify that this actually works...
    for i in 0 .. ((V_WIDTH * V_HEIGHT) / 2) {
        let framebuffer: &mut u32 = &mut (ebi::bank_address(ebi::BANK2) + i);
        *framebuffer = 0x0;
    }
}


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

    redraw = tft::direct_init(&tft_init);
    bsp::leds_set(0x8001);

    ebi::tfth_stride_set((V_WIDTH - D_WIDTH) * 2);
    tft_irq_enable(ebi::IF_VFPORCH | ebi::IF_HSYNC);
    tft_tisplay_clear();

    gamepad::init();

    loop {
        bsp::leds_set(0xf731);
    }
}
