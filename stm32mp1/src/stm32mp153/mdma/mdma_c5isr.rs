///Register `MDMA_C5ISR` reader
pub struct R(crate::R<MDMA_C5ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDMA_C5ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDMA_C5ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDMA_C5ISR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `TEIF` reader - TEIF
pub type TEIF_R = crate::BitReader<bool>;
///Field `CTCIF` reader - CTCIF
pub type CTCIF_R = crate::BitReader<bool>;
///Field `BRTIF` reader - BRTIF
pub type BRTIF_R = crate::BitReader<bool>;
///Field `BTIF` reader - BTIF
pub type BTIF_R = crate::BitReader<bool>;
///Field `TCIF` reader - TCIF
pub type TCIF_R = crate::BitReader<bool>;
///Field `CRQA` reader - CRQA
pub type CRQA_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - TEIF
    #[inline(always)]
    pub fn teif(&self) -> TEIF_R {
        TEIF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CTCIF
    #[inline(always)]
    pub fn ctcif(&self) -> CTCIF_R {
        CTCIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - BRTIF
    #[inline(always)]
    pub fn brtif(&self) -> BRTIF_R {
        BRTIF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - BTIF
    #[inline(always)]
    pub fn btif(&self) -> BTIF_R {
        BTIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TCIF
    #[inline(always)]
    pub fn tcif(&self) -> TCIF_R {
        TCIF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 16 - CRQA
    #[inline(always)]
    pub fn crqa(&self) -> CRQA_R {
        CRQA_R::new(((self.bits >> 16) & 1) != 0)
    }
}
///MDMA channel 5 interrupt/status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mdma_c5isr](index.html) module
pub struct MDMA_C5ISR_SPEC;
impl crate::RegisterSpec for MDMA_C5ISR_SPEC {
    type Ux = u32;
}
///`read()` method returns [mdma_c5isr::R](R) reader structure
impl crate::Readable for MDMA_C5ISR_SPEC {
    type Reader = R;
}
///`reset()` method sets MDMA_C5ISR to value 0
impl crate::Resettable for MDMA_C5ISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
