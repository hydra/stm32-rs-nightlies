///Register `HMONR` reader
pub struct R(crate::R<HMONR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HMONR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HMONR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HMONR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `HITMON` reader - HITMON
pub type HITMON_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - HITMON
    #[inline(always)]
    pub fn hitmon(&self) -> HITMON_R {
        HITMON_R::new(self.bits)
    }
}
///ICACHE hit monitor register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hmonr](index.html) module
pub struct HMONR_SPEC;
impl crate::RegisterSpec for HMONR_SPEC {
    type Ux = u32;
}
///`read()` method returns [hmonr::R](R) reader structure
impl crate::Readable for HMONR_SPEC {
    type Reader = R;
}
///`reset()` method sets HMONR to value 0
impl crate::Resettable for HMONR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
