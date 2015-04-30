#![no_std]
#![feature(core, no_std)]

extern crate core;
extern crate libc;
extern crate emlib;

use core::intrinsics::transmute;

use emlib::{chip, dma};

static mut DATA_SRC: [u16; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
static mut DATA_DST: [u16; 8] = [0, 0, 0, 0, 0, 0, 0, 0];

fn main() {

    chip::init();

    setup_dma();

    let dma0 = dma::DMA::channel0();
    unsafe { dma0.activate_auto::<u16>(true, transmute(&mut DATA_DST), transmute(&mut DATA_SRC), 8); }

    loop {}
}

fn setup_dma() {

    dma::init(&dma::Init{
        hprot: 0,
        control_block: dma::dma_control_block(),
    });

    let dma0 = dma::DMA::channel0();
    dma0.configure_channel(&dma::CfgChannel {
        high_pri: false,
        enable_int: false,
        select: 0,
        cb: dma::null_cb()
    });
    dma0.configure_descriptor(true, &dma::CfgDescriptor {
        dst_inc: dma::DataInc::Inc4,
        src_inc: dma::DataInc::Inc4,
        size: dma::DataSize::Size4,
        arb_rate: dma::ArbiterConfig::Arbitrate1,
        hprot: 0,
    });
}
