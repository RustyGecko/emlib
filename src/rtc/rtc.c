#include "em_rtc.h"

uint32_t STATIC_INLINE_RTC_CounterGet(void) {
    return RTC_CounterGet();
}

void STATIC_INLINE_RTC_IntClear(uint32_t flags) {
    RTC_IntClear(flags);
}

void STATIC_INLINE_RTC_IntDisable(uint32_t flags) {
    RTC_IntDisable(flags);
}

void STATIC_INLINE_RTC_IntEnable(uint32_t flags) {
    RTC_IntEnable(flags);
}

uint32_t STATIC_INLINE_RTC_IntGet(void) {
    return RTC_IntGet();
}

void STATIC_INLINE_RTC_IntSet(uint32_t flags) {
    RTC_IntSet(flags);
}
