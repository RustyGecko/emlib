pub mod sys_tick {

    extern {
        pub fn STATIC_INLINE_SysTick_Config(ticks: u32) -> u32;
    }
    
    pub fn config(ticks: u32) -> u32 {
        unsafe { STATIC_INLINE_SysTick_Config(ticks) }
    }
}

pub mod itm;
pub mod nvic;
