#![no_std]
#![no_main]
#![feature(lang_items, start, core, no_std)]

extern crate core;
extern crate emlib;

use emlib::{ chip, gpio, rtc, cmu, emu };
use emlib::cmsis::nvic;
use emlib::emdrv::gpioint;

use core::str::StrExt;
use core::slice::SliceExt;
use core::mem::transmute;
use core::default::Default;
use core::option::Option::{Some, None};

struct Lcd;

impl Lcd {

    pub fn new() -> Lcd { Lcd }

    pub fn init(&self, use_boost: bool) {
        unsafe { SegmentLCD_Init(use_boost) }
    }

    pub fn all_on(&self) {
        unsafe { SegmentLCD_AllOn() }
    }

    pub fn all_off(&self) {
        unsafe { SegmentLCD_AllOff() }
    }

    pub fn number(&self, value: u32) {
        unsafe { SegmentLCD_Number(value) }
    }

    pub fn write(&self, string: &str) {
        unsafe {
            
            let c_string: *const u8 = transmute(string.as_ptr());

            SegmentLCD_Write(c_string)
        }
    }
}

struct Display { lcd: Lcd }

impl Display {

    pub fn new() -> Display {
        let lcd = Lcd::new();
        lcd.init(false);
        lcd.all_off();
        
        Display{ lcd: lcd }
    }

    pub fn set(&self, id: &str, time: u32) {

        let (mins, secs) = ((time / 60) as u8, (time % 60) as u8);
        
        self.lcd.write(id);
        
        let seconds = (secs as u32) + (mins as u32) * 100;
        
        self.lcd.number(seconds);
        
    }
}

static mut COUNTER: u32 = 0;
static mut ROUTE: u8 = 3;

fn inc_counter(value: u32) {
    unsafe {
        let counter = COUNTER;
        COUNTER = (counter + value) % (24*60*60);
    }
}

extern {
    pub fn SegmentLCD_Init(useBoost: bool);
    pub fn SegmentLCD_AllOn();
    pub fn SegmentLCD_AllOff();
    pub fn SegmentLCD_Number(value: u32);
    pub fn SegmentLCD_Write(string: *const u8);
}

struct Route {
    number: &'static str,
    depatures: &'static [u32]
}

impl Route {

    fn get_next_time(&self, counter: u32) -> u32 {

        let mut last = 0;
        for &time in self.depatures {
            if counter < time {
                return time;
            }
            last = time;
        }

        return 0;
        
    }
    
    pub fn next(&self, counter: u32) -> u32 {
        self.get_next_time(counter) - counter
    }
    
}

const LFXO_FREQ: u32 = 32768;
const RTC_TIMEOUT_S: u32 = 1;

enum Mode {
    SetHour,
    SetMinute,
    Operation
}

static mut MODE: Mode = Mode::SetHour;

#[no_mangle]
pub extern fn main() {

    chip::init();

    setup_gpio();
    setup_rtc();

    let display = Display::new();

    loop {

        match unsafe { &MODE } {

            &Mode::SetHour => {
                display.set("SetHour", unsafe { COUNTER / 60 });
                emu::enter_em1();
            },

            &Mode::SetMinute => {
                display.set("SetMinute", unsafe { COUNTER / 60 });
                emu::enter_em1();
            },

            &Mode::Operation => {
                
                let counter = unsafe { COUNTER };
                
                let route = match unsafe { ROUTE } {
                    3 => Some(&Three),
                    4 => Some(&Four),
                    _ => None
                };

                let (id, next) = match route {
                    Some(route) => {
                        (route.number, route.next(counter))
                    },
                    None => ("", 0)
                };

                display.set(id, next);
                
                emu::enter_em1();
            }

        }
    }
}

extern fn inc_minute(_pin: u8) {
    inc_counter(60);
}

extern fn inc_hour(_pin: u8) {
    inc_counter(60*60);
}

extern fn next_mode(_pin: u8) {
    match unsafe { &MODE } {
        &Mode::SetHour => {
            gpioint::callback_register(9, inc_minute);
            gpioint::callback_register(10, next_mode);
            unsafe { MODE = Mode::SetMinute; }
        },
        &Mode::SetMinute => {
            gpioint::callback_register(9, change_route);
            gpioint::callback_register(10, change_route);
            unsafe {
                COUNTER = COUNTER % 60; // Set seconds to zero
                MODE = Mode::Operation;
            }
        }
        _ => (),
    }
}
    
extern fn change_route(pin: u8) {
    
    match pin {
        9  => unsafe { ROUTE = 3; },
        10 => unsafe { ROUTE = 4; },
        _  => ()
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern fn RTC_IRQHandler() {
    rtc::int_clear(rtc::RTC_IEN_COMP0);

    inc_counter(1);
    
}

fn setup_gpio() {
    cmu::clock_enable(cmu::Clock::GPIO, true);

    gpio::pin_mode_set(gpio::Port::B, 9, gpio::Mode::Input, 0);
    gpio::pin_mode_set(gpio::Port::B, 10, gpio::Mode::Input, 0);

    gpioint::init();

    gpioint::callback_register(9, inc_hour);
    gpioint::callback_register(10, next_mode);

    gpio::int_config(gpio::Port::B, 9, false, true, true);
    gpio::int_config(gpio::Port::B, 10, false, true, true);
    
}

fn setup_rtc() {
    
    cmu::clock_enable(cmu::Clock::CORELE, true);
    cmu::clock_select_set(cmu::Clock::LFA, cmu::Select::LFXO);
    cmu::clock_enable(cmu::Clock::RTC, true);

    rtc::init(&rtc::Init {
        enable: false,
        ..Default::default()
    });

    rtc::compare_set(0, LFXO_FREQ * RTC_TIMEOUT_S);

    nvic::enable_irq(nvic::IRQn::RTC);
    rtc::int_enable(rtc::RTC_IEN_COMP0);
    
    rtc::enable(true);
}

#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"]
pub extern fn rust_begin_unwind(_msg: core::fmt::Arguments, _file: &'static str, _line: usize) -> ! {
    loop { }
}
