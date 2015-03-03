use emlib::adc;
use emlib::usart;
use core::default::Default;
use core::slice::SliceExt;

fn setup() {
    unsafe {
        Mockadc_Init();
        Mockem_adc_Init();
    }
}
fn tear_down() {
    unsafe {
        Mockadc_Verify();
        Mockem_adc_Verify();
    }
}
fn tear_down_tests() {
    unsafe {
        Mockadc_Destroy();
        Mockem_adc_Destroy();
    }
}

fn test_init_called_with_default() {

    unsafe { adc_expect_init_called_with_default(); }

    let adc0 = adc::Adc::adc0();
    adc0.init(&Default::default());

}

fn test_init_single_called_with_default() {

    unsafe { adc_expect_init_single_called_with_default(); }

    let adc0 = adc::Adc::adc0();
    adc0.init_single(&Default::default());

}

fn test_timebase_calc_called_with_value() {

    unsafe { adc_expect_timebase_calc_called_with_value(); }
    adc::timebase_calc(0);

}

fn test_prescale_calc_called_with_value() {

    unsafe { adc_expect_prescale_calc_called_with_value(); }
    adc::prescale_calc(7_000_000, 0);

}

fn test_start_called() {

    unsafe { adc_expect_adc_start_called(); }

    let adc0 = adc::Adc::adc0();
    adc0.start(adc::Start::Single);
}

fn test_data_single_get_called() {

    unsafe { adc_expect_data_single_get_called(); }

    let adc0 = adc::Adc::adc0();
    adc0.data_single_get();

}

pub fn run_tests() {
    let usart1 = usart::Usart::usart1();

    let tests: [fn(); 6] = [
        test_init_called_with_default,
        test_init_single_called_with_default,
        test_timebase_calc_called_with_value,
        test_prescale_calc_called_with_value,
        test_start_called,
        test_data_single_get_called,
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

    fn Mockadc_Init();
    fn Mockadc_Destroy();
    fn Mockadc_Verify();

    fn Mockem_adc_Init();
    fn Mockem_adc_Destroy();
    fn Mockem_adc_Verify();

    fn adc_expect_init_called_with_default();
    fn adc_expect_init_single_called_with_default();
    fn adc_expect_timebase_calc_called_with_value();
    fn adc_expect_prescale_calc_called_with_value();
    fn adc_expect_adc_start_called();
    fn adc_expect_data_single_get_called();
}
