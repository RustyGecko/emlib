
use gcc::Config;

#[allow(dead_code)]
pub fn kit_config(config: &mut Config) -> &mut Config {
    println!("Configuring STK3700");
    super::common_config(config)
        .include("efm32-common/kits/EFM32GG_STK3700/config")

        .file("efm32-common/kits/common/bsp/bsp_stk.c")
        .file("efm32-common/kits/common/bsp/bsp_bcc.c")
}
