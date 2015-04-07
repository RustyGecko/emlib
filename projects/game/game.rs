#![no_std]
#![no_main]
#![warn(warnings)]
#![allow(unsigned_negation)]
#![feature(lang_items, core, no_std, asm)]
#![feature(collections)]
#![feature(negate_unsigned)]

#[macro_use(assert, panic)] extern crate core;
#[macro_use(format)] extern crate collections;

extern crate emlib;

use emlib::ebi;
use emlib::cmu;
use emlib::gpio;
use emlib::cmsis;

use emlib::dk::{bc, bsp};

pub mod gamepad;
pub mod utils;
pub mod display;

fn rand() -> i32 {
    1
}

#[no_mangle]
pub extern fn main() {
    bsp::init(bsp::EBI);
    init();
    run();
}

#[derive(Copy, Clone)]
pub struct Rectangle {
    dx: i32,
    dy: i32,
    width: i32,
    height: i32,
}

static mut circle1_rect: Rectangle = Rectangle {
    dx: 0,
    dy: 0,
    width: 0,
    height: 0,
};

static mut circle2_rect: Rectangle = Rectangle {
    dx: 0,
    dy: 0,
    width: 0,
    height: 0,
};

static mut obstacle_rect: Rectangle = Rectangle {
    dx: 0,
    dy: 0,
    width: 0,
    height: 0,
};

static mut color1: u16 = 2000;
static mut color2: u16 = 12000;

static CIRCLE_Y: [i32; 35] = [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,20,21,21,22,22,23,23,23,23,24,24,24,24,24];
static CIRCLE_X: [i32; 35] = [24,24,24,24,24,23,23,23,23,22,22,21,21,20,20,19,18,17,16,15,14,13,12,11,10,9,8,7,6,5,4,3,2,1,0];
const CIRCLE_SAMPLES: usize = 4 + 33 * 4;
static mut circle_offsets: [i32; CIRCLE_SAMPLES] = [0; CIRCLE_SAMPLES];

static mut obstacle: [bool; display::WIDTH as usize] = [false; display::WIDTH as usize];

static mut circle1_center: i32 = 0;
static mut circle2_center: i32 = 0;
static mut obstacle_pos: usize = 0;
static mut gap1_start: i32 = 0;
static mut gap1_end: i32 = 0;
static mut gap2_start: i32 = 0;
static mut gap2_end: i32 = 0;
static mut gap2_active: bool = false;
static mut score: u32 = 0;
static mut max_score: u32 = 0;

static mut frame: i32 = 0;

fn calculate_circle_offsets() {
    let mut quadranttop = true;
    let mut quadrantright = false;
    let mut j = 0;

    unsafe {
        for i in 0 .. CIRCLE_SAMPLES {
            let x = if quadrantright { CIRCLE_X[j] } else { -CIRCLE_X[j] };
            let y = if quadranttop { -CIRCLE_Y[j] } else { CIRCLE_Y[j] };
            circle_offsets[i] = x as i32 + y as i32 * display::V_WIDTH as i32;

            match i {
                34  => quadrantright = true,
                68  => quadranttop = false,
                102 => quadrantright = false,
                _   => ()
            }

            if quadrantright && quadranttop || !quadrantright && !quadranttop {
                j -= 1;
            } else {
                j += 1;
            }
        }
    }
}

fn clear_circle(center: i32) {
    let buf = display::frame_buffer::<u16>();
    unsafe {
        for i in 0 .. CIRCLE_SAMPLES {
            let idx = center + circle_offsets[i];
            if idx > 0 {
                buf[idx as usize] = 0;
            }
        }
    }
}

fn draw_circle(center: i32, mut color: u16) {
    let buf = display::frame_buffer::<u16>();
    unsafe {
        for i in 0 .. CIRCLE_SAMPLES {
            let idx = center + circle_offsets[i];
            if idx > 0 {
                buf[idx as usize] = color;
                color += 32;
            }
        }
    }
}

fn increment_color(color: &mut u16, default: u16) {
    *color += 32;
    if *color + 64 > default + CIRCLE_SAMPLES as u16 * 32 {
        *color = default;
    }
}

unsafe fn detect_circle_collision(old_rect1: Rectangle, old_rect2: Rectangle) {
    let mut diff: i32 = (circle1_rect.dx - circle2_rect.dx) * (circle1_rect.dx - circle2_rect.dx) + (circle1_rect.dy - circle2_rect.dy) * (circle1_rect.dy - circle2_rect.dy);
    if diff < 2500 { // COLLISSION
        let mut diff2 = (old_rect1.dx - circle2_rect.dx) * (old_rect1.dx - circle2_rect.dx) + (circle1_rect.dy - circle2_rect.dy) * (circle1_rect.dy - circle2_rect.dy);
        if diff2 > diff { // undo x movement in circle1 if that improves it
            assert!(circle1_rect.dx - old_rect1.dx == 1 || circle1_rect.dx - old_rect1.dx == -1);
            circle1_center += (old_rect1.dx - circle1_rect.dx) as i32;
            circle1_rect.dx = old_rect1.dx;
            diff = diff2;
        }
        if diff < 2500 { // COLLISSION
            diff2 = (circle1_rect.dx - circle2_rect.dx) * (circle1_rect.dx - circle2_rect.dx) + (old_rect1.dy - circle2_rect.dy) * (old_rect1.dy - circle2_rect.dy);
            if diff2 > diff { // undo y movement in circle1
                assert!(circle1_rect.dy - old_rect1.dy == 1 || circle1_rect.dy - old_rect1.dy == -1);
                circle1_center += if (old_rect1.dy - circle1_rect.dy) == 1 { display::V_WIDTH } else { -display::V_WIDTH } as i32;
                circle1_rect.dy = old_rect1.dy;
                diff = diff2;
            }
            if diff < 2500 { // COLLISSION
                diff2 = (circle1_rect.dx - old_rect2.dx) * (circle1_rect.dx - old_rect2.dx) + (circle1_rect.dy - circle2_rect.dy) * (circle1_rect.dy - circle2_rect.dy);
                if diff2 > diff { //  undo x movement in circle 2
                    assert!(circle2_rect.dx - old_rect2.dx == 1 || circle2_rect.dx - old_rect2.dx == -1);
                    circle2_center += (old_rect2.dx - circle2_rect.dx) as i32;
                    circle2_rect.dx = old_rect2.dx;
                    diff = diff2;
                }
                if diff < 2500 { // COLLISSION
                    diff2 = (circle1_rect.dx - circle2_rect.dx) * (circle1_rect.dx - circle2_rect.dx) + (circle1_rect.dy - old_rect2.dy) * (circle1_rect.dy - old_rect2.dy);
                    if diff2 > diff {  // undo y movement in circle 2
                        assert!(circle2_rect.dy - old_rect2.dy == 1 || circle2_rect.dy - old_rect2.dy == -1);
                        circle2_center += if (old_rect2.dy - circle2_rect.dy) == 1 { display::V_WIDTH } else { -display::V_WIDTH } as i32;
                        circle2_rect.dy = old_rect2.dy;
                        diff = diff2;
                    }
                }
            }
        }
    }
    assert!(diff >= 2500);
}

unsafe fn detect_collission(rect: Rectangle) -> bool {
    let dx: i32 = rect.dx as i32;
    let dy: i32 = rect.dy as i32;
    if dy <= frame && dy + 50 >= frame { // y is right for collission
        if dx + 25 > gap1_start && dx + 25 < gap1_end {
            let diffy: i32 = (dy + 25 - frame) * (dy + 25 - frame);
            let mut diff: i32 = (dx + 25 - gap1_start) * (dx + 25 - gap1_start) + diffy;
            if diff < 625 {
                restart();
                return true;
            }
            diff = (dx + 25 - gap1_end) * (dx + 25 - gap1_end) + diffy;
            if diff < 625 {
                restart();
                return true;
            }
        } else {
            if gap2_active && dx + 25 > gap2_start && dx + 25 < gap2_end  {
                let diffy: i32 = (dy as i32 + 25 - frame) * (dy as i32 + 25 - frame);
                let mut diff: i32 = (dx + 25 - gap2_start) * (dx + 25 - gap2_start) + diffy;
                if diff < 625 {
                    restart();
                    return true;
                }
                diff = ((dx + 25 - gap2_end) * (dx + 25 - gap2_end)) as i32 + diffy;
                if diff < 625 {
                    restart();
                    return true;
                }
            } else {
                restart();
                return true;
            }
        }
    }
    return false;
}

unsafe fn generate_obstacle() {
    for i in 0 .. display::WIDTH as usize {
        obstacle[i] = true;
    }
    gap2_active = rand() % 2 == 1;
    let gap_size = if gap2_active { 70 } else { 90 };
    let gap_area = if gap2_active { 90 } else { 230 };
    gap1_start = 70 + rand() % gap_area;
    gap1_end = gap1_start + gap_size + 1;

    let i = (gap1_start + 1) as usize;
    for j in 0 .. gap_size as usize {
        obstacle[i + j] = false;
    }
    if gap2_active {
        gap2_start = 160 + rand() % gap_area;
        gap2_end = gap2_start + gap_size + 1;
        let i = (gap2_start + 1) as usize;
        for j in 0 .. gap_size as usize {
            obstacle[i + j] = false;
        }
    }

}

unsafe fn update_obstacle() {
    let mut buf = display::frame_buffer::<u16>();

    obstacle_rect.dy = frame - 2;
    if obstacle_rect.dy < 0 {
        obstacle_rect.dy = 0;
    }
    frame += 1;
    obstacle_pos += display::V_WIDTH as usize;
    if frame == 240 {
        score += 1;
        if score > max_score {
            max_score = score;
        }
        frame = 1;
        obstacle_pos = display::V_WIDTH as usize;
        obstacle_rect.dy = 0;
        generate_obstacle();
    }

    for i in 0 .. display::WIDTH as usize {
        if obstacle[i] {
            buf[obstacle_pos + i] = 63488;
            if obstacle_pos >= 600 {
                buf[obstacle_pos + i - display::V_WIDTH as usize] = 57344;
            }
            if obstacle_pos >= 1200 {
                buf[obstacle_pos + i - 2 * display::V_WIDTH as usize] = 63;
            }
            if obstacle_pos >= 2000 {
                buf[obstacle_pos + i - 3 * display::V_WIDTH as usize] = 0;
            }
        }
    }
}

unsafe fn restart() {
    circle1_center = 100 * display::V_WIDTH as i32 + 100;
    circle2_center = 174 * display::V_WIDTH as i32 + 200;
    obstacle_pos = 0;
    gap1_start = 0;
    gap1_end = 0;
    gap2_active = false;
    // gap2_start = 0;
    // gap2_end = 0;
    score = 0;

    // srand(1);
    generate_obstacle();

    display::clear();

    circle1_rect.dx = 76;
    circle1_rect.dy = 76;
    circle1_rect.width = 51;
    circle1_rect.height = 51;

    circle2_rect.dx = 176;
    circle2_rect.dy = 150;
    circle2_rect.width = 51;
    circle2_rect.height = 51;

    obstacle_rect.dx = 0;
    obstacle_rect.dy = 0;
    obstacle_rect.width = 320;
    obstacle_rect.height = 5;

    frame = 0;
}

fn init() {
    // Configure for 48MHz HFXO operation of core clock
    cmu::clock_select_set(cmu::Clock::HF, cmu::Select::HFXO);

    // Setup SysTick Timer for 1 msec interrupts
    if cmsis::sys_tick::config(cmu::clock_freq_get(cmu::Clock::CORE) / 1000) != 0 {
        loop {}
    }

    // AEM has to be exited in order for the MCU to control the screen
    let bcreg = bc::BC::bc_register();
    while bcreg.UIF_AEM != bc::UIF_AEM_EFM {
        utils::blink(1)
    }

    let _ = display::init();
    bsp::leds_set(0x8001);

    ebi::tfth_stride_set((display::V_WIDTH - display::WIDTH) as u32 * 2);
    display::irq_enable(ebi::IF_VFPORCH | ebi::IF_HSYNC);
    display::clear();

    gamepad::init();

    unsafe { restart(); }
    calculate_circle_offsets();
}

fn run() {
    unsafe { loop {
        // Clear any gpio interrupts
        let flags = gpio::int_get();
        gpio::int_clear(flags);

        // Read status of gpio pins
        let buttons = gpio::port_in_get(gpio::Port::C);

        let old_rect1: Rectangle = circle1_rect;
        clear_circle(circle1_center);

        if buttons & 0x1 == 0 && circle1_rect.dx > 0 {
            circle1_center -= 1;
            circle1_rect.dx -= 1;
        }
        if buttons & 0x2 == 0 && circle1_rect.dy > 0 {
            circle1_center -= display::V_WIDTH as i32;
            circle1_rect.dy -= 1;
        }
        if buttons & 0x4 == 0 && circle1_rect.dx < 268 {
            circle1_center += 1;
            circle1_rect.dx += 1;
        }
        if buttons & 0x8 == 0 && circle1_rect.dy < 189 {
            circle1_center += display::V_WIDTH as i32;
            circle1_rect.dy += 1;
        }

        let old_rect2: Rectangle = circle2_rect;
        clear_circle(circle2_center);

        if buttons & 0x10 == 0 && circle2_rect.dx > 0 {
            circle2_center -= 1;
            circle2_rect.dx -= 1;
        }
        if buttons & 0x20 == 0 && circle2_rect.dy > 0 {
            circle2_center -= display::V_WIDTH as i32;
            circle2_rect.dy -= 1;
        }
        if buttons & 0x40 == 0 && circle2_rect.dx < 268 {
            circle2_center += 1;
            circle2_rect.dx += 1;
        }
        if buttons & 0x80 == 0 && circle2_rect.dy < 189 {
            circle2_center += display::V_WIDTH as i32;
            circle2_rect.dy += 1;
        }

        detect_circle_collision(old_rect1, old_rect2);

        if detect_collission(circle1_rect) { continue; }
        if detect_collission(circle2_rect) { continue; }

        update_obstacle();

        // Draw circles
        draw_circle(circle1_center, color1);
        increment_color(&mut color1, 2000);

        draw_circle(circle2_center, color2);
        increment_color(&mut color2, 12000);

        // Button info
        display::draw_number(buttons as usize, 10 + 10 * display::V_WIDTH, 0xffff);

        display::draw_number(score as usize, 250 + 10 * display::V_WIDTH, 0xffff);
        display::draw_number(max_score as usize, 276 + 10 * display::V_WIDTH, 0x2ee0);
    } }
}
