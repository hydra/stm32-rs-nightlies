///Register `FMC_CSQEMSR` reader
pub struct R(crate::R<FMC_CSQEMSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMC_CSQEMSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMC_CSQEMSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMC_CSQEMSR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `SEM` reader - SEM
pub type SEM_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:15 - SEM
    #[inline(always)]
    pub fn sem(&self) -> SEM_R {
        SEM_R::new((self.bits & 0xffff) as u16)
    }
}
///This register holds a sector error mapping status when the whole transfer is complete.
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fmc_csqemsr](index.html) module
pub struct FMC_CSQEMSR_SPEC;
impl crate::RegisterSpec for FMC_CSQEMSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [fmc_csqemsr::R](R) reader structure
impl crate::Readable for FMC_CSQEMSR_SPEC {
    type Reader = R;
}
///`reset()` method sets FMC_CSQEMSR to value 0
impl crate::Resettable for FMC_CSQEMSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
