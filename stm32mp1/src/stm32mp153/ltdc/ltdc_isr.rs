///Register `LTDC_ISR` reader
pub struct R(crate::R<LTDC_ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LTDC_ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LTDC_ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LTDC_ISR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `LIF` reader - LIF
pub type LIF_R = crate::BitReader<bool>;
///Field `FUIF` reader - FUIF
pub type FUIF_R = crate::BitReader<bool>;
///Field `TERRIF` reader - TERRIF
pub type TERRIF_R = crate::BitReader<bool>;
///Field `RRIF` reader - RRIF
pub type RRIF_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - LIF
    #[inline(always)]
    pub fn lif(&self) -> LIF_R {
        LIF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - FUIF
    #[inline(always)]
    pub fn fuif(&self) -> FUIF_R {
        FUIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TERRIF
    #[inline(always)]
    pub fn terrif(&self) -> TERRIF_R {
        TERRIF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - RRIF
    #[inline(always)]
    pub fn rrif(&self) -> RRIF_R {
        RRIF_R::new(((self.bits >> 3) & 1) != 0)
    }
}
///This register returns the interrupt status flag.
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ltdc_isr](index.html) module
pub struct LTDC_ISR_SPEC;
impl crate::RegisterSpec for LTDC_ISR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ltdc_isr::R](R) reader structure
impl crate::Readable for LTDC_ISR_SPEC {
    type Reader = R;
}
///`reset()` method sets LTDC_ISR to value 0
impl crate::Resettable for LTDC_ISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
