///Register `AHB1SMENR` reader
pub struct R(crate::R<AHB1SMENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB1SMENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB1SMENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB1SMENR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `AHB1SMENR` writer
pub struct W(crate::W<AHB1SMENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB1SMENR_SPEC>;
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
impl From<crate::W<AHB1SMENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB1SMENR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DMA1SMEN` reader - DMA1 clocks enable during Sleep and Stop modes
pub type DMA1SMEN_R = crate::BitReader<bool>;
///Field `DMA1SMEN` writer - DMA1 clocks enable during Sleep and Stop modes
pub type DMA1SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1SMENR_SPEC, bool, O>;
///Field `DMA2SMEN` reader - DMA2 clocks enable during Sleep and Stop modes
pub type DMA2SMEN_R = crate::BitReader<bool>;
///Field `DMA2SMEN` writer - DMA2 clocks enable during Sleep and Stop modes
pub type DMA2SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1SMENR_SPEC, bool, O>;
///Field `FLASHSMEN` reader - Flash memory interface clocks enable during Sleep and Stop modes
pub type FLASHSMEN_R = crate::BitReader<bool>;
///Field `FLASHSMEN` writer - Flash memory interface clocks enable during Sleep and Stop modes
pub type FLASHSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1SMENR_SPEC, bool, O>;
///Field `SRAM1SMEN` reader - SRAM1 interface clocks enable during Sleep and Stop modes
pub type SRAM1SMEN_R = crate::BitReader<bool>;
///Field `SRAM1SMEN` writer - SRAM1 interface clocks enable during Sleep and Stop modes
pub type SRAM1SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1SMENR_SPEC, bool, O>;
///Field `CRCSMEN` reader - CRCSMEN
pub type CRCSMEN_R = crate::BitReader<bool>;
///Field `CRCSMEN` writer - CRCSMEN
pub type CRCSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1SMENR_SPEC, bool, O>;
///Field `TSCSMEN` reader - Touch Sensing Controller clocks enable during Sleep and Stop modes
pub type TSCSMEN_R = crate::BitReader<bool>;
///Field `TSCSMEN` writer - Touch Sensing Controller clocks enable during Sleep and Stop modes
pub type TSCSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1SMENR_SPEC, bool, O>;
impl R {
    ///Bit 0 - DMA1 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn dma1smen(&self) -> DMA1SMEN_R {
        DMA1SMEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DMA2 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn dma2smen(&self) -> DMA2SMEN_R {
        DMA2SMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 8 - Flash memory interface clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn flashsmen(&self) -> FLASHSMEN_R {
        FLASHSMEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - SRAM1 interface clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn sram1smen(&self) -> SRAM1SMEN_R {
        SRAM1SMEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 12 - CRCSMEN
    #[inline(always)]
    pub fn crcsmen(&self) -> CRCSMEN_R {
        CRCSMEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 16 - Touch Sensing Controller clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn tscsmen(&self) -> TSCSMEN_R {
        TSCSMEN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - DMA1 clocks enable during Sleep and Stop modes
    #[inline(always)]
    #[must_use]
    pub fn dma1smen(&mut self) -> DMA1SMEN_W<0> {
        DMA1SMEN_W::new(self)
    }
    ///Bit 1 - DMA2 clocks enable during Sleep and Stop modes
    #[inline(always)]
    #[must_use]
    pub fn dma2smen(&mut self) -> DMA2SMEN_W<1> {
        DMA2SMEN_W::new(self)
    }
    ///Bit 8 - Flash memory interface clocks enable during Sleep and Stop modes
    #[inline(always)]
    #[must_use]
    pub fn flashsmen(&mut self) -> FLASHSMEN_W<8> {
        FLASHSMEN_W::new(self)
    }
    ///Bit 9 - SRAM1 interface clocks enable during Sleep and Stop modes
    #[inline(always)]
    #[must_use]
    pub fn sram1smen(&mut self) -> SRAM1SMEN_W<9> {
        SRAM1SMEN_W::new(self)
    }
    ///Bit 12 - CRCSMEN
    #[inline(always)]
    #[must_use]
    pub fn crcsmen(&mut self) -> CRCSMEN_W<12> {
        CRCSMEN_W::new(self)
    }
    ///Bit 16 - Touch Sensing Controller clocks enable during Sleep and Stop modes
    #[inline(always)]
    #[must_use]
    pub fn tscsmen(&mut self) -> TSCSMEN_W<16> {
        TSCSMEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///AHB1 peripheral clocks enable in Sleep and Stop modes register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ahb1smenr](index.html) module
pub struct AHB1SMENR_SPEC;
impl crate::RegisterSpec for AHB1SMENR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ahb1smenr::R](R) reader structure
impl crate::Readable for AHB1SMENR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ahb1smenr::W](W) writer structure
impl crate::Writable for AHB1SMENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets AHB1SMENR to value 0x0001_1303
impl crate::Resettable for AHB1SMENR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_1303;
}
