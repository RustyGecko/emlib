use core::intrinsics::copy_nonoverlapping;
use core::mem::size_of;

const N: usize = 512;
const M: usize = 8;

static mut INTERNAL_TEMPERATURE_STORE: [[u8; N]; M] = [[0; N]; M];
static mut TEMPERATURE_STORE:          [[u8; N]; M] = [[0; N]; M];
static mut HUMIDITY_RELATIVE_STORE:    [[u8; N]; M] = [[0; N]; M];
static mut INDEX: usize = 0;

pub fn init() {}

pub enum Kind {
    InternalTemperature,
    Temperature,
    HumidityRelative
}

unsafe fn get_store(kind: Kind) -> &'static mut [[u8; N]] {
    match kind {
        Kind::InternalTemperature => &mut INTERNAL_TEMPERATURE_STORE[..],
        Kind::Temperature => &mut TEMPERATURE_STORE[..],
        Kind::HumidityRelative => &mut HUMIDITY_RELATIVE_STORE[..],
    }
}

pub fn write<T>(kind: Kind, buffer: &[T]) {

    unsafe {
        let store = get_store(kind);

        copy_nonoverlapping(buffer.as_ptr(), store[INDEX].as_mut_ptr() as *mut T, size_of::<u8>() * N);
        INDEX = (INDEX + 1) % M;
    }

}

pub fn read<T>(kind: Kind, page: usize, buffer: &mut[T]) {



    unsafe {
        let store = get_store(kind);

        copy_nonoverlapping(
            store[page % M].as_ptr() as *const T,
            buffer.as_mut_ptr(),
            size_of::<u8>() * N
        );
    }

}
