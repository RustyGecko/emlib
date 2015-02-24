use emlib::timer;
use core::default::Default;
use core::slice::SliceExt;

fn setup() { unsafe { Mockem_timer_Init() } }
fn tear_down() { unsafe { Mockem_timer_Verify() } }
fn tear_down_tests() { unsafe { Mockem_timer_Destroy() } }

fn test_init_called() {

    unsafe { expect_init_called(); }
    
    let timer = timer::Timer::timer0();
    timer.init(&Default::default());
}

pub fn run_tests() {

    let tests: [fn(); 1] = [
        test_init_called,
    ];

    for test in tests.iter() {
        setup();
        test();
        tear_down();
    }

    tear_down_tests();
}

extern {

    fn Mockem_timer_Init();
    fn Mockem_timer_Destroy();
    fn Mockem_timer_Verify();

    fn expect_init_called();
}
