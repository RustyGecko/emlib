#include "leuart.h"

void STATIC_INLINE_LEUART_IntClear(LEUART_TypeDef *leuart, uint32_t flags) {
    LEUART_IntClear(leuart, flags);
}

void STATIC_INLINE_LEUART_IntEnable(LEUART_TypeDef *leuart, uint32_t flags) {
    LEUART_IntEnable(leuart, flags);
}

void STATIC_INLINE_LEUART_IntDisable(LEUART_TypeDef *leuart, uint32_t flags) {
    LEUART_IntDisable(leuart, flags);
}

uint32_t STATIC_INLINE_LEUART_IntGet(LEUART_TypeDef *leuart) {
    return LEUART_IntGet(leuart);
}

void STATIC_INLINE_LEUART_IntSet(LEUART_TypeDef *leuart, uint32_t flags) {
    LEUART_IntSet(leuart, flags);
}
