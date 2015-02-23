use emlib::timer;
use led_test::{assert};
use core::default::Default;
use core::slice::SliceExt;

fn setup() {
    unsafe { TIMER_setup(); }
}

fn tear_down() {}

fn test_init_called() {
    let timer = timer::Timer::timer0();
    timer.init(&Default::default());

    assert(unsafe { check_test_called() });
}

fn test_init_called_with_timer0() {

    let timer = timer::Timer::timer0();
    timer.init(&Default::default());

    assert(unsafe { check_test_called_with_timer0() });
}

fn test_init_called_with_default() {
    let timer = timer::Timer::timer0();
    timer.init(&Default::default());

    assert(unsafe { check_test_called_with_default() });
}

pub fn tests() {

    let tests: [fn(); 3] = [
        test_init_called,
        test_init_called_with_timer0,
        test_init_called_with_default
    ];

    for test in tests.iter() {
        setup();
        test();
        tear_down();
    }
    
}

extern {
    fn TIMER_setup();
    
    fn check_test_called() -> bool;
    fn check_test_called_with_timer0() -> bool;
    fn check_test_called_with_default() -> bool;
}
