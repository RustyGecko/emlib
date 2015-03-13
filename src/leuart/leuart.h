#include <stdint.h>
#include "em_leuart.h"

void STATIC_INLINE_LEUART_IntClear(LEUART_TypeDef *leuart, uint32_t flags);
void STATIC_INLINE_LEUART_IntEnable(LEUART_TypeDef *leuart, uint32_t flags);
void STATIC_INLINE_LEUART_IntDisable(LEUART_TypeDef *leuart, uint32_t flags);
uint32_t STATIC_INLINE_LEUART_IntGet(LEUART_TypeDef *leuart);
void STATIC_INLINE_LEUART_IntSet(LEUART_TypeDef *leuart, uint32_t flags);
