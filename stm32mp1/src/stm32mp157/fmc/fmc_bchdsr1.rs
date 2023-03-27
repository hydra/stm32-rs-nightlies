///Register `FMC_BCHDSR1` reader
pub struct R(crate::R<FMC_BCHDSR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMC_BCHDSR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMC_BCHDSR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMC_BCHDSR1_SPEC>) -> Self {
        R(reader)
    }
}
///Field `EBP1` reader - EBP1
pub type EBP1_R = crate::FieldReader<u16, u16>;
///Field `EBP2` reader - EBP2
pub type EBP2_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:12 - EBP1
    #[inline(always)]
    pub fn ebp1(&self) -> EBP1_R {
        EBP1_R::new((self.bits & 0x1fff) as u16)
    }
    ///Bits 16:28 - EBP2
    #[inline(always)]
    pub fn ebp2(&self) -> EBP2_R {
        EBP2_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
///The maximum error correction capability of the BCH block embedded in the FMC is 8 errors
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fmc_bchdsr1](index.html) module
pub struct FMC_BCHDSR1_SPEC;
impl crate::RegisterSpec for FMC_BCHDSR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [fmc_bchdsr1::R](R) reader structure
impl crate::Readable for FMC_BCHDSR1_SPEC {
    type Reader = R;
}
///`reset()` method sets FMC_BCHDSR1 to value 0
impl crate::Resettable for FMC_BCHDSR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
