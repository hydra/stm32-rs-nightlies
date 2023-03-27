///Register `RHMONR` reader
pub struct R(crate::R<RHMONR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RHMONR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RHMONR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RHMONR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `RHITMON` reader - cache read-hit monitor counter
pub type RHITMON_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - cache read-hit monitor counter
    #[inline(always)]
    pub fn rhitmon(&self) -> RHITMON_R {
        RHITMON_R::new(self.bits)
    }
}
///DCACHE read-hit monitor register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rhmonr](index.html) module
pub struct RHMONR_SPEC;
impl crate::RegisterSpec for RHMONR_SPEC {
    type Ux = u32;
}
///`read()` method returns [rhmonr::R](R) reader structure
impl crate::Readable for RHMONR_SPEC {
    type Reader = R;
}
///`reset()` method sets RHMONR to value 0
impl crate::Resettable for RHMONR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
