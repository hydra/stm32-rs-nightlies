///Register `SPDIFRX_SIDR` reader
pub struct R(crate::R<SPDIFRX_SIDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPDIFRX_SIDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPDIFRX_SIDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPDIFRX_SIDR_SPEC>) -> Self {
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
///SPDIFRX size identification register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [spdifrx_sidr](index.html) module
pub struct SPDIFRX_SIDR_SPEC;
impl crate::RegisterSpec for SPDIFRX_SIDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [spdifrx_sidr::R](R) reader structure
impl crate::Readable for SPDIFRX_SIDR_SPEC {
    type Reader = R;
}
///`reset()` method sets SPDIFRX_SIDR to value 0xa3c5_dd01
impl crate::Resettable for SPDIFRX_SIDR_SPEC {
    const RESET_VALUE: Self::Ux = 0xa3c5_dd01;
}
