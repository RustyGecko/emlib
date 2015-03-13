use core::default::Default;

pub const IF_CH6: u32 = 0x1 << 6;

#[repr(C)]
#[derive(Copy)]
pub struct Init {
    pub core_ctrl: CoreCtrlDesc,
    pub time_ctrl: TimeCtrlDesc,
    pub per_ctrl:  PerCtrlDesc,
    pub dec_ctrl:  DecCtrlDesc,
}

impl Default for Init {
    fn default() -> Init {
        Init {
            core_ctrl: Default::default(),
            time_ctrl: Default::default(),
            per_ctrl:  Default::default(),
            dec_ctrl:  Default::default(),
        }
    }
}

pub fn scan_start() {
    unsafe { LESENSE_ScanStart() }
}

pub fn init(init: &Init, req_reset: bool) {
    unsafe { LESENSE_Init(init, req_reset) }
}

pub fn channel_config(conf_ch: &ChDesc, ch_idx: u32) {
    unsafe { LESENSE_ChannelConfig(conf_ch, ch_idx) }
}

pub fn alt_ex_config(conf_alt_ex: &ConfAltEx) {
    unsafe {  LESENSE_AltExConfig(conf_alt_ex) }
}

pub fn scan_freq_set(ref_freq: u32, scan_freq: u32) -> u32 {
    unsafe { LESENSE_ScanFreqSet(ref_freq, scan_freq) }
}

pub fn clk_div_set(clk: ChClk, clk_div: ClkPresc) {
    unsafe { LESENSE_ClkDivSet(clk, clk_div) }
}

// Lesense peripheral accessor functions
pub fn decoder_stop() {
    unsafe { STATIC_INLINE_LESENSE_DecoderStop() }
}

pub fn status_get() -> u32 {
    unsafe { STATIC_INLINE_LESENSE_StatusGet() }
}

pub fn status_wait(flag: u32) {
    unsafe { STATIC_INLINE_LESENSE_StatusWait(flag) }
}

pub fn channel_active_get() -> u32 {
    unsafe { STATIC_INLINE_LESENSE_ChannelActiveGet() }
}

pub fn scan_result_get() -> u32 {
    unsafe { STATIC_INLINE_LESENSE_ScanResultGet() }
}

pub fn scan_result_data_get() -> u32 {
    unsafe { STATIC_INLINE_LESENSE_ScanResultDataGet() }
}

pub fn scan_result_data_buffer_get(idx: u32) -> u32 {
    unsafe { STATIC_INLINE_LESENSE_ScanResultDataBufferGet(idx) }
}

pub fn sensor_state_get() -> u32 {
    unsafe { STATIC_INLINE_LESENSE_SensorStateGet() }
}

pub fn ram_power_down() {
    unsafe { STATIC_INLINE_LESENSE_RAMPowerDown() }
}

pub fn int_clear(flags: u32) {
    unsafe { STATIC_INLINE_LESENSE_IntClear(flags) }
}

pub fn int_enable(flags: u32) {
    unsafe { STATIC_INLINE_LESENSE_IntEnable(flags) }
}

pub fn int_disable(flags: u32) {
    unsafe { STATIC_INLINE_LESENSE_IntDisable(flags) }
}

pub fn int_set(flags: u32) {
    unsafe { STATIC_INLINE_LESENSE_IntSet(flags) }
}

pub fn int_get() -> u32 {
    unsafe { STATIC_INLINE_LESENSE_IntGet() }
}

pub fn int_get_enabled() -> u32 {
    unsafe { STATIC_INLINE_LESENSE_IntGetEnabled() }
}

#[repr(C)]
#[derive(Copy)]
pub struct CoreCtrlDesc {
    pub scan_start:     ScanMode,
    pub prs_sel:        PRSSel,
    pub scan_conf_sel:  ScanConfSel,
    pub inv_acmp0:      bool,
    pub inv_acmp1:      bool,
    pub dual_sample:    bool,
    pub store_scan_res: bool,
    pub buf_over_wr:    bool,
    pub buf_trig_level: BufTrigLevel,
    pub wakeup_on_dma:  DMAWakeUp,
    pub bias_mode:      BiasMode,
    pub debug_run:      bool,
}

impl Default for CoreCtrlDesc {
    fn default() -> CoreCtrlDesc {
        CoreCtrlDesc {
            scan_start:     ScanMode::StartPeriodic,
            prs_sel:        PRSSel::PRSCh0,
            scan_conf_sel:  ScanConfSel::DirMap,
            inv_acmp0:      false,
            inv_acmp1:      false,
            dual_sample:    false,
            store_scan_res: true,
            buf_over_wr:    true,
            buf_trig_level: BufTrigLevel::Half,
            wakeup_on_dma:  DMAWakeUp::Disable,
            bias_mode:      BiasMode::DontTouch,
            debug_run:      true,
        }
    }
}

#[repr(u8)]
#[derive(Copy)]
pub enum ScanMode {
  StartPeriodic = 0x0 << 0,
  StartOneShot  = 0x1 << 0,
  StartPRS      = 0x2 << 0,
}

#[repr(u8)]
#[derive(Copy)]
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
#[derive(Copy)]
pub enum ScanConfSel {
    DirMap = 0x0 << 6,
    InvMap = 0x1 << 6,
    Toggle = 0x2 << 6,
    DecDef = 0x3 << 6,
}

#[repr(u32)]
#[derive(Copy)]
pub enum BufTrigLevel {
    Half = 0x0 << 18,
    Full = 0x1 << 18,
}

#[repr(u32)]
#[derive(Copy)]
pub enum DMAWakeUp {
    Disable  = 0x0 << 20,
    BufValid = 0x1 << 20,
    BufLevel = 0x2 << 20,
}

#[repr(u8)]
#[derive(Copy)]
pub enum BiasMode {
    DutyCycle = 0x0 << 0,
    HighAcc   = 0x1 << 0,
    DontTouch = 0x2 << 0,
}

#[repr(C)]
#[derive(Copy)]
pub struct TimeCtrlDesc {
    pub start_delay: u8,
}

impl Default for TimeCtrlDesc {
    fn default() -> TimeCtrlDesc {
        TimeCtrlDesc {
            start_delay: 0
        }
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct PerCtrlDesc {
    pub dac_ch0_data:      ControlDACData,
    pub dac_ch0_conv_mode: ControlDACConv,
    pub dac_ch0_out_mode:  ControlDACOut,
    pub dac_ch1_data:      ControlDACData,
    pub dac_ch1_conv_mode: ControlDACConv,
    pub dac_ch1_out_mode:  ControlDACOut,
    pub dac_presc:         u8,
    pub dac_ref:           DACRef,
    pub acmp0_mode:        ControlACMP,
    pub acmp1_mode:        ControlACMP,
    pub warmup_mode:       WarmupMode,
}

impl Default for PerCtrlDesc {
    fn default() -> PerCtrlDesc {
        PerCtrlDesc {
            dac_ch0_data:      ControlDACData::DACIfData,
            dac_ch0_conv_mode: ControlDACConv::ModeDisable,
            dac_ch0_out_mode:  ControlDACOut::ModeDisable,
            dac_ch1_data:      ControlDACData::DACIfData,
            dac_ch1_conv_mode: ControlDACConv::ModeDisable,
            dac_ch1_out_mode:  ControlDACOut::ModeDisable,
            dac_presc:         0,
            dac_ref:           DACRef::Vdd,
            acmp0_mode:        ControlACMP::MuxThres,
            acmp1_mode:        ControlACMP::MuxThres,
            warmup_mode:       WarmupMode::KeepWarm,
        }
    }
}

#[repr(u8)]
#[derive(Copy)]
pub enum ControlDACData {
  DACIfData = 0x0,
  ACMPThres = 0x1,
}

#[repr(u8)]
#[derive(Copy)]
pub enum ControlDACConv {
    ModeDisable    = 0x0,
    ModeContinuous = 0x1,
    ModeSampleHold = 0x2,
    ModeSampleOff  = 0x3,
}

#[repr(u8)]
#[derive(Copy)]
pub enum ControlDACOut {
    ModeDisable    = 0x0,
    ModePin        = 0x1,
    ModeADCACMP    = 0x2,
    ModePinADCACMP = 0x3,
}

#[repr(u32)]
#[derive(Copy)]
pub enum DACRef {
    Vdd     = 0x0 << 18,
    BandGap = 0x1 << 18,
}

#[repr(u8)]
#[derive(Copy)]
pub enum ControlACMP {
    Disable  = 0x0,
    Mux      = 0x1,
    MuxThres = 0x2,
}

#[repr(u32)]
#[derive(Copy)]
pub enum WarmupMode {
    Normal   = 0x0 << 26,
    ACMP     = 0x1 << 26,
    DAC      = 0x2 << 26,
    KeepWarm = 0x3 << 26,
}

#[repr(C)]
#[derive(Copy)]
pub struct DecCtrlDesc {
    pub dec_input:   DecInput,
    pub init_state:  u32,
    pub chk_state:   bool,
    pub int_map:     bool,
    pub hyst_prs0:   bool,
    pub hyst_prs1:   bool,
    pub hyst_prs2:   bool,
    pub hyst_irq:    bool,
    pub prs_count:   bool,
    pub prs_ch_sel0: PRSSel,
    pub prs_ch_sel1: PRSSel,
    pub prs_ch_sel2: PRSSel,
    pub prs_ch_sel3: PRSSel,
}

impl Default for DecCtrlDesc {
    fn default() -> DecCtrlDesc {
        DecCtrlDesc {
            dec_input:   DecInput::SensorSt,
            init_state:  0,
            chk_state:   false,
            int_map:     true,
            hyst_prs0:   true,
            hyst_prs1:   true,
            hyst_prs2:   true,
            hyst_irq:    true,
            prs_count:   false,
            prs_ch_sel0: PRSSel::PRSCh0,
            prs_ch_sel1: PRSSel::PRSCh1,
            prs_ch_sel2: PRSSel::PRSCh2,
            prs_ch_sel3: PRSSel::PRSCh3,
        }
    }
}

#[repr(u16)]
#[derive(Copy)]
pub enum DecInput {
    SensorSt = 0x0 << 8,
    PRS      = 0x1 << 8,
}

#[repr(C)]
#[derive(Copy)]
pub struct ChDesc {
    pub ena_scan_ch:      bool,
    pub ena_pin:          bool,
    pub ena_int:          bool,
    pub ch_pin_ex_mode:   ChPinExMode,
    pub ch_pin_idle_mode: ChPinIdleMode,
    pub use_alt_ex:       bool,
    pub shift_res:        bool,
    pub inv_res:          bool,
    pub store_cnt_res:    bool,
    pub ex_clk:           ChClk,
    pub sample_clk:       ChClk,
    pub ex_time:          u8,
    pub sample_delay:     u8,
    pub meas_delay:       u8,
    pub acmp_thres:       u16,
    pub sample_mode:      ChSampleMode,
    pub int_mode:         ChIntMode,
    pub cnt_thres:        u16,
    pub comp_mode:        ChCompMode,
}

impl Default for ChDesc {
    fn default() -> ChDesc {
        ChDesc {
            ena_scan_ch:      true,
            ena_pin:          true,
            ena_int:          true,
            ch_pin_ex_mode:   ChPinExMode::High,
            ch_pin_idle_mode: ChPinIdleMode::Low,
            use_alt_ex:       false,
            shift_res:        false,
            inv_res:          false,
            store_cnt_res:    false,
            ex_clk:           ChClk::LF,
            sample_clk:       ChClk::LF,
            ex_time:          0x03,
            sample_delay:     0x09,
            meas_delay:       0x06,
            acmp_thres:       0x00,
            sample_mode:      ChSampleMode::ACMP,
            int_mode:         ChIntMode::SetIntNone,
            cnt_thres:        0xFF,
            comp_mode:        ChCompMode::Less
        }
    }
}

#[repr(u32)]
#[derive(Copy)]
pub enum ChPinExMode {
    Dis    = 0x0 << 15,
    High   = 0x1 << 15,
    Low    = 0x2 << 15,
    DACOut = 0x3 << 15,
}

#[repr(u8)]
#[derive(Copy)]
pub enum ChPinIdleMode {
    Dis    = 0x0,
    High   = 0x1,
    Low    = 0x2,
    DACCh0 = 0x3,
    // DACCh1 = 0x3, #FIXME This field has the same value, so it's ignored for now.
}

#[repr(u8)]
#[derive(Copy)]
pub enum ChClk {
    LF = 0x0,
    HF = 0x1,
}

#[repr(u16)]
#[derive(Copy)]
pub enum ChSampleMode {
    Counter = 0x0 << 12,
    ACMP    = 0x1 << 12,
}

#[repr(u16)]
#[derive(Copy)]
pub enum ChIntMode {
    SetIntNone    = 0x0 << 13,
    SetIntLevel   = 0x1 << 13,
    SetIntPosEdge = 0x2 << 13,
    SetIntNegEdge = 0x3 << 13,
}

#[repr(u32)]
#[derive(Copy)]
pub enum ChCompMode {
    Less        = 0x0 << 16,
    GreaterOrEq = 0x1 << 16,
}

#[repr(C)]
#[derive(Copy)]
pub struct ConfAltEx {
    pub alt_ex_map: AltExMap,
    pub alt_ex:     [AltExDesc; 16],
}

impl Default for ConfAltEx {
    fn default() -> ConfAltEx {
        ConfAltEx {
            alt_ex_map: AltExMap::ACMP,
            alt_ex: [Default::default(); 16]
        }
    }
}

#[repr(u8)]
#[derive(Copy)]
pub enum AltExMap {
    ALTEX = 0x0,
    ACMP  = 0x1,
}

#[repr(C)]
#[derive(Copy)]
pub struct AltExDesc {
    pub enable_pin: bool,
    pub idle_conf:  AltExPinIdle,
    pub always_ex:  bool,
}

impl Default for AltExDesc {
    fn default() -> AltExDesc {
        AltExDesc {
            enable_pin: false,
            idle_conf:  AltExPinIdle::Dis,
            always_ex:  false
        }
    }
}

#[repr(u8)]
#[derive(Copy)]
pub enum AltExPinIdle {
    Dis  = 0x0,
    High = 0x1,
    Low  = 0x2,
}

#[repr(C)]
#[derive(Copy)]
pub enum ClkPresc {
    ClkDiv1   = 0,
    ClkDiv2   = 1,
    ClkDiv4   = 2,
    ClkDiv8   = 3,
    ClkDiv16  = 4,
    ClkDiv32  = 5,
    ClkDiv64  = 6,
    ClkDiv128 = 7,
}

extern {
    fn LESENSE_ScanStart();
    fn LESENSE_Init(init: *const Init, req_reset: bool);
    fn LESENSE_ChannelConfig(conf_ch: *const ChDesc, req_idx: u32);
    fn LESENSE_AltExConfig(conf_alt_ex: *const ConfAltEx);
    fn LESENSE_ScanFreqSet(ref_freq: u32, scan_freq: u32) -> u32;
    fn LESENSE_ClkDivSet(clk: ChClk, clk_div: ClkPresc);

    // Lesense peripheral accessor functions
    fn STATIC_INLINE_LESENSE_DecoderStop();
    fn STATIC_INLINE_LESENSE_StatusGet() -> u32;
    fn STATIC_INLINE_LESENSE_StatusWait(flag: u32);
    fn STATIC_INLINE_LESENSE_ChannelActiveGet() -> u32;
    fn STATIC_INLINE_LESENSE_ScanResultGet() -> u32;
    fn STATIC_INLINE_LESENSE_ScanResultDataGet() -> u32;
    fn STATIC_INLINE_LESENSE_ScanResultDataBufferGet(idx: u32) -> u32;
    fn STATIC_INLINE_LESENSE_SensorStateGet() -> u32;
    fn STATIC_INLINE_LESENSE_RAMPowerDown();
    fn STATIC_INLINE_LESENSE_IntClear(flags: u32);
    fn STATIC_INLINE_LESENSE_IntEnable(flags: u32);
    fn STATIC_INLINE_LESENSE_IntDisable(flags: u32);
    fn STATIC_INLINE_LESENSE_IntSet(flags: u32);
    fn STATIC_INLINE_LESENSE_IntGet() -> u32;
    fn STATIC_INLINE_LESENSE_IntGetEnabled() -> u32;
}
