///Register `SDMMC_RESPCMDR` reader
pub struct R(crate::R<SDMMC_RESPCMDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDMMC_RESPCMDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDMMC_RESPCMDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDMMC_RESPCMDR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `RESPCMD` reader - Response command index
pub type RESPCMD_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:5 - Response command index
    #[inline(always)]
    pub fn respcmd(&self) -> RESPCMD_R {
        RESPCMD_R::new((self.bits & 0x3f) as u8)
    }
}
///SDMMC command response register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sdmmc_respcmdr](index.html) module
pub struct SDMMC_RESPCMDR_SPEC;
impl crate::RegisterSpec for SDMMC_RESPCMDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [sdmmc_respcmdr::R](R) reader structure
impl crate::Readable for SDMMC_RESPCMDR_SPEC {
    type Reader = R;
}
///`reset()` method sets SDMMC_RESPCMDR to value 0xa3c5_dd01
impl crate::Resettable for SDMMC_RESPCMDR_SPEC {
    const RESET_VALUE: Self::Ux = 0xa3c5_dd01;
}
