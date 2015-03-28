
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
pub const IF_HSYNC: u32   = (0x1UL << 1);
pub const IF_VFPORCH: u32 = (0x1UL << 3);

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
