///Register `RCC_APB1RSTR2` reader
pub struct R(crate::R<RCC_APB1RSTR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_APB1RSTR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_APB1RSTR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_APB1RSTR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RCC_APB1RSTR2` writer
pub struct W(crate::W<RCC_APB1RSTR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_APB1RSTR2_SPEC>;
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
impl From<crate::W<RCC_APB1RSTR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_APB1RSTR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `I2C4RST` reader - I2C4 reset Set and cleared by software
pub type I2C4RST_R = crate::BitReader<bool>;
///Field `I2C4RST` writer - I2C4 reset Set and cleared by software
pub type I2C4RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB1RSTR2_SPEC, bool, O>;
///Field `LPTIM2RST` reader - LPTIM2 reset Set and cleared by software.
pub type LPTIM2RST_R = crate::BitReader<bool>;
///Field `LPTIM2RST` writer - LPTIM2 reset Set and cleared by software.
pub type LPTIM2RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB1RSTR2_SPEC, bool, O>;
///Field `FDCAN1RST` reader - FDCAN1 reset Set and cleared by software.
pub type FDCAN1RST_R = crate::BitReader<bool>;
///Field `FDCAN1RST` writer - FDCAN1 reset Set and cleared by software.
pub type FDCAN1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB1RSTR2_SPEC, bool, O>;
///Field `UCPD1RST` reader - UCPD1 reset Set and cleared by software.
pub type UCPD1RST_R = crate::BitReader<bool>;
///Field `UCPD1RST` writer - UCPD1 reset Set and cleared by software.
pub type UCPD1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB1RSTR2_SPEC, bool, O>;
impl R {
    ///Bit 1 - I2C4 reset Set and cleared by software
    #[inline(always)]
    pub fn i2c4rst(&self) -> I2C4RST_R {
        I2C4RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 5 - LPTIM2 reset Set and cleared by software.
    #[inline(always)]
    pub fn lptim2rst(&self) -> LPTIM2RST_R {
        LPTIM2RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 9 - FDCAN1 reset Set and cleared by software.
    #[inline(always)]
    pub fn fdcan1rst(&self) -> FDCAN1RST_R {
        FDCAN1RST_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 23 - UCPD1 reset Set and cleared by software.
    #[inline(always)]
    pub fn ucpd1rst(&self) -> UCPD1RST_R {
        UCPD1RST_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    ///Bit 1 - I2C4 reset Set and cleared by software
    #[inline(always)]
    #[must_use]
    pub fn i2c4rst(&mut self) -> I2C4RST_W<1> {
        I2C4RST_W::new(self)
    }
    ///Bit 5 - LPTIM2 reset Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn lptim2rst(&mut self) -> LPTIM2RST_W<5> {
        LPTIM2RST_W::new(self)
    }
    ///Bit 9 - FDCAN1 reset Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn fdcan1rst(&mut self) -> FDCAN1RST_W<9> {
        FDCAN1RST_W::new(self)
    }
    ///Bit 23 - UCPD1 reset Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn ucpd1rst(&mut self) -> UCPD1RST_W<23> {
        UCPD1RST_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RCC APB1 peripheral reset register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rcc_apb1rstr2](index.html) module
pub struct RCC_APB1RSTR2_SPEC;
impl crate::RegisterSpec for RCC_APB1RSTR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [rcc_apb1rstr2::R](R) reader structure
impl crate::Readable for RCC_APB1RSTR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rcc_apb1rstr2::W](W) writer structure
impl crate::Writable for RCC_APB1RSTR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RCC_APB1RSTR2 to value 0
impl crate::Resettable for RCC_APB1RSTR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
