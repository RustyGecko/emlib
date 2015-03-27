#![no_std]
#![no_main]
#![feature(core, no_std)]

extern crate core;
extern crate emlib;

use emlib::{chip, dma};

static mut DATA_SRC: [u16; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
static mut DATA_DST: [u16; 8] = [0, 0, 0, 0, 0, 0, 0, 0];

static CB: dma::CB = dma::CB {
    cb_func: transfer_complete,
    user_ptr: 0,
    primary: 0
};

#[no_mangle]
pub extern fn main() {

    chip::init();

    setup_dma();

    let dma0 = dma::DMA::channel0();
    unsafe { dma0.activate_auto(true, &mut DATA_DST, &mut DATA_SRC, 8); }

    loop {}
}

extern fn transfer_complete(_channel: u32, _primary: bool, _user: u32) {}

fn setup_dma() {

    dma::init(&dma::Init{
        hprot: 0,
        control_block: dma::dma_control_block(),
    });

    let dma0 = dma::DMA::channel0();
    dma0.configure_channel(&dma::CfgChannel {
        high_pri: false,
        enable_int: true,
        select: 0,
        cb: &CB,
    });
    dma0.configure_descriptor(true, &dma::CfgDescriptor {
        dst_inc: dma::DataInc::Inc4,
        src_inc: dma::DataInc::Inc4,
        size: dma::DataSize::Size4,
        arb_rate: dma::ArbiterConfig::Arbitrate1,
        hprot: 0,
    });
}
