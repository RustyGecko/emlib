use emlib;
use emlib::{cmu, adc};
use core::prelude::*;
use core::default::Default;

use sensor::Sensor;

pub struct InternalTemperature {
    adc: &'static adc::Adc
}

impl InternalTemperature {

    pub fn new(adc: &'static adc::Adc) -> InternalTemperature {
        InternalTemperature { adc: adc }
    }

    pub fn init(&self) {

        self.adc.init(&adc::Init {
            ..Default::default()
        });

        self.adc.init_single(&adc::InitSingle {
            reference: adc::Ref::Ref2V5,
            input: adc::SingleInput::Temp,
            resolution: adc::Res::Res8Bit,
            ..Default::default()
        });
    }

}

impl Sensor<u8> for InternalTemperature {

    fn measure(&mut self) -> u8 {
        self.adc.start(adc::Start::Single);

        while self.adc.STATUS & adc::STATUS_SINGLEACT != 0 {}

        self.adc.data_single_get() as u8
    }

}
