#include "em_gpio.h"
#include "em_int.h"
#include "gpiointerrupt.h"
#include <stdint.h>

void STATIC_INLINE_GPIOINT_CallbackUnRegister(uint8_t pin) {
    GPIOINT_CallbackUnRegister(pin);
}
