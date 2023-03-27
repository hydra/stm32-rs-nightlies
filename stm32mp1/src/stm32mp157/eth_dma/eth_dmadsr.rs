///Register `ETH_DMADSR` reader
pub struct R(crate::R<ETH_DMADSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_DMADSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_DMADSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_DMADSR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `AXWHSTS` reader - AHB Master Write Channel
pub type AXWHSTS_R = crate::BitReader<bool>;
///Field `AXRHSTS` reader - AXRHSTS
pub type AXRHSTS_R = crate::BitReader<bool>;
///Field `RPS0` reader - RPS0
pub type RPS0_R = crate::FieldReader<u8, u8>;
///Field `TPS0` reader - TPS0
pub type TPS0_R = crate::FieldReader<u8, u8>;
///Field `RPS1` reader - RPS1
pub type RPS1_R = crate::FieldReader<u8, u8>;
///Field `TPS1` reader - TPS1
pub type TPS1_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bit 0 - AHB Master Write Channel
    #[inline(always)]
    pub fn axwhsts(&self) -> AXWHSTS_R {
        AXWHSTS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - AXRHSTS
    #[inline(always)]
    pub fn axrhsts(&self) -> AXRHSTS_R {
        AXRHSTS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 8:11 - RPS0
    #[inline(always)]
    pub fn rps0(&self) -> RPS0_R {
        RPS0_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - TPS0
    #[inline(always)]
    pub fn tps0(&self) -> TPS0_R {
        TPS0_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:19 - RPS1
    #[inline(always)]
    pub fn rps1(&self) -> RPS1_R {
        RPS1_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - TPS1
    #[inline(always)]
    pub fn tps1(&self) -> TPS1_R {
        TPS1_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
///Debug status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [eth_dmadsr](index.html) module
pub struct ETH_DMADSR_SPEC;
impl crate::RegisterSpec for ETH_DMADSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [eth_dmadsr::R](R) reader structure
impl crate::Readable for ETH_DMADSR_SPEC {
    type Reader = R;
}
///`reset()` method sets ETH_DMADSR to value 0
impl crate::Resettable for ETH_DMADSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
