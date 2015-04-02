#![allow(warnings)]

use core::default::Default;

use emlib::modules::Usart;

use emlib::ebi;
use emlib::ebi::{TFTInit};
use emlib::cmu;
use emlib::cmsis;
use emlib::emdrv::tft;

use emlib::dk::{bc, bsp};

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

pub fn start() {
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

    bsp::init(bsp::EBI);
    bsp::leds_set(0xffff);

    let bcreg = bc::BC::bc_register();
    while (bcreg.UIF_AEM != bc::UIF_AEM_EFM) {
        bsp::leds_set(0x4002);
    }

    redraw = tft::direct_init(&tft_init);

    loop {
        bsp::leds_set(0x8001);
    }
}
