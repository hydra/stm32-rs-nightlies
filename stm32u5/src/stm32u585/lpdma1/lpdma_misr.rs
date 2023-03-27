///Register `LPDMA_MISR` reader
pub struct R(crate::R<LPDMA_MISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPDMA_MISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPDMA_MISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPDMA_MISR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `MIS0` reader - MIS0
pub type MIS0_R = crate::BitReader<bool>;
///Field `MIS1` reader - MIS1
pub type MIS1_R = crate::BitReader<bool>;
///Field `MIS2` reader - MIS2
pub type MIS2_R = crate::BitReader<bool>;
///Field `MIS3` reader - MIS3
pub type MIS3_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - MIS0
    #[inline(always)]
    pub fn mis0(&self) -> MIS0_R {
        MIS0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - MIS1
    #[inline(always)]
    pub fn mis1(&self) -> MIS1_R {
        MIS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - MIS2
    #[inline(always)]
    pub fn mis2(&self) -> MIS2_R {
        MIS2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - MIS3
    #[inline(always)]
    pub fn mis3(&self) -> MIS3_R {
        MIS3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
///LPDMA non-secure masked interrupt status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [lpdma_misr](index.html) module
pub struct LPDMA_MISR_SPEC;
impl crate::RegisterSpec for LPDMA_MISR_SPEC {
    type Ux = u32;
}
///`read()` method returns [lpdma_misr::R](R) reader structure
impl crate::Readable for LPDMA_MISR_SPEC {
    type Reader = R;
}
///`reset()` method sets LPDMA_MISR to value 0
impl crate::Resettable for LPDMA_MISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
