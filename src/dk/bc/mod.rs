#![allow(non_snake_case)]
#![allow(dead_code)]

use core::intrinsics::transmute;

// AEM button state
// BC controls buttons
pub const UIF_AEM_BC: u16 = 0;
// EFM32 controls buttons
pub const UIF_AEM_EFM: u16 = 1;

// DK3750board controller register definition
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BC {
    pub RESERVERD0:        u16,
    pub EM:                u16,
    pub MAGIC:             u16,

    pub UIF_LEDS:          u16,
    pub UIF_PB:            u16,
    pub UIF_DIP:           u16,
    pub UIF_JOYSTICK:      u16,
    pub UIF_AEM:           u16,
    pub UIF_CTRL:          u16,
    pub DISPLAY_CTRL:      u16,
    pub EBI_CTRL:          u16,
    pub ARB_CTRL:          u16,
    pub PERICON:           u16,
    pub SPI_DEMUX:         u16,
    pub RESERVERD1:        u16,

    pub ADC_WRITE:         u16,
    pub ADC_STATUS:        u16,
    pub ADC_READ:          u16,

    pub CLKRST:            u16,

    pub HW_VERSION:        u16,
    pub FW_BUILDNO:        u16,
    pub FW_VERSION:        u16,

    pub SCRATCH_COMMON:    u16,

    pub SCRATCH_EFM0:      u16,
    pub SCRATCH_EFM1:      u16,
    pub SCRATCH_EFM2:      u16,
    pub SCRATCH_EFM3:      u16,

    pub SCRATCH_BC0:       u16,
    pub SCRATCH_BC1:       u16,
    pub SCRATCH_BC2:       u16,
    pub SCRATCH_BC3:       u16,

    pub INTFLAG:           u16,
    pub INTEN:             u16,
    pub INTCLEAR:          u16,
    pub INTSET:            u16,
    pub INTPCTRL:          u16,
    pub INTPLOW:           u16,
    pub INTPHIGH:          u16,

    pub RESERVERD3:        u16,

    pub BC_MBOX_TXCTRL:    u16,
    pub BC_MBOX_TXDATA:    u16,
    pub BC_MBOX_TXSTATUS0: u16,
    pub BC_MBOX_TXSTATUS1: u16,

    pub RESERVED4:         u16,

    pub MBOX_TXCTRL:       u16,
    pub MBOX_TXDATA:       u16,
    pub MBOX_TXSTATUS0:    u16,
    pub MBOX_TXSTATUS1:    u16,

    pub RESERVED5:         u16,

    pub BUF_CTRL:          u16,
}

impl BC {
    pub fn bc_register() -> &'static mut BC {
        unsafe { transmute(GET_BC_REGISTER()) }
    }
}

extern {
    #[allow(dead_code)] #[inline] fn GET_BC_REGISTER() -> *mut BC;
}