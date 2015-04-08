#[repr(C)]
#[derive(Copy, Clone)]
pub enum Clock {
    ACMP0       = 0x29200,
    ACMP1       = 0x2a200,
    ADC0        = 0x30200,
    LFA         = 0x60002,
    LFB         = 0x120003,
    I2C0        = 0x2b200,
    CORE        = 0x40020,
    DMA         = 0x40300,
    HFPER       = 0x28110,
    GPIO        = 0x2d200,
    LESENSE     = 0x100430,
    CORELE      = 0x44300,
    HF          = 0x51,
    TIMER0      = 0x25200,
    USART0      = 0x20200,
    USART1      = 0x21200,
    USART2      = 0x22200,
    UART1       = 0x24200,
    PRS         = 0x2f200,
    RTC         = 0x81430,
    LEUART0     = 0x140540,
    LEUART1     = 0x161540,
}

/** High frequency RC bands. */
#[repr(C)]
#[derive(Copy, Clone)]
pub enum HFRCOBand {
    _1MHz  = 0x00000000,
    _7MHz  = 0x00000001,
    _11MHz = 0x00000002,
    _14MHz = 0x00000003,
    _21MHz = 0x00000004,
    _28MHz = 0x00000005,
}

/** Oscillator types. */
#[repr(C)]
#[derive(Copy, Clone)]
pub enum Osc {
    LFXO,
    LFRCO,
    HFXO,
    HFRCO,
    AUXHFRCO,
    ULFRCO,
}

/** Selectable clock sources. */
#[repr(C)]
#[derive(Copy, Clone)]
pub enum Select {
    Error,
    Disabled,
    LFXO,
    LFRCO,
    HFXO,
    HFRCO,
    CORELEDIV2,
    AUXHFRCO,
    HFCLK,
    ULFRCO,
}

extern {
    pub fn CMU_ClockEnable(clock: Clock, enable: bool);
    pub fn CMU_ClockFreqGet(clock: Clock) -> u32;
    pub fn CMU_ClockSelectGet(clock: Clock) -> Select;
    pub fn CMU_ClockSelectSet(clock: Clock, reference: Select);
    pub fn CMU_ClockDivSet(clock: Clock, div: u32);
    pub fn CMU_HFRCOBandSet(band: HFRCOBand);
    pub fn CMU_OscillatorEnable(osc: Osc, enable: bool, wait: bool);
}

pub fn clock_enable(clock: Clock, enable: bool) {
    unsafe { CMU_ClockEnable(clock, enable) }
}

pub fn clock_freq_get(clock: Clock) -> u32 {
    unsafe { CMU_ClockFreqGet(clock) }
}

pub fn clock_select_get(clock: Clock) -> Select {
    unsafe { CMU_ClockSelectGet(clock) }
}

pub fn clock_select_set(clock: Clock, reference: Select) {
    unsafe { CMU_ClockSelectSet(clock, reference) }
}

pub fn clock_div_set(clock: Clock, div: u32) {
    unsafe { CMU_ClockDivSet(clock, div) }
}

pub fn hfrco_band_set(band: HFRCOBand) {
    unsafe { CMU_HFRCOBandSet(band) }
}

pub fn oscillator_enable(osc: Osc, enable: bool, wait: bool) {
    unsafe { CMU_OscillatorEnable(osc, enable, wait) }
}
