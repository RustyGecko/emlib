#[repr(C)]
#[derive(Copy)]
pub enum IRQn {
/******  Cortex-M3 Processor Exceptions Numbers *******************************************/
    NonMaskableInt   = -14,
    HardFault        = -13,
    MemoryManagement = -12,
    BusFault         = -11,
    UsageFault       = -10,
    SVCall           = -5,
    DebugMonitor     = -4,
    PendSV           = -2,
    SysTick          = -1,

/******  EFM32G Peripheral Interrupt Numbers **********************************************/
    DMA              = 0,
    GPIO_EVEN        = 1,
    TIMER0           = 2,
    USART0_RX        = 3,
    USART0_TX        = 4,
    USB              = 5,
    ACMP0            = 6,
    ADC0             = 7,
    DAC0             = 8,
    I2C0             = 9,
    I2C1             = 10,
    GPIO_ODD         = 11,
    TIMER1           = 12,
    TIMER2           = 13,
    TIMER3           = 14,
    USART1_RX        = 15,
    USART1_TX        = 16,
    LESENSE          = 17,
    USART2_RX        = 18,
    USART2_TX        = 19,
    UART0_RX         = 20,
    UART0_TX         = 21,
    UART1_RX         = 22,
    UART1_TX         = 23,
    LEUART0          = 24,
    LEUART1          = 25,
    LETIMER0         = 26,
    PCNT0            = 27,
    PCNT1            = 28,
    PCNT2            = 29,
    RTC              = 30,
    BURTC            = 31,
    CMU              = 32,
    VCMP             = 33,
    LCD              = 34,
    MSC              = 35,
    AES              = 36,
    EBI              = 37,
    EMU              = 38,
}

extern {
    pub fn STATIC_INLINE_NVIC_SetPriorityGrouping(priority_group: u32);
    pub fn STATIC_INLINE_NVIC_GetPriorityGrouping();
    pub fn STATIC_INLINE_NVIC_EnableIRQ(irq_n: IRQn);
    pub fn STATIC_INLINE_NVIC_DisableIRQ(irq_n: IRQn);
    pub fn STATIC_INLINE_NVIC_GetPendingIRQ(irq_n: IRQn);
    pub fn STATIC_INLINE_NVIC_SetPendingIRQ(irq_n: IRQn);
    pub fn STATIC_INLINE_NVIC_ClearPendingIRQ(irq_n: IRQn);
    pub fn STATIC_INLINE_NVIC_GetActive(irq_n: IRQn);
    pub fn STATIC_INLINE_NVIC_SetPriority(irq_n: IRQn, priority: u32);
    pub fn STATIC_INLINE_NVIC_GetPriority(irq_n: IRQn);
    pub fn STATIC_INLINE_NVIC_EncodePriority(priority_group: u32, preempt_priority: u32, sub_priority: u32);
    pub fn STATIC_INLINE_NVIC_DecodePriority(priority: u32, priority_group: u32, p_preempt_priority: *mut u32, p_sub_priority: *mut u32);
    pub fn STATIC_INLINE_NVIC_SystemReset();
}

pub fn set_priority_grouping(priority_group: u32) {
    unsafe { STATIC_INLINE_NVIC_SetPriorityGrouping(priority_group) }
}

pub fn get_priority_grouping() {
    unsafe { STATIC_INLINE_NVIC_GetPriorityGrouping() }
}

pub fn enable_irq(irq_n: IRQn) {
    unsafe { STATIC_INLINE_NVIC_EnableIRQ(irq_n) }
}

pub fn disable_irq(irq_n: IRQn) {
    unsafe { STATIC_INLINE_NVIC_DisableIRQ(irq_n) }
}

pub fn get_pending_irq(irq_n: IRQn) {
    unsafe { STATIC_INLINE_NVIC_GetPendingIRQ(irq_n) }
}

pub fn set_pending_irq(irq_n: IRQn) {
    unsafe { STATIC_INLINE_NVIC_SetPendingIRQ(irq_n) }
}

pub fn clear_pending_irq(irq_n: IRQn) {
    unsafe { STATIC_INLINE_NVIC_ClearPendingIRQ(irq_n) }
}

pub fn get_active(irq_n: IRQn) {
    unsafe { STATIC_INLINE_NVIC_GetActive(irq_n) }
}

pub fn set_priority(irq_n: IRQn, priority: u32) {
    unsafe { STATIC_INLINE_NVIC_SetPriority(irq_n, priority) }
}

pub fn get_priority(irq_n: IRQn) {
    unsafe { STATIC_INLINE_NVIC_GetPriority(irq_n) }
}

pub fn encode_priority(priority_group: u32, preempt_priority: u32, sub_priority: u32) {
    unsafe { STATIC_INLINE_NVIC_EncodePriority(priority_group, preempt_priority, sub_priority) }
}

pub fn decode_priority(priority: u32, priority_group: u32, p_preempt_priority: *mut u32, p_sub_priority: *mut u32) {
    unsafe { STATIC_INLINE_NVIC_DecodePriority(priority, priority_group, p_preempt_priority, p_sub_priority) }
}

pub fn system_reset() {
    unsafe { STATIC_INLINE_NVIC_SystemReset() }
}
