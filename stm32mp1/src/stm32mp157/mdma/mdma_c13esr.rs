///Register `MDMA_C13ESR` reader
pub struct R(crate::R<MDMA_C13ESR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDMA_C13ESR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDMA_C13ESR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDMA_C13ESR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `TEA` reader - TEA
pub type TEA_R = crate::FieldReader<u8, u8>;
///Field `TED` reader - TED
pub type TED_R = crate::BitReader<bool>;
///Field `TELD` reader - TELD
pub type TELD_R = crate::BitReader<bool>;
///Field `TEMD` reader - TEMD
pub type TEMD_R = crate::BitReader<bool>;
///Field `ASE` reader - ASE
pub type ASE_R = crate::BitReader<bool>;
///Field `BSE` reader - BSE
pub type BSE_R = crate::BitReader<bool>;
impl R {
    ///Bits 0:6 - TEA
    #[inline(always)]
    pub fn tea(&self) -> TEA_R {
        TEA_R::new((self.bits & 0x7f) as u8)
    }
    ///Bit 7 - TED
    #[inline(always)]
    pub fn ted(&self) -> TED_R {
        TED_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - TELD
    #[inline(always)]
    pub fn teld(&self) -> TELD_R {
        TELD_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - TEMD
    #[inline(always)]
    pub fn temd(&self) -> TEMD_R {
        TEMD_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - ASE
    #[inline(always)]
    pub fn ase(&self) -> ASE_R {
        ASE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - BSE
    #[inline(always)]
    pub fn bse(&self) -> BSE_R {
        BSE_R::new(((self.bits >> 11) & 1) != 0)
    }
}
///MDMA channel 13 error status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mdma_c13esr](index.html) module
pub struct MDMA_C13ESR_SPEC;
impl crate::RegisterSpec for MDMA_C13ESR_SPEC {
    type Ux = u32;
}
///`read()` method returns [mdma_c13esr::R](R) reader structure
impl crate::Readable for MDMA_C13ESR_SPEC {
    type Reader = R;
}
///`reset()` method sets MDMA_C13ESR to value 0
impl crate::Resettable for MDMA_C13ESR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
