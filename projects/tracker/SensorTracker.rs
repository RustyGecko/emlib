#![no_std]
#![no_main]
#![feature(lang_items, start, core, no_std)]
#![feature(collections, alloc)]

#[macro_use]
extern crate core;
extern crate emlib;
extern crate libc;
extern crate alloc;

#[macro_use]
extern crate collections;

use core::prelude::*;
use core::default::Default;
use core::fmt::Debug;

use collections::vec::Vec;

use emlib::{adc, chip, cmu, emu, i2c, rtc};
use emlib::modules::Usart;
use emlib::cmsis::nvic;
use emlib::utils::cmdparse::{get_command, Cmd};
use emlib::stk::io::{PB0, PB1};

use ram_store as store;
use fixed_size_vector::FixedSizeVector;
use sensor::Sensor;

use circular_buffer::CircularBuffer4;

mod hr_temp;
mod internal_temperature;
mod ram_store;
mod buffer;
mod circular_buffer;
mod fixed_size_vector;
mod sensor;

enum State {
    Connected,
    Unconnected
}

const INTERVAL: u32 = 50; // Time in ms between each sample

static mut MODE: State = State::Unconnected;

#[no_mangle]
pub extern fn main() {

    chip::init();

    PB0.init(); PB0.on_click(btn0_cb);

    PB1.init(); PB1.on_click(btn1_cb);

    cmu::clock_enable(cmu::Clock::ADC0, true);
    let it_sense = internal_temperature::InternalTemperature::new(adc::Adc::adc0());
    it_sense.init();

    let hr_temp_sense = hr_temp::HumidityRelativeAndTemperatureSensor::new(i2c::I2C::i2c1());
    hr_temp_sense.init();

    if !hr_temp_sense.detect() {
        panic!("Could not detect HumidityRelative and Temperature Sensor");
    }

    let mut it_store = FixedSizeVector::new(1024);
    let mut hr_store = FixedSizeVector::new(1024);
    let mut t_store = FixedSizeVector::new(1024);

    setup_rtc(INTERVAL);

    let mut uart: Usart = Default::default();
    uart.init_async();

    loop {
        match unsafe { &MODE } {
            &State::Connected => match get_command() {
                Cmd::Read(page) => {
                    match page % 3 {
                        0 => read(&it_store[..]),
                        1 => read(&hr_store[..]),
                        2 => read(&t_store[..]),
                        _ => ()
                    }
                },
                _ => ()
            },
            _ => {
                empty_queues(
                    &mut it_store,
                    &mut hr_store,
                    &mut t_store
                );
                emu::enter_em2(true);
            },
        }
    }
}

fn setup_rtc(interval: u32) {

    const LFXO_FREQ: u32 = 32768;
    const RTC_TIMEOUT_S: u32 = 2;

    cmu::clock_enable(cmu::Clock::CORELE, true);
    cmu::clock_enable(cmu::Clock::RTC, true);
    cmu::clock_select_set(cmu::Clock::LFA, cmu::Select::LFXO);

    rtc::init(&rtc::Init {
        enable: false,
        ..Default::default()
    });

    let freq = (LFXO_FREQ * RTC_TIMEOUT_S) / 2;
    rtc::compare_set(0, (freq / 1000) * interval);

    nvic::enable_irq(nvic::IRQn::RTC);
    rtc::int_enable(rtc::RTC_IEN_COMP0);

    rtc::enable(true);

}

static mut IT_BUFFER: CircularBuffer4<u8> = CircularBuffer4 {
    tail_index: 0,
    head_index: 0,
    data: [0; 4]
};

static mut HR_BUFFER: CircularBuffer4<u32> = CircularBuffer4 {
    tail_index: 0,
    head_index: 0,
    data: [0; 4]
};

static mut T_BUFFER: CircularBuffer4<i32> = CircularBuffer4 {
    tail_index: 0,
    head_index: 0,
    data: [0; 4]
};


#[no_mangle]
#[allow(non_snake_case)]
pub extern fn RTC_IRQHandler() {

    rtc::int_clear(rtc::RTC_IEN_COMP0);


    let mut it_sense = internal_temperature::InternalTemperature::new(adc::Adc::adc0());

    unsafe { IT_BUFFER.push(it_sense.measure()) }
      .ok()
      .expect("Circular buffer is full");


    let mut hr_temp_sense = hr_temp::HumidityRelativeAndTemperatureSensor::new(i2c::I2C::i2c1());
    let (relative_humidity, temperature) = hr_temp_sense.measure();

    unsafe { HR_BUFFER.push(relative_humidity) }
      .ok()
      .expect("Circular buffer is full");

    unsafe { T_BUFFER.push(temperature)}
      .ok()
      .expect("Circular buffer is full");


}

fn empty_queues(it_store: &mut FixedSizeVector<u8>, hr_store: &mut FixedSizeVector<u32>, t_store: &mut FixedSizeVector<i32>) {

    it_store.push_all(&unsafe { IT_BUFFER.pop_all() }[..]);
    hr_store.push_all(&unsafe { HR_BUFFER.pop_all() }[..]);
    t_store.push_all(&unsafe { T_BUFFER.pop_all() }[..]);

}

fn btn0_cb(_pin: u8) {
    unsafe { MODE = State::Connected; }
}

fn btn1_cb(_pin: u8) {
    unsafe { MODE = State::Unconnected; }
}

fn read<T: Debug>(samples: &[T])  {
    let uart: Usart = Default::default();

    for sample in samples {
        let s = format!("{:?} ", sample);
        uart.write_line(&s);
    }

    uart.newline();

}
