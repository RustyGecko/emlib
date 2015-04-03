
#[derive(Copy, Clone)]
#[repr(C)]
pub enum BodMode {
    Active,
    Inactive
}

#[derive(Copy, Clone)]
#[repr(C)]
pub enum Em4Oscillator {
    Ulfrco = 0x0,    // 0x0u64,
    Lfxo   = 0x2,    // 0x2u64,
    Lfrco  = 0x1,    // 0x1u64
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Em4Init{
    lock_config: bool,
    oscillator: Em4Oscillator,
    bu_rtc_wakeup: bool,
    vreg: bool
}

impl Em4Init {
    pub fn default() -> Em4Init {
        Em4Init {
            lock_config: false,
            oscillator: Em4Oscillator::Ulfrco,
            bu_rtc_wakeup: true,
            vreg: true
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub enum Probe {
    Disable = 0x0, // EMU_BUCTRL_PROBE_DISABLE,
    VDDDReg = 0x1, // EMU_BUCTRL_PROBE_VDDDREG,
    BUIN    = 0x2, // EMU_BUCTRL_PROBE_BUIN,
    BUOUT   = 0x3, // EMU_BUCTRL_PROBE_BUOUT
}

#[derive(Copy, Clone)]
#[repr(C)]
pub enum Resistor {
    Res0 = 0x0, // EMU_PWRCONF_PWRRES_RES0
    Res1 = 0x1, // EMU_PWRCONF_PWRRES_RES1
    Res2 = 0x2, // EMU_PWRCONF_PWRRES_RES2
    Res3 = 0x3, // EMU_PWRCONF_PWRRES_RES3
}

#[derive(Copy, Clone)]
#[repr(C)]
pub enum Power {
    None    = 0x0, // EMU_BUINACT_PWRCON_NONE
    BuMain  = 0x1, // EMU_BUINACT_PWRCON_BUMAIN
    MainBu  = 0x2, // EMU_BUINACT_PWRCON_MAINBU
    NoDiode = 0x3, // EMU_BUINACT_PWRCON_NODIODE
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct BupdInit {
    probe: Probe,
    bod_cal: bool,
    status_pin_enable: bool,
    resistor: Resistor,
    vout_strong: bool,
    vout_med: bool,
    vout_weak: bool,
    inactive_power: Power,
    active_power: Power,
    enable: bool
}

extern {
    pub fn EMU_BUPDInit(bupd_init: BupdInit);
    pub fn EMU_EM4Init(em4_init: Em4Init);
    pub fn EMU_EnterEM2(enable: bool);
    pub fn EMU_EnterEM3(enable: bool);
    pub fn EMU_EnterEM4();
    pub fn EMU_MemPwrDown(blocks: u32);
    pub fn EMU_UpdateOscConfig();
    pub fn EMU_BUThresholdSet(mode: BodMode, value: u32);
    pub fn EMU_BUThresRangeSet(mode: BodMode, value: u32);
    pub fn STATIC_INLINE_EMU_EnterEM1();
    pub fn STATIC_INLINE_EMU_EM4Lock(enable: bool);
    pub fn STATIC_INLINE_EMU_BUReady();
    pub fn STATIC_INLINE_EMU_BUPinEnable(enable: bool);
    pub fn STATIC_INLINE_EMU_Lock();
    pub fn STATIC_INLINE_EMU_Unlock();
    pub fn STATIC_INLINE_EMU_EM2Block();
    pub fn STATIC_INLINE_EMU_EM2UnBlock();
}

pub fn bupd_init(bupd_init: BupdInit) {
    unsafe { EMU_BUPDInit(bupd_init) }
}

pub fn em4_init(em4_init: Em4Init) {
    unsafe { EMU_EM4Init(em4_init) }
}

pub fn enter_em2(enable: bool) {
    unsafe { EMU_EnterEM2(enable) }
}

pub fn enter_em3(enable: bool) {
    unsafe { EMU_EnterEM3(enable) }
}

pub fn enter_em4() {
    unsafe { EMU_EnterEM4() }
}

pub fn mem_pwr_down(blocks: u32) {
    unsafe { EMU_MemPwrDown(blocks) }
}

pub fn update_osc_config() {
    unsafe { EMU_UpdateOscConfig() }
}

pub fn bu_threshold_set(mode: BodMode, value: u32) {
    unsafe { EMU_BUThresholdSet(mode, value) }
}

pub fn bu_thres_range_set(mode: BodMode, value: u32) {
    unsafe { EMU_BUThresRangeSet(mode, value) }
}

pub fn enter_em1() {
    unsafe { STATIC_INLINE_EMU_EnterEM1() }
}

pub fn em4_lock(enable: bool) {
    unsafe { STATIC_INLINE_EMU_EM4Lock(enable) }
}

pub fn bu_ready() {
    unsafe { STATIC_INLINE_EMU_BUReady() }
}

pub fn bu_pin_enable(enable: bool) {
    unsafe { STATIC_INLINE_EMU_BUPinEnable(enable) }
}

pub fn lock() {
    unsafe { STATIC_INLINE_EMU_Lock() }
}

pub fn unlock() {
    unsafe { STATIC_INLINE_EMU_Unlock() }
}

pub fn em2_block() {
    unsafe { STATIC_INLINE_EMU_EM2Block() }
}

pub fn em2_un_block() {
    unsafe { STATIC_INLINE_EMU_EM2UnBlock() }
}
