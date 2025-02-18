///Register `IWDG_SIDR` reader
pub struct R(crate::R<IWDG_SIDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IWDG_SIDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IWDG_SIDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IWDG_SIDR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `SID` reader - SID
pub type SID_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - SID
    #[inline(always)]
    pub fn sid(&self) -> SID_R {
        SID_R::new(self.bits)
    }
}
///IWDG size identification register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [iwdg_sidr](index.html) module
pub struct IWDG_SIDR_SPEC;
impl crate::RegisterSpec for IWDG_SIDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [iwdg_sidr::R](R) reader structure
impl crate::Readable for IWDG_SIDR_SPEC {
    type Reader = R;
}
///`reset()` method sets IWDG_SIDR to value 0xa3c5_dd01
impl crate::Resettable for IWDG_SIDR_SPEC {
    const RESET_VALUE: Self::Ux = 0xa3c5_dd01;
}
