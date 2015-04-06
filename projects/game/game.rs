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

static mut color1: u16 = 2000;
static mut color2: u16 = 12000;

const circle_samples: usize = 4 + 33 * 4;
static mut circle1_center: u32 = 100 * display::V_WIDTH + 100;
static mut circle2_center: u32 = 174 * display::V_WIDTH + 174;

static mut circle_offsets: [i32; circle_samples] = [0; circle_samples];
static circleY: [i32; 35] = [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,20,21,21,22,22,23,23,23,23,24,24,24,24,24];
static circleX: [i32; 35] = [24,24,24,24,24,23,23,23,23,22,22,21,21,20,20,19,18,17,16,15,14,13,12,11,10,9,8,7,6,5,4,3,2,1,0];

fn calculate_circle_offsets() {
    let mut quadranttop = true;
    let mut quadrantright = false;
    let mut j = 0;

    unsafe {
        for i in 0 .. circle_samples {
            let x = if quadrantright { circleX[j] } else { -circleX[j] };
            let y = if quadranttop { -circleY[j] } else { circleY[j] };
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

fn clear_circle(center: u32) {
    let mut buf = display::frame_buffer::<u16>();
    unsafe {
        for i in 0 .. circle_samples {
            let idx = (center as i32 + circle_offsets[i]);
            if idx > 0 {
                buf[idx as usize] = 0;
            }
        }
    }
}

fn draw_circle(center: u32, mut color: u16) {
    let mut buf = display::frame_buffer::<u16>();
    unsafe {
        for i in 0 .. circle_samples {
            let idx = (center as i32 + circle_offsets[i]);
            if idx > 0 {
                buf[idx as usize] = color;
                color += 32;
            }
        }
    }
}

fn increment_color(color: &mut u16, default: u16) {
    *color += 32;
    if *color + 64 > default + circle_samples as u16 * 32 {
        *color = default;
    }
}

pub fn run() {
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

    calculate_circle_offsets();

    unsafe { loop {
        // Clear any gpio interrupts
        let flags = gpio::int_get();
        gpio::int_clear(flags);

        // Read status of gpio pins
        let buttons = gpio::port_in_get(gpio::Port::C);

        clear_circle(circle1_center);
        clear_circle(circle2_center);

        if buttons & 0x1 == 0 {
            circle1_center -= 1;
            // rect1.dx -= 1;
            // draw_rect1.dx -= 1;
        }
        if buttons & 0x2 == 0 {
            circle1_center -= display::V_WIDTH;
            // rect1.dy -= 1;
            // draw_rect1.dy -= 1;
        }
        if buttons & 0x4 == 0 {
            circle1_center += 1;
            // rect1.dx += 1;
        }
        if buttons & 0x8 == 0 {
            circle1_center += display::V_WIDTH;
            // rect1.dy += 1;
        }
        if buttons & 0x10 == 0 {
            circle2_center -= 1;
            // rect2.dx -= 1;
            // draw_rect2.dx -= 1;
        }
        if buttons & 0x20 == 0 {
            circle2_center -= display::V_WIDTH;
            // rect2.dy -= 1;
            // draw_rect2.dy -= 1;
        }
        if buttons & 0x40 == 0 {
            circle2_center += 1;
            // rect2.dx += 1;
        }
        if buttons & 0x80 == 0 {
            circle2_center += display::V_WIDTH;
            // rect2.dy += 1;
        }

        // Draw circles
        let mut buf = display::frame_buffer::<u16>();
        draw_circle(circle1_center, color1);
        increment_color(&mut color1, 2000);

        draw_circle(circle2_center, color2);
        increment_color(&mut color2, 12000);

        display::draw_number(buttons as usize, (10 + 10 * display::V_WIDTH) as usize, 0xffffffff);
        utils::delay(10);
    } }
}
