#include <stdbool.h>
#include <em_timer.h>

#include "timer-mock.c"

bool check_test_called() {
    return TIMER_Init__called;
}

bool check_test_called_with_timer0() {
    return TIMER_Init__timer == TIMER0;
}

bool check_test_called_with_default() {
    TIMER_Init_TypeDef defaults = TIMER_INIT_DEFAULT;

    return TIMER_Init__init.enable == defaults.enable
        && TIMER_Init__init.debugRun == defaults.debugRun
        && TIMER_Init__init.prescale == defaults.prescale
        && TIMER_Init__init.clkSel == defaults.clkSel
        && TIMER_Init__init.count2x == defaults.count2x
        && TIMER_Init__init.ati == defaults.ati
        && TIMER_Init__init.fallAction == defaults.fallAction
        && TIMER_Init__init.riseAction == defaults.riseAction
        && TIMER_Init__init.mode == defaults.mode
        && TIMER_Init__init.dmaClrAct == defaults.dmaClrAct
        && TIMER_Init__init.quadModeX4 == defaults.quadModeX4
        && TIMER_Init__init.oneShot == defaults.oneShot
        && TIMER_Init__init.sync == defaults.sync;
}
