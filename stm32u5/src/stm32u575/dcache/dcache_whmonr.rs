///Register `DCACHE_WHMONR` reader
pub struct R(crate::R<DCACHE_WHMONR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCACHE_WHMONR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCACHE_WHMONR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCACHE_WHMONR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `WHITMON` reader - WHITMON
pub type WHITMON_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - WHITMON
    #[inline(always)]
    pub fn whitmon(&self) -> WHITMON_R {
        WHITMON_R::new(self.bits)
    }
}
///write-hit monitor register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dcache_whmonr](index.html) module
pub struct DCACHE_WHMONR_SPEC;
impl crate::RegisterSpec for DCACHE_WHMONR_SPEC {
    type Ux = u32;
}
///`read()` method returns [dcache_whmonr::R](R) reader structure
impl crate::Readable for DCACHE_WHMONR_SPEC {
    type Reader = R;
}
///`reset()` method sets DCACHE_WHMONR to value 0
impl crate::Resettable for DCACHE_WHMONR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
