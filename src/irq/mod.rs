
pub fn enable() -> u32 {
    unsafe { STATIC_INLINE_INT_Enable() }
}

pub fn disable() -> u32 {
    unsafe { STATIC_INLINE_INT_Disable() }
}

extern {
    fn STATIC_INLINE_INT_Disable() -> u32;
    fn STATIC_INLINE_INT_Enable() -> u32;
}
