#![allow(dead_code)]

pub const BANK0: u32 = (1 << 1);
pub const BANK1: u32 = (1 << 2);
pub const BANK2: u32 = (1 << 3);
pub const BANK3: u32 = (1 << 4);

pub const CS0: u32   = (1 << 1);
pub const CS1: u32   = (1 << 2);
pub const CS2: u32   = (1 << 3);
pub const CS3: u32   = (1 << 4);

/// Bit fields for EBI IF
pub const IF_MASK: u32    = 0x0000003F;
pub const IF_HSYNC: u32   = (0x1 << 1);
pub const IF_VFPORCH: u32 = (0x1 << 3);

pub fn int_get() -> u32 {
    unsafe { STATIC_INLINE_EBI_IntGet() }
}

pub fn int_clear(flags: u32) {
    unsafe { STATIC_INLINE_EBI_IntClear(flags) }
}

pub fn int_disable(flags: u32) {
    unsafe { STATIC_INLINE_EBI_IntDisable(flags) }
}

pub fn int_enable(flags: u32) {
    unsafe { STATIC_INLINE_EBI_IntEnable(flags) }
}

pub fn bank_address(bank: u32) -> u32 {
    unsafe { EBI_BankAddress(bank) }
}

pub fn tft_alpha_blend_set(alpha: u8) {
    unsafe { STATIC_INLINE_EBI_TFTAlphaBlendSet(alpha) }
}

pub fn tftv_count() -> u32 {
    unsafe { STATIC_INLINE_EBI_TFTVCount() }
}

pub fn tft_frame_base_set(address: u32) {
    unsafe { STATIC_INLINE_EBI_TFTFrameBaseSet(address) }
}

pub fn tfth_stride_set(nbytes: u32) {
    unsafe { STATIC_INLINE_EBI_TFTHStrideSet(nbytes) }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TFTInit {
    pub bank:            TFTBank,
    pub width:           TFTWidth,
    pub colsrc:          TFTColorSrc,
    pub interleave:      TFTInterleave,
    pub fb_trigger:      TFTFrameBufTrigger,
    pub shift_dclk:      bool,
    pub mask_blend:      TFTMaskBlend,
    pub drive_mode:      TFTDDMode,
    pub cs_polarity:     Polarity,
    pub dclk_polarity:   Polarity,
    pub dataen_polarity: Polarity,
    pub hsync_polarity:  Polarity,
    pub vsync_polarity:  Polarity,
    pub hsize:           i32,
    pub h_porch_front:   i32,
    pub h_porch_back:    i32,
    pub h_pulse_width:   i32,
    pub vsize:           i32,
    pub v_porch_front:   i32,
    pub v_porch_back:    i32,
    pub v_pulse_width:   i32,
    pub address_offset:  u32,
    pub dclk_period:     i32,
    pub start_position:  i32,
    pub setup_cycles:    i32,
    pub hold_cycles:     i32,
}


#[repr(u32)]
#[derive(Copy, Clone)]
pub enum TFTBank {
    _0 = (0x00000000 << 20),
    _1 = (0x00000001 << 20),
    _2 = (0x00000002 << 20),
    _3 = (0x00000003 << 20),
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum TFTWidth {
    Byte     = (0x00000000 << 16),
    HalfWord = (0x00000001 << 16),
}

#[repr(u16)]
#[derive(Copy, Clone)]
pub enum TFTColorSrc {
    Mem    = (0x00000000 << 12),
    Pixel1 = (0x00000001 << 12),
}

#[repr(u16)]
#[derive(Copy, Clone)]
pub enum TFTInterleave {
    Unlimited  = (0x00000000 << 10),
    OnePerDClk = (0x00000001 << 10),
    Porch      = (0x00000002 << 10),
}

#[repr(u16)]
#[derive(Copy, Clone)]
pub enum TFTFrameBufTrigger {
    VSync = (0x00000000 << 9),
    HSync = (0x00000001 << 9),
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum TFTMaskBlend {
    Disabled   = (0x00000000 << 2),
    IMask      = (0x00000001 << 2),
    IAlpha     = (0x00000002 << 2),
    IMaskAlpha = (0x00000003 << 2),
    EMask      = (0x00000005 << 2),
    EAlpha     = (0x00000006 << 2),
    EMaskAlpha = (0x00000007 << 2),
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum TFTDDMode {
    Disabled = (0x00000000 << 0),
    Internal = (0x00000001 << 0),
    External = (0x00000002 << 0),
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum Polarity {
    ActiveLow  = 0,
    ActiveHigh = 1,
}


extern {
    fn STATIC_INLINE_EBI_IntGet() -> u32;
    fn STATIC_INLINE_EBI_IntClear(flags: u32);
    fn STATIC_INLINE_EBI_IntDisable(flags: u32);
    fn STATIC_INLINE_EBI_IntEnable(flags: u32);

    fn STATIC_INLINE_EBI_TFTAlphaBlendSet(alpha: u8);
    fn STATIC_INLINE_EBI_TFTVCount() -> u32;
    fn STATIC_INLINE_EBI_TFTFrameBaseSet(address: u32);
    fn STATIC_INLINE_EBI_TFTHStrideSet(nbytes: u32);

    fn EBI_BankAddress(bank: u32) -> u32;
}
