#include "em_lesense.h"

void STATIC_INLINE_LESENSE_DecoderStop(void);

uint32_t STATIC_INLINE_LESENSE_StatusGet(void);

void STATIC_INLINE_LESENSE_StatusWait(uint32_t flag);

uint32_t STATIC_INLINE_LESENSE_ChannelActiveGet(void);

uint32_t STATIC_INLINE_LESENSE_ScanResultGet(void);

uint32_t STATIC_INLINE_LESENSE_ScanResultDataGet(void);

uint32_t STATIC_INLINE_LESENSE_ScanResultDataBufferGet(uint32_t idx);

uint32_t STATIC_INLINE_LESENSE_SensorStateGet(void);

void STATIC_INLINE_LESENSE_RAMPowerDown(void);

void STATIC_INLINE_LESENSE_IntClear(uint32_t flags);

void STATIC_INLINE_LESENSE_IntEnable(uint32_t flags);

void STATIC_INLINE_LESENSE_IntDisable(uint32_t flags);

void STATIC_INLINE_LESENSE_IntSet(uint32_t flags);

uint32_t STATIC_INLINE_LESENSE_IntGet(void);

uint32_t STATIC_INLINE_LESENSE_IntGetEnabled(void);
