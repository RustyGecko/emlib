#include "em_timer.h"

TIMER_TypeDef* GET_TIMER0() {
    return TIMER0;
}

TIMER_TypeDef* GET_TIMER1() {
    return TIMER1;
}

TIMER_TypeDef* GET_TIMER2() {
    return TIMER2;
}

TIMER_TypeDef* GET_TIMER3() {
    return TIMER3;
}

uint32_t STATIC_INLINE_TIMER_CaptureGet(TIMER_TypeDef *timer, unsigned int ch) {
    return TIMER_CaptureGet(timer, ch);
}

void STATIC_INLINE_TIMER_CompareBufSet(TIMER_TypeDef *timer, unsigned int ch, uint32_t val) {
    TIMER_CompareBufSet(timer, ch, val);
}

void STATIC_INLINE_TIMER_CompareSet(TIMER_TypeDef *timer, unsigned int ch, uint32_t val) {
    TIMER_CompareSet(timer, ch, val);
}

uint32_t STATIC_INLINE_TIMER_CounterGet(TIMER_TypeDef *timer) {
    return TIMER_CounterGet(timer);
}

void STATIC_INLINE_TIMER_CounterSet(TIMER_TypeDef *timer, uint32_t val) {
    TIMER_CounterSet(timer, val);
}

void STATIC_INLINE_TIMER_Enable(TIMER_TypeDef *timer, bool enable) {
    TIMER_Enable(timer, enable);
}

void STATIC_INLINE_TIMER_EnableDTI(TIMER_TypeDef *timer, bool enable) {
    TIMER_EnableDTI(timer, enable);
}

uint32_t STATIC_INLINE_TIMER_GetDTIFault(TIMER_TypeDef *timer) {
    return TIMER_GetDTIFault(timer);
}

void STATIC_INLINE_TIMER_ClearDTIFault(TIMER_TypeDef *timer, uint32_t flags) {
    TIMER_ClearDTIFault(timer, flags);
}

void STATIC_INLINE_TIMER_IntClear(TIMER_TypeDef *timer, uint32_t flags) {
    TIMER_IntClear(timer, flags);
}

void STATIC_INLINE_TIMER_IntDisable(TIMER_TypeDef *timer, uint32_t flags) {
    TIMER_IntDisable(timer, flags);
}
    
void STATIC_INLINE_TIMER_IntEnable(TIMER_TypeDef *timer, uint32_t flags) {
    TIMER_IntEnable(timer, flags);
}

uint32_t STATIC_INLINE_TIMER_IntGet(TIMER_TypeDef *timer) {
    return TIMER_IntGet(timer);
}

uint32_t STATIC_INLINE_TIMER_IntGetEnabled(TIMER_TypeDef *timer) {
    return TIMER_IntGetEnabled(timer);
}

void STATIC_INLINE_TIMER_IntSet(TIMER_TypeDef *timer, uint32_t flags) {
    TIMER_IntSet(timer, flags);
}

void STATIC_INLINE_TIMER_Lock(TIMER_TypeDef *timer) {
    TIMER_Lock(timer);
}

void STATIC_INLINE_TIMER_TopBufSet(TIMER_TypeDef *timer, uint32_t val) {
    TIMER_TopBufSet(timer, val);
}

uint32_t STATIC_INLINE_TIMER_TopGet(TIMER_TypeDef *timer) {
    return TIMER_TopGet(timer);
}

void STATIC_INLINE_TIMER_TopSet(TIMER_TypeDef *timer, uint32_t val) {
    TIMER_TopSet(timer, val);
}

void STATIC_INLINE_TIMER_Unlock(TIMER_TypeDef *timer) {
        TIMER_Unlock(timer);
}
