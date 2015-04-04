use core::default::Default;

use emlib::ebi;
use emlib::ebi::{TFTInit};
use emlib::cmsis::nvic;
use emlib::emdrv::tft;

pub const D_WIDTH: u32 = 320;
pub const D_HEIGHT: u32 = 240;
pub const V_WIDTH: u32 = 672;
pub const V_HEIGHT: u32 = 240;

pub static tft_init: TFTInit = TFTInit {
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

pub fn init() -> bool {
    tft::direct_init(&tft_init)
}

pub fn irq_enable(flags: u32) {
    ebi::int_disable(ebi::IF_MASK);
    ebi::int_clear(ebi::IF_MASK);
    ebi::int_enable(flags);

    nvic::clear_pending_irq(nvic::IRQn::EBI);
    nvic::enable_irq(nvic::IRQn::EBI);
}

pub fn clear() {
    // Clear entire display using 32-bit write operations.
    // TODO: Verify that this actually works...
    for i in 0 .. ((V_WIDTH * V_HEIGHT) / 2) {
        let framebuffer: &mut u32 = &mut (ebi::bank_address(ebi::BANK2) + i);
        *framebuffer = 0x0;
    }
}

