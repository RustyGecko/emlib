use emlib::timer;
use led_test::{assert};
use core::default::Default;

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

    setup();
    test_init_called();
    tear_down();

    setup();
    test_init_called_with_timer0();
    tear_down();

    setup();
    test_init_called_with_default();
    tear_down();
    
}

extern {
    fn TIMER_setup();
    
    fn check_test_called() -> bool;
    fn check_test_called_with_timer0() -> bool;
    fn check_test_called_with_default() -> bool;
}
