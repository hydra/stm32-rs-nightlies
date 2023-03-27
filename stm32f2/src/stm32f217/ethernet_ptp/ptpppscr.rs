///Register `PTPPPSCR` reader
pub struct R(crate::R<PTPPPSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTPPPSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTPPPSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTPPPSCR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `PPSFREQ` reader - PPS frequency selection
pub type PPSFREQ_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:3 - PPS frequency selection
    #[inline(always)]
    pub fn ppsfreq(&self) -> PPSFREQ_R {
        PPSFREQ_R::new((self.bits & 0x0f) as u8)
    }
}
///Ethernet PTP PPS control register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ptpppscr](index.html) module
pub struct PTPPPSCR_SPEC;
impl crate::RegisterSpec for PTPPPSCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ptpppscr::R](R) reader structure
impl crate::Readable for PTPPPSCR_SPEC {
    type Reader = R;
}
///`reset()` method sets PTPPPSCR to value 0
impl crate::Resettable for PTPPPSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
