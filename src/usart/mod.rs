#![allow(dead_code)]

pub const ROUTE_RXPEN: u32 = 0x1 << 0;
pub const ROUTE_TXPEN: u32 = 0x1 << 1;
pub const ROUTE_CSPEN: u32 = 0x1 << 2;
pub const ROUTE_CLKPEN: u32 = 0x1 << 3;

pub const ROUTE_LOCATION_LOC1: u32 = 0x1 << 8;
pub const ROUTE_LOCATION_LOC2: u32 = 0x2 << 8;

pub const IEN_RXDATAV: u32 = 0x1 << 2;
pub const IF_RXDATAV:  u32 = 0x1 << 2;
pub const IF_MASK: u32     = 0x00001FFF;

// Usart status register
pub const USART_STATUS_RXDATAV: u32 = 0x1 << 7;
pub const USART_STATUS_TXBL: u32 = 0x1 << 6;

use core::intrinsics::transmute;
use core::default::Default;

#[repr(C)]
#[derive(Copy)]
#[allow(non_snake_case)]
pub struct Usart {
    pub CTRL: u32,
    pub FRAME: u32,
    pub TRIGCTRL: u32,
    pub CMD: u32,
    pub STATUS: u32,
    pub CLKDIV: u32,
    pub RXDATAX: u32,
    pub RXDATA: u32,
    pub RXDOUBLEX: u32,
    pub RXDOUBLE: u32,
    pub RXDATAXP: u32,
    pub RXDOUBLEXP: u32,
    pub TXDATAX: u32,
    pub TXDATA: u32,
    pub TXDOUBLEX: u32,
    pub TXDOUBLE: u32,
    pub IF: u32,
    pub IFS: u32,
    pub IFC: u32,
    pub IEN: u32,
    pub IRCTRL: u32,
    pub ROUTE: u32,
    pub INPUT: u32,
    pub I2SCTRL: u32,
}

impl Usart {

    #[inline]
    pub fn uart1() -> &'static mut Usart {
        unsafe { transmute(GET_UART1()) }
    }

    #[inline]
    pub fn usart1() -> &'static mut Usart {
        unsafe { transmute(GET_USART1()) }
    }

    pub fn enable(&self, enable: Enable) {
        unsafe { USART_Enable(self, enable) }
    }

    pub fn init_async(&mut self, init: &InitAsync) {
        unsafe { USART_InitAsync(self, init) }
    }

    pub fn int_clear(&self, flags: u32) {
        unsafe { STATIC_INLINE_USART_IntClear(self, flags) }
    }

    pub fn int_enable(&self, flags: u32) {
        unsafe { STATIC_INLINE_USART_IntEnable(self, flags) }
    }

    pub fn tx(&self, val: u8) {
        unsafe { USART_Tx(self, val) }
    }

    pub fn rx(&self) -> u8 {
        unsafe { USART_Rx(self) }
    }
}

#[repr(C)]
pub struct InitAsync {
    pub enable: Enable,
    pub ref_freq: u32,
    pub baudrate: u32,
    pub oversampling: OVS,
    pub databits: Databits,
    pub parity: Parity,
    pub stopbits: Stopbits,
    pub mv_dis: bool,
    pub prs_rx_enable: bool,
    pub prs_rx_ch: PrsRxCh,
}

impl Default for InitAsync {
    fn default() -> InitAsync {
        InitAsync {
            enable: Enable::Enable,
            ref_freq: 0,
            baudrate: 115200,
            oversampling: OVS::OVS16,
            databits: Databits::Databits8,
            parity: Parity::NoParity,
            stopbits: Stopbits::Stopbits1,
            mv_dis: false,
            prs_rx_enable: false,
            prs_rx_ch: PrsRxCh::Ch0,
        }
    }
}

#[repr(u8)]
#[derive(Copy)]
pub enum Enable {
    Disable = 0x0,
    Rx      = 0x1 << 0,
    Tx      = 0x1 << 2,
    Enable  = (0x1 << 0) | (0x1 << 2),
}

#[repr(u8)]
#[derive(Copy)]
pub enum OVS {
    OVS16 = 0x0 << 5,
    OVS8  = 0x1 << 5,
    OVS6  = 0x2 << 5,
    OVS4  = 0x3 << 5,
}

#[repr(u8)]
#[derive(Copy)]
pub enum Databits {
    Databits4  = 0x1,
    Databits5  = 0x2,
    Databits6  = 0x3,
    Databits7  = 0x4,
    Databits8  = 0x5,
    Databits9  = 0x6,
    Databits10 = 0x7,
    Databits11 = 0x8,
    Databits12 = 0x9,
    Databits13 = 0xA,
    Databits14 = 0xB,
    Databits15 = 0xC,
    Databits16 = 0xD,
}

#[repr(u16)]
#[derive(Copy)]
pub enum Parity {
    NoParity   = 0x0 << 8,
    EvenParity = 0x2 << 8,
    OddParity  = 0x3 << 8,
}

#[repr(u16)]
#[derive(Copy)]
pub enum Stopbits {
    Stopbits0p5 = 0x0 << 12,
    Stopbits1   = 0x1 << 12,
    Stopbits1p5 = 0x2 << 12,
    Stopbits2   = 0x3 << 12,
}

#[repr(u8)]
#[derive(Copy)]
pub enum PrsRxCh {
    Ch0  = 0x0,
    Ch1  = 0x1,
    Ch2  = 0x2,
    Ch3  = 0x3,
    Ch4  = 0x4,
    Ch5  = 0x5,
    Ch6  = 0x6,
    Ch7  = 0x7,
    Ch8  = 0x8,
    Ch9  = 0x9,
    Ch10 = 0xA,
    Ch11 = 0xB,
}

extern {
    #[inline] fn GET_UART1() -> *mut Usart;
    #[inline] fn GET_USART1() -> *mut Usart;

    #[inline] fn USART_InitAsync(usart: &Usart, init: &InitAsync);
    #[inline] fn USART_Rx(usart: &Usart) -> u8;
    #[inline] fn USART_Tx(usart: &Usart, data: u8);

    #[inline] fn USART_Enable(usart: &Usart, enable: Enable);

    #[inline] fn STATIC_INLINE_USART_IntClear(usart: &Usart, data: u32);
    #[inline] fn STATIC_INLINE_USART_IntEnable(usart: &Usart, data: u32);
}
