///Register `APBRSTR1` reader
pub struct R(crate::R<APBRSTR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APBRSTR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APBRSTR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APBRSTR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `APBRSTR1` writer
pub struct W(crate::W<APBRSTR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APBRSTR1_SPEC>;
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
impl From<crate::W<APBRSTR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APBRSTR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TIM2RST` reader - TIM2 timer reset
pub type TIM2RST_R = crate::BitReader<bool>;
///Field `TIM2RST` writer - TIM2 timer reset
pub type TIM2RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBRSTR1_SPEC, bool, O>;
///Field `TIM3RST` reader - TIM3 timer reset
pub type TIM3RST_R = crate::BitReader<bool>;
///Field `TIM3RST` writer - TIM3 timer reset
pub type TIM3RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBRSTR1_SPEC, bool, O>;
///Field `SPI2RST` reader - SPI2 reset
pub type SPI2RST_R = crate::BitReader<bool>;
///Field `SPI2RST` writer - SPI2 reset
pub type SPI2RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBRSTR1_SPEC, bool, O>;
///Field `USART2RST` reader - USART2 reset
pub type USART2RST_R = crate::BitReader<bool>;
///Field `USART2RST` writer - USART2 reset
pub type USART2RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBRSTR1_SPEC, bool, O>;
///Field `LPUART1RST` reader - LPUART1 reset
pub type LPUART1RST_R = crate::BitReader<bool>;
///Field `LPUART1RST` writer - LPUART1 reset
pub type LPUART1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBRSTR1_SPEC, bool, O>;
///Field `I2C1RST` reader - I2C1 reset
pub type I2C1RST_R = crate::BitReader<bool>;
///Field `I2C1RST` writer - I2C1 reset
pub type I2C1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBRSTR1_SPEC, bool, O>;
///Field `I2C2RST` reader - I2C2 reset
pub type I2C2RST_R = crate::BitReader<bool>;
///Field `I2C2RST` writer - I2C2 reset
pub type I2C2RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBRSTR1_SPEC, bool, O>;
///Field `DBGRST` reader - Debug support reset
pub type DBGRST_R = crate::BitReader<bool>;
///Field `DBGRST` writer - Debug support reset
pub type DBGRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBRSTR1_SPEC, bool, O>;
///Field `PWRRST` reader - Power interface reset
pub type PWRRST_R = crate::BitReader<bool>;
///Field `PWRRST` writer - Power interface reset
pub type PWRRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBRSTR1_SPEC, bool, O>;
///Field `LPTIM2RST` reader - Low Power Timer 2 reset
pub type LPTIM2RST_R = crate::BitReader<bool>;
///Field `LPTIM2RST` writer - Low Power Timer 2 reset
pub type LPTIM2RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBRSTR1_SPEC, bool, O>;
///Field `LPTIM1RST` reader - Low Power Timer 1 reset
pub type LPTIM1RST_R = crate::BitReader<bool>;
///Field `LPTIM1RST` writer - Low Power Timer 1 reset
pub type LPTIM1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBRSTR1_SPEC, bool, O>;
impl R {
    ///Bit 0 - TIM2 timer reset
    #[inline(always)]
    pub fn tim2rst(&self) -> TIM2RST_R {
        TIM2RST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TIM3 timer reset
    #[inline(always)]
    pub fn tim3rst(&self) -> TIM3RST_R {
        TIM3RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 14 - SPI2 reset
    #[inline(always)]
    pub fn spi2rst(&self) -> SPI2RST_R {
        SPI2RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 17 - USART2 reset
    #[inline(always)]
    pub fn usart2rst(&self) -> USART2RST_R {
        USART2RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 20 - LPUART1 reset
    #[inline(always)]
    pub fn lpuart1rst(&self) -> LPUART1RST_R {
        LPUART1RST_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - I2C1 reset
    #[inline(always)]
    pub fn i2c1rst(&self) -> I2C1RST_R {
        I2C1RST_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - I2C2 reset
    #[inline(always)]
    pub fn i2c2rst(&self) -> I2C2RST_R {
        I2C2RST_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 27 - Debug support reset
    #[inline(always)]
    pub fn dbgrst(&self) -> DBGRST_R {
        DBGRST_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Power interface reset
    #[inline(always)]
    pub fn pwrrst(&self) -> PWRRST_R {
        PWRRST_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 30 - Low Power Timer 2 reset
    #[inline(always)]
    pub fn lptim2rst(&self) -> LPTIM2RST_R {
        LPTIM2RST_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Low Power Timer 1 reset
    #[inline(always)]
    pub fn lptim1rst(&self) -> LPTIM1RST_R {
        LPTIM1RST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - TIM2 timer reset
    #[inline(always)]
    #[must_use]
    pub fn tim2rst(&mut self) -> TIM2RST_W<0> {
        TIM2RST_W::new(self)
    }
    ///Bit 1 - TIM3 timer reset
    #[inline(always)]
    #[must_use]
    pub fn tim3rst(&mut self) -> TIM3RST_W<1> {
        TIM3RST_W::new(self)
    }
    ///Bit 14 - SPI2 reset
    #[inline(always)]
    #[must_use]
    pub fn spi2rst(&mut self) -> SPI2RST_W<14> {
        SPI2RST_W::new(self)
    }
    ///Bit 17 - USART2 reset
    #[inline(always)]
    #[must_use]
    pub fn usart2rst(&mut self) -> USART2RST_W<17> {
        USART2RST_W::new(self)
    }
    ///Bit 20 - LPUART1 reset
    #[inline(always)]
    #[must_use]
    pub fn lpuart1rst(&mut self) -> LPUART1RST_W<20> {
        LPUART1RST_W::new(self)
    }
    ///Bit 21 - I2C1 reset
    #[inline(always)]
    #[must_use]
    pub fn i2c1rst(&mut self) -> I2C1RST_W<21> {
        I2C1RST_W::new(self)
    }
    ///Bit 22 - I2C2 reset
    #[inline(always)]
    #[must_use]
    pub fn i2c2rst(&mut self) -> I2C2RST_W<22> {
        I2C2RST_W::new(self)
    }
    ///Bit 27 - Debug support reset
    #[inline(always)]
    #[must_use]
    pub fn dbgrst(&mut self) -> DBGRST_W<27> {
        DBGRST_W::new(self)
    }
    ///Bit 28 - Power interface reset
    #[inline(always)]
    #[must_use]
    pub fn pwrrst(&mut self) -> PWRRST_W<28> {
        PWRRST_W::new(self)
    }
    ///Bit 30 - Low Power Timer 2 reset
    #[inline(always)]
    #[must_use]
    pub fn lptim2rst(&mut self) -> LPTIM2RST_W<30> {
        LPTIM2RST_W::new(self)
    }
    ///Bit 31 - Low Power Timer 1 reset
    #[inline(always)]
    #[must_use]
    pub fn lptim1rst(&mut self) -> LPTIM1RST_W<31> {
        LPTIM1RST_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///APB peripheral reset register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [apbrstr1](index.html) module
pub struct APBRSTR1_SPEC;
impl crate::RegisterSpec for APBRSTR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [apbrstr1::R](R) reader structure
impl crate::Readable for APBRSTR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [apbrstr1::W](W) writer structure
impl crate::Writable for APBRSTR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets APBRSTR1 to value 0
impl crate::Resettable for APBRSTR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
