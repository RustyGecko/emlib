
pub fn ebi_init() -> i32 {
    unsafe { BSP_EbiInit() }
}


extern {
    fn BSP_EbiInit() -> i32;
}
