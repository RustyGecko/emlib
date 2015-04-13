use core::prelude::*;
use collections::vec::Vec;

const WRAP_BIT: usize = 1 << 31;

macro_rules! def_fixed_size_buffer(
    ($name:ident, $size:expr) => (

        pub struct $name<T> {
            pub tail_index: usize,
            pub head_index: usize,
            pub data: [T; $size]
        }

        impl<T: Copy> $name<T> {

            fn is_full(&self) -> bool {
                self.head() == self.tail() && (self.head_index & WRAP_BIT != self.tail_index & WRAP_BIT)
            }

            fn is_empty(&self) -> bool {
                self.head() == self.tail() && (self.head_index & WRAP_BIT == self.tail_index & WRAP_BIT)
            }

            fn head(&self) -> usize {
                self.head_index & !WRAP_BIT
            }

            fn tail(&self) -> usize {
                self.tail_index & !WRAP_BIT
            }

            fn inc_wrapping(&self, mut value: usize) -> usize {

                let prev_wrap_bit = value & WRAP_BIT;

                value = prev_wrap_bit | (value + 1) % $size;

                if value == 0 {
                    if prev_wrap_bit == 0 {
                        value |= WRAP_BIT;
                    } else {
                        value &= !WRAP_BIT;
                    }
                }

                value
            }

            fn inc_head(&mut self) {
                self.head_index = self.inc_wrapping(self.head_index);
            }

            fn inc_tail(&mut self) {
                self.tail_index = self.inc_wrapping(self.tail_index);
            }

            pub fn push(&mut self, val: T) -> Result<(),&str> {
                if self.is_full() {
                    Err("Overflow")
                } else {
                    self.data[self.tail()] = val;

                    self.inc_tail();

                    Ok(())
                }
            }

            pub fn pop(&mut self) -> Result<T, &str> {

                if self.is_empty() {
                    Err("Underflow")
                } else {
                    let val = self.data[self.head()];

                    self.inc_head();

                    Ok(val)
                }
            }

            pub fn pop_all(&mut self) -> Vec<T> {
                self.by_ref().collect::<Vec<T>>()
            }
        }

        impl<T: Copy> Iterator for $name<T> {

            type Item = T;

            fn next(&mut self) -> Option<T> {

                match self.pop() {
                    Ok(val) => Some(val),
                    Err(_) => None,
                }
            }

        }
    )
);

def_fixed_size_buffer!(CircularBuffer4, 4);
def_fixed_size_buffer!(CircularBuffer8, 8);
def_fixed_size_buffer!(CircularBuffer16, 16);
def_fixed_size_buffer!(CircularBuffer128, 128);
def_fixed_size_buffer!(CircularBuffer512, 512);

#[cfg(test)]
mod tests {

    use super::*;

    fn new() -> CircularBuffer4<u8> {
        CircularBuffer4 {
            tail_index: 0,
            head_index: 0,
            data: [0; 4]
        }
    }

    #[test]
    fn should_read_and_write_4_elements() {

        let mut buf = new();

        assert!(buf.push(1).is_ok());
        assert!(buf.push(2).is_ok());
        assert!(buf.push(3).is_ok());
        assert!(buf.push(4).is_ok());

        assert_eq!(buf.pop(), Ok(1));
        assert_eq!(buf.pop(), Ok(2));
        assert_eq!(buf.pop(), Ok(3));
        assert_eq!(buf.pop(), Ok(4));

    }

    #[test]
    fn initial_read_should_trigger_underflow() {

        let mut buf = new();

        assert_eq!(buf.pop(), Err("Underflow"));

    }

    #[test]
    fn five_consecutive_writes_should_trigger_overflow() {

        let mut buf = new();

        assert!(buf.push(1).is_ok());
        assert!(buf.push(2).is_ok());
        assert!(buf.push(3).is_ok());
        assert!(buf.push(4).is_ok());

        assert_eq!(buf.push(5), Err("Overflow"));

    }

    #[test]
    fn one_write_to_read_should_trigger_underflow() {

        let mut buf = new();

        assert!(buf.push(1).is_ok());
        assert!(buf.pop().is_ok());
        assert_eq!(buf.pop(), Err("Underflow"));
    }

}
