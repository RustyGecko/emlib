#[allow(dead_code)]

use rlibc;
use libc::{c_void, size_t, c_int};

#[no_mangle]
pub unsafe fn __aeabi_memmove(dest: *mut u8, src: *const u8,
                             n: usize) -> *mut u8 {
    rlibc::memmove(dest, src, n)
}

#[no_mangle]
pub extern fn posix_memalign(memptr: *mut *mut c_void, alignment: size_t, size: size_t) -> c_int {
    unsafe { memalign(memptr, alignment, size) }
}

extern {
    fn memalign(memptr: *mut *mut c_void, alignment: size_t, size: size_t) -> c_int;
}
