///Register `DCNTR` reader
pub struct R(crate::R<DCNTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCNTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCNTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCNTR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `DATACOUNT` reader - Data count value When read, the number of remaining data bytes to be transferred is returned. Write has no effect.
pub type DATACOUNT_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:24 - Data count value When read, the number of remaining data bytes to be transferred is returned. Write has no effect.
    #[inline(always)]
    pub fn datacount(&self) -> DATACOUNT_R {
        DATACOUNT_R::new(self.bits & 0x01ff_ffff)
    }
}
///The SDMMC_DCNTR register loads the value from the data length register (see SDMMC_DLENR) when the DPSM moves from the Idle state to the Wait_R or Wait_S state. As data is transferred, the counter decrements the value until it reaches 0. The DPSM then moves to the Idle state and when there has been no error, the data status end flag (DATAEND) is set.
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dcntr](index.html) module
pub struct DCNTR_SPEC;
impl crate::RegisterSpec for DCNTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [dcntr::R](R) reader structure
impl crate::Readable for DCNTR_SPEC {
    type Reader = R;
}
///`reset()` method sets DCNTR to value 0
impl crate::Resettable for DCNTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
