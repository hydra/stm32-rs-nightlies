///Register `DMADSR` reader
pub struct R(crate::R<DMADSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMADSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMADSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMADSR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `AXWHSTS` reader - AHB Master Write Channel
pub type AXWHSTS_R = crate::BitReader<bool>;
///Field `RPS0` reader - DMA Channel Receive Process State
pub type RPS0_R = crate::FieldReader<u8, u8>;
///Field `TPS0` reader - DMA Channel Transmit Process State
pub type TPS0_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bit 0 - AHB Master Write Channel
    #[inline(always)]
    pub fn axwhsts(&self) -> AXWHSTS_R {
        AXWHSTS_R::new((self.bits & 1) != 0)
    }
    ///Bits 8:11 - DMA Channel Receive Process State
    #[inline(always)]
    pub fn rps0(&self) -> RPS0_R {
        RPS0_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - DMA Channel Transmit Process State
    #[inline(always)]
    pub fn tps0(&self) -> TPS0_R {
        TPS0_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
///Debug status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dmadsr](index.html) module
pub struct DMADSR_SPEC;
impl crate::RegisterSpec for DMADSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [dmadsr::R](R) reader structure
impl crate::Readable for DMADSR_SPEC {
    type Reader = R;
}
///`reset()` method sets DMADSR to value 0
impl crate::Resettable for DMADSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
