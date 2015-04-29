#include "em_ebi.h"

uint32_t STATIC_INLINE_EBI_IntGet() {
    return EBI_IntGet();
}

void STATIC_INLINE_EBI_IntClear(uint32_t flags) {
    EBI_IntClear(flags);
}

void STATIC_INLINE_EBI_IntDisable(uint32_t flags) {
    EBI_IntDisable(flags);
}

void STATIC_INLINE_EBI_IntEnable(uint32_t flags) {
    EBI_IntEnable(flags);
}

void STATIC_INLINE_EBI_TFTAlphaBlendSet(uint8_t alpha) {
    EBI_TFTAlphaBlendSet(alpha);
}

uint32_t STATIC_INLINE_EBI_TFTVCount() {
    return EBI_TFTVCount();
}

void STATIC_INLINE_EBI_TFTFrameBaseSet(uint32_t address) {
    EBI_TFTFrameBaseSet(address);
}

void STATIC_INLINE_EBI_TFTHStrideSet(uint32_t nbytes) {
    EBI_TFTHStrideSet(nbytes);
}