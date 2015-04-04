use emlib::dk::bsp;

static mut ms_ticks: u32 = 0;

#[no_mangle]
pub unsafe extern fn SysTick_Handler() {
    ms_ticks += 1;
}

pub fn delay(num_ticks: u32) {
    unsafe {
        let cur_ticks = ms_ticks;
        while ((ms_ticks - cur_ticks) < num_ticks) {}
    }
}


pub fn blink(n: u32) {
    for _ in 0 .. n {
        bsp::leds_set(0xffff);
        delay(100);
        bsp::leds_set(0x0000);
        delay(100);
    }
}
