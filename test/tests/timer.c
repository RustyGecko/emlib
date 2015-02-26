#include <stdbool.h>
#include <em_timer.h>

#include "unity.h"
#include "cmock.h"
#include "Mocktimer.h"
#include "Mockem_timer.h"

void expect_init_called_with_default() {

    static TIMER_Init_TypeDef timer_init = TIMER_INIT_DEFAULT;
    TIMER_Init_Expect(TIMER0, &timer_init);

}

void expect_init_cc_called_with_default() {

    static TIMER_InitCC_TypeDef timer_init_cc = TIMER_INITCC_DEFAULT;
    TIMER_InitCC_Expect(TIMER0, 0, &timer_init_cc);

}

void expect_init_dti_called_with_default() {

    static TIMER_InitDTI_TypeDef timer_init_dti = TIMER_INITDTI_DEFAULT;
    TIMER_InitDTI_Expect(TIMER0, &timer_init_dti);

}

void expect_reset_called() {

    TIMER_Reset_Expect(TIMER0);

}

void expect_enable_called() {

    STATIC_INLINE_TIMER_Enable_Expect(TIMER0, true);

}
