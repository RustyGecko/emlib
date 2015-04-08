#![allow(dead_code)]
use core::intrinsics::transmute;
use libc::c_void;

pub const REQ_ADC0_SINGLE: u32    = ((8 << 16) + 0);
pub const REQ_ADC0_SCAN: u32      = ((8 << 16) + 1);
pub const REQ_USART1_RXDATAV: u32 = ((13 << 16) + 0);
pub const REQ_USART1_TXBL: u32    = ((13 << 15) + 1);
pub const DMAREQ_TIMER0_UFOF: u32 = ((24 << 16) + 0);

pub type FuncPtr = extern fn(channel: u32, primary: bool, user: *mut c_void);


#[derive(Copy, Clone)]
pub struct DMA { pub channel: u32 }

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

    pub fn activate_auto<T>(&self, primary: bool, dst: *mut c_void, src: *mut c_void, n: u32) {
        unsafe {

            DMA_ActivateAuto(
                self.channel,
                primary,
                dst,
                src,
                n - 1
            );
        }
    }

    pub fn activate_basic<T>(&self, primary: bool, use_burst: bool, dst: *mut c_void, src: *mut c_void, n_minus_1: u32) {

        unsafe {
            DMA_ActivateBasic(
                self.channel,
                primary,
                use_burst,
                dst,
                src,
                n_minus_1
            );
        }
    }

    pub fn activate_ping_pong<T>(
        &self,
        use_burst: bool,
        prim_dst: *mut c_void,
        prim_src: *mut c_void,
        prim_n_minus_1: u32,
        alt_dst: *mut c_void,
        alt_src: *mut c_void,
        alt_n_minus_1: u32) {

        unsafe {
            DMA_ActivatePingPong(
                self.channel,
                use_burst,
                prim_dst,
                prim_src,
                prim_n_minus_1,
                alt_dst,
                alt_src,
                alt_n_minus_1
            );
        };
    }

    pub fn refresh_ping_pong<T>(
        &self,
        primary: bool,
        use_burst: bool,
        dst: *mut c_void,
        src: *mut c_void,
        n_minus_1: u32,
        stop: bool) {

        unsafe {
            DMA_RefreshPingPong(
                self.channel,
                primary,
                use_burst,
                dst,
                src,
                n_minus_1,
                stop
            );
        };
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Init {
    pub hprot: u8,
    pub control_block: &'static Descriptor,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CfgChannel {
    pub high_pri: bool,
    pub enable_int: bool,
    pub select: u32,
    pub cb: &'static CB,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CfgDescriptor {
    pub dst_inc: DataInc,
    pub src_inc: DataInc,
    pub size: DataSize,
    pub arb_rate: ArbiterConfig,
    pub hprot: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
#[allow(non_snake_case)]
pub struct Descriptor {
    pub SRCEND: u32,
    pub DSTEND: u32,
    pub CTRL: u32,
    pub USER: u32,
}

// FIXME: Removed derive_Copy because it requires Clone, and FuncPtr cant inherit Clone. We need to
// figure out what to do here...
#[repr(C)]
pub struct CB {
    pub cb_func: FuncPtr,
    pub user_ptr: *const c_void,
    pub primary: u8,
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum DataInc {
    Inc1 = 0x0,
    Inc2 = 0x1,
    Inc4 = 0x2,
    IncNone = 0x3,
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum DataSize {
    Size1 = 0x0,
    Size2 = 0x1,
    Size4 = 0x2,
}

#[repr(u8)]
#[derive(Copy, Clone)]
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


#[allow(warnings)]
extern {
    fn GET_DMA_CONTROL_BLOCK() -> &'static Descriptor;

    fn DMA_Init(init: *const Init);

    fn DMA_CfgChannel(channel: u32, cfg: *const CfgChannel);

    fn DMA_CfgDescr(channel: u32, primary: bool, cfg: *const CfgDescriptor);
    fn DMA_ActivateAuto(
        channel: u32,
        primary: bool,
        dst: *mut c_void,
        src: *mut c_void,
        n_minus_1: u32);

    fn DMA_ActivateBasic(
        channel: u32,
        primary: bool,
        use_burst: bool,
        dst: *mut c_void,
        src: *mut c_void,
        n_minus_1: u32);

    fn DMA_ActivatePingPong(
        channel: u32,
        use_burst: bool,
        prim_dst: *mut c_void,
        prim_src: *mut c_void,
        prim_n_minus_1: u32,
        alt_dst: *mut c_void,
        alt_src: *mut c_void,
        alt_n_minus_1: u32);

    fn DMA_RefreshPingPong(
        channel: u32,
        primary: bool,
        use_burst: bool,
        dst: *mut c_void,
        src: *mut c_void,
        n_minus_1: u32,
        stop: bool);

}
