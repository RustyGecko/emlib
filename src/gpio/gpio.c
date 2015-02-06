#include "em_gpio.h"

void STATIC_INLINE_GPIO_DbgSWDClkEnable(bool enable) {
  GPIO_DbgSWDClkEnable(enable);
}

void STATIC_INLINE_GPIO_DbgSWDIOEnable(bool enable) {
  GPIO_DbgSWDIOEnable(enable);
}

void STATIC_INLINE_GPIO_DbgSWOEnable(bool enable) {
  GPIO_DbgSWOEnable(enable);
}

void STATIC_INLINE_GPIO_EM4DisablePinWakeup(uint32_t pinmask) {
  GPIO_EM4DisablePinWakeup(pinmask);
}

void STATIC_INLINE_GPIO_EM4EnablePinWakeup(uint32_t pinmask, uint32_t polaritymask) {
  GPIO_EM4EnablePinWakeup(pinmask, polaritymask);
}

uint32_t STATIC_INLINE_GPIO_EM4GetPinWakeupCause() {
  return GPIO_EM4GetPinWakeupCause();
}

void STATIC_INLINE_GPIO_EM4SetPinRetention(bool enable) {
  GPIO_EM4SetPinRetention(enable);
}

void STATIC_INLINE_GPIO_InputSenseSet(uint32_t val, uint32_t mask) {
  GPIO_InputSenseSet(val, mask);
}

void STATIC_INLINE_GPIO_IntClear(uint32_t flags) {
  GPIO_IntClear(flags);
}

void STATIC_INLINE_GPIO_IntDisable(uint32_t flags) {
  GPIO_IntDisable(flags);
}

void STATIC_INLINE_GPIO_IntEnable(uint32_t flags) {
  GPIO_IntEnable(flags);
}

uint32_t STATIC_INLINE_GPIO_IntGet() {
  return GPIO_IntGet();
}

uint32_t STATIC_INLINE_GPIO_IntGetEnabled() {
  return GPIO_IntGetEnabled();
}

void STATIC_INLINE_GPIO_IntSet(uint32_t flags) {
  GPIO_IntSet(flags);
}

void STATIC_INLINE_GPIO_Lock() {
  GPIO_Lock();
}

unsigned int STATIC_INLINE_GPIO_PinInGet(GPIO_Port_TypeDef port, unsigned int pin) {
  return GPIO_PinInGet(port, pin);
}

void STATIC_INLINE_GPIO_PinOutClear(GPIO_Port_TypeDef port, unsigned int pin) {
  GPIO_PinOutClear(port, pin);
}

unsigned int STATIC_INLINE_GPIO_PinOutGet(GPIO_Port_TypeDef port, unsigned int pin) {
  return GPIO_PinOutGet(port, pin);
}

void STATIC_INLINE_GPIO_PinOutSet(GPIO_Port_TypeDef port, unsigned int pin) {
  GPIO_PinOutSet(port, pin);
}

void STATIC_INLINE_GPIO_PinOutToggle(GPIO_Port_TypeDef port, unsigned int pin) {
  GPIO_PinOutToggle(port, pin);
}

uint32_t STATIC_INLINE_GPIO_PortInGet(GPIO_Port_TypeDef port) {
  return GPIO_PortInGet(port);
}

void STATIC_INLINE_GPIO_PortOutClear(GPIO_Port_TypeDef port, uint32_t pins) {
  GPIO_PortOutClear(port, pins);
}

uint32_t STATIC_INLINE_GPIO_PortOutGet(GPIO_Port_TypeDef port) {
  return GPIO_PortOutGet(port);
}

void STATIC_INLINE_GPIO_PortOutSet(GPIO_Port_TypeDef port, uint32_t pins) {
  GPIO_PortOutSet(port, pins);
}

void STATIC_INLINE_GPIO_PortOutSetVal(GPIO_Port_TypeDef port, uint32_t val, uint32_t mask) {
  GPIO_PortOutSetVal(port, val, mask);
}

void STATIC_INLINE_GPIO_PortOutToggle(GPIO_Port_TypeDef port, uint32_t pins) {
  GPIO_PortOutToggle(port, pins);
}

void STATIC_INLINE_GPIO_Unlock() {
  GPIO_Unlock();
}
