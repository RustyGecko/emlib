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
        "/home/sondre/opt/SimplicityStudio_v2/developer/sdks/efm32/v2/Device/SiliconLabs/EFM32GG/Source/system_efm32gg.c",
        "/home/sondre/opt/SimplicityStudio_v2/developer/sdks/efm32/v2/emlib/src/em_cmu.c",
        "/home/sondre/opt/SimplicityStudio_v2/developer/sdks/efm32/v2/emlib/src/em_emu.c",
        "/home/sondre/opt/SimplicityStudio_v2/developer/sdks/efm32/v2/emlib/src/em_gpio.c",
        "/home/sondre/opt/SimplicityStudio_v2/developer/sdks/efm32/v2/emlib/src/em_rtc.c",
        "/home/sondre/opt/SimplicityStudio_v2/developer/sdks/efm32/v2/emlib/src/em_system.c",
        "/home/sondre/opt/SimplicityStudio_v2/developer/sdks/efm32/v2/emlib/src/em_timer.c",
        "/home/sondre/opt/SimplicityStudio_v2/developer/sdks/efm32/v2/emlib/src/em_int.c",
        "/home/sondre/opt/SimplicityStudio_v2/developer/sdks/efm32/v2/kits/common/drivers/retargetio.c",
        "/home/sondre/opt/SimplicityStudio_v2/developer/sdks/efm32/v2/emdrv/gpiointerrupt/src/gpiointerrupt.c",
        "src/gpio/gpio.c",
        "src/chip/chip.c",
        "src/emdrv/gpiointerrupt.c",
        "/home/sondre/opt/SimplicityStudio_v2/developer/sdks/efm32/v2/Device/SiliconLabs/EFM32GG/Source/GCC/startup_efm32gg.S"
    ];

    let emlib_include_directories = vec!(
        Path::new("/home/sondre/opt/SimplicityStudio_v2/developer/sdks/efm32/v2/CMSIS/Include"),
        Path::new("/home/sondre/opt/SimplicityStudio_v2/developer/sdks/efm32/v2/Device/SiliconLabs/EFM32GG/Include"),
        Path::new("/home/sondre/opt/SimplicityStudio_v2/developer/sdks/efm32/v2/emlib/inc"),
        Path::new("/home/sondre/opt/SimplicityStudio_v2/developer/sdks/efm32/v2/kits/common/drivers"),
        Path::new("/home/sondre/opt/SimplicityStudio_v2/developer/sdks/efm32/v2/kits/common/bsp"),
        Path::new("/home/sondre/opt/SimplicityStudio_v2/developer/sdks/efm32/v2/kits/EFM32GG_STK3700/config"),
        Path::new("/home/sondre/opt/SimplicityStudio_v2/developer/sdks/efm32/v2/emdrv/gpiointerrupt/inc"),
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
