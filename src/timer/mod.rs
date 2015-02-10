#![allow(dead_code)]

use core::intrinsics::transmute;
use core::default::Default;

pub const TIMER_IF_OF: u32     = (0x1 << 0);
pub const TIMER_IF_UF: u32     = (0x1 << 1);
pub const TIMER_IF_CC0: u32    = (0x1 << 4);
pub const TIMER_IF_CC1: u32    = (0x1 << 5);
pub const TIMER_IF_CC2: u32    = (0x1 << 6);
pub const TIMER_IF_ICBOF0: u32 = (0x1 << 8);
pub const TIMER_IF_ICBOF1: u32 = (0x1 << 9);
pub const TIMER_IF_ICBOF2: u32 = (0x1 << 10);

#[repr(C)]
#[allow(non_snake_case)]
pub struct CC {
    pub CTRL: u32,
    pub CCV: u32,
    pub CCVP: u32,
    pub Ccvn: u32,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct Timer {
    pub CTRL: u32,
    pub CMD: u32,
    pub STATUS: u32,
    pub IEN: u32,
    pub IF: u32,
    pub IFS: u32,
    pub IFC: u32,
    pub TOP: u32,
    pub TOPB: u32,
    pub CNT: u32,
    pub ROUTE: u32,
    pub RESERVED0: u32,
    pub CC: [CC; 3],
    pub RESERVED1: [u32; 4],
    pub DTCTRL: u32,
    pub DTTIME: u32,
    pub DTFC: u32,
    pub DTOGEN: u32,
    pub DTFAULT: u32,
    pub DTFAULTC: u32,
    pub DTLOCK: u32,
}

impl Timer {

    #[inline]
    pub fn timer0() -> &'static mut Timer {
        unsafe { transmute(GET_TIMER0()) }
    }

    #[inline]
    pub fn timer1() -> &'static Timer {
        unsafe { transmute(GET_TIMER1()) }
    }

    #[inline]
    pub fn timer2() -> &'static Timer {
        unsafe { transmute(GET_TIMER2()) }
    }

    #[inline]
    pub fn timer3() -> &'static Timer {
        unsafe { transmute(GET_TIMER3()) }
    }

    pub fn capture_get(&self, ch: u32) -> u32 {
        unsafe { STATIC_INLINE_TIMER_CaptureGet(self, ch) }
    }

    pub fn compare_buf_set(&self, ch: u32, val: u32) {
        unsafe { STATIC_INLINE_TIMER_CompareBufSet(self, ch, val) }
    }

    pub fn compare_set(&self, ch: u32, val: u32) {
        unsafe { STATIC_INLINE_TIMER_CompareSet(self, ch, val) }
    }

    pub fn counter_get(&self) -> u32 {
        unsafe { STATIC_INLINE_TIMER_CounterGet(self) }
    }

    pub fn counter_set(&self, val: u32) {
        unsafe { STATIC_INLINE_TIMER_CounterSet(self, val) }
    }

    pub fn enable(&self, enable: bool) {
        unsafe { STATIC_INLINE_TIMER_Enable(self, enable) }
    }

    pub fn init(&self, init: &Init) {
        unsafe { TIMER_Init(self, init) }
    }

    pub fn init_cc(&self, ch: u32, init: &InitCC) {
        unsafe { TIMER_InitCC(self, ch, init) }
    }

    pub fn init_dti(&self, init: &InitDTI) {
        unsafe { TIMER_InitDTI(self, init) }
    }

    pub fn enable_dti(&self, enable: bool) {
        unsafe { STATIC_INLINE_TIMER_EnableDTI(self, enable) }
    }

    pub fn get_dti_fault(&self) -> u32 {
        unsafe { STATIC_INLINE_TIMER_GetDTIFault(self) }
    }

    pub fn clear_dti_fault(&self, flags: u32) {
        unsafe { STATIC_INLINE_TIMER_ClearDTIFault(self, flags) }
    }

    pub fn int_clear(&self, flags: u32) {
        unsafe { STATIC_INLINE_TIMER_IntClear(self, flags) }
    }

    pub fn int_disable(&self, flags: u32) {
        unsafe { STATIC_INLINE_TIMER_IntDisable(self, flags) }
    }

    pub fn int_enable(&self, flags: u32) {
        unsafe { STATIC_INLINE_TIMER_IntEnable(self, flags)}
    }

    pub fn int_get(&self) -> u32 {
        unsafe { STATIC_INLINE_TIMER_IntGet(self) }
    }

    pub fn int_get_enabled(&self) -> u32 {
        unsafe { STATIC_INLINE_TIMER_IntGetEnabled(self) }
    }

    pub fn int_set(&self, flags: u32) {
        unsafe { STATIC_INLINE_TIMER_IntSet(self, flags) }
    }

    pub fn lock(&self) {
        unsafe { STATIC_INLINE_TIMER_Lock(self) }
    }

    pub fn reset(&self) {
        unsafe { TIMER_Reset(self) }
    }

    pub fn top_buf_set(&self, val: u32) {
        unsafe { STATIC_INLINE_TIMER_TopBufSet(self, val) }
    }

    pub fn top_get(&self) -> u32 {
        unsafe { STATIC_INLINE_TIMER_TopGet(self) }
    }

    pub fn top_set(&self, val: u32) {
        unsafe { STATIC_INLINE_TIMER_TopSet(self, val) }
    }

    pub fn unlock(&self) {
        unsafe { STATIC_INLINE_TIMER_Unlock(self) }
    }

}

#[repr(u8)]
#[derive(Copy)]
pub enum CCMode {
    Off = 0x0,
    Capture = 0x1,
    Compare = 0x2,
    PWM = 0x3,
}

#[repr(u8)]
#[derive(Copy)]
pub enum ClkSel {
    HFPerClk = 0x0,
    CC1 = 0x1,
    Cascade = 0x2,
}

#[repr(u8)]
#[derive(Copy)]
pub enum Edge {
    Rising = 0x0,
    Falling = 0x1,
    Both = 0x2,
    None = 0x3,
}

#[repr(u8)]
#[derive(Copy)]
pub enum Event {
    EveryEdge = 0x0,
    Every2ndEdge = 0x1,
    Rising = 0x2,
    Falling = 0x3,
}

#[repr(u8)]
#[derive(Copy)]
pub enum InputAction {
    None = 0x0,
    Start = 0x1,
    Stop = 0x2,
    ReloadStart = 0x3,
}

#[repr(u8)]
#[derive(Copy)]
pub enum Mode {
    Up = 0x0,
    Down = 0x1,
    UpDown = 0x2,
    QDec = 0x3,
}

#[repr(u8)]
#[derive(Copy)]
pub enum OutputAction {
    None = 0x0,
    Toggle = 0x1,
    Clear = 0x2,
    Set = 0x3,
}

#[repr(u8)]
#[derive(Copy)]
pub enum Prescale {
    Prescale1    = 0x0,
    Prescale2    = 0x1,
    Prescale4    = 0x2,
    Prescale8    = 0x3,
    Prescale16   = 0x4,
    Prescale32   = 0x5,
    Prescale64   = 0x6,
    Prescale128  = 0x7,
    Prescale256  = 0x8,
    Prescale512  = 0x9,
    Prescale1024 = 0xA,
}

#[repr(u8)]
#[derive(Copy)]
pub enum PRSSEL {
    Ch0 = 0x0,
    Ch1 = 0x1,
    Ch2 = 0x2,
    Ch3 = 0x3,
    Ch4 = 0x4,
    Ch5 = 0x5,
    Ch6 = 0x6,
    Ch7 = 0x7,
    Ch8  = 0x8,
    Ch9  = 0x9,
    Ch10 = 0xA,
    Ch11 = 0xB,
}

#[repr(u8)]
#[derive(Copy)]
pub enum DtiFaultAction {
    None = 0x0,
    Inactive = 0x1,
    Clear = 0x2,
    Tristate = 0x3,
}

#[repr(C)]
#[derive(Copy)]
pub struct Init {
    pub enable: bool,
    pub debug_run: bool,
    pub prescale: Prescale,
    pub clk_sel: ClkSel,
    pub count_2x: bool,
    pub ati: bool,
    pub fall_action: InputAction,
    pub rise_action: InputAction,
    pub mode: Mode,
    pub dma_clr_act: bool,
    pub quad_mode_x4: bool,
    pub one_shot: bool,
    pub sync: bool,
}

impl Default for Init {
    fn default() -> Init {
        Init {
            enable:       true,
            debug_run:    false,
            prescale:     Prescale::Prescale1,
            clk_sel:      ClkSel::HFPerClk,
            count_2x:     false,
            ati:          false,
            fall_action:  InputAction::None,
            rise_action:  InputAction::None,
            mode:         Mode::Up,
            dma_clr_act:  false,
            quad_mode_x4: false,
            one_shot:     false,
            sync:         false
        }
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct InitCC {
    event_ctrl: Event,
    edge: Edge,
    prs_sel: PRSSEL,
    cufoa: OutputAction,
    cofoa: OutputAction,
    cmoa: OutputAction,
    mode: CCMode,
    filter: bool,
    prs_input: bool,
    coist: bool,
    out_invert: bool,
}

impl Default for InitCC {
    fn default() -> InitCC {
        InitCC {
            event_ctrl: Event::EveryEdge,
            edge:       Edge::Rising,
            prs_sel:    PRSSEL::Ch0,
            cufoa:      OutputAction::None,
            cofoa:      OutputAction::None,
            cmoa:       OutputAction::None,
            mode:       CCMode::Off,
            filter:     false,
            prs_input:  false,
            coist:      false,
            out_invert: false,
        }
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct InitDTI {
    enable: bool,
    active_low_out: bool,
    invert_complementary_out: bool,
    auto_restart: bool,
    enable_prs_source: bool,
    prs_sel: PRSSEL,
    prescale: Prescale,
    rise_time: u32,
    fall_time: u32,
    outputs_enable_mask: u32,
    enable_fault_source_core_lockup: bool,
    enable_fault_source_debugger: bool,
    enable_fault_source_prs_sel_0: bool,
    fault_source_prs_sel_0: PRSSEL,
    enable_fault_source_prs_sel_1: bool,
    fault_source_prs_sel_1: PRSSEL,
    fault_action: DtiFaultAction,
}

impl Default for InitDTI {
    fn default() -> InitDTI {
        InitDTI {
            enable:                          true,
            active_low_out:                  false,
            invert_complementary_out:        false,
            auto_restart:                    false,
            enable_prs_source:               false,
            prs_sel:                         PRSSEL::Ch0,
            prescale:                        Prescale::Prescale1,
            rise_time:                       0,
            fall_time:                       0,
            outputs_enable_mask:             (0x1 << 0 | 0x1<<3),
            enable_fault_source_core_lockup: true,
            enable_fault_source_debugger:    true,
            enable_fault_source_prs_sel_0:   false,
            fault_source_prs_sel_0:          PRSSEL::Ch0,
            enable_fault_source_prs_sel_1:   false,
            fault_source_prs_sel_1:          PRSSEL::Ch0,
            fault_action:                    DtiFaultAction::Inactive,
        }
    }
}

extern {

    #[allow(dead_code)] #[inline] fn GET_TIMER0() -> *mut Timer;
    #[allow(dead_code)] #[inline] fn GET_TIMER1() -> *mut Timer;
    #[allow(dead_code)] #[inline] fn GET_TIMER2() -> *mut Timer;
    #[allow(dead_code)] #[inline] fn GET_TIMER3() -> *mut Timer;

    #[allow(dead_code)] fn STATIC_INLINE_TIMER_CaptureGet(timer: &Timer, ch: u32) -> u32;
    #[allow(dead_code)] fn STATIC_INLINE_TIMER_CompareBufSet(timer: &Timer, ch: u32, val: u32);
    #[allow(dead_code)] fn STATIC_INLINE_TIMER_CompareSet(timer: &Timer, ch: u32, val: u32);
    #[allow(dead_code)] fn STATIC_INLINE_TIMER_CounterGet(timer: &Timer) -> u32;
    #[allow(dead_code)] fn STATIC_INLINE_TIMER_CounterSet(timer: &Timer, val: u32);
    #[allow(dead_code)] fn STATIC_INLINE_TIMER_Enable(timer: &Timer, enable: bool);
    #[allow(dead_code)] fn TIMER_Init(timer: &Timer, init: &Init);
    #[allow(dead_code)] fn TIMER_InitCC(timer: &Timer, ch: u32, init: &InitCC);
    #[allow(dead_code)] fn TIMER_InitDTI(timer: &Timer, init: &InitDTI);
    #[allow(dead_code)] fn STATIC_INLINE_TIMER_EnableDTI(timer: &Timer, enable: bool);
    #[allow(dead_code)] fn STATIC_INLINE_TIMER_GetDTIFault(timer: &Timer) -> u32;
    #[allow(dead_code)] fn STATIC_INLINE_TIMER_ClearDTIFault(timer: &Timer, flags: u32);
    #[allow(dead_code)] fn STATIC_INLINE_TIMER_IntClear(timer: &Timer, flags: u32);
    #[allow(dead_code)] fn STATIC_INLINE_TIMER_IntDisable(timer: &Timer, flags: u32);
    #[allow(dead_code)] fn STATIC_INLINE_TIMER_IntEnable(timer: &Timer, flags: u32);
    #[allow(dead_code)] fn STATIC_INLINE_TIMER_IntGet(timer: &Timer) -> u32;
    #[allow(dead_code)] fn STATIC_INLINE_TIMER_IntGetEnabled(timer: &Timer) -> u32;
    #[allow(dead_code)] fn STATIC_INLINE_TIMER_IntSet(timer: &Timer, flags: u32);
    #[allow(dead_code)] fn STATIC_INLINE_TIMER_Lock(timer: &Timer);
    #[allow(dead_code)] fn TIMER_Reset(timer: &Timer);
    #[allow(dead_code)] fn STATIC_INLINE_TIMER_TopBufSet(timer: &Timer, val: u32);
    #[allow(dead_code)] fn STATIC_INLINE_TIMER_TopGet(timer: &Timer) -> u32;
    #[allow(dead_code)] fn STATIC_INLINE_TIMER_TopSet(timer: &Timer, val: u32);
    #[allow(dead_code)] fn STATIC_INLINE_TIMER_Unlock(timer: &Timer);

}
