///Register `FMC_BCHDSR4` reader
pub struct R(crate::R<FMC_BCHDSR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMC_BCHDSR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMC_BCHDSR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMC_BCHDSR4_SPEC>) -> Self {
        R(reader)
    }
}
///Field `EBP7` reader - EBP7
pub type EBP7_R = crate::FieldReader<u16, u16>;
///Field `EBP8` reader - EBP8
pub type EBP8_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:12 - EBP7
    #[inline(always)]
    pub fn ebp7(&self) -> EBP7_R {
        EBP7_R::new((self.bits & 0x1fff) as u16)
    }
    ///Bits 16:28 - EBP8
    #[inline(always)]
    pub fn ebp8(&self) -> EBP8_R {
        EBP8_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
///The maximum error correction capability of the BCH block embedded in the FMC is 8 errors. This register contains the positions of the 7th and 8th error bits in EBP7 and EPB8 fields, respectively. .
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fmc_bchdsr4](index.html) module
pub struct FMC_BCHDSR4_SPEC;
impl crate::RegisterSpec for FMC_BCHDSR4_SPEC {
    type Ux = u32;
}
///`read()` method returns [fmc_bchdsr4::R](R) reader structure
impl crate::Readable for FMC_BCHDSR4_SPEC {
    type Reader = R;
}
///`reset()` method sets FMC_BCHDSR4 to value 0
impl crate::Resettable for FMC_BCHDSR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
