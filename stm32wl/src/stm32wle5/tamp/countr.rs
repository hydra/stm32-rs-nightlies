///Register `COUNTR` reader
pub struct R(crate::R<COUNTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COUNTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COUNTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COUNTR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `COUNT` reader - COUNT
pub type COUNT_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - COUNT
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new(self.bits)
    }
}
///monotonic counter register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [countr](index.html) module
pub struct COUNTR_SPEC;
impl crate::RegisterSpec for COUNTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [countr::R](R) reader structure
impl crate::Readable for COUNTR_SPEC {
    type Reader = R;
}
///`reset()` method sets COUNTR to value 0
impl crate::Resettable for COUNTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
