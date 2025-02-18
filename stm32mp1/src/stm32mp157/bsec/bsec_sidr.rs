///Register `BSEC_SIDR` reader
pub struct R(crate::R<BSEC_SIDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BSEC_SIDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BSEC_SIDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BSEC_SIDR_SPEC>) -> Self {
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
///BSEC size identification register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [bsec_sidr](index.html) module
pub struct BSEC_SIDR_SPEC;
impl crate::RegisterSpec for BSEC_SIDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [bsec_sidr::R](R) reader structure
impl crate::Readable for BSEC_SIDR_SPEC {
    type Reader = R;
}
///`reset()` method sets BSEC_SIDR to value 0xa3c5_dd04
impl crate::Resettable for BSEC_SIDR_SPEC {
    const RESET_VALUE: Self::Ux = 0xa3c5_dd04;
}
