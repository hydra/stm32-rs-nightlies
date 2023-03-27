///Register `TZSC_PRIVCFGR3` reader
pub struct R(crate::R<TZSC_PRIVCFGR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZSC_PRIVCFGR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZSC_PRIVCFGR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZSC_PRIVCFGR3_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TZSC_PRIVCFGR3` writer
pub struct W(crate::W<TZSC_PRIVCFGR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TZSC_PRIVCFGR3_SPEC>;
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
impl From<crate::W<TZSC_PRIVCFGR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TZSC_PRIVCFGR3_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MDF1PRIV` reader - privileged access mode for MDF1
pub type MDF1PRIV_R = crate::BitReader<bool>;
///Field `MDF1PRIV` writer - privileged access mode for MDF1
pub type MDF1PRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_PRIVCFGR3_SPEC, bool, O>;
///Field `CORDICPRIV` reader - privileged access mode for CORDIC
pub type CORDICPRIV_R = crate::BitReader<bool>;
///Field `CORDICPRIV` writer - privileged access mode for CORDIC
pub type CORDICPRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_PRIVCFGR3_SPEC, bool, O>;
///Field `FMACPRIV` reader - privileged access mode for FMAC
pub type FMACPRIV_R = crate::BitReader<bool>;
///Field `FMACPRIV` writer - privileged access mode for FMAC
pub type FMACPRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_PRIVCFGR3_SPEC, bool, O>;
///Field `CRCPRIV` reader - privileged access mode for CRC
pub type CRCPRIV_R = crate::BitReader<bool>;
///Field `CRCPRIV` writer - privileged access mode for CRC
pub type CRCPRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_PRIVCFGR3_SPEC, bool, O>;
///Field `TSCPRIV` reader - privileged access mode for TSC
pub type TSCPRIV_R = crate::BitReader<bool>;
///Field `TSCPRIV` writer - privileged access mode for TSC
pub type TSCPRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_PRIVCFGR3_SPEC, bool, O>;
///Field `DMA2DPRIV` reader - privileged access mode for register of DMA2D
pub type DMA2DPRIV_R = crate::BitReader<bool>;
///Field `DMA2DPRIV` writer - privileged access mode for register of DMA2D
pub type DMA2DPRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_PRIVCFGR3_SPEC, bool, O>;
///Field `ICACHE_REGPRIV` reader - privileged access mode for ICACHE registers
pub type ICACHE_REGPRIV_R = crate::BitReader<bool>;
///Field `ICACHE_REGPRIV` writer - privileged access mode for ICACHE registers
pub type ICACHE_REGPRIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZSC_PRIVCFGR3_SPEC, bool, O>;
///Field `DCACHE_REGPRIV` reader - privileged access mode for DCACHE registers
pub type DCACHE_REGPRIV_R = crate::BitReader<bool>;
///Field `DCACHE_REGPRIV` writer - privileged access mode for DCACHE registers
pub type DCACHE_REGPRIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZSC_PRIVCFGR3_SPEC, bool, O>;
///Field `ADC1PRIV` reader - privileged access mode for ADC1
pub type ADC1PRIV_R = crate::BitReader<bool>;
///Field `ADC1PRIV` writer - privileged access mode for ADC1
pub type ADC1PRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_PRIVCFGR3_SPEC, bool, O>;
///Field `DCMIPRIV` reader - privileged access mode for DCMI
pub type DCMIPRIV_R = crate::BitReader<bool>;
///Field `DCMIPRIV` writer - privileged access mode for DCMI
pub type DCMIPRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_PRIVCFGR3_SPEC, bool, O>;
///Field `OTGFSPRIV` reader - privileged access mode for OTG_FS
pub type OTGFSPRIV_R = crate::BitReader<bool>;
///Field `OTGFSPRIV` writer - privileged access mode for OTG_FS
pub type OTGFSPRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_PRIVCFGR3_SPEC, bool, O>;
///Field `AESPRIV` reader - privileged access mode for AES
pub type AESPRIV_R = crate::BitReader<bool>;
///Field `AESPRIV` writer - privileged access mode for AES
pub type AESPRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_PRIVCFGR3_SPEC, bool, O>;
///Field `HASHPRIV` reader - privileged access mode for HASH
pub type HASHPRIV_R = crate::BitReader<bool>;
///Field `HASHPRIV` writer - privileged access mode for HASH
pub type HASHPRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_PRIVCFGR3_SPEC, bool, O>;
///Field `RNGPRIV` reader - privileged access mode for RNG
pub type RNGPRIV_R = crate::BitReader<bool>;
///Field `RNGPRIV` writer - privileged access mode for RNG
pub type RNGPRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_PRIVCFGR3_SPEC, bool, O>;
///Field `PKAPRIV` reader - privileged access mode for PKA
pub type PKAPRIV_R = crate::BitReader<bool>;
///Field `PKAPRIV` writer - privileged access mode for PKA
pub type PKAPRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_PRIVCFGR3_SPEC, bool, O>;
///Field `SAESPRIV` reader - privileged access mode for SAES
pub type SAESPRIV_R = crate::BitReader<bool>;
///Field `SAESPRIV` writer - privileged access mode for SAES
pub type SAESPRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_PRIVCFGR3_SPEC, bool, O>;
///Field `OCTOSPIMPRIV` reader - privileged access mode for OCTOSPIM
pub type OCTOSPIMPRIV_R = crate::BitReader<bool>;
///Field `OCTOSPIMPRIV` writer - privileged access mode for OCTOSPIM
pub type OCTOSPIMPRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_PRIVCFGR3_SPEC, bool, O>;
///Field `SDMMC1PRIV` reader - privileged access mode for SDMMC2
pub type SDMMC1PRIV_R = crate::BitReader<bool>;
///Field `SDMMC1PRIV` writer - privileged access mode for SDMMC2
pub type SDMMC1PRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_PRIVCFGR3_SPEC, bool, O>;
///Field `SDMMC2PRIV` reader - privileged access mode for SDMMC1
pub type SDMMC2PRIV_R = crate::BitReader<bool>;
///Field `SDMMC2PRIV` writer - privileged access mode for SDMMC1
pub type SDMMC2PRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_PRIVCFGR3_SPEC, bool, O>;
///Field `FSMC_REGPRIV` reader - privileged access mode for FSMC registers
pub type FSMC_REGPRIV_R = crate::BitReader<bool>;
///Field `FSMC_REGPRIV` writer - privileged access mode for FSMC registers
pub type FSMC_REGPRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_PRIVCFGR3_SPEC, bool, O>;
///Field `OCTOSPI1_REGPRIV` reader - privileged access mode for OCTOSPI1
pub type OCTOSPI1_REGPRIV_R = crate::BitReader<bool>;
///Field `OCTOSPI1_REGPRIV` writer - privileged access mode for OCTOSPI1
pub type OCTOSPI1_REGPRIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZSC_PRIVCFGR3_SPEC, bool, O>;
///Field `OCTOSPI2_REGPRIV` reader - privileged access mode for OCTOSPI2
pub type OCTOSPI2_REGPRIV_R = crate::BitReader<bool>;
///Field `OCTOSPI2_REGPRIV` writer - privileged access mode for OCTOSPI2
pub type OCTOSPI2_REGPRIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZSC_PRIVCFGR3_SPEC, bool, O>;
///Field `RAMCFGPRIV` reader - privileged access mode for RAMCFG
pub type RAMCFGPRIV_R = crate::BitReader<bool>;
///Field `RAMCFGPRIV` writer - privileged access mode for RAMCFG
pub type RAMCFGPRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_PRIVCFGR3_SPEC, bool, O>;
impl R {
    ///Bit 0 - privileged access mode for MDF1
    #[inline(always)]
    pub fn mdf1priv(&self) -> MDF1PRIV_R {
        MDF1PRIV_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - privileged access mode for CORDIC
    #[inline(always)]
    pub fn cordicpriv(&self) -> CORDICPRIV_R {
        CORDICPRIV_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - privileged access mode for FMAC
    #[inline(always)]
    pub fn fmacpriv(&self) -> FMACPRIV_R {
        FMACPRIV_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - privileged access mode for CRC
    #[inline(always)]
    pub fn crcpriv(&self) -> CRCPRIV_R {
        CRCPRIV_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - privileged access mode for TSC
    #[inline(always)]
    pub fn tscpriv(&self) -> TSCPRIV_R {
        TSCPRIV_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - privileged access mode for register of DMA2D
    #[inline(always)]
    pub fn dma2dpriv(&self) -> DMA2DPRIV_R {
        DMA2DPRIV_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - privileged access mode for ICACHE registers
    #[inline(always)]
    pub fn icache_regpriv(&self) -> ICACHE_REGPRIV_R {
        ICACHE_REGPRIV_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - privileged access mode for DCACHE registers
    #[inline(always)]
    pub fn dcache_regpriv(&self) -> DCACHE_REGPRIV_R {
        DCACHE_REGPRIV_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - privileged access mode for ADC1
    #[inline(always)]
    pub fn adc1priv(&self) -> ADC1PRIV_R {
        ADC1PRIV_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - privileged access mode for DCMI
    #[inline(always)]
    pub fn dcmipriv(&self) -> DCMIPRIV_R {
        DCMIPRIV_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - privileged access mode for OTG_FS
    #[inline(always)]
    pub fn otgfspriv(&self) -> OTGFSPRIV_R {
        OTGFSPRIV_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - privileged access mode for AES
    #[inline(always)]
    pub fn aespriv(&self) -> AESPRIV_R {
        AESPRIV_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - privileged access mode for HASH
    #[inline(always)]
    pub fn hashpriv(&self) -> HASHPRIV_R {
        HASHPRIV_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - privileged access mode for RNG
    #[inline(always)]
    pub fn rngpriv(&self) -> RNGPRIV_R {
        RNGPRIV_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - privileged access mode for PKA
    #[inline(always)]
    pub fn pkapriv(&self) -> PKAPRIV_R {
        PKAPRIV_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - privileged access mode for SAES
    #[inline(always)]
    pub fn saespriv(&self) -> SAESPRIV_R {
        SAESPRIV_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - privileged access mode for OCTOSPIM
    #[inline(always)]
    pub fn octospimpriv(&self) -> OCTOSPIMPRIV_R {
        OCTOSPIMPRIV_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - privileged access mode for SDMMC2
    #[inline(always)]
    pub fn sdmmc1priv(&self) -> SDMMC1PRIV_R {
        SDMMC1PRIV_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - privileged access mode for SDMMC1
    #[inline(always)]
    pub fn sdmmc2priv(&self) -> SDMMC2PRIV_R {
        SDMMC2PRIV_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - privileged access mode for FSMC registers
    #[inline(always)]
    pub fn fsmc_regpriv(&self) -> FSMC_REGPRIV_R {
        FSMC_REGPRIV_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - privileged access mode for OCTOSPI1
    #[inline(always)]
    pub fn octospi1_regpriv(&self) -> OCTOSPI1_REGPRIV_R {
        OCTOSPI1_REGPRIV_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - privileged access mode for OCTOSPI2
    #[inline(always)]
    pub fn octospi2_regpriv(&self) -> OCTOSPI2_REGPRIV_R {
        OCTOSPI2_REGPRIV_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - privileged access mode for RAMCFG
    #[inline(always)]
    pub fn ramcfgpriv(&self) -> RAMCFGPRIV_R {
        RAMCFGPRIV_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - privileged access mode for MDF1
    #[inline(always)]
    #[must_use]
    pub fn mdf1priv(&mut self) -> MDF1PRIV_W<0> {
        MDF1PRIV_W::new(self)
    }
    ///Bit 1 - privileged access mode for CORDIC
    #[inline(always)]
    #[must_use]
    pub fn cordicpriv(&mut self) -> CORDICPRIV_W<1> {
        CORDICPRIV_W::new(self)
    }
    ///Bit 2 - privileged access mode for FMAC
    #[inline(always)]
    #[must_use]
    pub fn fmacpriv(&mut self) -> FMACPRIV_W<2> {
        FMACPRIV_W::new(self)
    }
    ///Bit 3 - privileged access mode for CRC
    #[inline(always)]
    #[must_use]
    pub fn crcpriv(&mut self) -> CRCPRIV_W<3> {
        CRCPRIV_W::new(self)
    }
    ///Bit 4 - privileged access mode for TSC
    #[inline(always)]
    #[must_use]
    pub fn tscpriv(&mut self) -> TSCPRIV_W<4> {
        TSCPRIV_W::new(self)
    }
    ///Bit 5 - privileged access mode for register of DMA2D
    #[inline(always)]
    #[must_use]
    pub fn dma2dpriv(&mut self) -> DMA2DPRIV_W<5> {
        DMA2DPRIV_W::new(self)
    }
    ///Bit 6 - privileged access mode for ICACHE registers
    #[inline(always)]
    #[must_use]
    pub fn icache_regpriv(&mut self) -> ICACHE_REGPRIV_W<6> {
        ICACHE_REGPRIV_W::new(self)
    }
    ///Bit 7 - privileged access mode for DCACHE registers
    #[inline(always)]
    #[must_use]
    pub fn dcache_regpriv(&mut self) -> DCACHE_REGPRIV_W<7> {
        DCACHE_REGPRIV_W::new(self)
    }
    ///Bit 8 - privileged access mode for ADC1
    #[inline(always)]
    #[must_use]
    pub fn adc1priv(&mut self) -> ADC1PRIV_W<8> {
        ADC1PRIV_W::new(self)
    }
    ///Bit 9 - privileged access mode for DCMI
    #[inline(always)]
    #[must_use]
    pub fn dcmipriv(&mut self) -> DCMIPRIV_W<9> {
        DCMIPRIV_W::new(self)
    }
    ///Bit 10 - privileged access mode for OTG_FS
    #[inline(always)]
    #[must_use]
    pub fn otgfspriv(&mut self) -> OTGFSPRIV_W<10> {
        OTGFSPRIV_W::new(self)
    }
    ///Bit 11 - privileged access mode for AES
    #[inline(always)]
    #[must_use]
    pub fn aespriv(&mut self) -> AESPRIV_W<11> {
        AESPRIV_W::new(self)
    }
    ///Bit 12 - privileged access mode for HASH
    #[inline(always)]
    #[must_use]
    pub fn hashpriv(&mut self) -> HASHPRIV_W<12> {
        HASHPRIV_W::new(self)
    }
    ///Bit 13 - privileged access mode for RNG
    #[inline(always)]
    #[must_use]
    pub fn rngpriv(&mut self) -> RNGPRIV_W<13> {
        RNGPRIV_W::new(self)
    }
    ///Bit 14 - privileged access mode for PKA
    #[inline(always)]
    #[must_use]
    pub fn pkapriv(&mut self) -> PKAPRIV_W<14> {
        PKAPRIV_W::new(self)
    }
    ///Bit 15 - privileged access mode for SAES
    #[inline(always)]
    #[must_use]
    pub fn saespriv(&mut self) -> SAESPRIV_W<15> {
        SAESPRIV_W::new(self)
    }
    ///Bit 16 - privileged access mode for OCTOSPIM
    #[inline(always)]
    #[must_use]
    pub fn octospimpriv(&mut self) -> OCTOSPIMPRIV_W<16> {
        OCTOSPIMPRIV_W::new(self)
    }
    ///Bit 17 - privileged access mode for SDMMC2
    #[inline(always)]
    #[must_use]
    pub fn sdmmc1priv(&mut self) -> SDMMC1PRIV_W<17> {
        SDMMC1PRIV_W::new(self)
    }
    ///Bit 18 - privileged access mode for SDMMC1
    #[inline(always)]
    #[must_use]
    pub fn sdmmc2priv(&mut self) -> SDMMC2PRIV_W<18> {
        SDMMC2PRIV_W::new(self)
    }
    ///Bit 19 - privileged access mode for FSMC registers
    #[inline(always)]
    #[must_use]
    pub fn fsmc_regpriv(&mut self) -> FSMC_REGPRIV_W<19> {
        FSMC_REGPRIV_W::new(self)
    }
    ///Bit 20 - privileged access mode for OCTOSPI1
    #[inline(always)]
    #[must_use]
    pub fn octospi1_regpriv(&mut self) -> OCTOSPI1_REGPRIV_W<20> {
        OCTOSPI1_REGPRIV_W::new(self)
    }
    ///Bit 21 - privileged access mode for OCTOSPI2
    #[inline(always)]
    #[must_use]
    pub fn octospi2_regpriv(&mut self) -> OCTOSPI2_REGPRIV_W<21> {
        OCTOSPI2_REGPRIV_W::new(self)
    }
    ///Bit 22 - privileged access mode for RAMCFG
    #[inline(always)]
    #[must_use]
    pub fn ramcfgpriv(&mut self) -> RAMCFGPRIV_W<22> {
        RAMCFGPRIV_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TZSC privilege configuration register 3
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tzsc_privcfgr3](index.html) module
pub struct TZSC_PRIVCFGR3_SPEC;
impl crate::RegisterSpec for TZSC_PRIVCFGR3_SPEC {
    type Ux = u32;
}
///`read()` method returns [tzsc_privcfgr3::R](R) reader structure
impl crate::Readable for TZSC_PRIVCFGR3_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [tzsc_privcfgr3::W](W) writer structure
impl crate::Writable for TZSC_PRIVCFGR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets TZSC_PRIVCFGR3 to value 0
impl crate::Resettable for TZSC_PRIVCFGR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
