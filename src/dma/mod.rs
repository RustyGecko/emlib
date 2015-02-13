#![allow(dead_code)]


use core::intrinsics::transmute;
use core::slice::SliceExt;
use core::cmp::min;

#[repr(u8)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum c_void {
    __variant1,
    __variant2,
}

pub const DMAREQ_TIMER0_UFOF: u32 = ((24 << 16) + 0);

pub type FuncPtr = extern fn(channel: u32, primary: bool, user: u32);

#[derive(Copy)]
pub struct DMA { channel: u32 }

impl DMA {
    pub fn channel0() -> DMA {
        DMA { channel: 0 }
    }

    pub fn configure_channel(&self, cfg: &CfgChannel) {
        unsafe { DMA_CfgChannel(self.channel, cfg) }
    }

    pub fn configure_descriptor(&self, primary: bool, cfg: &CfgDescriptor) {
        unsafe { DMA_CfgDescr(self.channel, primary, cfg) }
    }

    pub fn activate_auto<T>(&self, primary: bool, dst: &'static mut[T], src: &'static mut[T]) {
        unsafe {

            let n = min(dst.len(), src.len());
            
            DMA_ActivateAuto(
                self.channel,
                primary,
                transmute(dst.as_ptr()),
                transmute(src.as_ptr()),
                n - 1
            );
        }
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct Init {
    pub hprot: u8,
    pub control_block: &'static Descriptor,
}

#[repr(C)]
#[derive(Copy)]
pub struct CfgChannel {
    pub high_pri: bool,
    pub enable_int: bool,
    pub select: u32,
    pub cb: &'static CB,
}

#[repr(C)]
#[derive(Copy)]
pub struct CfgDescriptor {
    pub dst_inc: DataInc,
    pub src_inc: DataInc,
    pub size: DataSize,
    pub arb_rate: ArbiterConfig,
    pub hprot: u8,
}

#[repr(C)]
#[derive(Copy)]
#[allow(non_snake_case)]
pub struct Descriptor {
    pub SRCEND: u32,
    pub DSTEND: u32,
    pub CTRL: u32,
    pub USER: u32,
}

#[repr(C)]
#[derive(Copy)]
pub struct CB {
    pub cb_func: FuncPtr,
    pub user_ptr: u32,
    pub primary: u8,
}

#[repr(u8)]
#[derive(Copy)]
pub enum DataInc {
    Inc1 = 0x0,
    Inc2 = 0x1,
    Inc4 = 0x2,
    IncNone = 0x3,
}

#[repr(u8)]
#[derive(Copy)]
pub enum DataSize {
    Size1 = 0x0,
    Size2 = 0x1,
    Size4 = 0x2,
}

#[repr(u8)]
#[derive(Copy)]
pub enum ArbiterConfig {
    Arbitrate1 = 0x0,
    Arbitrate2 = 0x1,
    Arbitrate4 = 0x2,
    Arbitrate8 = 0x3,
    Arbitrate16 = 0x4,
    Arbitrate32 = 0x5,
    Arbitrate64 = 0x6,
    Arbitrate128 = 0x7,
    Arbitrate256 = 0x8,
    Arbitrate512 = 0x9,
    Arbitrate1024 = 0xa,
}

pub fn init(init: &Init) {
    unsafe { DMA_Init(init) }
}

pub fn dma_control_block() -> &'static Descriptor {
    unsafe { transmute(GET_DMA_CONTROL_BLOCK()) }
}


extern {
    fn GET_DMA_CONTROL_BLOCK() -> &'static Descriptor;

    #[allow(warnings)]
    fn DMA_Init(init: *const Init);

    #[allow(warnings)]
    fn DMA_CfgChannel(channel: u32, cfg: *const CfgChannel);

    #[allow(warnings)]
    fn DMA_CfgDescr(channel: u32, primary: bool, cfg: *const CfgDescriptor);
    fn DMA_ActivateAuto(
        channel: u32,
        use_burst: bool,
        dst: *mut c_void,
        src: *mut c_void,
        n_minus_1: usize);
}
