///Register `SR3` reader
pub struct R(crate::R<SR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR3_SPEC>) -> Self {
        R(reader)
    }
}
///Field `MDF1F` reader - illegal access flag for MDF1
pub type MDF1F_R = crate::BitReader<bool>;
///Field `CORDICF` reader - illegal access flag for CORDIC
pub type CORDICF_R = crate::BitReader<bool>;
///Field `FMACF` reader - illegal access flag for FMAC
pub type FMACF_R = crate::BitReader<bool>;
///Field `CRCF` reader - illegal access flag for CRC
pub type CRCF_R = crate::BitReader<bool>;
///Field `TSCF` reader - illegal access flag for TSC
pub type TSCF_R = crate::BitReader<bool>;
///Field `DMA2DF` reader - illegal access flag for register of DMA2D
pub type DMA2DF_R = crate::BitReader<bool>;
///Field `ICACHEF` reader - illegal access flag for ICACHE registers
pub type ICACHEF_R = crate::BitReader<bool>;
///Field `DCACHEF` reader - illegal access flag for DCACHE registers
pub type DCACHEF_R = crate::BitReader<bool>;
///Field `ADC1F` reader - illegal access flag for ADC1
pub type ADC1F_R = crate::BitReader<bool>;
///Field `DCMIF` reader - illegal access flag for DCMI
pub type DCMIF_R = crate::BitReader<bool>;
///Field `OTGFSF` reader - illegal access flag for OTG_FS
pub type OTGFSF_R = crate::BitReader<bool>;
///Field `AESF` reader - illegal access flag for AES
pub type AESF_R = crate::BitReader<bool>;
///Field `HASHF` reader - illegal access flag for HASH
pub type HASHF_R = crate::BitReader<bool>;
///Field `RNGF` reader - illegal access flag for RNG
pub type RNGF_R = crate::BitReader<bool>;
///Field `PKAF` reader - illegal access flag for PKA
pub type PKAF_R = crate::BitReader<bool>;
///Field `SAESF` reader - illegal access flag for SAES
pub type SAESF_R = crate::BitReader<bool>;
///Field `OCTOSPIMF` reader - illegal access flag for OCTOSPIM
pub type OCTOSPIMF_R = crate::BitReader<bool>;
///Field `SDMMC1F` reader - illegal access flag for SDMMC2
pub type SDMMC1F_R = crate::BitReader<bool>;
///Field `SDMMC2F` reader - illegal access flag for SDMMC1
pub type SDMMC2F_R = crate::BitReader<bool>;
///Field `FSMCF` reader - illegal access flag for FSMC registers
pub type FSMCF_R = crate::BitReader<bool>;
///Field `OCTOSPI1F` reader - illegal access flag for OCTOSPI1 registers
pub type OCTOSPI1F_R = crate::BitReader<bool>;
///Field `OCTOSPI2F` reader - illegal access flag for OCTOSPI2 registers
pub type OCTOSPI2F_R = crate::BitReader<bool>;
///Field `RAMCFGF` reader - illegal access flag for RAMCFG
pub type RAMCFGF_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - illegal access flag for MDF1
    #[inline(always)]
    pub fn mdf1f(&self) -> MDF1F_R {
        MDF1F_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - illegal access flag for CORDIC
    #[inline(always)]
    pub fn cordicf(&self) -> CORDICF_R {
        CORDICF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - illegal access flag for FMAC
    #[inline(always)]
    pub fn fmacf(&self) -> FMACF_R {
        FMACF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - illegal access flag for CRC
    #[inline(always)]
    pub fn crcf(&self) -> CRCF_R {
        CRCF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - illegal access flag for TSC
    #[inline(always)]
    pub fn tscf(&self) -> TSCF_R {
        TSCF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - illegal access flag for register of DMA2D
    #[inline(always)]
    pub fn dma2df(&self) -> DMA2DF_R {
        DMA2DF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - illegal access flag for ICACHE registers
    #[inline(always)]
    pub fn icachef(&self) -> ICACHEF_R {
        ICACHEF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - illegal access flag for DCACHE registers
    #[inline(always)]
    pub fn dcachef(&self) -> DCACHEF_R {
        DCACHEF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - illegal access flag for ADC1
    #[inline(always)]
    pub fn adc1f(&self) -> ADC1F_R {
        ADC1F_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - illegal access flag for DCMI
    #[inline(always)]
    pub fn dcmif(&self) -> DCMIF_R {
        DCMIF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - illegal access flag for OTG_FS
    #[inline(always)]
    pub fn otgfsf(&self) -> OTGFSF_R {
        OTGFSF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - illegal access flag for AES
    #[inline(always)]
    pub fn aesf(&self) -> AESF_R {
        AESF_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - illegal access flag for HASH
    #[inline(always)]
    pub fn hashf(&self) -> HASHF_R {
        HASHF_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - illegal access flag for RNG
    #[inline(always)]
    pub fn rngf(&self) -> RNGF_R {
        RNGF_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - illegal access flag for PKA
    #[inline(always)]
    pub fn pkaf(&self) -> PKAF_R {
        PKAF_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - illegal access flag for SAES
    #[inline(always)]
    pub fn saesf(&self) -> SAESF_R {
        SAESF_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - illegal access flag for OCTOSPIM
    #[inline(always)]
    pub fn octospimf(&self) -> OCTOSPIMF_R {
        OCTOSPIMF_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - illegal access flag for SDMMC2
    #[inline(always)]
    pub fn sdmmc1f(&self) -> SDMMC1F_R {
        SDMMC1F_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - illegal access flag for SDMMC1
    #[inline(always)]
    pub fn sdmmc2f(&self) -> SDMMC2F_R {
        SDMMC2F_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - illegal access flag for FSMC registers
    #[inline(always)]
    pub fn fsmcf(&self) -> FSMCF_R {
        FSMCF_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - illegal access flag for OCTOSPI1 registers
    #[inline(always)]
    pub fn octospi1f(&self) -> OCTOSPI1F_R {
        OCTOSPI1F_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - illegal access flag for OCTOSPI2 registers
    #[inline(always)]
    pub fn octospi2f(&self) -> OCTOSPI2F_R {
        OCTOSPI2F_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - illegal access flag for RAMCFG
    #[inline(always)]
    pub fn ramcfgf(&self) -> RAMCFGF_R {
        RAMCFGF_R::new(((self.bits >> 22) & 1) != 0)
    }
}
///TZIC status register 3
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sr3](index.html) module
pub struct SR3_SPEC;
impl crate::RegisterSpec for SR3_SPEC {
    type Ux = u32;
}
///`read()` method returns [sr3::R](R) reader structure
impl crate::Readable for SR3_SPEC {
    type Reader = R;
}
///`reset()` method sets SR3 to value 0
impl crate::Resettable for SR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
