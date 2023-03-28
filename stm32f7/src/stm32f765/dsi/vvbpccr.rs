///Register `VVBPCCR` reader
pub struct R(crate::R<VVBPCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VVBPCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VVBPCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VVBPCCR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `VBP` reader - Vertical Back-Porch duration
pub type VBP_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:9 - Vertical Back-Porch duration
    #[inline(always)]
    pub fn vbp(&self) -> VBP_R {
        VBP_R::new((self.bits & 0x03ff) as u16)
    }
}
///DSI Host Video VBP Current Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [vvbpccr](index.html) module
pub struct VVBPCCR_SPEC;
impl crate::RegisterSpec for VVBPCCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [vvbpccr::R](R) reader structure
impl crate::Readable for VVBPCCR_SPEC {
    type Reader = R;
}
///`reset()` method sets VVBPCCR to value 0
impl crate::Resettable for VVBPCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
