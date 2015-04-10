use core::intrinsics::copy_nonoverlapping;
use core::mem::size_of;

pub const PAGE_SIZE: usize = 512;
pub const NUM_PAGES: usize = 3;

static mut STORE: [[u8; PAGE_SIZE]; NUM_PAGES] = [[0; PAGE_SIZE]; NUM_PAGES];

pub fn init() {}

pub fn write<T>(page: usize, buffer: &[T]) {

    if page < NUM_PAGES {
        unsafe {
            copy_nonoverlapping(
                buffer.as_ptr(),
                STORE[page].as_mut_ptr() as *mut T,
                size_of::<u8>() * PAGE_SIZE
            );
        }
    } else {
        panic!("Could not write to page {}: Out of bounds (NUM_PAGES: {})", page, NUM_PAGES)
    }

}

pub fn read<T>(page: usize, buffer: &mut[T]) {

    if page < NUM_PAGES {
        unsafe {
            copy_nonoverlapping(
                STORE[page].as_ptr() as *const T,
                buffer.as_mut_ptr(),
                size_of::<u8>() * PAGE_SIZE
            );
        }
    } else {
        panic!("Could not write to page {}: Out of bounds (NUM_PAGES: {})", page, NUM_PAGES)
    }

}
