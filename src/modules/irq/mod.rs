use alloc::boxed::Box;
use collections::BTreeMap;
use core::prelude::*;

static mut interrupt_hub: Option<*mut (EventListener + 'static)> = None;

pub unsafe fn set_interrupt_hub(hub: *mut (EventListener + 'static)) {
    match interrupt_hub {
        None => interrupt_hub = Some(hub),
        _ => ()
    }
}

pub fn dispatch(event: Event) {
    unsafe {
        match interrupt_hub {
            Some(hub) => (*hub).dispatch(event),
            _ => ()
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Event {
    DMA,
    GPIO_EVEN,
    TIMER0,
    USART0_RX,
    USART0_TX,
    USB,
    ACMP0,
    ADC0,
    DAC0,
    I2C0,
    I2C1,
    GPIO_ODD,
    TIMER1,
    TIMER2,
    TIMER3,
    USART1_RX,
    USART1_TX,
    LESENSE,
    USART2_RX,
    USART2_TX,
    UART0_RX,
    UART0_TX,
    UART1_RX,
    UART1_TX,
    LEUART0,
    LEUART1,
    LETIMER0,
    PCNT0,
    PCNT1,
    PCNT2,
    RTC,
    BURTC,
    CMU,
    VCMP,
    LCD,
    MSC,
    AES,
    EBI,
    EMU,
}

pub trait EventListener {
    fn dispatch(&mut self, event: Event);
}

pub struct Hub {
    handlers: BTreeMap<Event, Box<FnMut()>>
}

impl Hub {

    pub fn new() -> Hub {

        Hub {
            handlers: BTreeMap::new()
        }
    }

    pub fn on(&mut self, event: Event, callback: Box<FnMut()>) {

        self.handlers.insert(event, callback);
    }

}

impl EventListener for Hub {

    fn dispatch(&mut self, event: Event) {

        match self.handlers.get_mut(&event) {
            Some(ref mut handler) => handler.call_mut(()),
            _ => ()
        }
    }
}

#[no_mangle] #[allow(non_snake_case)]
pub unsafe extern fn TIMER0_IRQHandler() {
    dispatch(Event::TIMER0);
}

#[no_mangle] #[allow(non_snake_case)]
pub unsafe extern fn USART0_RX_IRQHandler() {
    dispatch(Event::USART0_RX);
}

#[no_mangle] #[allow(non_snake_case)]
pub unsafe extern fn USART0_TX_IRQHandler() {
    dispatch(Event::USART0_TX);
}

#[no_mangle] #[allow(non_snake_case)]
pub unsafe extern fn USB_IRQHandler() {
    dispatch(Event::USB);
}

#[no_mangle] #[allow(non_snake_case)]
pub unsafe extern fn ACMP0_IRQHandler() {
    dispatch(Event::ACMP0);
}

#[no_mangle] #[allow(non_snake_case)]
pub unsafe extern fn ADC0_IRQHandler() {
    dispatch(Event::ADC0);
}

#[no_mangle] #[allow(non_snake_case)]
pub unsafe extern fn DAC0_IRQHandler() {
    dispatch(Event::DAC0);
}

#[no_mangle] #[allow(non_snake_case)]
pub unsafe extern fn I2C0_IRQHandler() {
    dispatch(Event::I2C0);
}

#[no_mangle] #[allow(non_snake_case)]
pub unsafe extern fn I2C1_IRQHandler() {
    dispatch(Event::I2C1);
}

#[no_mangle] #[allow(non_snake_case)]
pub unsafe extern fn TIMER1_IRQHandler() {
    dispatch(Event::TIMER1);
}

#[no_mangle] #[allow(non_snake_case)]
pub unsafe extern fn TIMER2_IRQHandler() {
    dispatch(Event::TIMER2);
}

#[no_mangle] #[allow(non_snake_case)]
pub unsafe extern fn TIMER3_IRQHandler() {
    dispatch(Event::TIMER3);
}

#[no_mangle] #[allow(non_snake_case)]
pub unsafe extern fn USART1_RX_IRQHandler() {
    dispatch(Event::USART1_RX);
}

#[no_mangle] #[allow(non_snake_case)]
pub unsafe extern fn USART1_TX_IRQHandler() {
    dispatch(Event::USART1_TX);
}

#[no_mangle] #[allow(non_snake_case)]
pub unsafe extern fn LESENSE_IRQHandler() {
    dispatch(Event::LESENSE);
}

#[no_mangle] #[allow(non_snake_case)]
pub unsafe extern fn USART2_RX_IRQHandler() {
    dispatch(Event::USART2_RX);
}

#[no_mangle] #[allow(non_snake_case)]
pub unsafe extern fn USART2_TX_IRQHandler() {
    dispatch(Event::USART2_TX);
}

#[no_mangle] #[allow(non_snake_case)]
pub unsafe extern fn UART0_RX_IRQHandler() {
    dispatch(Event::UART0_RX);
}

#[no_mangle] #[allow(non_snake_case)]
pub unsafe extern fn UART0_TX_IRQHandler() {
    dispatch(Event::UART0_TX);
}

#[no_mangle] #[allow(non_snake_case)]
pub unsafe extern fn UART1_RX_IRQHandler() {
    dispatch(Event::UART1_RX);
}

#[no_mangle] #[allow(non_snake_case)]
pub unsafe extern fn UART1_TX_IRQHandler() {
    dispatch(Event::UART1_TX);
}

#[no_mangle] #[allow(non_snake_case)]
pub unsafe extern fn LEUART0_IRQHandler() {
    dispatch(Event::LEUART0);
}

#[no_mangle] #[allow(non_snake_case)]
pub unsafe extern fn LEUART1_IRQHandler() {
    dispatch(Event::LEUART1);
}

#[no_mangle] #[allow(non_snake_case)]
pub unsafe extern fn LETIMER0_IRQHandler() {
    dispatch(Event::LETIMER0);
}

#[no_mangle] #[allow(non_snake_case)]
pub unsafe extern fn PCNT0_IRQHandler() {
    dispatch(Event::PCNT0);
}

#[no_mangle] #[allow(non_snake_case)]
pub unsafe extern fn PCNT1_IRQHandler() {
    dispatch(Event::PCNT1);
}

#[no_mangle] #[allow(non_snake_case)]
pub unsafe extern fn PCNT2_IRQHandler() {
    dispatch(Event::PCNT2);
}

#[no_mangle] #[allow(non_snake_case)]
pub unsafe extern fn RTC_IRQHandler() {
    dispatch(Event::RTC);
}

#[no_mangle] #[allow(non_snake_case)]
pub unsafe extern fn BURTC_IRQHandler() {
    dispatch(Event::BURTC);
}

#[no_mangle] #[allow(non_snake_case)]
pub unsafe extern fn CMU_IRQHandler() {
    dispatch(Event::CMU);
}

#[no_mangle] #[allow(non_snake_case)]
pub unsafe extern fn VCMP_IRQHandler() {
    dispatch(Event::VCMP);
}

#[no_mangle] #[allow(non_snake_case)]
pub unsafe extern fn LCD_IRQHandler() {
    dispatch(Event::LCD);
}

#[no_mangle] #[allow(non_snake_case)]
pub unsafe extern fn MSC_IRQHandler() {
    dispatch(Event::MSC);
}

#[no_mangle] #[allow(non_snake_case)]
pub unsafe extern fn AES_IRQHandler() {
    dispatch(Event::AES);
}

#[no_mangle] #[allow(non_snake_case)]
pub unsafe extern fn EBI_IRQHandler() {
    dispatch(Event::EBI);
}

#[no_mangle] #[allow(non_snake_case)]
pub unsafe extern fn EMU_IRQHandler() {
    dispatch(Event::EMU);
}
