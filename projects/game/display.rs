use core::default::Default;
use core::slice::from_raw_parts_mut;

use emlib::ebi;
use emlib::ebi::{TFTInit};
use emlib::cmsis::nvic;
use emlib::emdrv::tft;

use game::utils;

pub const D_WIDTH: u32 = 320;
pub const D_HEIGHT: u32 = 240;
pub const V_WIDTH: u32 = 672;
pub const V_HEIGHT: u32 = 240;

pub static tft_init: TFTInit = TFTInit {
    bank:            ebi::TFTBank::_2,
    width:           ebi::TFTWidth::HalfWord,
    colsrc:          ebi::TFTColorSrc::Mem,
    interleave:      ebi::TFTInterleave::Unlimited,
    fb_trigger:      ebi::TFTFrameBufTrigger::HSync,
    shift_dclk:      false,
    mask_blend:      ebi::TFTMaskBlend::Disabled,
    drive_mode:      ebi::TFTDDMode::External,
    cs_polarity:     ebi::Polarity::ActiveLow,
    dclk_polarity:   ebi::Polarity::ActiveHigh,
    dataen_polarity: ebi::Polarity::ActiveLow,
    hsync_polarity:  ebi::Polarity::ActiveLow,
    vsync_polarity:  ebi::Polarity::ActiveLow,
    hsize:           320,
    h_porch_front:   1,
    h_porch_back:    30,
    h_pulse_width:   2,
    vsize:           240,
    v_porch_front:   1,
    v_porch_back:    4,
    v_pulse_width:   2,
    address_offset:  0x0000,
    dclk_period:     8,
    start_position:  0,
    setup_cycles:    0,
    hold_cycles:     0,
};


static numbers: [[[bool; 3]; 5]; 10] = [[
    [true, true, true],
    [true, false, true],
    [true, false, true],
    [true, false, true],
    [true, true, true],
],[
    [false, false, true],
    [false, false, true],
    [false, false, true],
    [false, false, true],
    [false, false, true],
],[
    [true, true, true],
    [false, false, true],
    [true, true, true],
    [true, false, false],
    [true, true, true],
],[
    [true, true, true],
    [false, false, true],
    [true, true, true],
    [false, false, true],
    [true, true, true],
],[
    [true, false, true],
    [true, false, true],
    [true, true, true],
    [false, false, true],
    [false, false, true],
],[
    [true, true, true],
    [true, false, false],
    [true, true, true],
    [false, false, true],
    [true, true, true],
],[
    [true, true, true],
    [true, false, false],
    [true, true, true],
    [true, false, true],
    [true, true, true],
],[
    [true, true, true],
    [false, false, true],
    [false, true, true],
    [false, true, false],
    [false, true, false],
],[
    [true, true, true],
    [true, false, true],
    [true, true, true],
    [true, false, true],
    [true, true, true],
],[
    [true, true, true],
    [true, false, true],
    [true, true, true],
    [false, false, true],
    [false, false, true],
],];


pub fn init() -> bool {
    tft::direct_init(&tft_init)
}

pub fn irq_enable(flags: u32) {
    ebi::int_disable(ebi::IF_MASK);
    ebi::int_clear(ebi::IF_MASK);
    ebi::int_enable(flags);

    nvic::clear_pending_irq(nvic::IRQn::EBI);
    nvic::enable_irq(nvic::IRQn::EBI);
}

trait BufferLen {
    fn buffer_len() -> usize;
}

impl BufferLen for u8 {
    fn buffer_len() -> usize { (V_WIDTH * V_HEIGHT * 2) as usize }
}

impl BufferLen for u16 {
    fn buffer_len() -> usize { (V_WIDTH * V_HEIGHT) as usize }
}

impl BufferLen for u32 {
    fn buffer_len() -> usize { (V_WIDTH * V_HEIGHT / 2) as usize }
}

impl BufferLen for u64 {
    fn buffer_len() -> usize { (V_WIDTH * V_HEIGHT / 4) as usize }
}

// Treat ebi::BANK2 to a slice of data
fn frame_buffer<'a, T: BufferLen>() -> &'a mut [T] {
    let address = ebi::bank_address(ebi::BANK2) as *mut T;
    unsafe { from_raw_parts_mut(address, T::buffer_len()) }
}

pub fn clear() {
    // Clear entire display using 32-bit write operations.

    // It is cleaner to treat the framebuffer like an array, but it might be slower due to
    // bounds checking, so probably not the optimal solution.
    let mut buf = frame_buffer::<u32>();
    for i in 0 .. buf.len() {
        buf[i] = 0x00000000;
    }

    // Alternate solution:
    // let mut framebuffer: *mut u32 = ebi::bank_address(ebi::BANK2) as *mut u32;
    // for i in 0 .. ((V_WIDTH * V_HEIGHT) / 2) {
    //     unsafe {
    //         *framebuffer = 0x00000000;
    //         framebuffer = framebuffer.offset(1);
    //     }
    // }
}

pub fn draw_number(number: usize, mut pos: usize, color: u16) {
    let mut current_score = number;
    pos = pos + 16; // Start with the third position

    let mut buf = frame_buffer::<u16>();
    for figures in 0 .. 3 {
        let mut num: usize = current_score % 10;
        current_score = current_score / 10;
        let mut yy: usize = 0;
        for y in 0 .. 5 {
            let mut xx: usize = 0;
            for x in 0 .. 3 {
                buf[pos+xx+yy] = if numbers[num][y][x] { color } else { 0 };
                xx += 1;
                buf[pos+xx+yy] = if numbers[num][y][x] { color } else { 0 };
                xx += 1;
            }
            yy += V_WIDTH as usize;
            xx = 0;
            for x in 0 .. 3 {
                buf[pos+xx+yy] = if numbers[num][y][x] { color } else { 0 };
                xx += 1;
                buf[pos+xx+yy] = if numbers[num][y][x] { color } else { 0 };
                xx += 1;
            }
            yy += V_WIDTH as usize;
        }
        pos -= 8;
    }
}

pub fn debug_count() {
    let mut num = 999;
    loop {
        draw_number(num, (250 + 10 * V_WIDTH) as usize, 0xffffffff);
        num = if num == 0 { 999 } else { num - 1 };
        utils::delay(10);
    }
}
