#include "em_timer.h"

uint32_t STATIC_INLINE_TIMER_CaptureGet(TIMER_TypeDef *timer, unsigned int ch);
void STATIC_INLINE_TIMER_CompareBufSet(TIMER_TypeDef *timer, unsigned int ch, uint32_t val);
void STATIC_INLINE_TIMER_CompareSet(TIMER_TypeDef *timer, unsigned int ch, uint32_t val);
uint32_t STATIC_INLINE_TIMER_CounterGet(TIMER_TypeDef *timer);
void STATIC_INLINE_TIMER_CounterSet(TIMER_TypeDef *timer, uint32_t val);
void STATIC_INLINE_TIMER_Enable(TIMER_TypeDef *timer, bool enable);
void STATIC_INLINE_TIMER_EnableDTI(TIMER_TypeDef *timer, bool enable);
uint32_t STATIC_INLINE_TIMER_GetDTIFault(TIMER_TypeDef *timer);
void STATIC_INLINE_TIMER_ClearDTIFault(TIMER_TypeDef *timer, uint32_t flags);
void STATIC_INLINE_TIMER_IntClear(TIMER_TypeDef *timer, uint32_t flags);
void STATIC_INLINE_TIMER_IntDisable(TIMER_TypeDef *timer, uint32_t flags);
void STATIC_INLINE_TIMER_IntEnable(TIMER_TypeDef *timer, uint32_t flags);
uint32_t STATIC_INLINE_TIMER_IntGet(TIMER_TypeDef *timer);
uint32_t STATIC_INLINE_TIMER_IntGetEnabled(TIMER_TypeDef *timer);
void STATIC_INLINE_TIMER_IntSet(TIMER_TypeDef *timer, uint32_t flags);
void STATIC_INLINE_TIMER_Lock(TIMER_TypeDef *timer);
void STATIC_INLINE_TIMER_TopBufSet(TIMER_TypeDef *timer, uint32_t val);
uint32_t STATIC_INLINE_TIMER_TopGet(TIMER_TypeDef *timer);
void STATIC_INLINE_TIMER_TopSet(TIMER_TypeDef *timer, uint32_t val);
void STATIC_INLINE_TIMER_Unlock(TIMER_TypeDef *timer);
