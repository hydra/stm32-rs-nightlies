///Register `RCC_APB1RSTR1` reader
pub struct R(crate::R<RCC_APB1RSTR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_APB1RSTR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_APB1RSTR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_APB1RSTR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RCC_APB1RSTR1` writer
pub struct W(crate::W<RCC_APB1RSTR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_APB1RSTR1_SPEC>;
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
impl From<crate::W<RCC_APB1RSTR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_APB1RSTR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TIM2RST` reader - TIM2 reset Set and cleared by software.
pub type TIM2RST_R = crate::BitReader<bool>;
///Field `TIM2RST` writer - TIM2 reset Set and cleared by software.
pub type TIM2RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB1RSTR1_SPEC, bool, O>;
///Field `TIM3RST` reader - TIM3 reset Set and cleared by software.
pub type TIM3RST_R = crate::BitReader<bool>;
///Field `TIM3RST` writer - TIM3 reset Set and cleared by software.
pub type TIM3RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB1RSTR1_SPEC, bool, O>;
///Field `TIM4RST` reader - TIM4 reset Set and cleared by software.
pub type TIM4RST_R = crate::BitReader<bool>;
///Field `TIM4RST` writer - TIM4 reset Set and cleared by software.
pub type TIM4RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB1RSTR1_SPEC, bool, O>;
///Field `TIM5RST` reader - TIM5 reset Set and cleared by software.
pub type TIM5RST_R = crate::BitReader<bool>;
///Field `TIM5RST` writer - TIM5 reset Set and cleared by software.
pub type TIM5RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB1RSTR1_SPEC, bool, O>;
///Field `TIM6RST` reader - TIM6 reset Set and cleared by software.
pub type TIM6RST_R = crate::BitReader<bool>;
///Field `TIM6RST` writer - TIM6 reset Set and cleared by software.
pub type TIM6RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB1RSTR1_SPEC, bool, O>;
///Field `TIM7RST` reader - TIM7 reset Set and cleared by software.
pub type TIM7RST_R = crate::BitReader<bool>;
///Field `TIM7RST` writer - TIM7 reset Set and cleared by software.
pub type TIM7RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB1RSTR1_SPEC, bool, O>;
///Field `SPI2RST` reader - SPI2 reset Set and cleared by software.
pub type SPI2RST_R = crate::BitReader<bool>;
///Field `SPI2RST` writer - SPI2 reset Set and cleared by software.
pub type SPI2RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB1RSTR1_SPEC, bool, O>;
///Field `USART2RST` reader - USART2 reset Set and cleared by software.
pub type USART2RST_R = crate::BitReader<bool>;
///Field `USART2RST` writer - USART2 reset Set and cleared by software.
pub type USART2RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB1RSTR1_SPEC, bool, O>;
///Field `USART3RST` reader - USART3 reset Set and cleared by software.
pub type USART3RST_R = crate::BitReader<bool>;
///Field `USART3RST` writer - USART3 reset Set and cleared by software.
pub type USART3RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB1RSTR1_SPEC, bool, O>;
///Field `UART4RST` reader - UART4 reset Set and cleared by software.
pub type UART4RST_R = crate::BitReader<bool>;
///Field `UART4RST` writer - UART4 reset Set and cleared by software.
pub type UART4RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB1RSTR1_SPEC, bool, O>;
///Field `UART5RST` reader - UART5 reset Set and cleared by software.
pub type UART5RST_R = crate::BitReader<bool>;
///Field `UART5RST` writer - UART5 reset Set and cleared by software.
pub type UART5RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB1RSTR1_SPEC, bool, O>;
///Field `I2C1RST` reader - I2C1 reset Set and cleared by software.
pub type I2C1RST_R = crate::BitReader<bool>;
///Field `I2C1RST` writer - I2C1 reset Set and cleared by software.
pub type I2C1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB1RSTR1_SPEC, bool, O>;
///Field `I2C2RST` reader - I2C2 reset Set and cleared by software.
pub type I2C2RST_R = crate::BitReader<bool>;
///Field `I2C2RST` writer - I2C2 reset Set and cleared by software.
pub type I2C2RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB1RSTR1_SPEC, bool, O>;
///Field `CRSRST` reader - CRS reset Set and cleared by software.
pub type CRSRST_R = crate::BitReader<bool>;
///Field `CRSRST` writer - CRS reset Set and cleared by software.
pub type CRSRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB1RSTR1_SPEC, bool, O>;
impl R {
    ///Bit 0 - TIM2 reset Set and cleared by software.
    #[inline(always)]
    pub fn tim2rst(&self) -> TIM2RST_R {
        TIM2RST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TIM3 reset Set and cleared by software.
    #[inline(always)]
    pub fn tim3rst(&self) -> TIM3RST_R {
        TIM3RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TIM4 reset Set and cleared by software.
    #[inline(always)]
    pub fn tim4rst(&self) -> TIM4RST_R {
        TIM4RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TIM5 reset Set and cleared by software.
    #[inline(always)]
    pub fn tim5rst(&self) -> TIM5RST_R {
        TIM5RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TIM6 reset Set and cleared by software.
    #[inline(always)]
    pub fn tim6rst(&self) -> TIM6RST_R {
        TIM6RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TIM7 reset Set and cleared by software.
    #[inline(always)]
    pub fn tim7rst(&self) -> TIM7RST_R {
        TIM7RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 14 - SPI2 reset Set and cleared by software.
    #[inline(always)]
    pub fn spi2rst(&self) -> SPI2RST_R {
        SPI2RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 17 - USART2 reset Set and cleared by software.
    #[inline(always)]
    pub fn usart2rst(&self) -> USART2RST_R {
        USART2RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - USART3 reset Set and cleared by software.
    #[inline(always)]
    pub fn usart3rst(&self) -> USART3RST_R {
        USART3RST_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - UART4 reset Set and cleared by software.
    #[inline(always)]
    pub fn uart4rst(&self) -> UART4RST_R {
        UART4RST_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - UART5 reset Set and cleared by software.
    #[inline(always)]
    pub fn uart5rst(&self) -> UART5RST_R {
        UART5RST_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - I2C1 reset Set and cleared by software.
    #[inline(always)]
    pub fn i2c1rst(&self) -> I2C1RST_R {
        I2C1RST_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - I2C2 reset Set and cleared by software.
    #[inline(always)]
    pub fn i2c2rst(&self) -> I2C2RST_R {
        I2C2RST_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 24 - CRS reset Set and cleared by software.
    #[inline(always)]
    pub fn crsrst(&self) -> CRSRST_R {
        CRSRST_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - TIM2 reset Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn tim2rst(&mut self) -> TIM2RST_W<0> {
        TIM2RST_W::new(self)
    }
    ///Bit 1 - TIM3 reset Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn tim3rst(&mut self) -> TIM3RST_W<1> {
        TIM3RST_W::new(self)
    }
    ///Bit 2 - TIM4 reset Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn tim4rst(&mut self) -> TIM4RST_W<2> {
        TIM4RST_W::new(self)
    }
    ///Bit 3 - TIM5 reset Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn tim5rst(&mut self) -> TIM5RST_W<3> {
        TIM5RST_W::new(self)
    }
    ///Bit 4 - TIM6 reset Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn tim6rst(&mut self) -> TIM6RST_W<4> {
        TIM6RST_W::new(self)
    }
    ///Bit 5 - TIM7 reset Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn tim7rst(&mut self) -> TIM7RST_W<5> {
        TIM7RST_W::new(self)
    }
    ///Bit 14 - SPI2 reset Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn spi2rst(&mut self) -> SPI2RST_W<14> {
        SPI2RST_W::new(self)
    }
    ///Bit 17 - USART2 reset Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn usart2rst(&mut self) -> USART2RST_W<17> {
        USART2RST_W::new(self)
    }
    ///Bit 18 - USART3 reset Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn usart3rst(&mut self) -> USART3RST_W<18> {
        USART3RST_W::new(self)
    }
    ///Bit 19 - UART4 reset Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn uart4rst(&mut self) -> UART4RST_W<19> {
        UART4RST_W::new(self)
    }
    ///Bit 20 - UART5 reset Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn uart5rst(&mut self) -> UART5RST_W<20> {
        UART5RST_W::new(self)
    }
    ///Bit 21 - I2C1 reset Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn i2c1rst(&mut self) -> I2C1RST_W<21> {
        I2C1RST_W::new(self)
    }
    ///Bit 22 - I2C2 reset Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn i2c2rst(&mut self) -> I2C2RST_W<22> {
        I2C2RST_W::new(self)
    }
    ///Bit 24 - CRS reset Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn crsrst(&mut self) -> CRSRST_W<24> {
        CRSRST_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RCC APB1 peripheral reset register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rcc_apb1rstr1](index.html) module
pub struct RCC_APB1RSTR1_SPEC;
impl crate::RegisterSpec for RCC_APB1RSTR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [rcc_apb1rstr1::R](R) reader structure
impl crate::Readable for RCC_APB1RSTR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rcc_apb1rstr1::W](W) writer structure
impl crate::Writable for RCC_APB1RSTR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RCC_APB1RSTR1 to value 0
impl crate::Resettable for RCC_APB1RSTR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
