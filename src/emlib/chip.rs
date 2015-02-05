extern {
    pub fn STATIC_INLINE_CHIP_Init();
}

pub fn init() {
    unsafe { STATIC_INLINE_CHIP_Init(); }
}
