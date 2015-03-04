#include "em_int.h"

uint32_t STATIC_INLINE_INT_Disable(void) {
    return INT_Disable();
}

uint32_t STATIC_INLINE_INT_Enable(void) {
    return INT_Enable();
}
