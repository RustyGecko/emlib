#include <em_i2c.h>

I2C_TypeDef* GET_I2C0() {
    return I2C0;
}

void STATIC_INLINE_I2C_IntClear(I2C_TypeDef *i2c, uint32_t flags) {
    I2C_IntClear(i2c, flags);
}

void STATIC_INLINE_I2C_IntDisable(I2C_TypeDef *i2c, uint32_t flags) {
    I2C_IntDisable(i2c, flags);
}

void STATIC_INLINE_I2C_IntEnable(I2C_TypeDef *i2c, uint32_t flags) {
    I2C_IntEnable(i2c, flags);
}
