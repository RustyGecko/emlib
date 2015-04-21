use i2c;

pub fn init(init: &i2c::Init) {
    unsafe { I2C1DRV_Init(init) }
}

pub fn transfer(seq: &mut i2c::TransferSeq) -> i2c::TransferReturn {
    unsafe { I2C1DRV_Transfer(seq) }

}

extern {
    fn I2C1DRV_Init(init: &i2c::Init);
    fn I2C1DRV_Transfer(seq: &mut i2c::TransferSeq) -> i2c::TransferReturn;
}
