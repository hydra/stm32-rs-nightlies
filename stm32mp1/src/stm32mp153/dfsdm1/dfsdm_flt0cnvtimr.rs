///Register `DFSDM_FLT0CNVTIMR` reader
pub struct R(crate::R<DFSDM_FLT0CNVTIMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFSDM_FLT0CNVTIMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFSDM_FLT0CNVTIMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFSDM_FLT0CNVTIMR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `CNVCNT` reader - CNVCNT
pub type CNVCNT_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 4:31 - CNVCNT
    #[inline(always)]
    pub fn cnvcnt(&self) -> CNVCNT_R {
        CNVCNT_R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
///DFSDM filter 0 conversion timer register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_flt0cnvtimr](index.html) module
pub struct DFSDM_FLT0CNVTIMR_SPEC;
impl crate::RegisterSpec for DFSDM_FLT0CNVTIMR_SPEC {
    type Ux = u32;
}
///`read()` method returns [dfsdm_flt0cnvtimr::R](R) reader structure
impl crate::Readable for DFSDM_FLT0CNVTIMR_SPEC {
    type Reader = R;
}
///`reset()` method sets DFSDM_FLT0CNVTIMR to value 0
impl crate::Resettable for DFSDM_FLT0CNVTIMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
