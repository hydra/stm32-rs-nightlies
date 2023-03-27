///Register `RMMONR` reader
pub struct R(crate::R<RMMONR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RMMONR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RMMONR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RMMONR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `RMISSMON` reader - cache read-miss monitor counter
pub type RMISSMON_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:15 - cache read-miss monitor counter
    #[inline(always)]
    pub fn rmissmon(&self) -> RMISSMON_R {
        RMISSMON_R::new((self.bits & 0xffff) as u16)
    }
}
///DCACHE read-miss monitor register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rmmonr](index.html) module
pub struct RMMONR_SPEC;
impl crate::RegisterSpec for RMMONR_SPEC {
    type Ux = u32;
}
///`read()` method returns [rmmonr::R](R) reader structure
impl crate::Readable for RMMONR_SPEC {
    type Reader = R;
}
///`reset()` method sets RMMONR to value 0
impl crate::Resettable for RMMONR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
