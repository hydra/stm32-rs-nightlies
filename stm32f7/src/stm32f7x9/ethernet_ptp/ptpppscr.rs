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
///Field `TSSO` reader - TSSO
pub type TSSO_R = crate::BitReader<bool>;
///Field `TSTTR` reader - TSTTR
pub type TSTTR_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - TSSO
    #[inline(always)]
    pub fn tsso(&self) -> TSSO_R {
        TSSO_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TSTTR
    #[inline(always)]
    pub fn tsttr(&self) -> TSTTR_R {
        TSTTR_R::new(((self.bits >> 1) & 1) != 0)
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
