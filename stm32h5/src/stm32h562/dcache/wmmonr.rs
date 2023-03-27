///Register `WMMONR` reader
pub struct R(crate::R<WMMONR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WMMONR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WMMONR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WMMONR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `WMISSMON` reader - cache write-miss monitor counter
pub type WMISSMON_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:15 - cache write-miss monitor counter
    #[inline(always)]
    pub fn wmissmon(&self) -> WMISSMON_R {
        WMISSMON_R::new((self.bits & 0xffff) as u16)
    }
}
///DCACHE write-miss monitor register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [wmmonr](index.html) module
pub struct WMMONR_SPEC;
impl crate::RegisterSpec for WMMONR_SPEC {
    type Ux = u32;
}
///`read()` method returns [wmmonr::R](R) reader structure
impl crate::Readable for WMMONR_SPEC {
    type Reader = R;
}
///`reset()` method sets WMMONR to value 0
impl crate::Resettable for WMMONR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
