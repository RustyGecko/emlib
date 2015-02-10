#include "em_device.h"
#include "core_cm3.h"

volatile int32_t ITM_RxBuffer;

void STATIC_INLINE_NVIC_SetPriorityGrouping(uint32_t PriorityGroup) {
  NVIC_SetPriorityGrouping(PriorityGroup);
}

uint32_t STATIC_INLINE_NVIC_GetPriorityGrouping(void) {
  return NVIC_GetPriorityGrouping();
}

void STATIC_INLINE_NVIC_EnableIRQ(IRQn_Type IRQn) {
  NVIC_EnableIRQ(IRQn);
}

void STATIC_INLINE_NVIC_DisableIRQ(IRQn_Type IRQn) {
  NVIC_DisableIRQ(IRQn);
}

uint32_t STATIC_INLINE_NVIC_GetPendingIRQ(IRQn_Type IRQn) {
  return NVIC_GetPendingIRQ(IRQn);
}

void STATIC_INLINE_NVIC_SetPendingIRQ(IRQn_Type IRQn) {
  NVIC_SetPendingIRQ(IRQn);
}

void STATIC_INLINE_NVIC_ClearPendingIRQ(IRQn_Type IRQn) {
  NVIC_ClearPendingIRQ(IRQn);
}

uint32_t STATIC_INLINE_NVIC_GetActive(IRQn_Type IRQn) {
  return NVIC_GetActive(IRQn);
}

void STATIC_INLINE_NVIC_SetPriority(IRQn_Type IRQn, uint32_t priority) {
  NVIC_SetPriority(IRQn, priority);
}

uint32_t STATIC_INLINE_NVIC_GetPriority(IRQn_Type IRQn) {
  return NVIC_GetPriority(IRQn);
}

uint32_t STATIC_INLINE_NVIC_EncodePriority(uint32_t PriorityGroup, uint32_t PreemptPriority, uint32_t SubPriority) {
  return NVIC_EncodePriority (PriorityGroup, PreemptPriority, SubPriority);
}

void STATIC_INLINE_NVIC_DecodePriority(uint32_t Priority, uint32_t PriorityGroup, uint32_t* pPreemptPriority, uint32_t* pSubPriority) {
  NVIC_DecodePriority (Priority, PriorityGroup, pPreemptPriority, pSubPriority);
}

void STATIC_INLINE_NVIC_SystemReset(void) {
  NVIC_SystemReset();
}

uint32_t STATIC_INLINE_SysTick_Config(uint32_t ticks) {
  return SysTick_Config(ticks);
}

uint32_t STATIC_INLINE_ITM_SendChar (uint32_t ch) {
  return ITM_SendChar(ch);
}

int32_t STATIC_INLINE_ITM_ReceiveChar (void) {
  return ITM_ReceiveChar();
}

int32_t STATIC_INLINE_ITM_CheckChar (void) {
  return ITM_CheckChar();
}  
