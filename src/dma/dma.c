#include "em_dma.h"
#include "dmactrl.h"

DMA_DESCRIPTOR_TypeDef* GET_DMA_CONTROL_BLOCK() {
    return dmaControlBlock;
}
