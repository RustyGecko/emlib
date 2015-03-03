#include "adc.h"

void  STATIC_INLINE_ADC_Start(ADC_TypeDef *adc, ADC_Start_TypeDef cmd) {
    ADC_Start(adc, cmd);
}

uint32_t STATIC_INLINE_ADC_DataSingleGet(ADC_TypeDef *adc) {
    return ADC_DataSingleGet(adc);
}
