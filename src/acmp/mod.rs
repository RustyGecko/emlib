use core::intrinsics::transmute;
use core::default::Default;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Init {
    pub full_bias:                   bool,
    pub half_bias:                   bool,
    pub bias_prog:                   u32,
    pub interrupt_on_falling_edge:   bool,
    pub interrupt_on_rising_edge:    bool,
    pub warm_time:                   WarmTime,
    pub hysteresis_level:            HysteresisLevel,
    pub inactive_value:              bool,
    pub low_power_reference_enabled: bool,
    pub vdd_level:                   u32,
    pub enable:                      bool,
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
#[derive(Copy, Clone)]
pub struct Acmp {
    pub CTRL:     u32,
    pub INPUTSEL: u32,
    pub STATUS:   u32,
    pub IEN:      u32,
    pub IF:       u32,
    pub IFS:      u32,
    pub IFC:      u32,
    pub ROUTE:    u32,
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

    pub fn init(&self, init: &Init) {
        unsafe { ACMP_Init(self, init) }
    }

    pub fn gpio_setup(&self, location: u32, enable: bool, invert: bool) {
        unsafe { ACMP_GPIOSetup(self, location, enable, invert) }
    }

    pub fn channel_set(&self, neg_sel: Channel, pos_sel: Channel) {
        unsafe { ACMP_ChannelSet(self, neg_sel, pos_sel) }
    }
}

#[repr(u8)]
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
pub struct CapsenseInit {
    pub full_bias:                   bool,
    pub half_bias:                   bool,
    pub bias_prog:                   u32,
    pub warm_time:                   WarmTime,
    pub hysteresis_level:            HysteresisLevel,
    pub resistor:                    CapsenseResistor,
    pub low_power_reference_enabled: bool,
    pub vdd_level:                   u32,
    pub enable:                      bool,
}

impl Default for CapsenseInit {
    fn default() -> CapsenseInit {
        CapsenseInit {
            full_bias:                   false,
            half_bias:                   false,
            bias_prog:                   0x7,
            warm_time:                   WarmTime::_512,
            hysteresis_level:            HysteresisLevel::_5,
            resistor:                    CapsenseResistor::_3,
            low_power_reference_enabled: false,
            vdd_level:                   0x3D,
            enable:                      true,
        }
    }
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum CapsenseResistor {
    _0 = 0x0,
    _1 = 0x1,
    _2 = 0x2,
    _3 = 0x3,
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum Channel {
    _0        = 0x0,
    _1        = 0x1,
    _2        = 0x2,
    _3        = 0x3,
    _4        = 0x4,
    _5        = 0x5,
    _6        = 0x6,
    _7        = 0x7,
    _1V25     = 0x8,
    _2V5      = 0x9,
    VDD       = 0xA,
    CapSense  = 0xB,
    DAC0Ch0   = 0xC,
    DAC0Ch1   = 0xD,
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