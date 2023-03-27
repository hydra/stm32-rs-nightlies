///Register `IER3` reader
pub struct R(crate::R<IER3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IER3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IER3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IER3_SPEC>) -> Self {
        R(reader)
    }
}
///Register `IER3` writer
pub struct W(crate::W<IER3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IER3_SPEC>;
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
impl From<crate::W<IER3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IER3_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MDF1IE` reader - illegal access interrupt enable for MDF1
pub type MDF1IE_R = crate::BitReader<bool>;
///Field `MDF1IE` writer - illegal access interrupt enable for MDF1
pub type MDF1IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER3_SPEC, bool, O>;
///Field `CORDICIE` reader - illegal access interrupt enable for CORDIC
pub type CORDICIE_R = crate::BitReader<bool>;
///Field `CORDICIE` writer - illegal access interrupt enable for CORDIC
pub type CORDICIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER3_SPEC, bool, O>;
///Field `FMACIE` reader - illegal access interrupt enable for FMAC
pub type FMACIE_R = crate::BitReader<bool>;
///Field `FMACIE` writer - illegal access interrupt enable for FMAC
pub type FMACIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER3_SPEC, bool, O>;
///Field `CRCIE` reader - illegal access interrupt enable for CRC
pub type CRCIE_R = crate::BitReader<bool>;
///Field `CRCIE` writer - illegal access interrupt enable for CRC
pub type CRCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER3_SPEC, bool, O>;
///Field `TSCIE` reader - illegal access interrupt enable for TSC
pub type TSCIE_R = crate::BitReader<bool>;
///Field `TSCIE` writer - illegal access interrupt enable for TSC
pub type TSCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER3_SPEC, bool, O>;
///Field `DMA2DIE` reader - illegal access interrupt enable for register of DMA2D
pub type DMA2DIE_R = crate::BitReader<bool>;
///Field `DMA2DIE` writer - illegal access interrupt enable for register of DMA2D
pub type DMA2DIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER3_SPEC, bool, O>;
///Field `ICACHEIE` reader - illegal access interrupt enable for ICACHE registers
pub type ICACHEIE_R = crate::BitReader<bool>;
///Field `ICACHEIE` writer - illegal access interrupt enable for ICACHE registers
pub type ICACHEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER3_SPEC, bool, O>;
///Field `DCACHEIE` reader - illegal access interrupt enable for DCACHE registers
pub type DCACHEIE_R = crate::BitReader<bool>;
///Field `DCACHEIE` writer - illegal access interrupt enable for DCACHE registers
pub type DCACHEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER3_SPEC, bool, O>;
///Field `ADC1IE` reader - illegal access interrupt enable for ADC1
pub type ADC1IE_R = crate::BitReader<bool>;
///Field `ADC1IE` writer - illegal access interrupt enable for ADC1
pub type ADC1IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER3_SPEC, bool, O>;
///Field `DCMIIE` reader - illegal access interrupt enable for DCMI
pub type DCMIIE_R = crate::BitReader<bool>;
///Field `DCMIIE` writer - illegal access interrupt enable for DCMI
pub type DCMIIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER3_SPEC, bool, O>;
///Field `OTGFSIE` reader - illegal access interrupt enable for OTG_FS
pub type OTGFSIE_R = crate::BitReader<bool>;
///Field `OTGFSIE` writer - illegal access interrupt enable for OTG_FS
pub type OTGFSIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER3_SPEC, bool, O>;
///Field `AESIE` reader - illegal access interrupt enable for AES
pub type AESIE_R = crate::BitReader<bool>;
///Field `AESIE` writer - illegal access interrupt enable for AES
pub type AESIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER3_SPEC, bool, O>;
///Field `HASHIE` reader - illegal access interrupt enable for HASH
pub type HASHIE_R = crate::BitReader<bool>;
///Field `HASHIE` writer - illegal access interrupt enable for HASH
pub type HASHIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER3_SPEC, bool, O>;
///Field `RNGIE` reader - illegal access interrupt enable for RNG
pub type RNGIE_R = crate::BitReader<bool>;
///Field `RNGIE` writer - illegal access interrupt enable for RNG
pub type RNGIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER3_SPEC, bool, O>;
///Field `PKAIE` reader - illegal access interrupt enable for PKA
pub type PKAIE_R = crate::BitReader<bool>;
///Field `PKAIE` writer - illegal access interrupt enable for PKA
pub type PKAIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER3_SPEC, bool, O>;
///Field `SAESIE` reader - illegal access interrupt enable for SAES
pub type SAESIE_R = crate::BitReader<bool>;
///Field `SAESIE` writer - illegal access interrupt enable for SAES
pub type SAESIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER3_SPEC, bool, O>;
///Field `OCTOSPIMIE` reader - illegal access interrupt enable for OCTOSPIM
pub type OCTOSPIMIE_R = crate::BitReader<bool>;
///Field `OCTOSPIMIE` writer - illegal access interrupt enable for OCTOSPIM
pub type OCTOSPIMIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER3_SPEC, bool, O>;
///Field `SDMMC1IE` reader - illegal access interrupt enable for SDMMC2
pub type SDMMC1IE_R = crate::BitReader<bool>;
///Field `SDMMC1IE` writer - illegal access interrupt enable for SDMMC2
pub type SDMMC1IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER3_SPEC, bool, O>;
///Field `SDMMC2IE` reader - illegal access interrupt enable for SDMMC1
pub type SDMMC2IE_R = crate::BitReader<bool>;
///Field `SDMMC2IE` writer - illegal access interrupt enable for SDMMC1
pub type SDMMC2IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER3_SPEC, bool, O>;
///Field `FSMCIE` reader - illegal access interrupt enable for FSMC registers
pub type FSMCIE_R = crate::BitReader<bool>;
///Field `FSMCIE` writer - illegal access interrupt enable for FSMC registers
pub type FSMCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER3_SPEC, bool, O>;
///Field `OCTOSPI1IE` reader - illegal access interrupt enable for OCTOSPI1 registers
pub type OCTOSPI1IE_R = crate::BitReader<bool>;
///Field `OCTOSPI1IE` writer - illegal access interrupt enable for OCTOSPI1 registers
pub type OCTOSPI1IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER3_SPEC, bool, O>;
///Field `OCTOSPI2IE` reader - illegal access interrupt enable for OCTOSPI2 registers
pub type OCTOSPI2IE_R = crate::BitReader<bool>;
///Field `OCTOSPI2IE` writer - illegal access interrupt enable for OCTOSPI2 registers
pub type OCTOSPI2IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER3_SPEC, bool, O>;
///Field `RAMCFGIE` reader - illegal access interrupt enable for RAMCFG
pub type RAMCFGIE_R = crate::BitReader<bool>;
///Field `RAMCFGIE` writer - illegal access interrupt enable for RAMCFG
pub type RAMCFGIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER3_SPEC, bool, O>;
impl R {
    ///Bit 0 - illegal access interrupt enable for MDF1
    #[inline(always)]
    pub fn mdf1ie(&self) -> MDF1IE_R {
        MDF1IE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - illegal access interrupt enable for CORDIC
    #[inline(always)]
    pub fn cordicie(&self) -> CORDICIE_R {
        CORDICIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - illegal access interrupt enable for FMAC
    #[inline(always)]
    pub fn fmacie(&self) -> FMACIE_R {
        FMACIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - illegal access interrupt enable for CRC
    #[inline(always)]
    pub fn crcie(&self) -> CRCIE_R {
        CRCIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - illegal access interrupt enable for TSC
    #[inline(always)]
    pub fn tscie(&self) -> TSCIE_R {
        TSCIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - illegal access interrupt enable for register of DMA2D
    #[inline(always)]
    pub fn dma2die(&self) -> DMA2DIE_R {
        DMA2DIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - illegal access interrupt enable for ICACHE registers
    #[inline(always)]
    pub fn icacheie(&self) -> ICACHEIE_R {
        ICACHEIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - illegal access interrupt enable for DCACHE registers
    #[inline(always)]
    pub fn dcacheie(&self) -> DCACHEIE_R {
        DCACHEIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - illegal access interrupt enable for ADC1
    #[inline(always)]
    pub fn adc1ie(&self) -> ADC1IE_R {
        ADC1IE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - illegal access interrupt enable for DCMI
    #[inline(always)]
    pub fn dcmiie(&self) -> DCMIIE_R {
        DCMIIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - illegal access interrupt enable for OTG_FS
    #[inline(always)]
    pub fn otgfsie(&self) -> OTGFSIE_R {
        OTGFSIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - illegal access interrupt enable for AES
    #[inline(always)]
    pub fn aesie(&self) -> AESIE_R {
        AESIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - illegal access interrupt enable for HASH
    #[inline(always)]
    pub fn hashie(&self) -> HASHIE_R {
        HASHIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - illegal access interrupt enable for RNG
    #[inline(always)]
    pub fn rngie(&self) -> RNGIE_R {
        RNGIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - illegal access interrupt enable for PKA
    #[inline(always)]
    pub fn pkaie(&self) -> PKAIE_R {
        PKAIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - illegal access interrupt enable for SAES
    #[inline(always)]
    pub fn saesie(&self) -> SAESIE_R {
        SAESIE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - illegal access interrupt enable for OCTOSPIM
    #[inline(always)]
    pub fn octospimie(&self) -> OCTOSPIMIE_R {
        OCTOSPIMIE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - illegal access interrupt enable for SDMMC2
    #[inline(always)]
    pub fn sdmmc1ie(&self) -> SDMMC1IE_R {
        SDMMC1IE_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - illegal access interrupt enable for SDMMC1
    #[inline(always)]
    pub fn sdmmc2ie(&self) -> SDMMC2IE_R {
        SDMMC2IE_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - illegal access interrupt enable for FSMC registers
    #[inline(always)]
    pub fn fsmcie(&self) -> FSMCIE_R {
        FSMCIE_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - illegal access interrupt enable for OCTOSPI1 registers
    #[inline(always)]
    pub fn octospi1ie(&self) -> OCTOSPI1IE_R {
        OCTOSPI1IE_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - illegal access interrupt enable for OCTOSPI2 registers
    #[inline(always)]
    pub fn octospi2ie(&self) -> OCTOSPI2IE_R {
        OCTOSPI2IE_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - illegal access interrupt enable for RAMCFG
    #[inline(always)]
    pub fn ramcfgie(&self) -> RAMCFGIE_R {
        RAMCFGIE_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - illegal access interrupt enable for MDF1
    #[inline(always)]
    #[must_use]
    pub fn mdf1ie(&mut self) -> MDF1IE_W<0> {
        MDF1IE_W::new(self)
    }
    ///Bit 1 - illegal access interrupt enable for CORDIC
    #[inline(always)]
    #[must_use]
    pub fn cordicie(&mut self) -> CORDICIE_W<1> {
        CORDICIE_W::new(self)
    }
    ///Bit 2 - illegal access interrupt enable for FMAC
    #[inline(always)]
    #[must_use]
    pub fn fmacie(&mut self) -> FMACIE_W<2> {
        FMACIE_W::new(self)
    }
    ///Bit 3 - illegal access interrupt enable for CRC
    #[inline(always)]
    #[must_use]
    pub fn crcie(&mut self) -> CRCIE_W<3> {
        CRCIE_W::new(self)
    }
    ///Bit 4 - illegal access interrupt enable for TSC
    #[inline(always)]
    #[must_use]
    pub fn tscie(&mut self) -> TSCIE_W<4> {
        TSCIE_W::new(self)
    }
    ///Bit 5 - illegal access interrupt enable for register of DMA2D
    #[inline(always)]
    #[must_use]
    pub fn dma2die(&mut self) -> DMA2DIE_W<5> {
        DMA2DIE_W::new(self)
    }
    ///Bit 6 - illegal access interrupt enable for ICACHE registers
    #[inline(always)]
    #[must_use]
    pub fn icacheie(&mut self) -> ICACHEIE_W<6> {
        ICACHEIE_W::new(self)
    }
    ///Bit 7 - illegal access interrupt enable for DCACHE registers
    #[inline(always)]
    #[must_use]
    pub fn dcacheie(&mut self) -> DCACHEIE_W<7> {
        DCACHEIE_W::new(self)
    }
    ///Bit 8 - illegal access interrupt enable for ADC1
    #[inline(always)]
    #[must_use]
    pub fn adc1ie(&mut self) -> ADC1IE_W<8> {
        ADC1IE_W::new(self)
    }
    ///Bit 9 - illegal access interrupt enable for DCMI
    #[inline(always)]
    #[must_use]
    pub fn dcmiie(&mut self) -> DCMIIE_W<9> {
        DCMIIE_W::new(self)
    }
    ///Bit 10 - illegal access interrupt enable for OTG_FS
    #[inline(always)]
    #[must_use]
    pub fn otgfsie(&mut self) -> OTGFSIE_W<10> {
        OTGFSIE_W::new(self)
    }
    ///Bit 11 - illegal access interrupt enable for AES
    #[inline(always)]
    #[must_use]
    pub fn aesie(&mut self) -> AESIE_W<11> {
        AESIE_W::new(self)
    }
    ///Bit 12 - illegal access interrupt enable for HASH
    #[inline(always)]
    #[must_use]
    pub fn hashie(&mut self) -> HASHIE_W<12> {
        HASHIE_W::new(self)
    }
    ///Bit 13 - illegal access interrupt enable for RNG
    #[inline(always)]
    #[must_use]
    pub fn rngie(&mut self) -> RNGIE_W<13> {
        RNGIE_W::new(self)
    }
    ///Bit 14 - illegal access interrupt enable for PKA
    #[inline(always)]
    #[must_use]
    pub fn pkaie(&mut self) -> PKAIE_W<14> {
        PKAIE_W::new(self)
    }
    ///Bit 15 - illegal access interrupt enable for SAES
    #[inline(always)]
    #[must_use]
    pub fn saesie(&mut self) -> SAESIE_W<15> {
        SAESIE_W::new(self)
    }
    ///Bit 16 - illegal access interrupt enable for OCTOSPIM
    #[inline(always)]
    #[must_use]
    pub fn octospimie(&mut self) -> OCTOSPIMIE_W<16> {
        OCTOSPIMIE_W::new(self)
    }
    ///Bit 17 - illegal access interrupt enable for SDMMC2
    #[inline(always)]
    #[must_use]
    pub fn sdmmc1ie(&mut self) -> SDMMC1IE_W<17> {
        SDMMC1IE_W::new(self)
    }
    ///Bit 18 - illegal access interrupt enable for SDMMC1
    #[inline(always)]
    #[must_use]
    pub fn sdmmc2ie(&mut self) -> SDMMC2IE_W<18> {
        SDMMC2IE_W::new(self)
    }
    ///Bit 19 - illegal access interrupt enable for FSMC registers
    #[inline(always)]
    #[must_use]
    pub fn fsmcie(&mut self) -> FSMCIE_W<19> {
        FSMCIE_W::new(self)
    }
    ///Bit 20 - illegal access interrupt enable for OCTOSPI1 registers
    #[inline(always)]
    #[must_use]
    pub fn octospi1ie(&mut self) -> OCTOSPI1IE_W<20> {
        OCTOSPI1IE_W::new(self)
    }
    ///Bit 21 - illegal access interrupt enable for OCTOSPI2 registers
    #[inline(always)]
    #[must_use]
    pub fn octospi2ie(&mut self) -> OCTOSPI2IE_W<21> {
        OCTOSPI2IE_W::new(self)
    }
    ///Bit 22 - illegal access interrupt enable for RAMCFG
    #[inline(always)]
    #[must_use]
    pub fn ramcfgie(&mut self) -> RAMCFGIE_W<22> {
        RAMCFGIE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TZIC interrupt enable register 3
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ier3](index.html) module
pub struct IER3_SPEC;
impl crate::RegisterSpec for IER3_SPEC {
    type Ux = u32;
}
///`read()` method returns [ier3::R](R) reader structure
impl crate::Readable for IER3_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ier3::W](W) writer structure
impl crate::Writable for IER3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets IER3 to value 0
impl crate::Resettable for IER3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
