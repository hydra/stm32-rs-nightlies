///Register `DCACHE_RMMONR` reader
pub struct R(crate::R<DCACHE_RMMONR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCACHE_RMMONR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCACHE_RMMONR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCACHE_RMMONR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `MRISSMON` reader - RMISSMON
pub type MRISSMON_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:15 - RMISSMON
    #[inline(always)]
    pub fn mrissmon(&self) -> MRISSMON_R {
        MRISSMON_R::new((self.bits & 0xffff) as u16)
    }
}
///DCACHE read-miss monitor register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dcache_rmmonr](index.html) module
pub struct DCACHE_RMMONR_SPEC;
impl crate::RegisterSpec for DCACHE_RMMONR_SPEC {
    type Ux = u32;
}
///`read()` method returns [dcache_rmmonr::R](R) reader structure
impl crate::Readable for DCACHE_RMMONR_SPEC {
    type Reader = R;
}
///`reset()` method sets DCACHE_RMMONR to value 0
impl crate::Resettable for DCACHE_RMMONR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
