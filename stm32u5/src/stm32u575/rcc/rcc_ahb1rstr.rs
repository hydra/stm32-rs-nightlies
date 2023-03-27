///Register `RCC_AHB1RSTR` reader
pub struct R(crate::R<RCC_AHB1RSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_AHB1RSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_AHB1RSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_AHB1RSTR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RCC_AHB1RSTR` writer
pub struct W(crate::W<RCC_AHB1RSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_AHB1RSTR_SPEC>;
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
impl From<crate::W<RCC_AHB1RSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_AHB1RSTR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `GPDMA1RST` reader - GPDMA1 reset Set and cleared by software.
pub type GPDMA1RST_R = crate::BitReader<bool>;
///Field `GPDMA1RST` writer - GPDMA1 reset Set and cleared by software.
pub type GPDMA1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB1RSTR_SPEC, bool, O>;
///Field `CORDICRST` reader - CORDIC reset Set and cleared by software.
pub type CORDICRST_R = crate::BitReader<bool>;
///Field `CORDICRST` writer - CORDIC reset Set and cleared by software.
pub type CORDICRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB1RSTR_SPEC, bool, O>;
///Field `FMACRST` reader - FMAC reset Set and cleared by software.
pub type FMACRST_R = crate::BitReader<bool>;
///Field `FMACRST` writer - FMAC reset Set and cleared by software.
pub type FMACRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB1RSTR_SPEC, bool, O>;
///Field `MDF1RST` reader - MDF1 reset Set and cleared by software.
pub type MDF1RST_R = crate::BitReader<bool>;
///Field `MDF1RST` writer - MDF1 reset Set and cleared by software.
pub type MDF1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB1RSTR_SPEC, bool, O>;
///Field `CRCRST` reader - CRC reset Set and cleared by software.
pub type CRCRST_R = crate::BitReader<bool>;
///Field `CRCRST` writer - CRC reset Set and cleared by software.
pub type CRCRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB1RSTR_SPEC, bool, O>;
///Field `TSCRST` reader - TSC reset Set and cleared by software.
pub type TSCRST_R = crate::BitReader<bool>;
///Field `TSCRST` writer - TSC reset Set and cleared by software.
pub type TSCRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB1RSTR_SPEC, bool, O>;
///Field `RAMCFGRST` reader - RAMCFG reset Set and cleared by software.
pub type RAMCFGRST_R = crate::BitReader<bool>;
///Field `RAMCFGRST` writer - RAMCFG reset Set and cleared by software.
pub type RAMCFGRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB1RSTR_SPEC, bool, O>;
///Field `DMA2DRST` reader - DMA2D reset Set and cleared by software.
pub type DMA2DRST_R = crate::BitReader<bool>;
///Field `DMA2DRST` writer - DMA2D reset Set and cleared by software.
pub type DMA2DRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB1RSTR_SPEC, bool, O>;
impl R {
    ///Bit 0 - GPDMA1 reset Set and cleared by software.
    #[inline(always)]
    pub fn gpdma1rst(&self) -> GPDMA1RST_R {
        GPDMA1RST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CORDIC reset Set and cleared by software.
    #[inline(always)]
    pub fn cordicrst(&self) -> CORDICRST_R {
        CORDICRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - FMAC reset Set and cleared by software.
    #[inline(always)]
    pub fn fmacrst(&self) -> FMACRST_R {
        FMACRST_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - MDF1 reset Set and cleared by software.
    #[inline(always)]
    pub fn mdf1rst(&self) -> MDF1RST_R {
        MDF1RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 12 - CRC reset Set and cleared by software.
    #[inline(always)]
    pub fn crcrst(&self) -> CRCRST_R {
        CRCRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 16 - TSC reset Set and cleared by software.
    #[inline(always)]
    pub fn tscrst(&self) -> TSCRST_R {
        TSCRST_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - RAMCFG reset Set and cleared by software.
    #[inline(always)]
    pub fn ramcfgrst(&self) -> RAMCFGRST_R {
        RAMCFGRST_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - DMA2D reset Set and cleared by software.
    #[inline(always)]
    pub fn dma2drst(&self) -> DMA2DRST_R {
        DMA2DRST_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - GPDMA1 reset Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn gpdma1rst(&mut self) -> GPDMA1RST_W<0> {
        GPDMA1RST_W::new(self)
    }
    ///Bit 1 - CORDIC reset Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn cordicrst(&mut self) -> CORDICRST_W<1> {
        CORDICRST_W::new(self)
    }
    ///Bit 2 - FMAC reset Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn fmacrst(&mut self) -> FMACRST_W<2> {
        FMACRST_W::new(self)
    }
    ///Bit 3 - MDF1 reset Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn mdf1rst(&mut self) -> MDF1RST_W<3> {
        MDF1RST_W::new(self)
    }
    ///Bit 12 - CRC reset Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn crcrst(&mut self) -> CRCRST_W<12> {
        CRCRST_W::new(self)
    }
    ///Bit 16 - TSC reset Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn tscrst(&mut self) -> TSCRST_W<16> {
        TSCRST_W::new(self)
    }
    ///Bit 17 - RAMCFG reset Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn ramcfgrst(&mut self) -> RAMCFGRST_W<17> {
        RAMCFGRST_W::new(self)
    }
    ///Bit 18 - DMA2D reset Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn dma2drst(&mut self) -> DMA2DRST_W<18> {
        DMA2DRST_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RCC AHB1 peripheral reset register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rcc_ahb1rstr](index.html) module
pub struct RCC_AHB1RSTR_SPEC;
impl crate::RegisterSpec for RCC_AHB1RSTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [rcc_ahb1rstr::R](R) reader structure
impl crate::Readable for RCC_AHB1RSTR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rcc_ahb1rstr::W](W) writer structure
impl crate::Writable for RCC_AHB1RSTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RCC_AHB1RSTR to value 0
impl crate::Resettable for RCC_AHB1RSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
