///Register `TZSC_SECCFGR3` reader
pub struct R(crate::R<TZSC_SECCFGR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZSC_SECCFGR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZSC_SECCFGR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZSC_SECCFGR3_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TZSC_SECCFGR3` writer
pub struct W(crate::W<TZSC_SECCFGR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TZSC_SECCFGR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<TZSC_SECCFGR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TZSC_SECCFGR3_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MDF1SEC` reader - secure access mode for MDF1
pub type MDF1SEC_R = crate::BitReader<bool>;
///Field `MDF1SEC` writer - secure access mode for MDF1
pub type MDF1SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_SECCFGR3_SPEC, bool, O>;
///Field `CORDICSEC` reader - secure access mode for CORDIC
pub type CORDICSEC_R = crate::BitReader<bool>;
///Field `CORDICSEC` writer - secure access mode for CORDIC
pub type CORDICSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_SECCFGR3_SPEC, bool, O>;
///Field `FMACSEC` reader - secure access mode for FMAC
pub type FMACSEC_R = crate::BitReader<bool>;
///Field `FMACSEC` writer - secure access mode for FMAC
pub type FMACSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_SECCFGR3_SPEC, bool, O>;
///Field `CRCSEC` reader - secure access mode for CRC
pub type CRCSEC_R = crate::BitReader<bool>;
///Field `CRCSEC` writer - secure access mode for CRC
pub type CRCSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_SECCFGR3_SPEC, bool, O>;
///Field `TSCSEC` reader - secure access mode for TSC
pub type TSCSEC_R = crate::BitReader<bool>;
///Field `TSCSEC` writer - secure access mode for TSC
pub type TSCSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_SECCFGR3_SPEC, bool, O>;
///Field `DMA2DSEC` reader - secure access mode for register of DMA2D
pub type DMA2DSEC_R = crate::BitReader<bool>;
///Field `DMA2DSEC` writer - secure access mode for register of DMA2D
pub type DMA2DSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_SECCFGR3_SPEC, bool, O>;
///Field `ICACHE_REGSEC` reader - secure access mode for ICACHE registers
pub type ICACHE_REGSEC_R = crate::BitReader<bool>;
///Field `ICACHE_REGSEC` writer - secure access mode for ICACHE registers
pub type ICACHE_REGSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_SECCFGR3_SPEC, bool, O>;
///Field `DCACHE_REGSEC` reader - secure access mode for DCACHE registers
pub type DCACHE_REGSEC_R = crate::BitReader<bool>;
///Field `DCACHE_REGSEC` writer - secure access mode for DCACHE registers
pub type DCACHE_REGSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_SECCFGR3_SPEC, bool, O>;
///Field `ADC1SEC` reader - secure access mode for ADC1
pub type ADC1SEC_R = crate::BitReader<bool>;
///Field `ADC1SEC` writer - secure access mode for ADC1
pub type ADC1SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_SECCFGR3_SPEC, bool, O>;
///Field `DCMISEC` reader - secure access mode for DCMI
pub type DCMISEC_R = crate::BitReader<bool>;
///Field `DCMISEC` writer - secure access mode for DCMI
pub type DCMISEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_SECCFGR3_SPEC, bool, O>;
///Field `OTGFSSEC` reader - secure access mode for OTG_FS
pub type OTGFSSEC_R = crate::BitReader<bool>;
///Field `OTGFSSEC` writer - secure access mode for OTG_FS
pub type OTGFSSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_SECCFGR3_SPEC, bool, O>;
///Field `AESSEC` reader - secure access mode for AES
pub type AESSEC_R = crate::BitReader<bool>;
///Field `AESSEC` writer - secure access mode for AES
pub type AESSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_SECCFGR3_SPEC, bool, O>;
///Field `HASHSEC` reader - secure access mode for HASH
pub type HASHSEC_R = crate::BitReader<bool>;
///Field `HASHSEC` writer - secure access mode for HASH
pub type HASHSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_SECCFGR3_SPEC, bool, O>;
///Field `RNGSEC` reader - secure access mode for RNG
pub type RNGSEC_R = crate::BitReader<bool>;
///Field `RNGSEC` writer - secure access mode for RNG
pub type RNGSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_SECCFGR3_SPEC, bool, O>;
///Field `PKASEC` reader - secure access mode for PKA
pub type PKASEC_R = crate::BitReader<bool>;
///Field `PKASEC` writer - secure access mode for PKA
pub type PKASEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_SECCFGR3_SPEC, bool, O>;
///Field `SAESSEC` reader - secure access mode for SAES
pub type SAESSEC_R = crate::BitReader<bool>;
///Field `SAESSEC` writer - secure access mode for SAES
pub type SAESSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_SECCFGR3_SPEC, bool, O>;
///Field `OCTOSPIMSEC` reader - secure access mode for OCTOSPIM
pub type OCTOSPIMSEC_R = crate::BitReader<bool>;
///Field `OCTOSPIMSEC` writer - secure access mode for OCTOSPIM
pub type OCTOSPIMSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_SECCFGR3_SPEC, bool, O>;
///Field `SDMMC1SEC` reader - secure access mode for SDMMC2
pub type SDMMC1SEC_R = crate::BitReader<bool>;
///Field `SDMMC1SEC` writer - secure access mode for SDMMC2
pub type SDMMC1SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_SECCFGR3_SPEC, bool, O>;
///Field `SDMMC2SEC` reader - secure access mode for SDMMC1
pub type SDMMC2SEC_R = crate::BitReader<bool>;
///Field `SDMMC2SEC` writer - secure access mode for SDMMC1
pub type SDMMC2SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_SECCFGR3_SPEC, bool, O>;
///Field `FSMC_REGSEC` reader - secure access mode for FSMC registers
pub type FSMC_REGSEC_R = crate::BitReader<bool>;
///Field `FSMC_REGSEC` writer - secure access mode for FSMC registers
pub type FSMC_REGSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_SECCFGR3_SPEC, bool, O>;
///Field `OCTOSPI1_REGSEC` reader - secure access mode for OCTOSPI1 registers
pub type OCTOSPI1_REGSEC_R = crate::BitReader<bool>;
///Field `OCTOSPI1_REGSEC` writer - secure access mode for OCTOSPI1 registers
pub type OCTOSPI1_REGSEC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZSC_SECCFGR3_SPEC, bool, O>;
///Field `OCTOSPI2_REGSEC` reader - secure access mode for OCTOSPI2 registers
pub type OCTOSPI2_REGSEC_R = crate::BitReader<bool>;
///Field `OCTOSPI2_REGSEC` writer - secure access mode for OCTOSPI2 registers
pub type OCTOSPI2_REGSEC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZSC_SECCFGR3_SPEC, bool, O>;
///Field `RAMCFGSEC` reader - secure access mode for RAMCFG
pub type RAMCFGSEC_R = crate::BitReader<bool>;
///Field `RAMCFGSEC` writer - secure access mode for RAMCFG
pub type RAMCFGSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_SECCFGR3_SPEC, bool, O>;
impl R {
    ///Bit 0 - secure access mode for MDF1
    #[inline(always)]
    pub fn mdf1sec(&self) -> MDF1SEC_R {
        MDF1SEC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - secure access mode for CORDIC
    #[inline(always)]
    pub fn cordicsec(&self) -> CORDICSEC_R {
        CORDICSEC_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - secure access mode for FMAC
    #[inline(always)]
    pub fn fmacsec(&self) -> FMACSEC_R {
        FMACSEC_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - secure access mode for CRC
    #[inline(always)]
    pub fn crcsec(&self) -> CRCSEC_R {
        CRCSEC_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - secure access mode for TSC
    #[inline(always)]
    pub fn tscsec(&self) -> TSCSEC_R {
        TSCSEC_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - secure access mode for register of DMA2D
    #[inline(always)]
    pub fn dma2dsec(&self) -> DMA2DSEC_R {
        DMA2DSEC_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - secure access mode for ICACHE registers
    #[inline(always)]
    pub fn icache_regsec(&self) -> ICACHE_REGSEC_R {
        ICACHE_REGSEC_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - secure access mode for DCACHE registers
    #[inline(always)]
    pub fn dcache_regsec(&self) -> DCACHE_REGSEC_R {
        DCACHE_REGSEC_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - secure access mode for ADC1
    #[inline(always)]
    pub fn adc1sec(&self) -> ADC1SEC_R {
        ADC1SEC_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - secure access mode for DCMI
    #[inline(always)]
    pub fn dcmisec(&self) -> DCMISEC_R {
        DCMISEC_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - secure access mode for OTG_FS
    #[inline(always)]
    pub fn otgfssec(&self) -> OTGFSSEC_R {
        OTGFSSEC_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - secure access mode for AES
    #[inline(always)]
    pub fn aessec(&self) -> AESSEC_R {
        AESSEC_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - secure access mode for HASH
    #[inline(always)]
    pub fn hashsec(&self) -> HASHSEC_R {
        HASHSEC_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - secure access mode for RNG
    #[inline(always)]
    pub fn rngsec(&self) -> RNGSEC_R {
        RNGSEC_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - secure access mode for PKA
    #[inline(always)]
    pub fn pkasec(&self) -> PKASEC_R {
        PKASEC_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - secure access mode for SAES
    #[inline(always)]
    pub fn saessec(&self) -> SAESSEC_R {
        SAESSEC_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - secure access mode for OCTOSPIM
    #[inline(always)]
    pub fn octospimsec(&self) -> OCTOSPIMSEC_R {
        OCTOSPIMSEC_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - secure access mode for SDMMC2
    #[inline(always)]
    pub fn sdmmc1sec(&self) -> SDMMC1SEC_R {
        SDMMC1SEC_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - secure access mode for SDMMC1
    #[inline(always)]
    pub fn sdmmc2sec(&self) -> SDMMC2SEC_R {
        SDMMC2SEC_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - secure access mode for FSMC registers
    #[inline(always)]
    pub fn fsmc_regsec(&self) -> FSMC_REGSEC_R {
        FSMC_REGSEC_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - secure access mode for OCTOSPI1 registers
    #[inline(always)]
    pub fn octospi1_regsec(&self) -> OCTOSPI1_REGSEC_R {
        OCTOSPI1_REGSEC_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - secure access mode for OCTOSPI2 registers
    #[inline(always)]
    pub fn octospi2_regsec(&self) -> OCTOSPI2_REGSEC_R {
        OCTOSPI2_REGSEC_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - secure access mode for RAMCFG
    #[inline(always)]
    pub fn ramcfgsec(&self) -> RAMCFGSEC_R {
        RAMCFGSEC_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - secure access mode for MDF1
    #[inline(always)]
    #[must_use]
    pub fn mdf1sec(&mut self) -> MDF1SEC_W<0> {
        MDF1SEC_W::new(self)
    }
    ///Bit 1 - secure access mode for CORDIC
    #[inline(always)]
    #[must_use]
    pub fn cordicsec(&mut self) -> CORDICSEC_W<1> {
        CORDICSEC_W::new(self)
    }
    ///Bit 2 - secure access mode for FMAC
    #[inline(always)]
    #[must_use]
    pub fn fmacsec(&mut self) -> FMACSEC_W<2> {
        FMACSEC_W::new(self)
    }
    ///Bit 3 - secure access mode for CRC
    #[inline(always)]
    #[must_use]
    pub fn crcsec(&mut self) -> CRCSEC_W<3> {
        CRCSEC_W::new(self)
    }
    ///Bit 4 - secure access mode for TSC
    #[inline(always)]
    #[must_use]
    pub fn tscsec(&mut self) -> TSCSEC_W<4> {
        TSCSEC_W::new(self)
    }
    ///Bit 5 - secure access mode for register of DMA2D
    #[inline(always)]
    #[must_use]
    pub fn dma2dsec(&mut self) -> DMA2DSEC_W<5> {
        DMA2DSEC_W::new(self)
    }
    ///Bit 6 - secure access mode for ICACHE registers
    #[inline(always)]
    #[must_use]
    pub fn icache_regsec(&mut self) -> ICACHE_REGSEC_W<6> {
        ICACHE_REGSEC_W::new(self)
    }
    ///Bit 7 - secure access mode for DCACHE registers
    #[inline(always)]
    #[must_use]
    pub fn dcache_regsec(&mut self) -> DCACHE_REGSEC_W<7> {
        DCACHE_REGSEC_W::new(self)
    }
    ///Bit 8 - secure access mode for ADC1
    #[inline(always)]
    #[must_use]
    pub fn adc1sec(&mut self) -> ADC1SEC_W<8> {
        ADC1SEC_W::new(self)
    }
    ///Bit 9 - secure access mode for DCMI
    #[inline(always)]
    #[must_use]
    pub fn dcmisec(&mut self) -> DCMISEC_W<9> {
        DCMISEC_W::new(self)
    }
    ///Bit 10 - secure access mode for OTG_FS
    #[inline(always)]
    #[must_use]
    pub fn otgfssec(&mut self) -> OTGFSSEC_W<10> {
        OTGFSSEC_W::new(self)
    }
    ///Bit 11 - secure access mode for AES
    #[inline(always)]
    #[must_use]
    pub fn aessec(&mut self) -> AESSEC_W<11> {
        AESSEC_W::new(self)
    }
    ///Bit 12 - secure access mode for HASH
    #[inline(always)]
    #[must_use]
    pub fn hashsec(&mut self) -> HASHSEC_W<12> {
        HASHSEC_W::new(self)
    }
    ///Bit 13 - secure access mode for RNG
    #[inline(always)]
    #[must_use]
    pub fn rngsec(&mut self) -> RNGSEC_W<13> {
        RNGSEC_W::new(self)
    }
    ///Bit 14 - secure access mode for PKA
    #[inline(always)]
    #[must_use]
    pub fn pkasec(&mut self) -> PKASEC_W<14> {
        PKASEC_W::new(self)
    }
    ///Bit 15 - secure access mode for SAES
    #[inline(always)]
    #[must_use]
    pub fn saessec(&mut self) -> SAESSEC_W<15> {
        SAESSEC_W::new(self)
    }
    ///Bit 16 - secure access mode for OCTOSPIM
    #[inline(always)]
    #[must_use]
    pub fn octospimsec(&mut self) -> OCTOSPIMSEC_W<16> {
        OCTOSPIMSEC_W::new(self)
    }
    ///Bit 17 - secure access mode for SDMMC2
    #[inline(always)]
    #[must_use]
    pub fn sdmmc1sec(&mut self) -> SDMMC1SEC_W<17> {
        SDMMC1SEC_W::new(self)
    }
    ///Bit 18 - secure access mode for SDMMC1
    #[inline(always)]
    #[must_use]
    pub fn sdmmc2sec(&mut self) -> SDMMC2SEC_W<18> {
        SDMMC2SEC_W::new(self)
    }
    ///Bit 19 - secure access mode for FSMC registers
    #[inline(always)]
    #[must_use]
    pub fn fsmc_regsec(&mut self) -> FSMC_REGSEC_W<19> {
        FSMC_REGSEC_W::new(self)
    }
    ///Bit 20 - secure access mode for OCTOSPI1 registers
    #[inline(always)]
    #[must_use]
    pub fn octospi1_regsec(&mut self) -> OCTOSPI1_REGSEC_W<20> {
        OCTOSPI1_REGSEC_W::new(self)
    }
    ///Bit 21 - secure access mode for OCTOSPI2 registers
    #[inline(always)]
    #[must_use]
    pub fn octospi2_regsec(&mut self) -> OCTOSPI2_REGSEC_W<21> {
        OCTOSPI2_REGSEC_W::new(self)
    }
    ///Bit 22 - secure access mode for RAMCFG
    #[inline(always)]
    #[must_use]
    pub fn ramcfgsec(&mut self) -> RAMCFGSEC_W<22> {
        RAMCFGSEC_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TZSC secure configuration register 3
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tzsc_seccfgr3](index.html) module
pub struct TZSC_SECCFGR3_SPEC;
impl crate::RegisterSpec for TZSC_SECCFGR3_SPEC {
    type Ux = u32;
}
///`read()` method returns [tzsc_seccfgr3::R](R) reader structure
impl crate::Readable for TZSC_SECCFGR3_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [tzsc_seccfgr3::W](W) writer structure
impl crate::Writable for TZSC_SECCFGR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets TZSC_SECCFGR3 to value 0
impl crate::Resettable for TZSC_SECCFGR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
