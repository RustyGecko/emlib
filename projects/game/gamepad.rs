pub trait Gamepad {
    fn init(&self);
    fn get(&self) -> usize;
}
