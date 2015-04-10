use ebi::TFTInit;

pub fn direct_init(tft_init: &TFTInit) -> bool {
    unsafe { TFT_DirectInit(tft_init) }
}

extern {
    fn TFT_DirectInit(tft_init: &TFTInit) -> bool;
}
