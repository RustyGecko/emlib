use core::default::Default;

#[repr(C)]
#[derive(Copy]
pub struct Init {
    pub core_ctrl: CoreCtrlDesc,
    pub time_ctrl: TimeCtrlDesc,
    pub per_ctrl:  PerCtrlDesc,
    pub dec_ctrl:  DecCtrlDesc,
}

#[repr(C)]
#[derive(Copy]
pub struct CoreCtrlDesc {
    pub scan_start:     ScanMode,
    pub prs_sel:        PRSSel,
    pub scan_conf_sel:  ScanConfSel,
    pub inv_acmp0:      bool
    pub dual_sample:    bool,
    pub store_scan_res: bool,
    pub buf_over_wr:    bool,
    pub buf_trig_level: BufTrigLevel,
    pub wakeup_on_dma:  DMAWakeUp
    pub bias_mode:      BiasMode,
    pub debug_run:      bool,
}

#[repr(u8)]
#[derive(Copy]
pub enum ScanMode {
  StartPeriodic = 0x0 << 0,
  StartOneShot  = 0x1 << 0,
  StartPRS      = 0x2 << 0,
}

#[repr(u8)]
#[derive(Copy]
pub enum PRSSel {
    PRSCh0 = 0,
    PRSCh1 = 1,
    PRSCh2 = 2,
    PRSCh3 = 3,
    PRSCh4 = 4,
    PRSCh5 = 5,
    PRSCh6 = 6,
    PRSCh7 = 7,
}

#[repr(u8)]
#[derive(Copy]
pub enum ScanConfSel {
    DirMap = 0x0 << 6,
    InvMap = 0x1 << 6,
    Toggle = 0x2 << 6,
    DecDef = 0x3 << 6,
}

#[repr(u8)]
#[derive(Copy]
pub enum BufTrigLevel {
    half = 0x0 << 18,
    full = 0x1 << 18,
}

#[repr(u8)]
#[derive(Copy]
pub enum DMAWakeUp {
    Disable  = 0x0 << 20,
    BufValid = 0x1 << 20,
    BufLevel = 0x2 << 20,
}

#[repr(u8)]
#[derive(Copy]
pub enum BiasMode {
    DutyCycle = 0x0 << 0,
    HighAcc   = 0x1 << 0,
    DontTouch = 0x2 << 0,
}

#[repr(C)]
#[derive(Copy]
pub struct TimeCtrlDesc {
    start_delay: u8,
}

#[repr(C)]
#[derive(Copy]
pub struct PerCtrlDesc {
    dac_ch0_data:      ControlDACData,
    dac_ch0_conv_mode: ControlDACConv,
    dac_ch0_out_mode:  ControlDACOut,
    dac_ch1_data:      ControlDACData,
    dac_ch1_conv_mode: ControlDACConv,
    dac_ch1_out_mode:  ControlDACOut,
    dac_presc:         u8,
    dac_ref:           DACRef,
    acmp0_mode:        ControlACMP,
    acmp1_mode:        ControlACMP,
    warmup_mode:       WarmupMode,
}

#[repr(u8)]
#[derive(Copy]
pub enum ControlDACData {
  DACIfData = 0x0,
  ACMPThres = 0x1,
}

#[repr(u8)]
#[derive(Copy]
pub enum ControlDACConv {
    ModeDisable    = 0x0,
    ModeContinuous = 0x1,
    ModeSampleHold = 0x2,
    ModeSampleOff  = 0x3,
}

#[repr(u8)]
#[derive(Copy]
pub enum ControlDACOut {
    ModeDisable    = 0x0,
    ModePin        = 0x1,
    ModeADCACMP    = 0x2,
    ModePinADCACMP = 0x3,
}

#[repr(u8)]
#[derive(Copy]
pub enum DACRef {
    Vdd     = 0x0 << 18,
    BandGap = 0x1 << 18,
}

#[repr(u8)]
#[derive(Copy]
pub enum ControlACMP{
    Disable  = 0x0,
    Mux      = 0x1,
    MuxThres = 0x2,
}

#[repr(u8)]
#[derive(Copy]
pub enum WarmupMode {
    Normal   = 0x0 << 26,
    ACMP     = 0x1 << 26,
    DAC      = 0x2 << 26,
    KeepWarm = 0x3 << 26,
}

#[repr(C)]
#[derive(Copy]
pub struct DecCtrlDesc {
    dec_Input:   LESENSE_DecInput_TypeDef,
    init_State:  uint32_t,
    chk_State:   bool,
    int_Map:     bool,
    hyst_PRS0:   bool,
    hyst_PRS1:   bool,
    hyst_PRS2:   bool,
    hyst_IRQ:    bool,
    prs_Count:   bool,
    prs_Ch_Sel0: PRSSel,
    prs_Ch_Sel1: PRSSel,
    prs_Ch_Sel2: PRSSel,
    prs_Ch_Sel3: PRSSel,
}

#[repr(u8)]
#[derive(Copy]
pub enum DecInput {
    SensorSt = 0x0 << 8,
    PRS      = 0x1 << 8,
}
