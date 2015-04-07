#![no_std]
#![no_main]
#![warn(warnings)]
#![allow(unsigned_negation)]
#![feature(lang_items, core, no_std, asm)]
#![feature(negate_unsigned)]

#[macro_use(assert, panic)]
extern crate core;
extern crate emlib;

use core::prelude::*;

use emlib::ebi;
use emlib::cmu;
use emlib::gpio;
use emlib::cmsis;

use emlib::dk::{bc, bsp};

const CIRCLE_SAMPLES: usize = 4 + 33 * 4;
static CIRCLE_Y: [i32; 35] = [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,20,21,21,22,22,23,23,23,23,24,24,24,24,24];
static CIRCLE_X: [i32; 35] = [24,24,24,24,24,23,23,23,23,22,22,21,21,20,20,19,18,17,16,15,14,13,12,11,10,9,8,7,6,5,4,3,2,1,0];

static mut circle_offsets: [i32; CIRCLE_SAMPLES] = [0; CIRCLE_SAMPLES];

pub mod gamepad;
pub mod utils;
pub mod display;

//TODO: Replace with actual rand()
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

pub struct Circle {
    rect: Rectangle,
    center: i32,
    color: u16,
}

pub struct Obstacle {
    rect: Rectangle,
    obstacle: [bool; display::WIDTH],
    pos: usize,
    gap1: (i32, i32),
    gap2: Option<(i32, i32)>,
}

pub struct GameEnv {
    circle1: Circle,
    circle2: Circle,
    obstacle: Obstacle,
    score: u32,
    max_score: u32,
    frame: i32,
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

    calculate_circle_offsets();
}

fn run() {
    let mut env: GameEnv = restart(0);

    loop {
        // Clear any gpio interrupts
        let flags = gpio::int_get();
        gpio::int_clear(flags);

        // Read status of gpio pins
        let buttons = gpio::port_in_get(gpio::Port::C);

        let old_rect1: Rectangle = env.circle1.rect;
        clear_circle(&env.circle1);

        if buttons & 0x1 == 0 && env.circle1.rect.dx > 0 {
            env.circle1.center -= 1;
            env.circle1.rect.dx -= 1;
        }
        if buttons & 0x2 == 0 && env.circle1.rect.dy > 0 {
            env.circle1.center -= display::V_WIDTH as i32;
            env.circle1.rect.dy -= 1;
        }
        if buttons & 0x4 == 0 && env.circle1.rect.dx < 268 {
            env.circle1.center += 1;
            env.circle1.rect.dx += 1;
        }
        if buttons & 0x8 == 0 && env.circle1.rect.dy < 189 {
            env.circle1.center += display::V_WIDTH as i32;
            env.circle1.rect.dy += 1;
        }

        let old_rect2: Rectangle = env.circle2.rect;
        clear_circle(&env.circle2);

        if buttons & 0x10 == 0 && env.circle2.rect.dx > 0 {
            env.circle2.center -= 1;
            env.circle2.rect.dx -= 1;
        }
        if buttons & 0x20 == 0 && env.circle2.rect.dy > 0 {
            env.circle2.center -= display::V_WIDTH as i32;
            env.circle2.rect.dy -= 1;
        }
        if buttons & 0x40 == 0 && env.circle2.rect.dx < 268 {
            env.circle2.center += 1;
            env.circle2.rect.dx += 1;
        }
        if buttons & 0x80 == 0 && env.circle2.rect.dy < 189 {
            env.circle2.center += display::V_WIDTH as i32;
            env.circle2.rect.dy += 1;
        }

        detect_circle_collision(&mut env, old_rect1, old_rect2);

        if detect_collission(&env, env.circle1.rect) || detect_collission(&env, env.circle2.rect) {
            env = restart(env.max_score);
            continue;
        }

        update_obstacle(&mut env);

        draw_circle(&env.circle1);
        increment_color(&mut env.circle1, 2000);

        draw_circle(&env.circle2);
        increment_color(&mut env.circle2, 12000);

        display::draw_number(env.score as usize, 250 + 10 * display::V_WIDTH, 0xffff);
        display::draw_number(env.max_score as usize, 276 + 10 * display::V_WIDTH, 0x2ee0);
    }
}

fn restart(max_score: u32) -> GameEnv {
    display::clear();

    let circle1 = Circle {
        rect: Rectangle {
            dx: 76,
            dy: 76,
            width: 51,
            height: 51,
        },
        center: 100 * display::V_WIDTH as i32 + 100,
        color: 2000,
    };


    let circle2 = Circle {
        rect: Rectangle {
            dx: 176,
            dy: 150,
            width: 51,
            height: 51,
        },
        center: 174 * display::V_WIDTH as i32 + 200,
        color: 12000,
    };

    let obstacle = generate_obstacle();

    GameEnv {
        circle1: circle1,
        circle2: circle2,
        obstacle: obstacle,
        score: 0,
        max_score: max_score,
        frame: 0,
    }
}

fn calculate_circle_offsets() {
    let mut quadranttop = true;
    let mut quadrantright = false;
    let mut j = 0;

    for i in 0 .. CIRCLE_SAMPLES {
        let x = if quadrantright { CIRCLE_X[j] } else { -CIRCLE_X[j] };
        let y = if quadranttop { -CIRCLE_Y[j] } else { CIRCLE_Y[j] };
        unsafe {
            circle_offsets[i] = x as i32 + y as i32 * display::V_WIDTH as i32;
        }

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

fn clear_circle(circle: &Circle) {
    let buf = display::frame_buffer::<u16>();

    for i in 0 .. CIRCLE_SAMPLES {
        let idx = unsafe { circle.center as i32 + circle_offsets[i] } as usize;
        if idx > 0 {
            buf[idx] = 0;
        }
    }
}

fn draw_circle(circle: &Circle) {
    let buf = display::frame_buffer::<u16>();
    let mut color = circle.color;

    for i in 0 .. CIRCLE_SAMPLES {
        let idx = unsafe { circle.center as i32 + circle_offsets[i] } as usize;
        if idx > 0 {
            buf[idx] = color;
            color += 32;
        }
    }
}

fn increment_color(circle: &mut Circle, default: u16) {
    circle.color += 32;
    if circle.color + 64 > default + CIRCLE_SAMPLES as u16 * 32 {
        circle.color = default;
    }
}

fn detect_circle_collision(env: &mut GameEnv, old_rect1: Rectangle, old_rect2: Rectangle) {
    let rect1 = &mut env.circle1.rect;
    let rect2 = &mut env.circle2.rect;
    let mut diff: i32 = (rect1.dx - rect2.dx) * (rect1.dx - rect2.dx) + (rect1.dy - rect2.dy) * (rect1.dy - rect2.dy);

    if diff < 2500 { // COLLISSION
        let mut diff2 = (old_rect1.dx - rect2.dx) * (old_rect1.dx - rect2.dx) + (rect1.dy - rect2.dy) * (rect1.dy - rect2.dy);

        if diff2 > diff { // undo x movement in circle1 if that improves it
            assert!(rect1.dx - old_rect1.dx == 1 || rect1.dx - old_rect1.dx == -1);
            env.circle1.center += (old_rect1.dx - rect1.dx) as i32;
            rect1.dx = old_rect1.dx;
            diff = diff2;
        }

        if diff < 2500 { // COLLISSION
            diff2 = (rect1.dx - rect2.dx) * (rect1.dx - rect2.dx) + (old_rect1.dy - rect2.dy) * (old_rect1.dy - rect2.dy);

            if diff2 > diff { // undo y movement in circle1
                assert!(rect1.dy - old_rect1.dy == 1 || rect1.dy - old_rect1.dy == -1);
                env.circle1.center += if (old_rect1.dy - rect1.dy) == 1 { display::V_WIDTH } else { -display::V_WIDTH } as i32;
                rect1.dy = old_rect1.dy;
                diff = diff2;
            }

            if diff < 2500 { // COLLISSION
                diff2 = (rect1.dx - old_rect2.dx) * (rect1.dx - old_rect2.dx) + (rect1.dy - rect2.dy) * (rect1.dy - rect2.dy);

                if diff2 > diff { //  undo x movement in circle 2
                    assert!(rect2.dx - old_rect2.dx == 1 || rect2.dx - old_rect2.dx == -1);
                    env.circle2.center += (old_rect2.dx - rect2.dx) as i32;
                    rect2.dx = old_rect2.dx;
                    diff = diff2;
                }

                if diff < 2500 { // COLLISSION
                    diff2 = (rect1.dx - rect2.dx) * (rect1.dx - rect2.dx) + (rect1.dy - old_rect2.dy) * (rect1.dy - old_rect2.dy);

                    if diff2 > diff {  // undo y movement in circle 2
                        assert!(rect2.dy - old_rect2.dy == 1 || rect2.dy - old_rect2.dy == -1);
                        env.circle2.center += if (old_rect2.dy - rect2.dy) == 1 { display::V_WIDTH } else { -display::V_WIDTH } as i32;
                        rect2.dy = old_rect2.dy;
                        diff = diff2;
                    }
                }
            }
        }
    }

    assert!(diff >= 2500);
}

fn detect_collission(env: &GameEnv, rect: Rectangle) -> bool {
    let dx: i32 = rect.dx as i32;
    let dy: i32 = rect.dy as i32;

    let obs = &env.obstacle;

    if dy <= env.frame && dy + 50 >= env.frame { // y is right for collission
        if dx + 25 > obs.gap1.0 && dx + 25 < obs.gap1.1 {
            let diffy: i32 = (dy + 25 - env.frame) * (dy + 25 - env.frame);
            let mut diff: i32 = (dx + 25 - obs.gap1.0) * (dx + 25 - obs.gap1.0) + diffy;

            if diff < 625 {
                return true;
            }
            diff = (dx + 25 - obs.gap1.1) * (dx + 25 - obs.gap1.1) + diffy;

            if diff < 625 {
                return true;
            }
        } else {
            match obs.gap2 {
                Some((start, end)) if dx + 25 > start && dx + 25 < end => {
                    let diffy: i32 = (dy as i32 + 25 - env.frame) * (dy as i32 + 25 - env.frame);
                    let mut diff: i32 = (dx + 25 - start) * (dx + 25 - start) + diffy;
                    if diff < 625 {
                        return true;
                    }
                    diff = ((dx + 25 - end) * (dx + 25 - end)) as i32 + diffy;
                    if diff < 625 {
                        return true;
                    }
                },
                _ => {
                    return true;
                }
            }
        }
    }

    false
}

fn generate_obstacle() -> Obstacle {
    let mut obstacle = Obstacle {
        rect: Rectangle {
            dx: 0,
            dy: 0,
            width: 320,
            height: 5,
        },
        obstacle: [true; display::WIDTH],
        pos: 0,
        gap1: (0, 0),
        gap2: None,
    };

    let generate_gap2 = rand() % 2 == 1;
    let gap_size = if generate_gap2 { 70 } else { 90 };
    let gap_area = if generate_gap2 { 90 } else { 230 };

    obstacle.gap1.0 = 70 + rand() % gap_area;
    obstacle.gap1.1 = obstacle.gap1.0 + gap_size + 1;

    if generate_gap2 {
        let gap2_start = 160 + rand() % gap_area;
        obstacle.gap2 = Some((gap2_start, gap2_start + gap_size + 1));
    }

    let i = obstacle.gap1.0 as usize;
    for j in 0 .. gap_size as usize {
        obstacle.obstacle[i + j] = false;
    }

    match obstacle.gap2 {
        Some((start, _)) => {
            for j in 0 .. gap_size as usize {
                obstacle.obstacle[start as usize + j] = false;
            }
        },
        _ => ()
    }

    obstacle
}

fn update_obstacle(env: &mut GameEnv) {
    env.obstacle.rect.dy = env.frame - 2;
    if env.obstacle.rect.dy < 0 {
        env.obstacle.rect.dy = 0;
    }

    env.frame += 1;
    env.obstacle.pos += display::V_WIDTH as usize;
    if env.frame == 240 {
        env.score += 1;
        if env.score > env.max_score {
            env.max_score = env.score;
        }

        env.frame = 1;
        env.obstacle = generate_obstacle();
    }

    let mut buf = display::frame_buffer::<u16>();
    for i in 0 .. display::WIDTH as usize {
        if env.obstacle.obstacle[i] {
            buf[env.obstacle.pos + i] = 63488;
            if env.obstacle.pos >= 600 {
                buf[env.obstacle.pos + i - display::V_WIDTH as usize] = 57344;
            }
            if env.obstacle.pos >= 1200 {
                buf[env.obstacle.pos + i - 2 * display::V_WIDTH as usize] = 63;
            }
            if env.obstacle.pos >= 2000 {
                buf[env.obstacle.pos + i - 3 * display::V_WIDTH as usize] = 0;
            }
        }
    }
}
