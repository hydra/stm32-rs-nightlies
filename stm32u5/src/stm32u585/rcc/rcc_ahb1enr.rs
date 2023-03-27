///Register `RCC_AHB1ENR` reader
pub struct R(crate::R<RCC_AHB1ENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_AHB1ENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_AHB1ENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_AHB1ENR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RCC_AHB1ENR` writer
pub struct W(crate::W<RCC_AHB1ENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_AHB1ENR_SPEC>;
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
impl From<crate::W<RCC_AHB1ENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_AHB1ENR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `GPDMA1EN` reader - GPDMA1 clock enable Set and cleared by software.
pub type GPDMA1EN_R = crate::BitReader<bool>;
///Field `GPDMA1EN` writer - GPDMA1 clock enable Set and cleared by software.
pub type GPDMA1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB1ENR_SPEC, bool, O>;
///Field `CORDICEN` reader - CORDIC clock enable Set and cleared by software.
pub type CORDICEN_R = crate::BitReader<bool>;
///Field `CORDICEN` writer - CORDIC clock enable Set and cleared by software.
pub type CORDICEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB1ENR_SPEC, bool, O>;
///Field `FMACEN` reader - FMAC clock enable Set and reset by software.
pub type FMACEN_R = crate::BitReader<bool>;
///Field `FMACEN` writer - FMAC clock enable Set and reset by software.
pub type FMACEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB1ENR_SPEC, bool, O>;
///Field `MDF1EN` reader - MDF1 clock enable Set and reset by software.
pub type MDF1EN_R = crate::BitReader<bool>;
///Field `MDF1EN` writer - MDF1 clock enable Set and reset by software.
pub type MDF1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB1ENR_SPEC, bool, O>;
///Field `FLASHEN` reader - FLASH clock enable Set and cleared by software. This bit can be disabled only when the Flash memory is in power down mode.
pub type FLASHEN_R = crate::BitReader<bool>;
///Field `FLASHEN` writer - FLASH clock enable Set and cleared by software. This bit can be disabled only when the Flash memory is in power down mode.
pub type FLASHEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB1ENR_SPEC, bool, O>;
///Field `CRCEN` reader - CRC clock enable Set and cleared by software.
pub type CRCEN_R = crate::BitReader<bool>;
///Field `CRCEN` writer - CRC clock enable Set and cleared by software.
pub type CRCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB1ENR_SPEC, bool, O>;
///Field `TSCEN` reader - Touch sensing controller clock enable Set and cleared by software.
pub type TSCEN_R = crate::BitReader<bool>;
///Field `TSCEN` writer - Touch sensing controller clock enable Set and cleared by software.
pub type TSCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB1ENR_SPEC, bool, O>;
///Field `RAMCFGEN` reader - RAMCFG clock enable Set and cleared by software.
pub type RAMCFGEN_R = crate::BitReader<bool>;
///Field `RAMCFGEN` writer - RAMCFG clock enable Set and cleared by software.
pub type RAMCFGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB1ENR_SPEC, bool, O>;
///Field `DMA2DEN` reader - DMA2D clock enable Set and cleared by software.
pub type DMA2DEN_R = crate::BitReader<bool>;
///Field `DMA2DEN` writer - DMA2D clock enable Set and cleared by software.
pub type DMA2DEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB1ENR_SPEC, bool, O>;
///Field `GTZC1EN` reader - GTZC1 clock enable Set and reset by software.
pub type GTZC1EN_R = crate::BitReader<bool>;
///Field `GTZC1EN` writer - GTZC1 clock enable Set and reset by software.
pub type GTZC1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB1ENR_SPEC, bool, O>;
///Field `BKPSRAMEN` reader - BKPSRAM clock enable Set and reset by software.
pub type BKPSRAMEN_R = crate::BitReader<bool>;
///Field `BKPSRAMEN` writer - BKPSRAM clock enable Set and reset by software.
pub type BKPSRAMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB1ENR_SPEC, bool, O>;
///Field `DCACHE1EN` reader - DCACHE1 clock enable Set and reset by software. Note: DCACHE1 clock must be enabled when external memories are accessed through OCTOSPI1, OCTOSPI2 or FSMC, even if the DCACHE1 is bypassed.
pub type DCACHE1EN_R = crate::BitReader<bool>;
///Field `DCACHE1EN` writer - DCACHE1 clock enable Set and reset by software. Note: DCACHE1 clock must be enabled when external memories are accessed through OCTOSPI1, OCTOSPI2 or FSMC, even if the DCACHE1 is bypassed.
pub type DCACHE1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB1ENR_SPEC, bool, O>;
///Field `SRAM1EN` reader - SRAM1 clock enable Set and reset by software.
pub type SRAM1EN_R = crate::BitReader<bool>;
///Field `SRAM1EN` writer - SRAM1 clock enable Set and reset by software.
pub type SRAM1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB1ENR_SPEC, bool, O>;
impl R {
    ///Bit 0 - GPDMA1 clock enable Set and cleared by software.
    #[inline(always)]
    pub fn gpdma1en(&self) -> GPDMA1EN_R {
        GPDMA1EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CORDIC clock enable Set and cleared by software.
    #[inline(always)]
    pub fn cordicen(&self) -> CORDICEN_R {
        CORDICEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - FMAC clock enable Set and reset by software.
    #[inline(always)]
    pub fn fmacen(&self) -> FMACEN_R {
        FMACEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - MDF1 clock enable Set and reset by software.
    #[inline(always)]
    pub fn mdf1en(&self) -> MDF1EN_R {
        MDF1EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 8 - FLASH clock enable Set and cleared by software. This bit can be disabled only when the Flash memory is in power down mode.
    #[inline(always)]
    pub fn flashen(&self) -> FLASHEN_R {
        FLASHEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 12 - CRC clock enable Set and cleared by software.
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 16 - Touch sensing controller clock enable Set and cleared by software.
    #[inline(always)]
    pub fn tscen(&self) -> TSCEN_R {
        TSCEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - RAMCFG clock enable Set and cleared by software.
    #[inline(always)]
    pub fn ramcfgen(&self) -> RAMCFGEN_R {
        RAMCFGEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - DMA2D clock enable Set and cleared by software.
    #[inline(always)]
    pub fn dma2den(&self) -> DMA2DEN_R {
        DMA2DEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 24 - GTZC1 clock enable Set and reset by software.
    #[inline(always)]
    pub fn gtzc1en(&self) -> GTZC1EN_R {
        GTZC1EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 28 - BKPSRAM clock enable Set and reset by software.
    #[inline(always)]
    pub fn bkpsramen(&self) -> BKPSRAMEN_R {
        BKPSRAMEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 30 - DCACHE1 clock enable Set and reset by software. Note: DCACHE1 clock must be enabled when external memories are accessed through OCTOSPI1, OCTOSPI2 or FSMC, even if the DCACHE1 is bypassed.
    #[inline(always)]
    pub fn dcache1en(&self) -> DCACHE1EN_R {
        DCACHE1EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - SRAM1 clock enable Set and reset by software.
    #[inline(always)]
    pub fn sram1en(&self) -> SRAM1EN_R {
        SRAM1EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - GPDMA1 clock enable Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn gpdma1en(&mut self) -> GPDMA1EN_W<0> {
        GPDMA1EN_W::new(self)
    }
    ///Bit 1 - CORDIC clock enable Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn cordicen(&mut self) -> CORDICEN_W<1> {
        CORDICEN_W::new(self)
    }
    ///Bit 2 - FMAC clock enable Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn fmacen(&mut self) -> FMACEN_W<2> {
        FMACEN_W::new(self)
    }
    ///Bit 3 - MDF1 clock enable Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn mdf1en(&mut self) -> MDF1EN_W<3> {
        MDF1EN_W::new(self)
    }
    ///Bit 8 - FLASH clock enable Set and cleared by software. This bit can be disabled only when the Flash memory is in power down mode.
    #[inline(always)]
    #[must_use]
    pub fn flashen(&mut self) -> FLASHEN_W<8> {
        FLASHEN_W::new(self)
    }
    ///Bit 12 - CRC clock enable Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn crcen(&mut self) -> CRCEN_W<12> {
        CRCEN_W::new(self)
    }
    ///Bit 16 - Touch sensing controller clock enable Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn tscen(&mut self) -> TSCEN_W<16> {
        TSCEN_W::new(self)
    }
    ///Bit 17 - RAMCFG clock enable Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn ramcfgen(&mut self) -> RAMCFGEN_W<17> {
        RAMCFGEN_W::new(self)
    }
    ///Bit 18 - DMA2D clock enable Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn dma2den(&mut self) -> DMA2DEN_W<18> {
        DMA2DEN_W::new(self)
    }
    ///Bit 24 - GTZC1 clock enable Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn gtzc1en(&mut self) -> GTZC1EN_W<24> {
        GTZC1EN_W::new(self)
    }
    ///Bit 28 - BKPSRAM clock enable Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn bkpsramen(&mut self) -> BKPSRAMEN_W<28> {
        BKPSRAMEN_W::new(self)
    }
    ///Bit 30 - DCACHE1 clock enable Set and reset by software. Note: DCACHE1 clock must be enabled when external memories are accessed through OCTOSPI1, OCTOSPI2 or FSMC, even if the DCACHE1 is bypassed.
    #[inline(always)]
    #[must_use]
    pub fn dcache1en(&mut self) -> DCACHE1EN_W<30> {
        DCACHE1EN_W::new(self)
    }
    ///Bit 31 - SRAM1 clock enable Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn sram1en(&mut self) -> SRAM1EN_W<31> {
        SRAM1EN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RCC AHB1 peripheral clock enable register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rcc_ahb1enr](index.html) module
pub struct RCC_AHB1ENR_SPEC;
impl crate::RegisterSpec for RCC_AHB1ENR_SPEC {
    type Ux = u32;
}
///`read()` method returns [rcc_ahb1enr::R](R) reader structure
impl crate::Readable for RCC_AHB1ENR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rcc_ahb1enr::W](W) writer structure
impl crate::Writable for RCC_AHB1ENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RCC_AHB1ENR to value 0xd000_0100
impl crate::Resettable for RCC_AHB1ENR_SPEC {
    const RESET_VALUE: Self::Ux = 0xd000_0100;
}
