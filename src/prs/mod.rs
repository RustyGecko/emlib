pub const CH_CTRL_SOURCESEL_TIMER0: u32 = 0x1c << 16;
pub const CH_CTRL_SIGSEL_TIMER0OF: u32 = 0x1 << 0;

pub fn source_signal_set(ch: u32, source: u32, signal: u32, edge: Edge) {
    unsafe { PRS_SourceSignalSet(ch, source, signal, edge); }
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum Edge {
    Off  = 0x0 << 24,
    Pos  = 0x1 << 24,
    Neg  = 0x2 << 24,
    Both = 0x3 << 24
}

extern {
    fn PRS_SourceSignalSet(ch: u32, source: u32, signal: u32, edge: Edge);
}
