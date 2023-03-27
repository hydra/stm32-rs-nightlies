///Register `VVSACCR` reader
pub struct R(crate::R<VVSACCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VVSACCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VVSACCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VVSACCR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `VSA` reader - VSA
pub type VSA_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:9 - VSA
    #[inline(always)]
    pub fn vsa(&self) -> VSA_R {
        VSA_R::new((self.bits & 0x03ff) as u16)
    }
}
///DSI Host video VSA current configuration register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [vvsaccr](index.html) module
pub struct VVSACCR_SPEC;
impl crate::RegisterSpec for VVSACCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [vvsaccr::R](R) reader structure
impl crate::Readable for VVSACCR_SPEC {
    type Reader = R;
}
///`reset()` method sets VVSACCR to value 0
impl crate::Resettable for VVSACCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
