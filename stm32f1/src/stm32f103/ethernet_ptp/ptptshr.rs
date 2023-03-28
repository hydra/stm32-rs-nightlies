///Register `PTPTSHR` reader
pub struct R(crate::R<PTPTSHR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTPTSHR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTPTSHR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTPTSHR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `STS` reader - System time second
pub type STS_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - System time second
    #[inline(always)]
    pub fn sts(&self) -> STS_R {
        STS_R::new(self.bits)
    }
}
///Ethernet PTP time stamp high register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ptptshr](index.html) module
pub struct PTPTSHR_SPEC;
impl crate::RegisterSpec for PTPTSHR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ptptshr::R](R) reader structure
impl crate::Readable for PTPTSHR_SPEC {
    type Reader = R;
}
///`reset()` method sets PTPTSHR to value 0
impl crate::Resettable for PTPTSHR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
