#![feature(rustc_private)]

extern crate gcc;
extern crate submodules;
extern crate rustc;

use gcc::Config;

use std::env;

fn main() {
    submodules::update()
        .init()
        .recursive()
        .run();

    compile_emlib_library();
}

fn compile_emlib_library() {
    println!("The ARM embedded toolchain must be available in the PATH");
    env::set_var("CC", "arm-none-eabi-gcc");
    env::set_var("AR", "arm-none-eabi-ar");

    let mut config = Config::new();
    common_config(&mut config);

    let config = match env::var("BUILD_ENV") {
        Ok(ref val) if &val[..] == "prod" => prod_config(&mut config),
        Ok(ref val) if &val[..] == "test" => test_config(&mut config),
        _ => prod_config(&mut config)
    };

    config.compile("libemlib.a");
}

fn common_config(config: &mut Config) -> &mut Config {
    let path = env::var("CARGO_MANIFEST_DIR").ok().unwrap();

    config
        .define("EFM32GG990F1024", None)

        .include("efm32-common/CMSIS/Include")
        .include("efm32-common/Device/EFM32GG/Include")
        .include("efm32-common/emlib/inc")

        .file("efm32-common/emlib/src/em_cmu.c")
        .file("efm32-common/emlib/src/em_gpio.c")
        .file("efm32-common/emlib/src/em_usart.c")
        .file("efm32-common/emlib/src/em_emu.c")
        .file("efm32-common/emlib/src/em_ebi.c")
        .file("efm32-common/emlib/src/em_int.c")

        .flag("-Wall")
        .flag("-mthumb")
        .flag("-mcpu=cortex-m3")
        .flag("-fno-builtin")
        .flag("-ffunction-sections")
        .flag("-fdata-sections")

}

fn prod_config(config: &mut Config) -> &mut Config {

    config

        .include("efm32-common/kits/common/bsp")
        .include("src/timer")
        .include("src/adc")
        .include("src/leuart")
        .include("src/lesense")

        .file("efm32-common/emlib/src/em_acmp.c")
        .file("efm32-common/emlib/src/em_adc.c")
        .file("efm32-common/emlib/src/em_dma.c")
        .file("efm32-common/emlib/src/em_i2c.c")
        .file("efm32-common/emlib/src/em_leuart.c")
        .file("efm32-common/emlib/src/em_lesense.c")
        .file("efm32-common/emlib/src/em_prs.c")
        .file("efm32-common/emlib/src/em_rtc.c")
        .file("efm32-common/emlib/src/em_system.c")
        .file("efm32-common/emlib/src/em_timer.c")

        .file("src/adc/adc.c")
        .file("src/chip/chip.c")
        .file("src/ebi/ebi.c")
        .file("src/emu/emu.c")
        .file("src/gpio/gpio.c")
        .file("src/i2c/i2c.c")
        .file("src/irq/irq.c")
        .file("src/leuart/leuart.c")
        .file("src/lesense/lesense.c")
        .file("src/rtc/rtc.c")
        .file("src/timer/timer.c")
        .file("src/usart/usart.c")

        .file("src/acmp/get_acmp.c")
        .file("src/adc/get_adc.c")
        .file("src/timer/get_timer.c")
        .file("src/leuart/get_leuart.c")

}

fn test_config(config: &mut Config) -> &mut Config {

    config

        .flag("-DUNITY_OUTPUT_CHAR=print_char")
        .flag("-DNULL=0")

        .include("test/lib/Unity/src")
        .include("test/lib/cmock/src")
        .include("src/timer")
        .include("src/adc")

        .file("src/chip/chip.c")
        .file("src/gpio/gpio.c")
        .file("src/irq/irq.c")
        .file("src/usart/usart.c")

        .file("src/adc/get_adc.c")
        .file("src/timer/get_timer.c")

        .file("test/lib/Unity/src/unity.c")
        .file("test/lib/cmock/src/cmock.c")
        .file("test/util/usart_print.c")

        // Mocks
        .include("test/mocks")
        .file("test/mocks/Mockem_adc.c")
        .file("test/mocks/Mockem_timer.c")
        .file("test/mocks/Mockadc.c")
        .file("test/mocks/Mocktimer.c")

        // Tests
        .file("test/tests/adc.c")
        .file("test/tests/timer.c")
}
