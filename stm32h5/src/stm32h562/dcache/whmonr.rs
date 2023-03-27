///Register `WHMONR` reader
pub struct R(crate::R<WHMONR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WHMONR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WHMONR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WHMONR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `WHITMON` reader - cache write-hit monitor counter
pub type WHITMON_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - cache write-hit monitor counter
    #[inline(always)]
    pub fn whitmon(&self) -> WHITMON_R {
        WHITMON_R::new(self.bits)
    }
}
///DCACHE write-hit monitor register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [whmonr](index.html) module
pub struct WHMONR_SPEC;
impl crate::RegisterSpec for WHMONR_SPEC {
    type Ux = u32;
}
///`read()` method returns [whmonr::R](R) reader structure
impl crate::Readable for WHMONR_SPEC {
    type Reader = R;
}
///`reset()` method sets WHMONR to value 0
impl crate::Resettable for WHMONR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
