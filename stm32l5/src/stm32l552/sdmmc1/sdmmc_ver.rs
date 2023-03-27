///Register `SDMMC_VER` reader
pub struct R(crate::R<SDMMC_VER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDMMC_VER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDMMC_VER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDMMC_VER_SPEC>) -> Self {
        R(reader)
    }
}
///Field `MINREV` reader - IP minor revision number.
pub type MINREV_R = crate::FieldReader<u8, u8>;
///Field `MAJREV` reader - IP major revision number.
pub type MAJREV_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:3 - IP minor revision number.
    #[inline(always)]
    pub fn minrev(&self) -> MINREV_R {
        MINREV_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - IP major revision number.
    #[inline(always)]
    pub fn majrev(&self) -> MAJREV_R {
        MAJREV_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
///SDMMC IP version register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sdmmc_ver](index.html) module
pub struct SDMMC_VER_SPEC;
impl crate::RegisterSpec for SDMMC_VER_SPEC {
    type Ux = u32;
}
///`read()` method returns [sdmmc_ver::R](R) reader structure
impl crate::Readable for SDMMC_VER_SPEC {
    type Reader = R;
}
///`reset()` method sets SDMMC_VER to value 0x10
impl crate::Resettable for SDMMC_VER_SPEC {
    const RESET_VALUE: Self::Ux = 0x10;
}
