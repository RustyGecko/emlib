#![no_std]
#![no_main]
#![feature(lang_items, start, core, no_std)]

extern crate core;
extern crate emlib;

use emlib::{ chip, gpio, rtc, cmu, emu, usart };
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

    pub fn write(&self, string: [u8; 6]) {
        unsafe {
            
            let c_string: *const u8 = transmute(&string);

            SegmentLCD_Write(c_string)
        }
    }
}

static mut ROUTE: u8 = 3;

extern {
    pub fn SegmentLCD_Init(useBoost: bool);
    pub fn SegmentLCD_AllOn();
    pub fn SegmentLCD_AllOff();
    pub fn SegmentLCD_Number(value: u32);
    pub fn SegmentLCD_Write(string: *const u8);
}


const LFXO_FREQ: u32 = 32768;
const RTC_TIMEOUT_S: u32 = 10;


#[no_mangle]
pub extern fn main() {

    chip::init();

    setup_gpio();
    setup_rtc();
    setup_usart1();

    let lcd = Lcd::new();
    lcd.init(false);
    lcd.all_off();


    loop {
        let route = unsafe { ROUTE }; 

        lcd.number(route as u32);
        lcd.write([0,0,0,0,0,0]);
        let depature = get_depature(route);
        
        lcd.write(depature);

        emu::enter_em1();
    }
}

fn get_depature(route: u8) -> [u8; 6] {

    let usart1 = usart::Usart::usart1();

    usart1.tx(('0' as u8) + route);

    [
        usart1.rx(),
        usart1.rx(),
        usart1.rx(),
        usart1.rx(),
        usart1.rx(),
        0
    ]
}

fn setup_usart1() {
    cmu::clock_select_set(cmu::Clock::HF, cmu::Select::HFRCO);

    cmu::clock_enable(cmu::Clock::GPIO, true);

    let usart1 = usart::Usart::usart1();

    gpio::pin_mode_set(gpio::Port::D, 0, gpio::Mode::PushPull, 1);
    gpio::pin_mode_set(gpio::Port::D, 1, gpio::Mode::Input, 0);
    gpio::pin_mode_set(gpio::Port::D, 2, gpio::Mode::PushPull, 1);
    gpio::pin_mode_set(gpio::Port::D, 3, gpio::Mode::PushPull, 1);

    cmu::clock_enable(cmu::Clock::USART1, true);
    
    usart1.init_async(&usart::InitAsync {
        enable: usart::Enable::Enable,
        baudrate: 9600,
        ..Default::default()
    });

    usart1.ROUTE = usart::ROUTE_RXPEN
        | usart::ROUTE_TXPEN
        | usart::ROUTE_CLKPEN
        | usart::ROUTE_CSPEN
        | usart::ROUTE_LOCATION_LOC1;

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
    
}

fn setup_gpio() {
    cmu::clock_enable(cmu::Clock::GPIO, true);

    gpio::pin_mode_set(gpio::Port::B, 9, gpio::Mode::Input, 0);
    gpio::pin_mode_set(gpio::Port::B, 10, gpio::Mode::Input, 0);

    gpioint::init();

    gpioint::callback_register(9, change_route);
    gpioint::callback_register(10, change_route);

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
