#![allow(dead_code)]

pub const ROUTE_RXPEN: u32 = 0x1 << 0;
pub const ROUTE_TXPEN: u32 = 0x1 << 1;

pub const ROUTE_LOCATION_LOC0: u32 = 0x0 << 8;
pub const ROUTE_LOCATION_LOC1: u32 = 0x1 << 8;
pub const ROUTE_LOCATION_LOC2: u32 = 0x2 << 8;
pub const ROUTE_LOCATION_LOC3: u32 = 0x3 << 8;
pub const ROUTE_LOCATION_LOC4: u32 = 0x4 << 8;

pub const IF_TXC: u32 = 0x1 << 0;
pub const IF_TXBL: u32 = 0x1 << 1;
pub const IF_RXDATAV: u32 = 0x1 << 2;
pub const IF_RXOF: u32 = 0x1 << 3;
pub const IF_RXUF: u32 = 0x1 << 4;
pub const IF_TXOF: u32 = 0x1 << 5;
pub const IF_PERR: u32 = 0x1 << 6;
pub const IF_FERR: u32 = 0x1 << 7;
pub const IF_MPAF: u32 = 0x1 << 8;
pub const IF_STARTF: u32 = 0x1 << 9;
pub const IF_SIGF: u32 = 0x1 << 10;

pub const IEN_TXC: u32 = 0x1 << 0;
pub const IEN_TXBL: u32 = 0x1 << 1;
pub const IEN_RXDATAV: u32 = 0x1 << 2;
pub const IEN_RXOF: u32 = 0x1 << 3;
pub const IEN_RXUF: u32 = 0x1 << 4;
pub const IEN_TXOF: u32 = 0x1 << 5;
pub const IEN_PERR: u32 = 0x1 << 6;
pub const IEN_FERR: u32 = 0x1 << 7;
pub const IEN_MPAF: u32 = 0x1 << 8;
pub const IEN_STARTF: u32 = 0x1 << 9;
pub const IEN_SIGF: u32 = 0x1 << 10;

use core::intrinsics::transmute;
use core::default::Default;

#[repr(C)]
#[derive(Copy)]
#[allow(non_snake_case)]
pub struct Leuart {
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

impl Leuart {
    #[inline]
    pub fn leuart0() -> &'static mut Leuart {
        unsafe { transmute(GET_LEUART0()) }
    }

    #[inline]
    pub fn leuart1() -> &'static mut Leuart {
        unsafe { transmute(GET_LEUART1()) }
    }

    pub fn enable(&self, enable: Enable) {
        unsafe { LEUART_Enable(self, enable) }
    }

    pub fn init(&mut self, init: &Init) {
        unsafe { LEUART_Init(self, init) }
    }

    pub fn reset(&self) {
        unsafe { LEUART_Reset(self); }
    }

    pub fn rx(&self) -> u8 {
        unsafe { LEUART_Rx(self) }
    }

    pub fn rx_ext(&self) -> u8 {
        unsafe { LEUART_Rx(self) }
    }

    pub fn tx(&self, val: u8) {
        unsafe { LEUART_Tx(self, val); }
    }

    pub fn tx_ext(&self, val: u8) {
        unsafe { LEUART_Tx(self, val); }
    }

    pub fn int_clear(&self, flags: u32) {
        unsafe { STATIC_INLINE_LEUART_IntClear(self, flags); }
    }

    pub fn int_enable(&self, flags: u32) {
        unsafe { STATIC_INLINE_LEUART_IntEnable(self, flags); }
    }

    pub fn int_disable(&self, flags: u32) {
        unsafe { STATIC_INLINE_LEUART_IntDisable(self, flags); }
    }

    pub fn int_get(&self) -> u32 {
        unsafe { STATIC_INLINE_LEUART_IntGet(self) }
    }

    pub fn int_set(&self, flags: u32) {
        unsafe { STATIC_INLINE_LEUART_IntSet(self, flags); }
    }
}

#[repr(C)]
pub struct Init {
    pub enable: Enable,
    pub ref_freq: u32,
    pub baudrate: u32,
    pub databits: Databits,
    pub parity: Parity,
    pub stopbits: Stopbits
}

impl Default for Init {
    fn default() -> Init {
        Init {
            enable: Enable::Enable,
            ref_freq: 0,
            baudrate: 9600,
            databits: Databits::Databits8,
            parity: Parity::NoParity,
            stopbits: Stopbits::Stopbits1,
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
pub enum Databits {
    Databits8  = 0x0 << 1,
    Databits9  = 0x1 << 1,
}

#[repr(u16)]
#[derive(Copy)]
pub enum Parity {
    NoParity   = 0x0 << 2,
    EvenParity = 0x2 << 2,
    OddParity  = 0x3 << 2,
}

#[repr(u16)]
#[derive(Copy)]
pub enum Stopbits {
    Stopbits1   = 0x1 << 4,
    Stopbits2   = 0x3 << 4,
}

extern {
    #[inline] fn GET_LEUART0() -> *mut Leuart;
    #[inline] fn GET_LEUART1() -> *mut Leuart;

    #[inline] fn LEUART_BaudrateCalc(refFreq: u32, clkdiv: u32) -> u32;
    #[inline] fn LEUART_BaudrateGet(leuart: &Leuart) -> u32;
    #[inline] fn LEUART_BaudrateSet(leuart: &Leuart, refFreq: u32, baudrate: u32);
    #[inline] fn LEUART_Enable(leuart: &Leuart, enable: Enable);
    #[inline] fn LEUART_FreezeEnable(leuart: &Leuart, enable: bool);
    #[inline] fn LEUART_Init(leuart: &Leuart, init: &Init);
    #[inline] fn LEUART_TxDmaInEM2Enable(leuart: &Leuart, enable: bool);
    #[inline] fn LEUART_RxDmaInEM2Enable(leuart: &Leuart, enable: bool);

    #[inline] fn LEUART_Reset(leuart: &Leuart);
    #[inline] fn LEUART_Rx(leuart: &Leuart) -> u8;
    #[inline] fn LEUART_RxExt(leuart: &Leuart) -> u8;
    #[inline] fn LEUART_Tx(leuart: &Leuart, data: u8);
    #[inline] fn LEUART_TxExt(leuart: &Leuart, data: u8);

    #[inline] fn STATIC_INLINE_LEUART_IntClear(leuart: &Leuart, flags: u32);
    #[inline] fn STATIC_INLINE_LEUART_IntEnable(leuart: &Leuart, flags: u32);
    #[inline] fn STATIC_INLINE_LEUART_IntDisable(leuart: &Leuart, flags: u32);
    #[inline] fn STATIC_INLINE_LEUART_IntGet(leuart: &Leuart) -> u32;
    #[inline] fn STATIC_INLINE_LEUART_IntSet(leuart: &Leuart, flags: u32);
}
