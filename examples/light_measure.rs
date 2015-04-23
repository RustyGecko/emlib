#![no_std]
#![no_main]
#![feature(collections, core, no_std)]

extern crate emlib;
extern crate core;
extern crate cmsis;
#[macro_use] extern crate collections;

use core::default::Default;

use cmsis::nvic;
use emlib::{acmp, cmu, gpio, irq, lesense, emu, rtc};
use emlib::modules::{Button, Led, Usart};
use emlib::stk::bsp;
use emlib::stk::io::{PB0, PB1};
use emlib::emdrv::flash;

// RTC
const RTC_FREQ: u32       = 32768;

// LESENSE
const LIGHTSENSE_CH: u32          = 6;
const LIGHTSENSE_EXCITE_PORT: gpio::Port = gpio::Port::D;
const LIGHTSENSE_EXCITE_PIN: u32  = 6;
const LIGHTSENSE_SENSOR_PORT: gpio::Port = gpio::Port::C;
const LIGHTSENSE_SENSOR_PIN: u32  = 6;
const LCSENSE_SCAN_FREQ: u32      = 20; // in Hz

fn setup_cmu() {
    cmu::clock_enable(cmu::Clock::ACMP0, true);
    cmu::clock_enable(cmu::Clock::GPIO, true);
    /* Low energy peripherals
    *   LESENSE
    *   LFRCO clock must be enabled prior to enabling
    *   clock for the low energy peripherals */
    cmu::clock_select_set(cmu::Clock::LFA, cmu::Select::LFRCO);
    cmu::clock_enable(cmu::Clock::CORELE, true);
    cmu::clock_enable(cmu::Clock::LESENSE, true);
    /* RTC */
    cmu::clock_enable(cmu::Clock::RTC, true);
    /* Disable clock source for LFB clock. */
    cmu::clock_select_set(cmu::Clock::LFB, cmu::Select::Disabled);
}

fn setup_acmp() {
    /* Configuration structure for ACMP */
    let acmp_init = acmp::Init {
        half_bias:                   true, /* comparator bias current can be set to lowest setting.*/
        bias_prog:                   0x0,  /* Analog comparator will still be fast enough */
        vdd_level:                   0x00,
        enable:                      false,
        .. Default::default()
    };

    let acmp = acmp::Acmp::acmp0();
    /* Initialize ACMP */
    acmp.init(&acmp_init);
    /* Disable ACMP0 out to a pin. */
    acmp.gpio_setup(0, false, false);
    /* Set up ACMP negSel to VDD, posSel is controlled by LESENSE. */
    acmp.channel_set(acmp::Channel::VDD, acmp::Channel::_0);
    /* LESENSE controls ACMP thus ACMP_Enable(ACMP0) should NOT be called in order
    * to ensure lower current consumption. */
}

fn setup_rtc() {
    let rtc_init = Default::default();
    rtc::init(&rtc_init);
    /* Set compare value */
    rtc::compare_set(0, RTC_FREQ * 5); // seconds

    rtc::int_enable(0x1 << 1);
    nvic::enable_irq(nvic::IRQn::RTC);
}

fn setup_lesense() {
    // LESENSE configuration structure
    let init_lesense = lesense::Init {
        // LESENSE configured for periodic scan.
        core_ctrl: lesense::CoreCtrlDesc {
            store_scan_res: true,
            bias_mode:      lesense::BiasMode::DutyCycle,
            debug_run:      false,
            .. Default::default()
        },

        time_ctrl: Default::default(),

        per_ctrl: lesense::PerCtrlDesc {
            dac_ref:           lesense::DACRef::BandGap,
            acmp0_mode:        lesense::ControlACMP::MuxThres,
            acmp1_mode:        lesense::ControlACMP::MuxThres,
            warmup_mode:       lesense::WarmupMode::Normal,
            .. Default::default()
        },

        dec_ctrl: lesense::DecCtrlDesc {
            hyst_prs0:   false,
            hyst_prs1:   false,
            hyst_prs2:   false,
            hyst_irq:    false,
            prs_count:   true,
            .. Default::default()
        }
    };

    /* Channel configuration */
    /* Only one channel is configured for the lightsense application. */
    let init_lesense_ch = lesense::ChDesc {
        ena_pin:          false,
        ena_int:          false,
        ch_pin_idle_mode: lesense::ChPinIdleMode::Dis,
        use_alt_ex:       true,
        store_cnt_res:    true,
        ex_clk:           lesense::ChClk::LF,
        sample_clk:       lesense::ChClk::LF,
        ex_time:          0x01,
        sample_delay:     0x01,
        meas_delay:       0x00,
        acmp_thres:       0x38,
        int_mode:         lesense::ChIntMode::SetIntNegEdge,
        cnt_thres:        0x0000,
        .. Default::default()
    };

    let mut init_alt_ex = lesense::ConfAltEx {
        alt_ex_map: lesense::AltExMap::ALTEX,
        .. Default::default()
    };
    init_alt_ex.alt_ex[0] = lesense::AltExDesc {
        enable_pin: true,
        idle_conf: lesense::AltExPinIdle::Dis,
        always_ex: true
    };
    /* Initialize LESENSE interface _with_ RESET. */
    lesense::init(&init_lesense, true);
    /* Configure LESENSE channel */
    lesense::channel_config(&init_lesense_ch, LIGHTSENSE_CH);
    /* Configure alternate excitation channels */
    lesense::alt_ex_config(&init_alt_ex);
    /* Set scan frequency */
    lesense::scan_freq_set(0, LCSENSE_SCAN_FREQ);
    /* Set clock divisor for LF clock. */
    lesense::clk_div_set(lesense::ChClk::LF, lesense::ClkPresc::ClkDiv2);
    nvic::enable_irq(nvic::IRQn::LESENSE);
    lesense::scan_start();
}

fn setup_gpio() {
    /* Configure the drive strength of the ports for the light sensor. */
    gpio::drive_mode_set(LIGHTSENSE_EXCITE_PORT, gpio::DriveMode::Standard);
    gpio::drive_mode_set(LIGHTSENSE_SENSOR_PORT, gpio::DriveMode::Standard);
    /* Initialize the 2 GPIO pins of the light sensor setup. */
    gpio::pin_mode_set(LIGHTSENSE_EXCITE_PORT, LIGHTSENSE_EXCITE_PIN, gpio::Mode::PushPull, 0);
    gpio::pin_mode_set(LIGHTSENSE_SENSOR_PORT, LIGHTSENSE_SENSOR_PIN, gpio::Mode::Disabled, 0);

    PB0.init();
    PB0.on_click(print_light_sense);

    PB1.init();
    PB1.on_click(read_sense_data);

}

fn setup_flash() {
    bsp::ebi_init();
    flash::init(1);
}

fn print_light_sense(_: u8) {
    let desc = if is_dark() {
        "It is dark\n\r"
    } else {
        "The light is on\n\r"
    };

    let usart: Usart = Default::default();
    usart.write_line(desc);
}

fn read_sense_data(_: u8) {
    let usart: Usart = Default::default();
    usart.write_line("reading data...\n\r");
    let data_slice = unsafe { &mut data[0 .. pos as usize] };
    flash::read(page_num_to_addr(1, 0), data_slice);

    usart.write_line(&format!("Sense data: {:?}\n\r", data_slice));
}

static mut pos: u32 = 0;
static mut data: [u8; 16] = [0; 16];

#[no_mangle]
pub extern fn main() {
    irq::disable();
    emlib::chip::init();
    setup_cmu();
    setup_acmp();
    setup_gpio();
    setup_rtc();
    setup_lesense();
    setup_flash();
    irq::enable();

    let mut usart: Usart = Default::default();
    usart.init_async();
    loop {
        // emu::enter_em1();
    }
}

fn is_dark() -> bool {
    let data_reg = lesense::scan_result_data_get();
    lesense::scan_result_data_buffer_get(data_reg) == 0
}

fn page_num_to_addr(page_num: u32, loc: u32) -> u32 {
    let flash_info = flash::device_info();
    let page_size = flash_info.page_size;
    let base_addr = flash_info.base_address;
    page_num * page_size + base_addr + loc
}


#[no_mangle]
#[allow(non_snake_case, dead_code)]
pub extern fn RTC_IRQHandler() {
    /* Clear interrupt flag */
    rtc::int_clear(rtc::RTC_IEN_COMP0);
    let light: u8 = if is_dark() { 0 } else { 1 };
    let usart: Usart = Default::default();
    usart.write_line(&format!("Writing: {:?}\n\r", light));

    let addr = page_num_to_addr(1, unsafe { pos });
    flash::write(addr, &[light]);
    unsafe { pos = (pos + 1) % 16; }
}
