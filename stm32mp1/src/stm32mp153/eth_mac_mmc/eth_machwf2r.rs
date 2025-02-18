///Register `ETH_MACHWF2R` reader
pub struct R(crate::R<ETH_MACHWF2R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACHWF2R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACHWF2R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACHWF2R_SPEC>) -> Self {
        R(reader)
    }
}
///Field `RXQCNT` reader - RXQCNT
pub type RXQCNT_R = crate::FieldReader<u8, u8>;
///Field `TXQCNT` reader - TXQCNT
pub type TXQCNT_R = crate::FieldReader<u8, u8>;
///Field `RXCHCNT` reader - RXCHCNT
pub type RXCHCNT_R = crate::FieldReader<u8, u8>;
///Field `TXCHCNT` reader - TXCHCNT
pub type TXCHCNT_R = crate::FieldReader<u8, u8>;
///Field `PPSOUTNUM` reader - PPSOUTNUM
pub type PPSOUTNUM_R = crate::FieldReader<u8, u8>;
///Field `AUXSNAPNUM` reader - AUXSNAPNUM
pub type AUXSNAPNUM_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:3 - RXQCNT
    #[inline(always)]
    pub fn rxqcnt(&self) -> RXQCNT_R {
        RXQCNT_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 6:9 - TXQCNT
    #[inline(always)]
    pub fn txqcnt(&self) -> TXQCNT_R {
        TXQCNT_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    ///Bits 12:15 - RXCHCNT
    #[inline(always)]
    pub fn rxchcnt(&self) -> RXCHCNT_R {
        RXCHCNT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 18:21 - TXCHCNT
    #[inline(always)]
    pub fn txchcnt(&self) -> TXCHCNT_R {
        TXCHCNT_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    ///Bits 24:26 - PPSOUTNUM
    #[inline(always)]
    pub fn ppsoutnum(&self) -> PPSOUTNUM_R {
        PPSOUTNUM_R::new(((self.bits >> 24) & 7) as u8)
    }
    ///Bits 28:30 - AUXSNAPNUM
    #[inline(always)]
    pub fn auxsnapnum(&self) -> AUXSNAPNUM_R {
        AUXSNAPNUM_R::new(((self.bits >> 28) & 7) as u8)
    }
}
///This register indicates the presence of third set of the optional features or functions of the Ethernet peripheral. The software driver can use this register to dynamically enable or disable the programs related to the optional blocks.
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [eth_machwf2r](index.html) module
pub struct ETH_MACHWF2R_SPEC;
impl crate::RegisterSpec for ETH_MACHWF2R_SPEC {
    type Ux = u32;
}
///`read()` method returns [eth_machwf2r::R](R) reader structure
impl crate::Readable for ETH_MACHWF2R_SPEC {
    type Reader = R;
}
///`reset()` method sets ETH_MACHWF2R to value 0x4104_0041
impl crate::Resettable for ETH_MACHWF2R_SPEC {
    const RESET_VALUE: Self::Ux = 0x4104_0041;
}
