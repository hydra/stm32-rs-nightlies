///Register `VPCCR` reader
pub struct R(crate::R<VPCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VPCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VPCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VPCCR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `VPSIZE` reader - VPSIZE
pub type VPSIZE_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:13 - VPSIZE
    #[inline(always)]
    pub fn vpsize(&self) -> VPSIZE_R {
        VPSIZE_R::new((self.bits & 0x3fff) as u16)
    }
}
///DSI Host video packet current configuration register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [vpccr](index.html) module
pub struct VPCCR_SPEC;
impl crate::RegisterSpec for VPCCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [vpccr::R](R) reader structure
impl crate::Readable for VPCCR_SPEC {
    type Reader = R;
}
///`reset()` method sets VPCCR to value 0
impl crate::Resettable for VPCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
