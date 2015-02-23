
static const TIMER_Init_TypeDef Empty__TIMER_Init;

static bool TIMER_Init__called = false;
static TIMER_TypeDef* TIMER_Init__timer = 0;
static TIMER_Init_TypeDef TIMER_Init__init;

void TIMER_Init(TIMER_TypeDef *timer, const TIMER_Init_TypeDef *init) {
    TIMER_Init__called = true;
    TIMER_Init__timer = timer;
    TIMER_Init__init = *init;
}

static const TIMER_InitCC_TypeDef Empty__TIMER_InitCC;

static bool TIMER_InitCC__called = false;
static TIMER_TypeDef* TIMER_InitCC__timer = 0;
static unsigned int TIMER_InitCC__ch = 0;
static TIMER_InitCC_TypeDef TIMER_InitCC__init;

void TIMER_InitCC(TIMER_TypeDef *timer, unsigned int ch, const TIMER_InitCC_TypeDef *init) {
    TIMER_InitCC__called = true;
    TIMER_InitCC__timer = timer;
    TIMER_InitCC__ch = ch;
    TIMER_InitCC__init = *init;
}

void TIMER_setup() {
    TIMER_Init__called = false;
    TIMER_Init__timer = 0;
    TIMER_Init__init = Empty__TIMER_Init;

    TIMER_InitCC__called = false;
    TIMER_InitCC__timer = 0;
    TIMER_InitCC__ch = 0;
    TIMER_InitCC__init = Empty__TIMER_InitCC;
}
