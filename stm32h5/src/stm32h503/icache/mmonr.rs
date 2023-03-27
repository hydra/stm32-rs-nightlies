///Register `MMONR` reader
pub struct R(crate::R<MMONR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMONR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMONR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMONR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `MISSMON` reader - cache miss monitor counter
pub type MISSMON_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:15 - cache miss monitor counter
    #[inline(always)]
    pub fn missmon(&self) -> MISSMON_R {
        MISSMON_R::new((self.bits & 0xffff) as u16)
    }
}
///ICACHE miss monitor register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mmonr](index.html) module
pub struct MMONR_SPEC;
impl crate::RegisterSpec for MMONR_SPEC {
    type Ux = u32;
}
///`read()` method returns [mmonr::R](R) reader structure
impl crate::Readable for MMONR_SPEC {
    type Reader = R;
}
///`reset()` method sets MMONR to value 0
impl crate::Resettable for MMONR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
