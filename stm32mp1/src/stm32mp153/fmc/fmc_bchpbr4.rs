///Register `FMC_BCHPBR4` reader
pub struct R(crate::R<FMC_BCHPBR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMC_BCHPBR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMC_BCHPBR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMC_BCHPBR4_SPEC>) -> Self {
        R(reader)
    }
}
///Field `BCHPB` reader - BCHPB
pub type BCHPB_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:7 - BCHPB
    #[inline(always)]
    pub fn bchpb(&self) -> BCHPB_R {
        BCHPB_R::new((self.bits & 0xff) as u8)
    }
}
///FMC BCH Parity Bits Register 4
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fmc_bchpbr4](index.html) module
pub struct FMC_BCHPBR4_SPEC;
impl crate::RegisterSpec for FMC_BCHPBR4_SPEC {
    type Ux = u32;
}
///`read()` method returns [fmc_bchpbr4::R](R) reader structure
impl crate::Readable for FMC_BCHPBR4_SPEC {
    type Reader = R;
}
///`reset()` method sets FMC_BCHPBR4 to value 0
impl crate::Resettable for FMC_BCHPBR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
