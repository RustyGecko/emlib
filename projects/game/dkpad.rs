use emlib::dk::{bc, bsp};

use game::gamepad::Gamepad;

pub struct DKPad;

impl DKPad {
    pub fn new() -> DKPad { DKPad }
}

impl Gamepad for DKPad {

    fn init(&self) {}

    fn get(&self) -> usize {
        (bsp::push_buttons_get() << 4 | bsp::joystick_get()) as usize
    }

}
