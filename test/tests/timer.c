#include <stdbool.h>
#include <em_timer.h>
#include <em_usart.h>

#define NULL 0
#define UNITY_OUTPUT_CHAR(ch) print_char(ch)

#include "unity.h"
#include "cmock.h"
#include "Mockem_timer.h"

void expect_init_called() {

    TIMER_Init_TypeDef timer_init = TIMER_INIT_DEFAULT;
    TIMER_Init_Expect(TIMER0, &timer_init);

}

int print_char(int ch) {
    USART_Tx(USART1, ch);
    return 0;
}
