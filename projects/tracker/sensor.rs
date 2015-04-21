pub trait Sensor<T> {

    fn measure(&mut self) -> T;

}
