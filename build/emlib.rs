#![feature(env, os, path)]

extern crate gcc;

use std::env;
use std::ffi::OsString;

fn assert_env_var(var: &str, expected: &str) {
    match env::var(var) {
        Some(ref val) if *val == OsString::from_string(expected.to_string()) => (),
        _ => panic!("`{}` environment variable must be `{}`", var, expected)
    }
}

fn main() {
    assert_env_var("CC", "arm-none-eabi-gcc");
    assert_env_var("AR", "arm-none-eabi-ar");

    let emlib_sources = [
        "efm32-common/Device/EFM32GG/Source/GCC/startup_efm32gg.S",
        "efm32-common/Device/EFM32GG/Source/system_efm32gg.c",
        "efm32-common/emlib/src/em_cmu.c",
        "efm32-common/emlib/src/em_emu.c",
        "efm32-common/emlib/src/em_gpio.c",
        "efm32-common/emlib/src/em_rtc.c",
        "efm32-common/emlib/src/em_system.c",
        "efm32-common/emlib/src/em_timer.c",
        "efm32-common/emlib/src/em_int.c",
        "efm32-common/kits/common/drivers/retargetio.c",
        "efm32-common/emdrv/gpiointerrupt/src/gpiointerrupt.c",
        "src/gpio/gpio.c",
        "src/chip/chip.c",
        "src/emdrv/gpiointerrupt.c",
    ];

    let emlib_include_directories = vec!(
        Path::new("efm32-common/CMSIS/Include"),
        Path::new("efm32-common/Device/EFM32GG/Include"),
        Path::new("efm32-common/emlib/inc"),
        Path::new("efm32-common/kits/common/drivers"),
        Path::new("efm32-common/kits/common/bsp"),
        Path::new("efm32-common/kits/EFM32GG_STK3700/config"),
        Path::new("efm32-common/emdrv/gpiointerrupt/inc"),
    );

    let emlib_definitions = vec!(
        ("EFM32GG990F1024".to_string(), None)
    );

    let emlib_flags = vec!(
        "-g".to_string(),
        "-Wall".to_string(),
        "-mthumb".to_string(),
        "-mcpu=cortex-m3".to_string(),
        "-Wl,--start-group".to_string(),
        "-lgcc".to_string(),
        "-lc".to_string(),
        "-lnosys".to_string(),
        "-Wl,--end-group".to_string(),
    );

    let emlib_config = gcc::Config {
        include_directories: emlib_include_directories,
        definitions: emlib_definitions,
        objects: Vec::new(),
        flags: emlib_flags
    };

    gcc::compile_library("libcompiler-rt.a", &emlib_config, &emlib_sources);
}
