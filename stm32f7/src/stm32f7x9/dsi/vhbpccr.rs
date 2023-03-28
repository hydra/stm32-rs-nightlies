///Register `VHBPCCR` reader
pub struct R(crate::R<VHBPCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VHBPCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VHBPCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VHBPCCR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `HBP` reader - Horizontal Back-Porch duration
pub type HBP_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:11 - Horizontal Back-Porch duration
    #[inline(always)]
    pub fn hbp(&self) -> HBP_R {
        HBP_R::new((self.bits & 0x0fff) as u16)
    }
}
///DSI Host Video HBP Current Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [vhbpccr](index.html) module
pub struct VHBPCCR_SPEC;
impl crate::RegisterSpec for VHBPCCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [vhbpccr::R](R) reader structure
impl crate::Readable for VHBPCCR_SPEC {
    type Reader = R;
}
///`reset()` method sets VHBPCCR to value 0
impl crate::Resettable for VHBPCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
