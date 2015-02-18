#![deny(warnings)]
#![feature(core, env, io, path)]

extern crate gcc;

use std::env;
use std::old_io::File;
use std::old_io::IoResult;

fn assert_env_var(var: &str, expected: &str) {
    env::set_var(var, expected);
    match env::var(var) {
        Ok(ref val) if &val[] == expected => (),
        _ => panic!("`{}` environment variable must be `{}`", var, expected)
    }
}

fn main() {
    compile_emlib_library();

    match write_emlib_hash() {
        Ok(_) => (),
        Err(e) => panic!("{}", e)
    }
}

fn compile_emlib_library() {
    assert_env_var("CC", "arm-none-eabi-gcc");
    assert_env_var("AR", "arm-none-eabi-ar");

    let emlib_sources = [
        "efm32-common/Device/EFM32GG/Source/GCC/startup_efm32gg.S",
        "efm32-common/Device/EFM32GG/Source/system_efm32gg.c",
        "efm32-common/emlib/src/em_cmu.c",
        "efm32-common/emlib/src/em_dma.c",
        "efm32-common/emlib/src/em_emu.c",
        "efm32-common/emlib/src/em_gpio.c",
        "efm32-common/emlib/src/em_rtc.c",
        "efm32-common/emlib/src/em_system.c",
        "efm32-common/emlib/src/em_timer.c",
        "efm32-common/emlib/src/em_usart.c",
        "efm32-common/emlib/src/em_int.c",
        "efm32-common/kits/common/drivers/dmactrl.c",
        "efm32-common/emdrv/gpiointerrupt/src/gpiointerrupt.c",

        "src/chip/chip.c",
        "src/cmsis/cmsis.c",
        "src/emu/emu.c",
        "src/dma/dma.c",
        "src/gpio/gpio.c",
        "src/rtc/rtc.c",
        "src/timer/timer.c",
        "src/usart/usart.c",
        
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

fn write_emlib_hash() -> IoResult<()> {
    // Get OUT_DIR and convert it from OsString to String
    let out_dir = env::var("OUT_DIR").ok().unwrap();
    // Extract the hash
    let hash_token: String = out_dir.rsplitn(2, '/').nth(1).unwrap()
                                    .rsplitn(1, '-').nth(0).unwrap().to_string();
    let emlib_hash = format!("HASH={}", hash_token);
    println!("{}", emlib_hash);

    // Write to .emlib_hash file
    let emlib_hash_file = env::var("CARGO_MANIFEST_DIR").ok().unwrap() + "/.emlib_hash";
    let mut f = try!(File::create(&Path::new(emlib_hash_file)));
    f.write_line(emlib_hash.as_slice())
}
