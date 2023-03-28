///Register `GPIOA_SIDR` reader
pub struct R(crate::R<GPIOA_SIDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIOA_SIDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIOA_SIDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIOA_SIDR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `SIDR` reader - SIDR
pub type SIDR_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - SIDR
    #[inline(always)]
    pub fn sidr(&self) -> SIDR_R {
        SIDR_R::new(self.bits)
    }
}
///GPIO size identification register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [gpioa_sidr](index.html) module
pub struct GPIOA_SIDR_SPEC;
impl crate::RegisterSpec for GPIOA_SIDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [gpioa_sidr::R](R) reader structure
impl crate::Readable for GPIOA_SIDR_SPEC {
    type Reader = R;
}
///`reset()` method sets GPIOA_SIDR to value 0xa3c5_dd01
impl crate::Resettable for GPIOA_SIDR_SPEC {
    const RESET_VALUE: Self::Ux = 0xa3c5_dd01;
}
