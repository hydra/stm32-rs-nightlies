///Register `MMCRFAECR` reader
pub struct R(crate::R<MMCRFAECR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMCRFAECR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMCRFAECR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMCRFAECR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `RFAEC` reader - RFAEC
pub type RFAEC_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - RFAEC
    #[inline(always)]
    pub fn rfaec(&self) -> RFAEC_R {
        RFAEC_R::new(self.bits)
    }
}
///Ethernet MMC received frames with alignment error counter register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mmcrfaecr](index.html) module
pub struct MMCRFAECR_SPEC;
impl crate::RegisterSpec for MMCRFAECR_SPEC {
    type Ux = u32;
}
///`read()` method returns [mmcrfaecr::R](R) reader structure
impl crate::Readable for MMCRFAECR_SPEC {
    type Reader = R;
}
///`reset()` method sets MMCRFAECR to value 0
impl crate::Resettable for MMCRFAECR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
