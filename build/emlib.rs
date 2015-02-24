#![deny(warnings)]
#![feature(core, env, old_io, old_path)]

extern crate gcc;

use std::env;
use std::old_io::File;
use std::old_io::IoResult;

fn main() {
    compile_emlib_library();

    match write_emlib_hash() {
        Ok(_) => (),
        Err(e) => panic!("{}", e)
    }
}

fn compile_emlib_library() {

    println!("The ARM embedded toolchain must be available in the PATH");
    env::set_var("CC", "arm-none-eabi-gcc");
    env::set_var("AR", "arm-none-eabi-ar");

    let config = match env::get_var("BUILD_ENV") {
        Ok("prod") => prod_config(),
        Ok("test") => test_config(),
        _ => prod_config()
    }

    config.compile("libcompiler-rt.a");

}

fn base_config() {
    
    gcc::Config::new()
        .define("EFM32GG990F1024", None)

        .include("efm32-common/CMSIS/Include")
        .include("efm32-common/Device/EFM32GG/Include")
        .include("efm32-common/kits/EFM32GG_STK3700/config")
        .include("efm32-common/emlib/inc")
        
        .file("efm32-common/Device/EFM32GG/Source/GCC/startup_efm32gg.S")
        .file("efm32-common/Device/EFM32GG/Source/system_efm32gg.c")

        .file("efm32-common/emlib/src/em_cmu.c")
        .file("efm32-common/emlib/src/em_gpio.c")
        .file("efm32-common/emlib/src/em_usart.c")
        
        .flag("-g")
        .flag("-Wall")
        .flag("-mthumb")
        .flag("-mcpu=cortex-m3")

}

fn prod_config() {
    
    base_config()

        .include("efm32-common/kits/common/bsp")
        
        .file("efm32-common/emlib/src/em_dma.c")
        .file("efm32-common/emlib/src/em_emu.c")
        .file("efm32-common/emlib/src/em_rtc.c")
        .file("efm32-common/emlib/src/em_system.c")
        .file("efm32-common/emlib/src/em_timer.c")
        .file("efm32-common/emlib/src/em_int.c")

        .file("src/chip/chip.c")
        .file("src/cmsis/cmsis.c")
        .file("src/emu/emu.c")
        .file("src/dma/dma.c")
        .file("src/gpio/gpio.c")
        .file("src/rtc/rtc.c")
        .file("src/timer/timer.c")
        .file("src/usart/usart.c")

        .include("efm32-common/emdrv/gpiointerrupt/inc")
        .file("efm32-common/emdrv/gpiointerrupt/src/gpiointerrupt.c")
        .file("src/emdrv/gpiointerrupt.c")

        .include("efm32-common/kits/common/drivers")
        .file("efm32-common/kits/common/drivers/dmactrl.c")
        .file("efm32-common/kits/common/drivers/retargetio.c")

}

fn test_config() {

    base_config()

        .flag("-DUNITY_OUTPUT_CHAR=print_char")
        .flag("-DNULL=0")
        
        .file("src/chip/chip.c")
        .file("src/cmsis/cmsis.c")
        .file("src/timer/timer.c")
        .file("src/gpio/gpio.c")

        .file("test/lib/Unity/src/unity.c")
        .file("test/lib/cmock/src/cmock.c")
        .file("test/util/usart_print.c")
        
        // Mocks
        .file("test/mocks/Mockem_timer.c")

        // Tests
        .file("test/tests/timer.c")


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
