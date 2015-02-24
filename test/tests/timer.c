#include <stdbool.h>
#include <em_timer.h>

#include "unity.h"
#include "cmock.h"
#include "Mockem_timer.h"

void expect_init_called() {

    TIMER_Init_TypeDef timer_init = TIMER_INIT_DEFAULT;
    TIMER_Init_Expect(TIMER0, &timer_init);

}

