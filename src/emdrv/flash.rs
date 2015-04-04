use core::slice::SliceExt;
use core::intrinsics::transmute;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Info {
    pub base_address: u32,
    pub manufacturer_code: u8,
    pub device_code: u8,
    pub device_size: u32,
    pub page_size: u32,
    pub spare_size: u32,
    pub block_size: u32,
    pub ecc: u32,
    pub spare: [u8; 16],
    pub dma_ch: i32,
}


pub fn init(dma_channel: i32) -> i32 {
    unsafe { NANDFLASH_Init(dma_channel) }
}

pub fn device_info() -> &'static Info {
    unsafe { transmute(NANDFLASH_DeviceInfo()) }
}

pub fn write(address: u32, buffer: &[u8]) -> i32 {
    unsafe { NANDFLASH_WritePage(address, transmute(buffer.as_ptr())) }
}

pub fn read(address: u32, buffer: &'static mut[u8]) -> i32 {
    unsafe { NANDFLASH_ReadPage(address, transmute(buffer.as_ptr())) }
}

extern {

    fn NANDFLASH_Init(dma_channel: i32) -> i32;
    fn NANDFLASH_DeviceInfo() -> *mut Info;
    fn NANDFLASH_WritePage(address: u32, buffer: *mut u8) -> i32;
    fn NANDFLASH_ReadPage(address: u32, buffer: *mut u8) -> i32;
}
