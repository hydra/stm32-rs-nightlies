///Register `DFSDM_FLT0EXMAX` reader
pub struct R(crate::R<DFSDM_FLT0EXMAX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFSDM_FLT0EXMAX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFSDM_FLT0EXMAX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFSDM_FLT0EXMAX_SPEC>) -> Self {
        R(reader)
    }
}
///Field `EXMAXCH` reader - EXMAXCH
pub type EXMAXCH_R = crate::FieldReader<u8, u8>;
///Field `EXMAX` reader - EXMAX
pub type EXMAX_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:2 - EXMAXCH
    #[inline(always)]
    pub fn exmaxch(&self) -> EXMAXCH_R {
        EXMAXCH_R::new((self.bits & 7) as u8)
    }
    ///Bits 8:31 - EXMAX
    #[inline(always)]
    pub fn exmax(&self) -> EXMAX_R {
        EXMAX_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
///DFSDM filter 0 extremes detector maximum register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_flt0exmax](index.html) module
pub struct DFSDM_FLT0EXMAX_SPEC;
impl crate::RegisterSpec for DFSDM_FLT0EXMAX_SPEC {
    type Ux = u32;
}
///`read()` method returns [dfsdm_flt0exmax::R](R) reader structure
impl crate::Readable for DFSDM_FLT0EXMAX_SPEC {
    type Reader = R;
}
///`reset()` method sets DFSDM_FLT0EXMAX to value 0x8000_0000
impl crate::Resettable for DFSDM_FLT0EXMAX_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000_0000;
}
