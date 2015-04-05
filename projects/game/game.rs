#![allow(warnings)]

use core::default::Default;

use emlib::ebi;
use emlib::ebi::{TFTInit};
use emlib::cmu;
use emlib::gpio;
use emlib::cmsis;
use emlib::cmsis::nvic;
use emlib::emdrv::tft;

use emlib::modules::{Usart, Config, Location};
use emlib::modules::{Button, GpioPin};
use emlib::dk::{bc, bsp};

pub mod gamepad;
pub mod utils;
pub mod display;

// Keep track of horizontal offset
static mut hz_offset: u32 = 0;
static mut h_pos: u32 = 0;
static mut frame_ctr: u32 = 0;

#[no_mangle]
pub unsafe extern fn EBI_IRQHandler() {
    let flags = ebi::int_get();
    ebi::int_clear(flags);

    let mut line_number: u32 = 0;

    // Process vertical sync interrupt
    if ((flags & ebi::IF_VFPORCH) != 0) {
        // Keep track of number of frames drawn
        frame_ctr += 1;

        // Increase this increment to 2/4/8 to increase scroll speed
        hz_offset += 1;

        // TODO: Not sure if this if-statement is required or not. What does it do?
        // Wrap around when a full screen has been displayed
        // if (hz_offset == (D_WIDTH + font16x28.c_width)) {
        //     hz_offset = 0;
        // }
    }

    // Process horizontal sync interrupt
    if ((flags & ebi::IF_HSYNC) != 0) {
        line_number = ebi::tftv_count();

        // Adjust for porch size
        if (line_number >= 3) {
            line_number -= 3;
        }

        ebi::tft_frame_base_set(line_number * display::V_WIDTH * 2);
    }
}

pub fn run() {
    let mut usart: Usart = Default::default();
    usart.init_async();
    usart.write_line("Starting.......\n\r");

    let mut redraw: bool = false;

    // Configure for 48MHz HFXO operation of core clock
    cmu::clock_select_set(cmu::Clock::HF, cmu::Select::HFXO);

    // Setup SysTick Timer for 1 msec interrupts
    if (cmsis::sys_tick::config(cmu::clock_freq_get(cmu::Clock::CORE) / 1000) != 0) {
        loop {}
    }

    // AEM has to be exited in order for the MCU to control the screen
    let bcreg = bc::BC::bc_register();
    while (bcreg.UIF_AEM != bc::UIF_AEM_EFM) {
        utils::blink(1)
    }

    redraw = display::init();
    bsp::leds_set(0x8001);

    ebi::tfth_stride_set((display::V_WIDTH - display::D_WIDTH) * 2);
    display::irq_enable(ebi::IF_VFPORCH | ebi::IF_HSYNC);
    display::clear();

    gamepad::init();
    bsp::leds_set(0xf731);

    loop {
        display::draw_number(999, (250 + 10 * display::V_WIDTH) as usize, 0xffffffff);
    }
}
