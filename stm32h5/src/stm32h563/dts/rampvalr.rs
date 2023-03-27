///Register `RAMPVALR` reader
pub struct R(crate::R<RAMPVALR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RAMPVALR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RAMPVALR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RAMPVALR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `TS1_RAMP_COEFF` reader - Engineering value of the ramp coefficient for the temperature sensor 1. This value is expressed in Hz/�C.
pub type TS1_RAMP_COEFF_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:15 - Engineering value of the ramp coefficient for the temperature sensor 1. This value is expressed in Hz/�C.
    #[inline(always)]
    pub fn ts1_ramp_coeff(&self) -> TS1_RAMP_COEFF_R {
        TS1_RAMP_COEFF_R::new((self.bits & 0xffff) as u16)
    }
}
///Temperature sensor ramp value register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rampvalr](index.html) module
pub struct RAMPVALR_SPEC;
impl crate::RegisterSpec for RAMPVALR_SPEC {
    type Ux = u32;
}
///`read()` method returns [rampvalr::R](R) reader structure
impl crate::Readable for RAMPVALR_SPEC {
    type Reader = R;
}
///`reset()` method sets RAMPVALR to value 0
impl crate::Resettable for RAMPVALR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
