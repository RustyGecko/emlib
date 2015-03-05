use core::intrinsics::transmute;
use core::default::Default;

#[repr(C)]
#[derive(Copy)]
pub struct Init {
    full_bias:                   bool,
    half_bias:                   bool,
    bias_prog:                   u32,
    interrupt_on_falling_edge:   bool,
    interrupt_on_rising_edge:    bool,
    warm_time:                   WarmTime,
    hysteresis_level:            HysteresisLevel,
    inactive_value:              bool,
    low_power_reference_enabled: bool,
    vdd_level:                   u32,
    enable:                      bool,
}

impl Default for Init {
    fn default() -> Init {
        Init {
            full_bias:                   false,
            half_bias:                   false,
            bias_prog:                   0x7,
            interrupt_on_falling_edge:   false,
            interrupt_on_rising_edge:    false,
            warm_time:                   WarmTime::_512,
            hysteresis_level:            HysteresisLevel::_5,
            inactive_value:              false,
            low_power_reference_enabled: false,
            vdd_level:                   0x3D,
            enable:                      true,
        }
    }
}

#[allow(non_snake_case)]
#[repr(C)]
#[derive(Copy)]
pub struct Acmp {
    CTRL:     u32,
    INPUTSEL: u32,
    STATUS:   u32,
    IEN:      u32,
    IF:       u32,
    IFS:      u32,
    IFC:      u32,
    ROUTE:    u32,
}

impl Acmp {
    #[inline]
    pub fn acmp0() -> &'static mut Acmp {
        unsafe { transmute(GET_ACMP0()) }
    }

    #[inline]
    pub fn acmp1() -> &'static mut Acmp {
        unsafe { transmute(GET_ACMP1()) }
    }

}

#[repr(u8)]
#[derive(Copy)]
pub enum WarmTime {
    _4   = 0x0,
    _8   = 0x1,
    _16  = 0x2,
    _32  = 0x3,
    _64  = 0x4,
    _128 = 0x5,
    _256 = 0x6,
    _512 = 0x7,
}

#[repr(u8)]
#[derive(Copy)]
pub enum HysteresisLevel {
    _0 = 0x0,
    _1 = 0x1,
    _2 = 0x2,
    _3 = 0x3,
    _4 = 0x4,
    _5 = 0x5,
    _6 = 0x6,
    _7 = 0x7,
}

#[repr(C)]
#[derive(Copy)]
pub struct CapsenseInit {
    full_bias:                   bool,
    half_bias:                   bool,
    bias_prog:                   u32,
    warm_time:                   WarmTime,
    hysteresis_level:            HysteresisLevel,
    resistor:                    CapsenseResistor,
    low_power_reference_enabled: bool,
    vdd_level:                   u32,
    enable:                      bool,
}

#[repr(C)]
#[derive(Copy)]
pub enum CapsenseResistor {
    Resistor0 = 0x0,
    Resistor1 = 0x1,
    Resistor2 = 0x2,
    Resistor3 = 0x3,
}

#[repr(C)]
#[derive(Copy)]
pub enum Channel {
    acmpChannel0        = 0x0,
    acmpChannel1        = 0x1,
    acmpChannel2        = 0x2,
    acmpChannel3        = 0x3,
    acmpChannel4        = 0x4,
    acmpChannel5        = 0x5,
    acmpChannel6        = 0x6,
    acmpChannel7        = 0x7,
    acmpChannel1V25     = 0x8,
    acmpChannel2V5      = 0x9,
    acmpChannelVDD      = 0xA,
    acmpChannelCapSense = 0xB,
    acmpChannelDAC0Ch0  = 0xC,
    acmpChannelDAC0Ch1  = 0xD,
}

#[allow(dead_code)]
extern {
    #[inline] fn GET_ACMP0() -> *mut Acmp;
    #[inline] fn GET_ACMP1() -> *mut Acmp;

    fn ACMP_Init(acmp: &Acmp, init: &Init);
    fn ACMP_Reset(acmp: &Acmp);
    fn ACMP_Disable(acmp: &Acmp);
    fn ACMP_Enable(acmp: &Acmp);
    fn ACMP_GPIOSetup(acmp: &Acmp, location: u32, enable: bool, invert: bool);
    fn ACMP_CapsenseInit(acmp: &Acmp, init: &CapsenseInit);
    fn ACMP_CapsenseChannelSet(acmp: &Acmp, channel: &Channel);
    fn ACMP_ChannelSet(acmp: &Acmp, neg_sel: Channel, pos_sel: Channel);
}