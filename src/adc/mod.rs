use core::default::Default;
use core::intrinsics::transmute;


#[repr(C)]
#[derive(Copy)]
#[allow(non_snake_case)]
pub struct Adc {
    pub CTRL: u32,
    pub CMD: u32,
    pub STATUS: u32,
    pub SINGLECTRL: u32,
    pub SCANCTRL: u32,
    pub IEN: u32,
    pub IF: u32,
    pub IFS: u32,
    pub IFC: u32,
    pub SINGLEDATA: u32,
    pub SCANDATA: u32,
    pub SINGLEDATAP: u32,
    pub SCANDATAP: u32,
    pub CAL: u32,
    pub RESERVED0: u32,
    pub BIASPROG: u32,
}

impl Adc {

    pub fn adc0() -> &'static mut Adc {
        unsafe { transmute(GET_ADC0()) }
    }

    pub fn init(&self, init: &Init) {
        unsafe { ADC_Init(self, init) }
    }

    pub fn init_single(&self, init: &InitSingle) {
        unsafe { ADC_InitSingle(self, init) }
    }

    pub fn data_single_get(&self) -> u32 {
        unsafe { STATIC_INLINE_ADC_DataSingleGet(self) }
    }

}

pub fn timebase_calc(hfper_freq: u32) -> u8 {
    unsafe { ADC_TimebaseCalc(hfper_freq) }
}

pub fn prescale_calc(adc_freq: u32, hfper_freq: u32) -> u8 {
    unsafe { ADC_PrescaleCalc(adc_freq, hfper_freq) }
}

#[repr(C)]
#[derive(Copy)]
#[allow(non_snake_case)]
pub struct Init {
    pub ovs_rate_sel: OvsRateSel,
    pub lpf_mode: LPFilter,
    pub warm_up_mode: Warmup,
    pub timebase: u8,
    pub prescale: u8,
    pub tailgate: bool,
}

#[repr(C)]
#[derive(Copy)]
#[allow(non_snake_case)]
pub struct InitSingle {
    pub prs_sel: PRSSEL,
    pub acq_time: AcqTime,
    pub reference: Ref,
    pub resolution: Res,
    pub input: SingleInput,
    pub diff: bool,
    pub prs_enable: bool,
    pub left_adjust: bool,
    pub rep: bool,
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
}

#[repr(u8)]
#[derive(Copy)]
pub enum AcqTime {
    Time1   = 0x0,
    Time2   = 0x1,
    Time4   = 0x2,
    Time8   = 0x3,
    Time16  = 0x4,
    Time32  = 0x5,
    Time64  = 0x6,
    Time128 = 0x7,
    Time256 = 0x8
}

#[repr(u8)]
#[derive(Copy)]
pub enum Ref {
    Ref1V25      = 0x0,
    Ref2V5       = 0x1,
    RefVDD       = 0x2,
    Ref5VDIFF    = 0x3,
    RefExtSingle = 0x4,
    Ref2xExtDiff = 0x5,
    Ref2xVDD     = 0x6
}

#[repr(u8)]
#[derive(Copy)]
pub enum Res {
    Res12Bit = 0x0,
    Res8Bit  = 0x1,
    Res6Bit  = 0x2,
    ResOVS   = 0x3
}

#[repr(u8)]
#[derive(Copy)]
pub enum OvsRateSel {
    Sel2    = 0x0,
    Sel4    = 0x1,
    Sel8    = 0x2,
    Sel16   = 0x3,
    Sel32   = 0x4,
    Sel64   = 0x5,
    Sel128  = 0x6,
    Sel256  = 0x7,
    Sel512  = 0x8,
    Sel1024 = 0x9,
    Sel2048 = 0xA,
    Sel4096 = 0xB
}



#[repr(u8)]
#[derive(Copy)]
pub enum LPFilter {
    Bypass = 0x0,
    RC     = 0x1,
    DeCap  = 0x2
}

#[repr(u8)]
#[derive(Copy)]
pub enum Warmup {
    Normal          = 0x0,
    FastBG          = 0x1,
    KeepScanRefWarm = 0x2,
    KeepADCWarm     = 0x3
}

#[repr(u8)]
#[derive(Copy)]
pub enum SingleInput
{
    Ch0      = 0x0,
    Ch1      = 0x1,
    Ch2      = 0x2,
    Ch3      = 0x3,
    Ch4      = 0x4,
    Ch5      = 0x5,
    Ch6      = 0x6,
    Ch7      = 0x7,
    Temp     = 0x8,
    VDDDiv3  = 0x9,
    VDD      = 0xA,
    VSS      = 0xB,
    VrefDiv2 = 0xC,
    DACOut0  = 0xD,
    DACOut1  = 0xE,
    ATEST    = 15,

    //Ch0Ch1   = 0x0,
    //Ch2Ch3   = 0x1,
    //Ch4Ch5   = 0x2,
    //Ch6Ch7   = 0x3,
    //Diff0    = 4
}



impl Default for Init {

    fn default() -> Init {
        Init {
            ovs_rate_sel: OvsRateSel::Sel2,
            lpf_mode: LPFilter::Bypass,
            warm_up_mode: Warmup::Normal,
            timebase: 0x1F,
            prescale: 0x0,
            tailgate: false,
        }
    }
}

impl Default for InitSingle {

    fn default() -> InitSingle {
        InitSingle {
            prs_sel: PRSSEL::Ch0,
            acq_time: AcqTime::Time1,
            reference: Ref::Ref1V25,
            resolution: Res::Res12Bit,
            input: SingleInput::Ch0,
            diff: false,
            prs_enable: false,
            left_adjust: false,
            rep: false,
        }
    }
}


extern {
    fn GET_ADC0() -> *mut Adc;

    fn ADC_Init(adc: &Adc, init: &Init);
    fn ADC_InitSingle(adc: &Adc, init: &InitSingle);
    fn ADC_TimebaseCalc(hfper_freq: u32) -> u8;
    fn ADC_PrescaleCalc(adc_freq: u32, hfper_freq: u32) -> u8;
    fn STATIC_INLINE_ADC_DataSingleGet(adc: &Adc) -> u32;
}
