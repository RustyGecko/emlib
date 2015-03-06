#![no_std]
#![no_main]
#![feature(lang_items, core, no_std)]

extern crate emlib;
extern crate core;

use core::default::Default;
use emlib::cmsis::nvic;
use emlib::{acmp, cmu, gpio, irq, lesense, emu, rtc};

// ACMP
const ACMP_THRESHOLD: u16 = 0x38;

const USER_LED_SECONDS_ON: u32 = 2;

// RTC
const RTC_FREQ: u32       = 32768;
const RTC_COMP_VALUE: u32 = (RTC_FREQ * USER_LED_SECONDS_ON);

// GPIO
const LED_GPIO_PORT: gpio::Port = gpio::Port::E;
const LED_GPIO_PIN: u32 = 2;
// LESENSE
const LIGHTSENSE_CH: u32          = 6;
const LIGHTSENSE_EXCITE_PORT: gpio::Port = gpio::Port::D;
const LIGHTSENSE_EXCITE_PIN: u32  = 6;
const LIGHTSENSE_SENSOR_PORT: gpio::Port = gpio::Port::C;
const LIGHTSENSE_SENSOR_PIN: u32  = 6;
const LCSENSE_SCAN_FREQ: u32      = 20;
const LIGHTSENSE_INTERRUPT: u32   = (0x1 << 6); // LESENSE_IF_CH6

fn setup_cmu() {
    cmu::clock_enable(cmu::Clock::ACMP0, true);
    cmu::clock_enable(cmu::Clock::GPIO, true);
    /* Low energy peripherals
    *   LESENSE
    *   LFRCO clock must be enables prior to enabling
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
        full_bias:                   false,                     /* The lightsensor is slow acting, */
        half_bias:                   true,                      /* comparator bias current can be set to lowest setting.*/
        bias_prog:                   0x0,                       /* Analog comparator will still be fast enough */
        interrupt_on_falling_edge:   false,                     /* No comparator interrupt, lesense will issue interrupts. */
        interrupt_on_rising_edge:    false,
        warm_time:                   acmp::WarmTime::_512,      /* Not applicable, lesense controls this. */
        hysteresis_level:            acmp::HysteresisLevel::_5, /* Some hysteresis will prevent excessive toggling. */
        inactive_value:              false,                     /* Not applicable, lesense controls this. */
        low_power_reference_enabled: false,                     /* Can be enabled for even lower power. */
        vdd_level:                   0x00,                      /* Not applicable, lesense controls this through .acmpThres value. */
        enable:                      false                      /* Not applicable, lesense controls this. */
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

fn setup_gpio() {
    /* Configure the drive strength of the ports for the light sensor. */
    gpio::drive_mode_set(LIGHTSENSE_EXCITE_PORT, gpio::DriveMode::Standard);
    gpio::drive_mode_set(LIGHTSENSE_SENSOR_PORT, gpio::DriveMode::Standard);
    /* Initialize the 2 GPIO pins of the light sensor setup. */
    gpio::pin_mode_set(LIGHTSENSE_EXCITE_PORT, LIGHTSENSE_EXCITE_PIN, gpio::Mode::PushPull, 0);
    gpio::pin_mode_set(LIGHTSENSE_SENSOR_PORT, LIGHTSENSE_SENSOR_PIN, gpio::Mode::Disabled, 0);
    /* Configure user led as output */
    gpio::pin_mode_set(LED_GPIO_PORT, LED_GPIO_PIN, gpio::Mode::PushPull, 0);
}

fn setup_rtc() {
    let rtc_init = rtc::Init { enable: false, .. Default::default() };
    rtc::init(&rtc_init);
    /* Set compare value */
    rtc::compare_set(0, RTC_COMP_VALUE);

    rtc::int_enable((0x1 << 1)); // RTC_IFS_COMP0
    nvic::enable_irq(nvic::IRQn::RTC);
}

fn setup_lesense() {
    // LESENSE configuration structure
    let init_lesense = lesense::Init {
        // LESENSE configured for periodic scan.
        core_ctrl: lesense::CoreCtrlDesc {
            store_scan_res: true,
            bias_mode:      lesense::BiasMode::DutyCycle, /* Lesense should duty cycle comparator and related references etc. */
            debug_run:      false,
            .. Default::default()
        },

        time_ctrl: Default::default(),

        per_ctrl: lesense::PerCtrlDesc {
            dac_ref:           lesense::DACRef::BandGap,
            acmp0_mode:        lesense::ControlACMP::MuxThres, /* Allow LESENSE to control ACMP mux and reference threshold. */
            acmp1_mode:        lesense::ControlACMP::MuxThres,
            warmup_mode:       lesense::WarmupMode::Normal,    /* Normal mode means LESENSE is allowed to dutycycle comparator and reference. */
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
        ena_scan_ch:      true,
        ena_pin:          false,                             /* Pin is input, no enabling needed. Separate pin is exciting the sensor. */
        ena_int:          true,                              /* Enable interrupt for this channel. */
        ch_pin_ex_mode:   lesense::ChPinExMode::High,        /* Excite by pullin pin high. */
        ch_pin_idle_mode: lesense::ChPinIdleMode::Dis,       /* During Idle, excite pin should be disabled (tri-stated). */
        use_alt_ex:       true,                              /* Use alternate excite pin. */
        shift_res:        false,                             /* Not applicable, only for decoder operation. */
        inv_res:          false,                             /* No need to invert result. */
        store_cnt_res:    true,                              /* Not applicable, don't care really. */
        ex_clk:           lesense::ChClk::LF,                /* Using low frequency clock for timing the excitation. */
        sample_clk:       lesense::ChClk::LF,                /* Using low frequency clock for timing the sample instant. */
        ex_time:          0x01,                              /* 1 LFclk cycle is enough excitation time, this depends on response time of light sensor. */
        sample_delay:     0x01,                              /* Sampling should happen when excitation ends, it it happens earlier, excitation time might as well be reduced. */
        meas_delay:       0x00,                              /* Not used here, basically only used for applications which uses the counting feature. */
        acmp_thres:       ACMP_THRESHOLD,                    /* This is the analog comparator threshold setting, determines when the acmp triggers. */
        sample_mode:      lesense::ChSampleMode::ACMP,       /* Sampling acmp, not counting. */
        int_mode:         lesense::ChIntMode::SetIntNegEdge, /* Interrupt when voltage falls below threshold. */
        cnt_thres:        0x0000,                            /* Not applicable. */
        comp_mode:        lesense::ChCompMode::Less          /* Not applicable. */
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

    /* Enable interrupt in NVIC. */
    nvic::enable_irq(nvic::IRQn::LESENSE);

    /* Start scan. */
    lesense::scan_start();
}

#[no_mangle]
pub extern fn main() {
    irq::disable();
    setup_cmu();
    setup_acmp();
    setup_gpio();
    setup_rtc();
    setup_lesense();
    irq::enable();

    loop {
        emu::enter_em2(true);
    }
}

#[no_mangle]
#[allow(non_snake_case, dead_code)]
pub extern fn LESENSE_IRQHandler() {
    /* Clear interrupt flag */
    lesense::int_clear(LIGHTSENSE_INTERRUPT);

    /* Disable RTC first to reset counter */
    rtc::enable(false);
    /* Enable RTC */
    rtc::enable(true);
    /* Turn on user led */
    gpio::pin_out_set(LED_GPIO_PORT, LED_GPIO_PIN);
}

#[no_mangle]
#[allow(non_snake_case, dead_code)]
pub extern fn RTC_IRQHandler() {
    /* Clear interrupt flag */
    rtc::int_clear(rtc::RTC_IEN_COMP0);
    /* Disable RTC */
    rtc::enable(false);

    /* Turn off user led */
    gpio::pin_out_clear(LED_GPIO_PORT, LED_GPIO_PIN);
}
