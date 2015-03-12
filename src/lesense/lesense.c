#include "lesense.h"

void STATIC_INLINE_LESENSE_DecoderStop(void) {
    LESENSE_DecoderStop();
}

uint32_t STATIC_INLINE_LESENSE_StatusGet(void) {
    return LESENSE_StatusGet();
}

void STATIC_INLINE_LESENSE_StatusWait(uint32_t flag) {
    LESENSE_StatusWait(flag);
}

uint32_t STATIC_INLINE_LESENSE_ChannelActiveGet(void) {
    return LESENSE_ChannelActiveGet();
}

uint32_t STATIC_INLINE_LESENSE_ScanResultGet(void) {
    return LESENSE_ScanResultGet();
}

uint32_t STATIC_INLINE_LESENSE_ScanResultDataGet(void) {
    return LESENSE_ScanResultDataGet();
}

uint32_t STATIC_INLINE_LESENSE_ScanResultDataBufferGet(uint32_t idx) {
    return LESENSE_ScanResultDataBufferGet(idx);
}

uint32_t STATIC_INLINE_LESENSE_SensorStateGet(void) {
    return LESENSE_SensorStateGet();
}

void STATIC_INLINE_LESENSE_RAMPowerDown(void) {
    LESENSE_RAMPowerDown();
}

void STATIC_INLINE_LESENSE_IntClear(uint32_t flags) {
    LESENSE_IntClear(flags);
}

void STATIC_INLINE_LESENSE_IntEnable(uint32_t flags) {
    LESENSE_IntEnable(flags);
}

void STATIC_INLINE_LESENSE_IntDisable(uint32_t flags) {
    LESENSE_IntDisable(flags);
}

void STATIC_INLINE_LESENSE_IntSet(uint32_t flags) {
    LESENSE_IntSet(flags);
}

uint32_t STATIC_INLINE_LESENSE_IntGet(void) {
    return LESENSE_IntGet();
}

uint32_t STATIC_INLINE_LESENSE_IntGetEnabled(void) {
    return LESENSE_IntGetEnabled();
}
