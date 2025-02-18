///Register `DTS_RAMPVALR` reader
pub struct R(crate::R<DTS_RAMPVALR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTS_RAMPVALR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTS_RAMPVALR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTS_RAMPVALR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `TS1_RAMP_COEFF` reader - TS1_RAMP_COEFF
pub type TS1_RAMP_COEFF_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:15 - TS1_RAMP_COEFF
    #[inline(always)]
    pub fn ts1_ramp_coeff(&self) -> TS1_RAMP_COEFF_R {
        TS1_RAMP_COEFF_R::new((self.bits & 0xffff) as u16)
    }
}
///The DTS_RAMPVALR is the ramp coefficient for the temperature sensor. The system reset value is factory trimmed.
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dts_rampvalr](index.html) module
pub struct DTS_RAMPVALR_SPEC;
impl crate::RegisterSpec for DTS_RAMPVALR_SPEC {
    type Ux = u32;
}
///`read()` method returns [dts_rampvalr::R](R) reader structure
impl crate::Readable for DTS_RAMPVALR_SPEC {
    type Reader = R;
}
///`reset()` method sets DTS_RAMPVALR to value 0
impl crate::Resettable for DTS_RAMPVALR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
