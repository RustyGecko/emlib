use i2c::I2C;

pub const ADDR_0: u8 = (0x41 << 1);
pub const ADDR_1: u8 = (0x40 << 1);

pub fn detect(i2c: &I2C, addr: u8) -> bool {
    unsafe { Si7013_Detect(i2c, addr) }
}

pub fn measure_rh_and_temp(i2c: &I2C, addr: u8, rh_data: &mut u32, t_data: &mut i32) -> i32 {
    unsafe {
        Si7013_MeasureRHAndTemp(i2c, addr, rh_data, t_data)
    }
}

extern {
    fn Si7013_Detect(i2c: &I2C, addr: u8) -> bool;
    fn Si7013_MeasureRHAndTemp(i2c: &I2C, addr: u8, rh_data: &mut u32, t_data: &mut i32) -> i32;
}
