///Register `RCC_MC_AHB2ENSETR` reader
pub struct R(crate::R<RCC_MC_AHB2ENSETR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_MC_AHB2ENSETR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_MC_AHB2ENSETR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_MC_AHB2ENSETR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RCC_MC_AHB2ENSETR` writer
pub struct W(crate::W<RCC_MC_AHB2ENSETR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_MC_AHB2ENSETR_SPEC>;
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
impl From<crate::W<RCC_MC_AHB2ENSETR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_MC_AHB2ENSETR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DMA1EN` reader - DMA1EN
pub type DMA1EN_R = crate::BitReader<bool>;
///Field `DMA1EN` writer - DMA1EN
pub type DMA1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_AHB2ENSETR_SPEC, bool, O>;
///Field `DMA2EN` reader - DMA2EN
pub type DMA2EN_R = crate::BitReader<bool>;
///Field `DMA2EN` writer - DMA2EN
pub type DMA2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_AHB2ENSETR_SPEC, bool, O>;
///Field `DMAMUXEN` reader - DMAMUXEN
pub type DMAMUXEN_R = crate::BitReader<bool>;
///Field `DMAMUXEN` writer - DMAMUXEN
pub type DMAMUXEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_AHB2ENSETR_SPEC, bool, O>;
///Field `ADC12EN` reader - ADC12EN
pub type ADC12EN_R = crate::BitReader<bool>;
///Field `ADC12EN` writer - ADC12EN
pub type ADC12EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_AHB2ENSETR_SPEC, bool, O>;
///Field `USBOEN` reader - USBOEN
pub type USBOEN_R = crate::BitReader<bool>;
///Field `USBOEN` writer - USBOEN
pub type USBOEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_AHB2ENSETR_SPEC, bool, O>;
///Field `SDMMC3EN` reader - SDMMC3EN
pub type SDMMC3EN_R = crate::BitReader<bool>;
///Field `SDMMC3EN` writer - SDMMC3EN
pub type SDMMC3EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_AHB2ENSETR_SPEC, bool, O>;
impl R {
    ///Bit 0 - DMA1EN
    #[inline(always)]
    pub fn dma1en(&self) -> DMA1EN_R {
        DMA1EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DMA2EN
    #[inline(always)]
    pub fn dma2en(&self) -> DMA2EN_R {
        DMA2EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - DMAMUXEN
    #[inline(always)]
    pub fn dmamuxen(&self) -> DMAMUXEN_R {
        DMAMUXEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 5 - ADC12EN
    #[inline(always)]
    pub fn adc12en(&self) -> ADC12EN_R {
        ADC12EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - USBOEN
    #[inline(always)]
    pub fn usboen(&self) -> USBOEN_R {
        USBOEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 16 - SDMMC3EN
    #[inline(always)]
    pub fn sdmmc3en(&self) -> SDMMC3EN_R {
        SDMMC3EN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - DMA1EN
    #[inline(always)]
    #[must_use]
    pub fn dma1en(&mut self) -> DMA1EN_W<0> {
        DMA1EN_W::new(self)
    }
    ///Bit 1 - DMA2EN
    #[inline(always)]
    #[must_use]
    pub fn dma2en(&mut self) -> DMA2EN_W<1> {
        DMA2EN_W::new(self)
    }
    ///Bit 2 - DMAMUXEN
    #[inline(always)]
    #[must_use]
    pub fn dmamuxen(&mut self) -> DMAMUXEN_W<2> {
        DMAMUXEN_W::new(self)
    }
    ///Bit 5 - ADC12EN
    #[inline(always)]
    #[must_use]
    pub fn adc12en(&mut self) -> ADC12EN_W<5> {
        ADC12EN_W::new(self)
    }
    ///Bit 8 - USBOEN
    #[inline(always)]
    #[must_use]
    pub fn usboen(&mut self) -> USBOEN_W<8> {
        USBOEN_W::new(self)
    }
    ///Bit 16 - SDMMC3EN
    #[inline(always)]
    #[must_use]
    pub fn sdmmc3en(&mut self) -> SDMMC3EN_W<16> {
        SDMMC3EN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///This register is used to set the peripheral clock enable bit
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rcc_mc_ahb2ensetr](index.html) module
pub struct RCC_MC_AHB2ENSETR_SPEC;
impl crate::RegisterSpec for RCC_MC_AHB2ENSETR_SPEC {
    type Ux = u32;
}
///`read()` method returns [rcc_mc_ahb2ensetr::R](R) reader structure
impl crate::Readable for RCC_MC_AHB2ENSETR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rcc_mc_ahb2ensetr::W](W) writer structure
impl crate::Writable for RCC_MC_AHB2ENSETR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RCC_MC_AHB2ENSETR to value 0
impl crate::Resettable for RCC_MC_AHB2ENSETR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
