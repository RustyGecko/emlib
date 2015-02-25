use emlib::timer;
use emlib::usart;
use core::default::Default;
use core::slice::SliceExt;

fn setup() { unsafe { Mockem_timer_Init() } }
fn tear_down() { unsafe { Mockem_timer_Verify() } }
fn tear_down_tests() { unsafe { Mockem_timer_Destroy() } }

fn test_init_called_with_default() {

    unsafe { expect_init_called_with_default(); }

    let timer = timer::Timer::timer0();
    timer.init(&Default::default());
}

fn test_init_cc_called_with_default() {

    unsafe { expect_init_cc_called_with_default(); }

    let timer = timer::Timer::timer0();
    timer.init_cc(0, &Default::default());
}

fn test_init_dti_called_with_default() {

    unsafe { expect_init_dti_called_with_default(); }

    let timer = timer::Timer::timer0();
    timer.init_dti(&Default::default());
}

pub fn run_tests() {
    let usart1 = usart::Usart::usart1();

    let tests: [fn(); 3] = [
        test_init_called_with_default,
        test_init_cc_called_with_default,
        test_init_dti_called_with_default,
    ];

    for test in tests.iter() {
        setup();
        test();
        tear_down();
        usart1.tx('.' as u8);
    }

    tear_down_tests();
}

extern {

    fn Mockem_timer_Init();
    fn Mockem_timer_Destroy();
    fn Mockem_timer_Verify();

    fn expect_init_called_with_default();
    fn expect_init_cc_called_with_default();
    fn expect_init_dti_called_with_default();
}
