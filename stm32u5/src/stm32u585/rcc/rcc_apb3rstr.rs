///Register `RCC_APB3RSTR` reader
pub struct R(crate::R<RCC_APB3RSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_APB3RSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_APB3RSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_APB3RSTR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RCC_APB3RSTR` writer
pub struct W(crate::W<RCC_APB3RSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_APB3RSTR_SPEC>;
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
impl From<crate::W<RCC_APB3RSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_APB3RSTR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SYSCFGRST` reader - SYSCFG reset Set and cleared by software.
pub type SYSCFGRST_R = crate::BitReader<bool>;
///Field `SYSCFGRST` writer - SYSCFG reset Set and cleared by software.
pub type SYSCFGRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB3RSTR_SPEC, bool, O>;
///Field `SPI3RST` reader - SPI3 reset Set and cleared by software.
pub type SPI3RST_R = crate::BitReader<bool>;
///Field `SPI3RST` writer - SPI3 reset Set and cleared by software.
pub type SPI3RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB3RSTR_SPEC, bool, O>;
///Field `LPUART1RST` reader - LPUART1 reset Set and cleared by software.
pub type LPUART1RST_R = crate::BitReader<bool>;
///Field `LPUART1RST` writer - LPUART1 reset Set and cleared by software.
pub type LPUART1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB3RSTR_SPEC, bool, O>;
///Field `I2C3RST` reader - I2C3 reset Set and cleared by software.
pub type I2C3RST_R = crate::BitReader<bool>;
///Field `I2C3RST` writer - I2C3 reset Set and cleared by software.
pub type I2C3RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB3RSTR_SPEC, bool, O>;
///Field `LPTIM1RST` reader - LPTIM1 reset Set and cleared by software.
pub type LPTIM1RST_R = crate::BitReader<bool>;
///Field `LPTIM1RST` writer - LPTIM1 reset Set and cleared by software.
pub type LPTIM1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB3RSTR_SPEC, bool, O>;
///Field `LPTIM3RST` reader - LPTIM3 reset Set and cleared by software.
pub type LPTIM3RST_R = crate::BitReader<bool>;
///Field `LPTIM3RST` writer - LPTIM3 reset Set and cleared by software.
pub type LPTIM3RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB3RSTR_SPEC, bool, O>;
///Field `LPTIM4RST` reader - LPTIM4 reset Set and cleared by software.
pub type LPTIM4RST_R = crate::BitReader<bool>;
///Field `LPTIM4RST` writer - LPTIM4 reset Set and cleared by software.
pub type LPTIM4RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB3RSTR_SPEC, bool, O>;
///Field `OPAMPRST` reader - OPAMP reset Set and cleared by software.
pub type OPAMPRST_R = crate::BitReader<bool>;
///Field `OPAMPRST` writer - OPAMP reset Set and cleared by software.
pub type OPAMPRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB3RSTR_SPEC, bool, O>;
///Field `COMPRST` reader - COMP reset Set and cleared by software.
pub type COMPRST_R = crate::BitReader<bool>;
///Field `COMPRST` writer - COMP reset Set and cleared by software.
pub type COMPRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB3RSTR_SPEC, bool, O>;
///Field `VREFRST` reader - VREFBUF reset Set and cleared by software.
pub type VREFRST_R = crate::BitReader<bool>;
///Field `VREFRST` writer - VREFBUF reset Set and cleared by software.
pub type VREFRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB3RSTR_SPEC, bool, O>;
impl R {
    ///Bit 1 - SYSCFG reset Set and cleared by software.
    #[inline(always)]
    pub fn syscfgrst(&self) -> SYSCFGRST_R {
        SYSCFGRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 5 - SPI3 reset Set and cleared by software.
    #[inline(always)]
    pub fn spi3rst(&self) -> SPI3RST_R {
        SPI3RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - LPUART1 reset Set and cleared by software.
    #[inline(always)]
    pub fn lpuart1rst(&self) -> LPUART1RST_R {
        LPUART1RST_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - I2C3 reset Set and cleared by software.
    #[inline(always)]
    pub fn i2c3rst(&self) -> I2C3RST_R {
        I2C3RST_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 11 - LPTIM1 reset Set and cleared by software.
    #[inline(always)]
    pub fn lptim1rst(&self) -> LPTIM1RST_R {
        LPTIM1RST_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - LPTIM3 reset Set and cleared by software.
    #[inline(always)]
    pub fn lptim3rst(&self) -> LPTIM3RST_R {
        LPTIM3RST_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - LPTIM4 reset Set and cleared by software.
    #[inline(always)]
    pub fn lptim4rst(&self) -> LPTIM4RST_R {
        LPTIM4RST_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - OPAMP reset Set and cleared by software.
    #[inline(always)]
    pub fn opamprst(&self) -> OPAMPRST_R {
        OPAMPRST_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - COMP reset Set and cleared by software.
    #[inline(always)]
    pub fn comprst(&self) -> COMPRST_R {
        COMPRST_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 20 - VREFBUF reset Set and cleared by software.
    #[inline(always)]
    pub fn vrefrst(&self) -> VREFRST_R {
        VREFRST_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    ///Bit 1 - SYSCFG reset Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn syscfgrst(&mut self) -> SYSCFGRST_W<1> {
        SYSCFGRST_W::new(self)
    }
    ///Bit 5 - SPI3 reset Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn spi3rst(&mut self) -> SPI3RST_W<5> {
        SPI3RST_W::new(self)
    }
    ///Bit 6 - LPUART1 reset Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn lpuart1rst(&mut self) -> LPUART1RST_W<6> {
        LPUART1RST_W::new(self)
    }
    ///Bit 7 - I2C3 reset Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn i2c3rst(&mut self) -> I2C3RST_W<7> {
        I2C3RST_W::new(self)
    }
    ///Bit 11 - LPTIM1 reset Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn lptim1rst(&mut self) -> LPTIM1RST_W<11> {
        LPTIM1RST_W::new(self)
    }
    ///Bit 12 - LPTIM3 reset Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn lptim3rst(&mut self) -> LPTIM3RST_W<12> {
        LPTIM3RST_W::new(self)
    }
    ///Bit 13 - LPTIM4 reset Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn lptim4rst(&mut self) -> LPTIM4RST_W<13> {
        LPTIM4RST_W::new(self)
    }
    ///Bit 14 - OPAMP reset Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn opamprst(&mut self) -> OPAMPRST_W<14> {
        OPAMPRST_W::new(self)
    }
    ///Bit 15 - COMP reset Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn comprst(&mut self) -> COMPRST_W<15> {
        COMPRST_W::new(self)
    }
    ///Bit 20 - VREFBUF reset Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn vrefrst(&mut self) -> VREFRST_W<20> {
        VREFRST_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RCC APB3 peripheral reset register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rcc_apb3rstr](index.html) module
pub struct RCC_APB3RSTR_SPEC;
impl crate::RegisterSpec for RCC_APB3RSTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [rcc_apb3rstr::R](R) reader structure
impl crate::Readable for RCC_APB3RSTR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rcc_apb3rstr::W](W) writer structure
impl crate::Writable for RCC_APB3RSTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RCC_APB3RSTR to value 0
impl crate::Resettable for RCC_APB3RSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
