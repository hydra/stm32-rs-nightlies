///Register `M3FECR` reader
pub struct R(crate::R<M3FECR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<M3FECR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<M3FECR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<M3FECR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `FEC` reader - Failing error code
pub type FEC_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - Failing error code
    #[inline(always)]
    pub fn fec(&self) -> FEC_R {
        FEC_R::new(self.bits)
    }
}
///RAMECC monitor x failing ECC error code register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [m3fecr](index.html) module
pub struct M3FECR_SPEC;
impl crate::RegisterSpec for M3FECR_SPEC {
    type Ux = u32;
}
///`read()` method returns [m3fecr::R](R) reader structure
impl crate::Readable for M3FECR_SPEC {
    type Reader = R;
}
///`reset()` method sets M3FECR to value 0
impl crate::Resettable for M3FECR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
