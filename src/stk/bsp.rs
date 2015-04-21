
pub fn ebi_init() -> i32 {
    unsafe { BSP_EbiInit() }
}

pub fn trace_swo_setup() {
    unsafe { BSP_TraceSwoSetup(); }
}

extern {
    fn BSP_EbiInit() -> i32;

    // Trace
    fn BSP_TraceSwoSetup();
}
