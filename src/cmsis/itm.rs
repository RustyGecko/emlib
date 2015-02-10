extern {
    pub fn STATIC_INLINE_ITM_SendChar(ch: u32) -> u32;
    pub fn STATIC_INLINE_ITM_ReceiveChar() -> i32;
    pub fn STATIC_INLINE_ITM_CheckChar() -> i32;
}

pub fn send_char(ch: u32) -> u32 {
    unsafe { STATIC_INLINE_ITM_SendChar(ch) }
}

pub fn receive_char() -> i32 {
    unsafe { STATIC_INLINE_ITM_ReceiveChar() }
}

pub fn check_char() -> i32 {
    unsafe { STATIC_INLINE_ITM_CheckChar() }
}
