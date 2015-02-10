#![allow(dead_code)]
use core::default::Default;

pub const RTC_IEN_COMP0: u32 = (0x1 << 1);

#[repr(C)]
pub struct Init {
    pub enable: bool,
    pub debug_run: bool,
    pub comp0_top: bool
}

impl Default for Init {
    fn default() -> Init {
        Init {
            enable: true,
            debug_run: false,
            comp0_top: true
        }
    }
}

extern {
    fn RTC_CompareGet(comp: u32) -> u32;
    fn RTC_CompareSet(comp: u32, value: u32);
    fn STATIC_INLINE_RTC_CounterGet();
    fn STATIC_INLINE_RTC_IntClear(flags: u32);
    fn STATIC_INLINE_RTC_IntDisable(flags: u32);
    fn STATIC_INLINE_RTC_IntEnable(flags: u32);
    fn STATIC_INLINE_RTC_IntGet() -> u32;
    fn STATIC_INLINE_RTC_IntSet(flags: u32);
    fn RTC_Reset();

    fn RTC_Enable(enable: bool);
    fn RTC_FreezeEnable(enable: bool);
    fn RTC_Init(init: &Init);
    fn RTC_CounterReset();
}

pub fn compare_get(comp: u32) -> u32 {
    unsafe { RTC_CompareGet(comp) }
}

pub fn compare_set(comp: u32, value: u32) {
    unsafe { RTC_CompareSet(comp, value) }
}

pub fn counter_get() {
    unsafe { STATIC_INLINE_RTC_CounterGet() }
}

pub fn int_clear(flags: u32) {
    unsafe { STATIC_INLINE_RTC_IntClear(flags) }
}

pub fn int_disable(flags: u32) {
    unsafe { STATIC_INLINE_RTC_IntDisable(flags) }
}

pub fn int_enable(flags: u32) {
    unsafe { STATIC_INLINE_RTC_IntEnable(flags) }
}

pub fn int_get() -> u32 {
    unsafe { STATIC_INLINE_RTC_IntGet() }
}

pub fn int_set(flags: u32) {
    unsafe { STATIC_INLINE_RTC_IntSet(flags) }
}

pub fn reset() {
    unsafe { RTC_Reset() }
}

pub fn enable(enable: bool) {
    unsafe { RTC_Enable(enable) }
}

pub fn freeze_enable(enable: bool) {
    unsafe { RTC_FreezeEnable(enable) }
}

pub fn init(init: &Init) {
    unsafe { RTC_Init(init) }
}

pub fn counter_reset() {
    unsafe { RTC_CounterReset() }
}
