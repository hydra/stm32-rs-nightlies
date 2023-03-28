///Register `GTZC1_TZIC_SR3` reader
pub struct R(crate::R<GTZC1_TZIC_SR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTZC1_TZIC_SR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTZC1_TZIC_SR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTZC1_TZIC_SR3_SPEC>) -> Self {
        R(reader)
    }
}
///Field `LPTIM6F` reader - illegal access flag for LPTIM6
pub type LPTIM6F_R = crate::BitReader<bool>;
///Field `VREFBUFF` reader - illegal access flag for VREFBUF
pub type VREFBUFF_R = crate::BitReader<bool>;
///Field `CRCF` reader - illegal access flag for CRC
pub type CRCF_R = crate::BitReader<bool>;
///Field `CORDICF` reader - illegal access flag for CORDIC
pub type CORDICF_R = crate::BitReader<bool>;
///Field `FMACF` reader - illegal access flag for FMAC
pub type FMACF_R = crate::BitReader<bool>;
///Field `ICACHEF` reader - illegal access flag for ICACHE
pub type ICACHEF_R = crate::BitReader<bool>;
///Field `DCACHEF` reader - illegal access flag for DCACHE
pub type DCACHEF_R = crate::BitReader<bool>;
///Field `ADC12F` reader - illegal access flag for ADC1 and ADC2
pub type ADC12F_R = crate::BitReader<bool>;
///Field `DCMIF` reader - illegal access flag for DCMI
pub type DCMIF_R = crate::BitReader<bool>;
///Field `HASHF` reader - illegal access flag for HASH
pub type HASHF_R = crate::BitReader<bool>;
///Field `RNGF` reader - illegal access flag for RNG
pub type RNGF_R = crate::BitReader<bool>;
///Field `SDMMC1F` reader - illegal access flag for SDMMC1
pub type SDMMC1F_R = crate::BitReader<bool>;
///Field `FMCF` reader - illegal access flag for FMC
pub type FMCF_R = crate::BitReader<bool>;
///Field `OCTOSPI1F` reader - illegal access flag for OCTOSPI1
pub type OCTOSPI1F_R = crate::BitReader<bool>;
///Field `RAMCFGF` reader - illegal access flag for RAMSCFG
pub type RAMCFGF_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - illegal access flag for LPTIM6
    #[inline(always)]
    pub fn lptim6f(&self) -> LPTIM6F_R {
        LPTIM6F_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - illegal access flag for VREFBUF
    #[inline(always)]
    pub fn vrefbuff(&self) -> VREFBUFF_R {
        VREFBUFF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 8 - illegal access flag for CRC
    #[inline(always)]
    pub fn crcf(&self) -> CRCF_R {
        CRCF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - illegal access flag for CORDIC
    #[inline(always)]
    pub fn cordicf(&self) -> CORDICF_R {
        CORDICF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - illegal access flag for FMAC
    #[inline(always)]
    pub fn fmacf(&self) -> FMACF_R {
        FMACF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 12 - illegal access flag for ICACHE
    #[inline(always)]
    pub fn icachef(&self) -> ICACHEF_R {
        ICACHEF_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - illegal access flag for DCACHE
    #[inline(always)]
    pub fn dcachef(&self) -> DCACHEF_R {
        DCACHEF_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - illegal access flag for ADC1 and ADC2
    #[inline(always)]
    pub fn adc12f(&self) -> ADC12F_R {
        ADC12F_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - illegal access flag for DCMI
    #[inline(always)]
    pub fn dcmif(&self) -> DCMIF_R {
        DCMIF_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 17 - illegal access flag for HASH
    #[inline(always)]
    pub fn hashf(&self) -> HASHF_R {
        HASHF_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - illegal access flag for RNG
    #[inline(always)]
    pub fn rngf(&self) -> RNGF_R {
        RNGF_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 22 - illegal access flag for SDMMC1
    #[inline(always)]
    pub fn sdmmc1f(&self) -> SDMMC1F_R {
        SDMMC1F_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - illegal access flag for FMC
    #[inline(always)]
    pub fn fmcf(&self) -> FMCF_R {
        FMCF_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - illegal access flag for OCTOSPI1
    #[inline(always)]
    pub fn octospi1f(&self) -> OCTOSPI1F_R {
        OCTOSPI1F_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 26 - illegal access flag for RAMSCFG
    #[inline(always)]
    pub fn ramcfgf(&self) -> RAMCFGF_R {
        RAMCFGF_R::new(((self.bits >> 26) & 1) != 0)
    }
}
///TZIC status register 3
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [gtzc1_tzic_sr3](index.html) module
pub struct GTZC1_TZIC_SR3_SPEC;
impl crate::RegisterSpec for GTZC1_TZIC_SR3_SPEC {
    type Ux = u32;
}
///`read()` method returns [gtzc1_tzic_sr3::R](R) reader structure
impl crate::Readable for GTZC1_TZIC_SR3_SPEC {
    type Reader = R;
}
///`reset()` method sets GTZC1_TZIC_SR3 to value 0
impl crate::Resettable for GTZC1_TZIC_SR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
