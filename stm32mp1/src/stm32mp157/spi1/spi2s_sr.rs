///Register `SPI2S_SR` reader
pub struct R(crate::R<SPI2S_SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI2S_SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI2S_SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI2S_SR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `RXP` reader - RXP
pub type RXP_R = crate::BitReader<bool>;
///Field `TXP` reader - TXP
pub type TXP_R = crate::BitReader<bool>;
///Field `DXP` reader - DXP
pub type DXP_R = crate::BitReader<bool>;
///Field `EOT` reader - EOT
pub type EOT_R = crate::BitReader<bool>;
///Field `TXTF` reader - TXTF
pub type TXTF_R = crate::BitReader<bool>;
///Field `UDR` reader - UDR
pub type UDR_R = crate::BitReader<bool>;
///Field `OVR` reader - OVR
pub type OVR_R = crate::BitReader<bool>;
///Field `CRCE` reader - CRCE
pub type CRCE_R = crate::BitReader<bool>;
///Field `TIFRE` reader - TIFRE
pub type TIFRE_R = crate::BitReader<bool>;
///Field `MODF` reader - MODF
pub type MODF_R = crate::BitReader<bool>;
///Field `TSERF` reader - TSERF
pub type TSERF_R = crate::BitReader<bool>;
///Field `SUSP` reader - SUSP
pub type SUSP_R = crate::BitReader<bool>;
///Field `TXC` reader - TXC
pub type TXC_R = crate::BitReader<bool>;
///Field `RXPLVL` reader - RXPLVL
pub type RXPLVL_R = crate::FieldReader<u8, u8>;
///Field `RXWNE` reader - RXWNE
pub type RXWNE_R = crate::BitReader<bool>;
///Field `CTSIZE` reader - CTSIZE
pub type CTSIZE_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bit 0 - RXP
    #[inline(always)]
    pub fn rxp(&self) -> RXP_R {
        RXP_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TXP
    #[inline(always)]
    pub fn txp(&self) -> TXP_R {
        TXP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - DXP
    #[inline(always)]
    pub fn dxp(&self) -> DXP_R {
        DXP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - EOT
    #[inline(always)]
    pub fn eot(&self) -> EOT_R {
        EOT_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TXTF
    #[inline(always)]
    pub fn txtf(&self) -> TXTF_R {
        TXTF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - UDR
    #[inline(always)]
    pub fn udr(&self) -> UDR_R {
        UDR_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - OVR
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - CRCE
    #[inline(always)]
    pub fn crce(&self) -> CRCE_R {
        CRCE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - TIFRE
    #[inline(always)]
    pub fn tifre(&self) -> TIFRE_R {
        TIFRE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - MODF
    #[inline(always)]
    pub fn modf(&self) -> MODF_R {
        MODF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - TSERF
    #[inline(always)]
    pub fn tserf(&self) -> TSERF_R {
        TSERF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - SUSP
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - TXC
    #[inline(always)]
    pub fn txc(&self) -> TXC_R {
        TXC_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 13:14 - RXPLVL
    #[inline(always)]
    pub fn rxplvl(&self) -> RXPLVL_R {
        RXPLVL_R::new(((self.bits >> 13) & 3) as u8)
    }
    ///Bit 15 - RXWNE
    #[inline(always)]
    pub fn rxwne(&self) -> RXWNE_R {
        RXWNE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:31 - CTSIZE
    #[inline(always)]
    pub fn ctsize(&self) -> CTSIZE_R {
        CTSIZE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
///SPI/I2S status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [spi2s_sr](index.html) module
pub struct SPI2S_SR_SPEC;
impl crate::RegisterSpec for SPI2S_SR_SPEC {
    type Ux = u32;
}
///`read()` method returns [spi2s_sr::R](R) reader structure
impl crate::Readable for SPI2S_SR_SPEC {
    type Reader = R;
}
///`reset()` method sets SPI2S_SR to value 0x1002
impl crate::Resettable for SPI2S_SR_SPEC {
    const RESET_VALUE: Self::Ux = 0x1002;
}
