#![allow(dead_code)]

use libc::c_void;
use core::intrinsics::transmute;
use core::prelude::*;
use core::ptr;
use dma;

pub static mut dma0: Dma = Dma {
    device: dma::DMA{ channel: 0 },
    callback: None
};

pub static mut DMA0_CB: CB = CB {
    cb_func: transfer_complete,
    user_ptr: unsafe { &dma0 },
    primary: 0
};

pub extern fn transfer_complete(_channel: u32, _primary: bool, user: *mut Dma) {

    let dma: &mut Dma = unsafe { transmute(user) };

    match dma.callback {
        Some(func) => {
            dma.callback = None;
            func(dma)
        },
        _ => ()
    }
}

pub trait Readable {
    fn as_ptr(&self) -> *mut c_void;
    fn inc_size(&self) -> dma::DataInc;
    fn size(&self) -> dma::DataSize;
}

pub trait Writable {
    fn as_ptr(&self) -> *mut c_void;
    fn inc_size(&self) -> dma::DataInc;
    fn size(&self) -> dma::DataSize;
}

#[derive(Copy)]
pub enum Signal {
    AdcSingle,
    AdcScan
}

impl Signal {
    fn get(&self) -> u32 {
        dma::REQ_ADC0_SINGLE
    }
}

pub struct Dma {
    pub device: dma::DMA,
    pub callback: Option<fn(&mut Dma)>
}

pub fn init() {
    dma::init(&dma::Init {
        hprot: 0,
        control_block: dma::dma_control_block(),
    });
}

impl Dma {

    pub fn start_basic(&mut self, src: &Readable, dst: &Writable, on: Signal) -> &mut Dma {

        self.device.configure_channel(&dma::CfgChannel {
            high_pri: true,
            enable_int: true,
            select: on.get(),
            cb: unsafe { transmute(&DMA0_CB) }
        });

        self.device.configure_descriptor(true, &dma::CfgDescriptor {
            dst_inc: dst.inc_size(),
            src_inc: src.inc_size(),
            size: dst.size(),
            arb_rate: dma::ArbiterConfig::Arbitrate1,
            hprot: 0
        });

        self.device.activate_basic::<u8>(true, false, dst.as_ptr(), src.as_ptr(), self.size() - 1);

        self
    }

    pub fn then(&mut self, callback: fn(&mut Dma)) -> &mut Dma {
        self.callback = Some(callback);

        self
    }

    /*pub fn register(&self) {
        unsafe { CB.user_ptr = transmute(self) }
    }*/

    pub fn refresh(&mut self) -> &mut Dma {

        let null: *mut c_void = unsafe { transmute(ptr::null::<u8>()) };
        self.device.activate_basic::<u8>(true, false, null, null, self.size() - 1);

        self
    }

    fn size(&self) -> u32 {
        512
    }
}

pub type FuncPtr = extern fn(channel: u32, primary: bool, user: *mut Dma);

#[repr(C)]
#[derive(Copy)]
pub struct CB {
    pub cb_func: FuncPtr,
    pub user_ptr: *const Dma,
    pub primary: u8,
}

#[derive(Copy)]
pub struct Buffer {
    pub buffer: &'static [u8]
}

impl Writable for Buffer {

    fn as_ptr(&self) -> *mut c_void {
        unsafe { transmute(self.buffer.as_ptr()) }
    }

    fn inc_size(&self) -> dma::DataInc {
        dma::DataInc::Inc1
    }

    fn size(&self) -> dma::DataSize {
        dma::DataSize::Size1
    }
}

impl Readable for Buffer {

    fn as_ptr(&self) -> *mut c_void {
        unsafe { transmute(self.buffer.as_ptr()) }
    }

    fn inc_size(&self) -> dma::DataInc {
        dma::DataInc::Inc1
    }

    fn size(&self) -> dma::DataSize {
        dma::DataSize::Size1
    }
}
