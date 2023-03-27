///Register `VVFPCCR` reader
pub struct R(crate::R<VVFPCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VVFPCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VVFPCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VVFPCCR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `VFP` reader - VFP
pub type VFP_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:9 - VFP
    #[inline(always)]
    pub fn vfp(&self) -> VFP_R {
        VFP_R::new((self.bits & 0x03ff) as u16)
    }
}
///DSI Host video VFP current configuration register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [vvfpccr](index.html) module
pub struct VVFPCCR_SPEC;
impl crate::RegisterSpec for VVFPCCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [vvfpccr::R](R) reader structure
impl crate::Readable for VVFPCCR_SPEC {
    type Reader = R;
}
///`reset()` method sets VVFPCCR to value 0
impl crate::Resettable for VVFPCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
