#[derive(Copy)]
#[repr(C)]
pub enum Port {
    A = 0, B = 1, C = 2,
    D = 3, E = 4, F = 5,
}

#[derive(Copy)]
#[repr(C)]
pub enum DriveMode {
    Standard = 0x0,
    Lowest   = 0x1,
    High     = 0x2,
    Low      = 0x3,
}

#[derive(Copy)]
#[repr(C)]
pub enum Mode {
    Disabled                  = 0x0,
    Input                     = 0x1,
    InputPull                 = 0x2,
    InputPullFilter           = 0x3,
    PushPull                  = 0x4,
    PushPullDrive             = 0x5,
    WiredOr                   = 0x6,
    WiredOrPullDown           = 0x7,
    WiredAnd                  = 0x8,
    WiredAndFilter            = 0x9,
    WiredAndPullUp            = 0xA,
    WiredAndPullUpFilter      = 0xB,
    WiredAndDrive             = 0xC,
    WiredAndDriveFilter       = 0xD,
    WiredAndDrivePullUp       = 0xE,
    WiredAndDrivePullUpFilter = 0xF,
}
extern {
    pub fn GPIO_DbgLocationSet(location: u32);

    pub fn GPIO_IntConfig(
        port: Port,
        pin: u32,
        risingEdge: bool,
        fallingEdge: bool,
        enable: bool
    );
    
    pub fn GPIO_PinModeSet(
        port: Port,
        pins: u32,
        mode: Mode,
        out: u32
    );

    pub fn GPIO_DriveModeSet(port: Port, mode: DriveMode);
    pub fn STATIC_INLINE_GPIO_DbgSWDClkEnable(enable: bool);
    pub fn STATIC_INLINE_GPIO_DbgSWDIOEnable(enable: bool);
    pub fn STATIC_INLINE_GPIO_DbgSWOEnable(enable: bool);
    pub fn STATIC_INLINE_GPIO_EM4DisablePinWakeup(pinmask: u32);
    pub fn STATIC_INLINE_GPIO_EM4EnablePinWakeup(pinmask: u32, polaritymask: u32);
    pub fn STATIC_INLINE_GPIO_EM4GetPinWakeupCause() -> u32;
    pub fn STATIC_INLINE_GPIO_EM4SetPinRetention(enable: bool);
    pub fn STATIC_INLINE_GPIO_InputSenseSet(val: u32, mask: u32);
    pub fn STATIC_INLINE_GPIO_IntClear(flags: u32);
    pub fn STATIC_INLINE_GPIO_IntDisable(flags: u32);
    pub fn STATIC_INLINE_GPIO_IntEnable(flags: u32);
    pub fn STATIC_INLINE_GPIO_IntGet() -> u32;
    pub fn STATIC_INLINE_GPIO_IntGetEnabled() -> u32;
    pub fn STATIC_INLINE_GPIO_IntSet(flags: u32);
    pub fn STATIC_INLINE_GPIO_Lock();
    pub fn STATIC_INLINE_GPIO_PinInGet(port: Port, pin: u32) -> u32;
    pub fn STATIC_INLINE_GPIO_PinOutClear(port: Port, pin: u32);
    pub fn STATIC_INLINE_GPIO_PinOutGet(port: Port, pin: u32) -> u32;
    pub fn STATIC_INLINE_GPIO_PinOutSet(port: Port, pin: u32);
    pub fn STATIC_INLINE_GPIO_PinOutToggle(port: Port, pin: u32);
    pub fn STATIC_INLINE_GPIO_PortInGet(port: Port) -> u32;
    pub fn STATIC_INLINE_GPIO_PortOutClear(port: Port, pins: u32);
    pub fn STATIC_INLINE_GPIO_PortOutGet(port: Port) -> u32;
    pub fn STATIC_INLINE_GPIO_PortOutSet(port: Port, pins: u32);
    pub fn STATIC_INLINE_GPIO_PortOutSetVal(port: Port, val: u32, mask: u32);
    pub fn STATIC_INLINE_GPIO_PortOutToggle(port: Port, pins: u32);
    pub fn STATIC_INLINE_GPIO_Unlock();
}

pub fn dbg_location_set(location: u32) {
    unsafe { GPIO_DbgLocationSet(location) }
}

pub fn int_config(port: Port, pin: u32, rising_edge: bool, falling_edge: bool, enable: bool) {
    unsafe { GPIO_IntConfig(port, pin, rising_edge, falling_edge, enable) }
}

pub fn pin_mode_set(port: Port, pins: u32, mode: Mode, out: u32) {
    unsafe { GPIO_PinModeSet(port, pins, mode, out) }
}

pub fn drive_mode_set(port: Port, mode: DriveMode) {
    unsafe { GPIO_DriveModeSet(port, mode) }
}

pub fn dbg_swd_clk_enable(enable: bool) {
    unsafe { STATIC_INLINE_GPIO_DbgSWDClkEnable(enable) }
}

pub fn dbg_swdio_enable(enable: bool) {
    unsafe { STATIC_INLINE_GPIO_DbgSWDIOEnable(enable) }
}

pub fn dbg_swo_enable(enable: bool) {
    unsafe { STATIC_INLINE_GPIO_DbgSWOEnable(enable) }
}

pub fn em4_disable_pin_wakeup(pinmask: u32) {
    unsafe { STATIC_INLINE_GPIO_EM4DisablePinWakeup(pinmask) }
}

pub fn em4_enable_pin_wakeup(pinmask: u32, polaritymask: u32) {
    unsafe { STATIC_INLINE_GPIO_EM4EnablePinWakeup(pinmask, polaritymask) }
}

pub fn em4_get_pin_wakeup_cause() -> u32 {
    unsafe { STATIC_INLINE_GPIO_EM4GetPinWakeupCause() }
}

pub fn em4_set_pin_retention(enable: bool) {
    unsafe { STATIC_INLINE_GPIO_EM4SetPinRetention(enable) }
}

pub fn input_sense_set(val: u32, mask: u32) {
    unsafe { STATIC_INLINE_GPIO_InputSenseSet(val, mask) }
}

pub fn int_clear(flags: u32) {
    unsafe { STATIC_INLINE_GPIO_IntClear(flags) }
}

pub fn int_disable(flags: u32) {
    unsafe { STATIC_INLINE_GPIO_IntDisable(flags) }
}

pub fn int_enable(flags: u32) {
    unsafe { STATIC_INLINE_GPIO_IntEnable(flags) }
}

pub fn int_get() -> u32 {
    unsafe { STATIC_INLINE_GPIO_IntGet() }
}

pub fn int_get_enabled() -> u32 {
    unsafe { STATIC_INLINE_GPIO_IntGetEnabled() }
}

pub fn int_set(flags: u32) {
    unsafe { STATIC_INLINE_GPIO_IntSet(flags) }
}

pub fn lock() {
    unsafe { STATIC_INLINE_GPIO_Lock() }
}

pub fn pin_in_get(port: Port, pin: u32) -> u32 {
    unsafe { STATIC_INLINE_GPIO_PinInGet(port, pin) }
}

pub fn pin_out_set(port: Port, pins: u32) {
    unsafe { STATIC_INLINE_GPIO_PinOutSet(port, pins) }
}

pub fn pin_out_get(port: Port, pin: u32) -> u32 {
    unsafe { STATIC_INLINE_GPIO_PinOutGet(port, pin) }
}

pub fn pin_out_clear(port: Port, pins: u32) {
    unsafe { STATIC_INLINE_GPIO_PinOutClear(port, pins) }
}

pub fn pin_out_toggle(port: Port, pins: u32) {
    unsafe { STATIC_INLINE_GPIO_PinOutToggle(port, pins) }
}

pub fn port_in_get(port: Port) -> u32 {
    unsafe { STATIC_INLINE_GPIO_PortInGet(port) }
}

pub fn port_out_clear(port: Port, pins: u32) {
    unsafe { STATIC_INLINE_GPIO_PortOutClear(port, pins) }
}

pub fn port_out_get(port: Port) -> u32 {
    unsafe { STATIC_INLINE_GPIO_PortOutGet(port) }
}

pub fn port_out_set(port: Port, pins: u32) {
    unsafe { STATIC_INLINE_GPIO_PortOutSet(port, pins) }
}

pub fn port_out_set_val(port: Port, val: u32, mask: u32) {
    unsafe { STATIC_INLINE_GPIO_PortOutSetVal(port, val, mask) }
}

pub fn port_out_toggle(port: Port, pins: u32) {
    unsafe { STATIC_INLINE_GPIO_PortOutToggle(port, pins) }
}

pub fn unlock() {
    unsafe { STATIC_INLINE_GPIO_Unlock() }
}

