#[repr(C)]
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
    pub fn STATIC_INLINE_NVIC_SetPriorityGrouping(PriorityGroup: u32);
    pub fn STATIC_INLINE_NVIC_GetPriorityGrouping();
    pub fn STATIC_INLINE_NVIC_EnableIRQ(IRQn: IRQn);
    pub fn STATIC_INLINE_NVIC_DisableIRQ(IRQn: IRQn);
    pub fn STATIC_INLINE_NVIC_GetPendingIRQ(IRQn: IRQn);
    pub fn STATIC_INLINE_NVIC_SetPendingIRQ(IRQn: IRQn);
    pub fn STATIC_INLINE_NVIC_ClearPendingIRQ(IRQn: IRQn);
    pub fn STATIC_INLINE_NVIC_GetActive(IRQn: IRQn);
    pub fn STATIC_INLINE_NVIC_SetPriority(IRQn: IRQn, priority: u32);
    pub fn STATIC_INLINE_NVIC_GetPriority(IRQn: IRQn);
    pub fn STATIC_INLINE_NVIC_EncodePriority (PriorityGroup: u32, PreemptPriority: u32, SubPriority: u32);
    pub fn STATIC_INLINE_NVIC_DecodePriority (Priority: u32, PriorityGroup: u32, pPreemptPriority: *mut u32, pSubPriority: *mut u32);
    pub fn STATIC_INLINE_NVIC_SystemReset();
}

pub fn NVIC_SetPriorityGrouping(PriorityGroup: u32) {
    unsafe { STATIC_INLINE_NVIC_SetPriorityGrouping(PriorityGroup) }
}

pub fn NVIC_GetPriorityGrouping() {
    unsafe { STATIC_INLINE_NVIC_GetPriorityGrouping() }
}

pub fn enable_IRQ(IRQn: IRQn) {
    unsafe { STATIC_INLINE_NVIC_EnableIRQ(IRQn) }
}

pub fn NVIC_DisableIRQ(IRQn: IRQn) {
    unsafe { STATIC_INLINE_NVIC_DisableIRQ(IRQn) }
}

pub fn NVIC_GetPendingIRQ(IRQn: IRQn) {
    unsafe { STATIC_INLINE_NVIC_GetPendingIRQ(IRQn) }
}

pub fn NVIC_SetPendingIRQ(IRQn: IRQn) {
    unsafe { STATIC_INLINE_NVIC_SetPendingIRQ(IRQn) }
}

pub fn clear_pending_IRQ(IRQn: IRQn) {
    unsafe { STATIC_INLINE_NVIC_ClearPendingIRQ(IRQn) }
}

pub fn NVIC_GetActive(IRQn: IRQn) {
    unsafe { STATIC_INLINE_NVIC_GetActive(IRQn) }
}

pub fn NVIC_SetPriority(IRQn: IRQn, priority: u32) {
    unsafe { STATIC_INLINE_NVIC_SetPriority(IRQn, priority) }
}

pub fn NVIC_GetPriority(IRQn: IRQn) {
    unsafe { STATIC_INLINE_NVIC_GetPriority(IRQn) }
}

pub fn NVIC_EncodePriority(PriorityGroup: u32, PreemptPriority: u32, SubPriority: u32) {
    unsafe { STATIC_INLINE_NVIC_EncodePriority(PriorityGroup, PreemptPriority, SubPriority) }
}

pub fn NVIC_DecodePriority(Priority: u32, PriorityGroup: u32, pPreemptPriority: *mut u32, pSubPriority: *mut u32) {
    unsafe { STATIC_INLINE_NVIC_DecodePriority(Priority, PriorityGroup, pPreemptPriority, pSubPriority) }
}

pub fn NVIC_SystemReset() {
    unsafe { STATIC_INLINE_NVIC_SystemReset() }
}

