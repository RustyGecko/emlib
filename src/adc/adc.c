#include "adc.h"

uint32_t STATIC_INLINE_ADC_DataSingleGet(ADC_TypeDef *adc) {
    return ADC_DataSingleGet(adc);
}
