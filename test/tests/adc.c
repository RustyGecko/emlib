#include <stdbool.h>
#include <em_adc.h>

#include "unity.h"
#include "cmock.h"
#include "Mockadc.h"
#include "Mockem_adc.h"

void adc_expect_init_called_with_default() {

    static ADC_Init_TypeDef init = ADC_INIT_DEFAULT;
    ADC_Init_Expect(ADC0, &init);

}

void adc_expect_init_single_called_with_default() {

    static ADC_InitSingle_TypeDef init = ADC_INITSINGLE_DEFAULT;
    ADC_InitSingle_Expect(ADC0, &init);

}

void adc_expect_timebase_calc_called_with_value() {
    ADC_TimebaseCalc_ExpectAndReturn(0, 100);
}

void adc_expect_prescale_calc_called_with_value() {
    ADC_PrescaleCalc_ExpectAndReturn(7000000, 0, 100);
}

void adc_expect_data_single_get_called() {
    STATIC_INLINE_ADC_DataSingleGet_ExpectAndReturn(ADC0, 0);
}
