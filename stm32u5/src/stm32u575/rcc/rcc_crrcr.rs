///Register `RCC_CRRCR` reader
pub struct R(crate::R<RCC_CRRCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_CRRCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_CRRCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_CRRCR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `HSI48CAL` reader - HSI48 clock calibration These bits are initialized at startup with the factory-programmed HSI48 calibration trim value.
pub type HSI48CAL_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:8 - HSI48 clock calibration These bits are initialized at startup with the factory-programmed HSI48 calibration trim value.
    #[inline(always)]
    pub fn hsi48cal(&self) -> HSI48CAL_R {
        HSI48CAL_R::new((self.bits & 0x01ff) as u16)
    }
}
///RCC clock recovery RC register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rcc_crrcr](index.html) module
pub struct RCC_CRRCR_SPEC;
impl crate::RegisterSpec for RCC_CRRCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [rcc_crrcr::R](R) reader structure
impl crate::Readable for RCC_CRRCR_SPEC {
    type Reader = R;
}
///`reset()` method sets RCC_CRRCR to value 0
impl crate::Resettable for RCC_CRRCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
