///Register `GPIOE_SIDR` reader
pub struct R(crate::R<GPIOE_SIDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIOE_SIDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIOE_SIDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIOE_SIDR_SPEC>) -> Self {
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
///For information about available fields see [gpioe_sidr](index.html) module
pub struct GPIOE_SIDR_SPEC;
impl crate::RegisterSpec for GPIOE_SIDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [gpioe_sidr::R](R) reader structure
impl crate::Readable for GPIOE_SIDR_SPEC {
    type Reader = R;
}
///`reset()` method sets GPIOE_SIDR to value 0xa3c5_dd01
impl crate::Resettable for GPIOE_SIDR_SPEC {
    const RESET_VALUE: Self::Ux = 0xa3c5_dd01;
}
