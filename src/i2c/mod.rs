
use core::intrinsics::transmute;
use core::default::Default;

pub const FREQ_STANDARD_MAX: u32 = 93500;

pub const ROUTE_SDAPEN: u32 = (0x1 << 0);
pub const ROUTE_SCLPEN: u32 = (0x1 << 1);

pub const FLAG_WRITE: u16 = 0x0001;
pub const FLAG_READ:  u16 = 0x0002;

#[repr(C)]
#[allow(non_snake_case)]
pub struct I2C {
  pub CTRL: u32,
  pub CMD: u32,
  pub STATE: u32,
  pub STATUS: u32,
  pub CLKDIV: u32,
  pub SADDR: u32,
  pub SADDRMASK: u32,
  pub RXDATA: u32,
  pub RXDATAP: u32,
  pub TXDATA: u32,
  pub IF: u32,
  pub IFS: u32,
  pub IFC: u32,
  pub IEN: u32,
  pub ROUTE: u32,
}

impl I2C {

    #[inline]
    pub fn i2c0() -> &'static mut I2C {
        unsafe { transmute(GET_I2C0()) }
    }

    pub fn init(&self, init: &Init) {
        unsafe { I2C_Init(self, init) }
    }

    pub fn transfer_init(&self, transfer_seq: &TransferSeq) -> TransferReturn {
        unsafe { I2C_TransferInit(self, transfer_seq) }
    }

    pub fn transfer(&self) -> TransferReturn {
        unsafe { I2C_Transfer(self) }
    }
    
}

pub struct Init {
    enable: bool,
    master: bool,
    ref_freq: u32,
    freq: u32,
    clhr: ClockHLR,
}

impl Default for Init {

    fn default() -> Init {
        Init {
            enable: true,
            master: true,
            ref_freq: 0,
            freq: FREQ_STANDARD_MAX,
            clhr: ClockHLR::Standard
        }
    }
}

pub enum ClockHLR {
    Standard = 0x0,
    Asymmetric = 0x1,
    Fast = 0x2,
}

pub struct TransferSeq {
    pub addr: u16,
    pub flags: u16,
    pub buf0: &'static str,
    pub buf1: &'static str,
}

pub enum TransferReturn {
  InProgress =  1,
  Done       =  0,
  Nack       = -1,
  BusErr     = -2,
  ArbLost    = -3,
  UsageFault = -4,
  SwFault    = -5
}


extern {
    fn GET_I2C0() -> &'static mut I2C;

    pub fn I2C_Init(i2c: &I2C, init: &Init);
    pub fn I2C_TransferInit(i2c: &I2C, transfer_seq: &TransferSeq) -> TransferReturn;
    pub fn I2C_Transfer(i2c: &I2C) -> TransferReturn;

}
