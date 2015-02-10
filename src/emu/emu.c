#include "em_emu.h"

void STATIC_INLINE_EMU_EnterEM1(void) {
    EMU_EnterEM1();
}

#if defined( _EMU_EM4CONF_MASK )
void STATIC_INLINE_EMU_EM4Lock(bool enable) {
    EMU_EM4Lock(enable);
}

void STATIC_INLINE_EMU_BUReady(void) {
    EMU_BUReady();
}

void STATIC_INLINE_EMU_BUPinEnable(bool enable) {
    EMU_BUPinEnable(enable);
}
#endif

void STATIC_INLINE_EMU_Lock(void) {
    EMU_Lock();
}

void STATIC_INLINE_EMU_Unlock(void) {
    EMU_Unlock();
}

void STATIC_INLINE_EMU_EM2Block(void) {
    EMU_EM2Block();
}

void STATIC_INLINE_EMU_EM2UnBlock(void) {
    EMU_EM2UnBlock();
}
