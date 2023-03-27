///Register `RCC_AHB1SMENR` reader
pub struct R(crate::R<RCC_AHB1SMENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_AHB1SMENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_AHB1SMENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_AHB1SMENR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RCC_AHB1SMENR` writer
pub struct W(crate::W<RCC_AHB1SMENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_AHB1SMENR_SPEC>;
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
impl From<crate::W<RCC_AHB1SMENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_AHB1SMENR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `GPDMA1SMEN` reader - GPDMA1 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type GPDMA1SMEN_R = crate::BitReader<bool>;
///Field `GPDMA1SMEN` writer - GPDMA1 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type GPDMA1SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB1SMENR_SPEC, bool, O>;
///Field `CORDICSMEN` reader - CORDIC clocks enable during Sleep and Stop modes Set and cleared by software during Sleep mode.
pub type CORDICSMEN_R = crate::BitReader<bool>;
///Field `CORDICSMEN` writer - CORDIC clocks enable during Sleep and Stop modes Set and cleared by software during Sleep mode.
pub type CORDICSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB1SMENR_SPEC, bool, O>;
///Field `FMACSMEN` reader - FMAC clocks enable during Sleep and Stop modes. Set and cleared by software.
pub type FMACSMEN_R = crate::BitReader<bool>;
///Field `FMACSMEN` writer - FMAC clocks enable during Sleep and Stop modes. Set and cleared by software.
pub type FMACSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB1SMENR_SPEC, bool, O>;
///Field `MDF1SMEN` reader - MDF1 clocks enable during Sleep and Stop modes. Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type MDF1SMEN_R = crate::BitReader<bool>;
///Field `MDF1SMEN` writer - MDF1 clocks enable during Sleep and Stop modes. Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type MDF1SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB1SMENR_SPEC, bool, O>;
///Field `FLASHSMEN` reader - FLASH clocks enable during Sleep and Stop modes Set and cleared by software.
pub type FLASHSMEN_R = crate::BitReader<bool>;
///Field `FLASHSMEN` writer - FLASH clocks enable during Sleep and Stop modes Set and cleared by software.
pub type FLASHSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB1SMENR_SPEC, bool, O>;
///Field `CRCSMEN` reader - CRC clocks enable during Sleep and Stop modes Set and cleared by software.
pub type CRCSMEN_R = crate::BitReader<bool>;
///Field `CRCSMEN` writer - CRC clocks enable during Sleep and Stop modes Set and cleared by software.
pub type CRCSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB1SMENR_SPEC, bool, O>;
///Field `TSCSMEN` reader - TSC clocks enable during Sleep and Stop modes Set and cleared by software.
pub type TSCSMEN_R = crate::BitReader<bool>;
///Field `TSCSMEN` writer - TSC clocks enable during Sleep and Stop modes Set and cleared by software.
pub type TSCSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB1SMENR_SPEC, bool, O>;
///Field `RAMCFGSMEN` reader - RAMCFG clocks enable during Sleep and Stop modes Set and cleared by software.
pub type RAMCFGSMEN_R = crate::BitReader<bool>;
///Field `RAMCFGSMEN` writer - RAMCFG clocks enable during Sleep and Stop modes Set and cleared by software.
pub type RAMCFGSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB1SMENR_SPEC, bool, O>;
///Field `DMA2DSMEN` reader - DMA2D clocks enable during Sleep and Stop modes Set and cleared by software.
pub type DMA2DSMEN_R = crate::BitReader<bool>;
///Field `DMA2DSMEN` writer - DMA2D clocks enable during Sleep and Stop modes Set and cleared by software.
pub type DMA2DSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB1SMENR_SPEC, bool, O>;
///Field `GTZC1SMEN` reader - GTZC1 clocks enable during Sleep and Stop modes Set and cleared by software.
pub type GTZC1SMEN_R = crate::BitReader<bool>;
///Field `GTZC1SMEN` writer - GTZC1 clocks enable during Sleep and Stop modes Set and cleared by software.
pub type GTZC1SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB1SMENR_SPEC, bool, O>;
///Field `BKPSRAMSMEN` reader - BKPSRAM clocks enable during Sleep and Stop modes Set and cleared by software
pub type BKPSRAMSMEN_R = crate::BitReader<bool>;
///Field `BKPSRAMSMEN` writer - BKPSRAM clocks enable during Sleep and Stop modes Set and cleared by software
pub type BKPSRAMSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB1SMENR_SPEC, bool, O>;
///Field `ICACHESMEN` reader - ICACHE clocks enable during Sleep and Stop modes Set and cleared by software.
pub type ICACHESMEN_R = crate::BitReader<bool>;
///Field `ICACHESMEN` writer - ICACHE clocks enable during Sleep and Stop modes Set and cleared by software.
pub type ICACHESMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB1SMENR_SPEC, bool, O>;
///Field `DCACHE1SMEN` reader - DCACHE1 clocks enable during Sleep and Stop modes Set and cleared by software.
pub type DCACHE1SMEN_R = crate::BitReader<bool>;
///Field `DCACHE1SMEN` writer - DCACHE1 clocks enable during Sleep and Stop modes Set and cleared by software.
pub type DCACHE1SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB1SMENR_SPEC, bool, O>;
///Field `SRAM1SMEN` reader - SRAM1 clocks enable during Sleep and Stop modes Set and cleared by software.
pub type SRAM1SMEN_R = crate::BitReader<bool>;
///Field `SRAM1SMEN` writer - SRAM1 clocks enable during Sleep and Stop modes Set and cleared by software.
pub type SRAM1SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB1SMENR_SPEC, bool, O>;
impl R {
    ///Bit 0 - GPDMA1 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn gpdma1smen(&self) -> GPDMA1SMEN_R {
        GPDMA1SMEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CORDIC clocks enable during Sleep and Stop modes Set and cleared by software during Sleep mode.
    #[inline(always)]
    pub fn cordicsmen(&self) -> CORDICSMEN_R {
        CORDICSMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - FMAC clocks enable during Sleep and Stop modes. Set and cleared by software.
    #[inline(always)]
    pub fn fmacsmen(&self) -> FMACSMEN_R {
        FMACSMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - MDF1 clocks enable during Sleep and Stop modes. Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn mdf1smen(&self) -> MDF1SMEN_R {
        MDF1SMEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 8 - FLASH clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn flashsmen(&self) -> FLASHSMEN_R {
        FLASHSMEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 12 - CRC clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn crcsmen(&self) -> CRCSMEN_R {
        CRCSMEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 16 - TSC clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn tscsmen(&self) -> TSCSMEN_R {
        TSCSMEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - RAMCFG clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn ramcfgsmen(&self) -> RAMCFGSMEN_R {
        RAMCFGSMEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - DMA2D clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn dma2dsmen(&self) -> DMA2DSMEN_R {
        DMA2DSMEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 24 - GTZC1 clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn gtzc1smen(&self) -> GTZC1SMEN_R {
        GTZC1SMEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 28 - BKPSRAM clocks enable during Sleep and Stop modes Set and cleared by software
    #[inline(always)]
    pub fn bkpsramsmen(&self) -> BKPSRAMSMEN_R {
        BKPSRAMSMEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - ICACHE clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn icachesmen(&self) -> ICACHESMEN_R {
        ICACHESMEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - DCACHE1 clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn dcache1smen(&self) -> DCACHE1SMEN_R {
        DCACHE1SMEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - SRAM1 clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn sram1smen(&self) -> SRAM1SMEN_R {
        SRAM1SMEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - GPDMA1 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    #[must_use]
    pub fn gpdma1smen(&mut self) -> GPDMA1SMEN_W<0> {
        GPDMA1SMEN_W::new(self)
    }
    ///Bit 1 - CORDIC clocks enable during Sleep and Stop modes Set and cleared by software during Sleep mode.
    #[inline(always)]
    #[must_use]
    pub fn cordicsmen(&mut self) -> CORDICSMEN_W<1> {
        CORDICSMEN_W::new(self)
    }
    ///Bit 2 - FMAC clocks enable during Sleep and Stop modes. Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn fmacsmen(&mut self) -> FMACSMEN_W<2> {
        FMACSMEN_W::new(self)
    }
    ///Bit 3 - MDF1 clocks enable during Sleep and Stop modes. Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    #[must_use]
    pub fn mdf1smen(&mut self) -> MDF1SMEN_W<3> {
        MDF1SMEN_W::new(self)
    }
    ///Bit 8 - FLASH clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn flashsmen(&mut self) -> FLASHSMEN_W<8> {
        FLASHSMEN_W::new(self)
    }
    ///Bit 12 - CRC clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn crcsmen(&mut self) -> CRCSMEN_W<12> {
        CRCSMEN_W::new(self)
    }
    ///Bit 16 - TSC clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn tscsmen(&mut self) -> TSCSMEN_W<16> {
        TSCSMEN_W::new(self)
    }
    ///Bit 17 - RAMCFG clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn ramcfgsmen(&mut self) -> RAMCFGSMEN_W<17> {
        RAMCFGSMEN_W::new(self)
    }
    ///Bit 18 - DMA2D clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn dma2dsmen(&mut self) -> DMA2DSMEN_W<18> {
        DMA2DSMEN_W::new(self)
    }
    ///Bit 24 - GTZC1 clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn gtzc1smen(&mut self) -> GTZC1SMEN_W<24> {
        GTZC1SMEN_W::new(self)
    }
    ///Bit 28 - BKPSRAM clocks enable during Sleep and Stop modes Set and cleared by software
    #[inline(always)]
    #[must_use]
    pub fn bkpsramsmen(&mut self) -> BKPSRAMSMEN_W<28> {
        BKPSRAMSMEN_W::new(self)
    }
    ///Bit 29 - ICACHE clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn icachesmen(&mut self) -> ICACHESMEN_W<29> {
        ICACHESMEN_W::new(self)
    }
    ///Bit 30 - DCACHE1 clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn dcache1smen(&mut self) -> DCACHE1SMEN_W<30> {
        DCACHE1SMEN_W::new(self)
    }
    ///Bit 31 - SRAM1 clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn sram1smen(&mut self) -> SRAM1SMEN_W<31> {
        SRAM1SMEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RCC AHB1 peripheral clocks enable in Sleep and Stop modes register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rcc_ahb1smenr](index.html) module
pub struct RCC_AHB1SMENR_SPEC;
impl crate::RegisterSpec for RCC_AHB1SMENR_SPEC {
    type Ux = u32;
}
///`read()` method returns [rcc_ahb1smenr::R](R) reader structure
impl crate::Readable for RCC_AHB1SMENR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rcc_ahb1smenr::W](W) writer structure
impl crate::Writable for RCC_AHB1SMENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RCC_AHB1SMENR to value 0xffff_ffff
impl crate::Resettable for RCC_AHB1SMENR_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
