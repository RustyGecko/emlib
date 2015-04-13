#![allow(unused_attributes)]
#![feature(collections, core)]

use collections::vec::Vec;
use core::ops;

#[cfg(test)] extern crate collections;
#[cfg(test)] extern crate core;

pub struct FixedSizeVector<T> {
    vec: Vec<T>
}

impl<T> FixedSizeVector<T> {

    pub fn new(size: usize) -> FixedSizeVector<T> {
        FixedSizeVector {
            vec: Vec::with_capacity(size)
        }
    }

    pub fn push(&mut self, value: T) {

        if !self.is_full() {
            self.vec.push(value);
        }

    }

    #[inline]
    pub fn is_full(&self) -> bool {
        self.len() >= self.capacity()
    }

    #[inline]
    pub fn reset(&mut self) {
        unsafe { self.vec.set_len(0) };
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.vec.len()
    }

    #[inline]
    pub fn capacity(&self) -> usize {
        self.vec.capacity()
    }

}

impl<T> ops::Index<usize> for FixedSizeVector<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &T {
        self.vec.index(index)
    }

}

impl <T> ops::Index<ops::RangeFull> for FixedSizeVector<T> {
    type Output = [T];

    #[inline]
    fn index(&self, index: ops::RangeFull) -> &[T] {
        self.vec.index(index)
    }

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_be_pushable() {
        let mut vec = FixedSizeVector::new(4);

        vec.push(1);
        vec.push(2);
        vec.push(3);
        vec.push(4);

        assert_eq!(&vec[..], &[1,2,3,4]);
    }

    #[test]
    fn should_ignore_push_over_capacity() {

        let mut vec = FixedSizeVector::new(2);

        vec.push(1);
        vec.push(2);
        vec.push(3);

        assert_eq!(&vec[..], &[1,2]);
    }

    #[test]
    fn reset_should_start_with_zero_elements() {

        let mut vec = FixedSizeVector::new(2);

        vec.push(1);
        vec.push(2);
        vec.reset();
        vec.push(3);

        assert_eq!(&vec[..], &[3]);
    }

}
