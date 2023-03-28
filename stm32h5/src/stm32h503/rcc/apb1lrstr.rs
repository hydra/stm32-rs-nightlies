///Register `APB1LRSTR` reader
pub struct R(crate::R<APB1LRSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB1LRSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB1LRSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB1LRSTR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `APB1LRSTR` writer
pub struct W(crate::W<APB1LRSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB1LRSTR_SPEC>;
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
impl From<crate::W<APB1LRSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB1LRSTR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TIM2RST` reader - TIM2 block reset Set and reset by software.
pub type TIM2RST_R = crate::BitReader<bool>;
///Field `TIM2RST` writer - TIM2 block reset Set and reset by software.
pub type TIM2RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LRSTR_SPEC, bool, O>;
///Field `TIM3RST` reader - TIM3 block reset Set and reset by software.
pub type TIM3RST_R = crate::BitReader<bool>;
///Field `TIM3RST` writer - TIM3 block reset Set and reset by software.
pub type TIM3RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LRSTR_SPEC, bool, O>;
///Field `TIM6RST` reader - TIM6 block reset Set and reset by software.
pub type TIM6RST_R = crate::BitReader<bool>;
///Field `TIM6RST` writer - TIM6 block reset Set and reset by software.
pub type TIM6RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LRSTR_SPEC, bool, O>;
///Field `TIM7RST` reader - TIM7 block reset Set and reset by software.
pub type TIM7RST_R = crate::BitReader<bool>;
///Field `TIM7RST` writer - TIM7 block reset Set and reset by software.
pub type TIM7RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LRSTR_SPEC, bool, O>;
///Field `OPAMPRST` reader - OPAMP block reset Set and reset by software.
pub type OPAMPRST_R = crate::BitReader<bool>;
///Field `OPAMPRST` writer - OPAMP block reset Set and reset by software.
pub type OPAMPRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LRSTR_SPEC, bool, O>;
///Field `SPI2RST` reader - SPI2 block reset Set and reset by software.
pub type SPI2RST_R = crate::BitReader<bool>;
///Field `SPI2RST` writer - SPI2 block reset Set and reset by software.
pub type SPI2RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LRSTR_SPEC, bool, O>;
///Field `SPI3RST` reader - SPI3 block reset Set and reset by software.
pub type SPI3RST_R = crate::BitReader<bool>;
///Field `SPI3RST` writer - SPI3 block reset Set and reset by software.
pub type SPI3RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LRSTR_SPEC, bool, O>;
///Field `COMPRST` reader - COMP block reset Set and reset by software.
pub type COMPRST_R = crate::BitReader<bool>;
///Field `COMPRST` writer - COMP block reset Set and reset by software.
pub type COMPRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LRSTR_SPEC, bool, O>;
///Field `USART2RST` reader - USART2 block reset Set and reset by software.
pub type USART2RST_R = crate::BitReader<bool>;
///Field `USART2RST` writer - USART2 block reset Set and reset by software.
pub type USART2RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LRSTR_SPEC, bool, O>;
///Field `USART3RST` reader - USART3 block reset Set and reset by software.
pub type USART3RST_R = crate::BitReader<bool>;
///Field `USART3RST` writer - USART3 block reset Set and reset by software.
pub type USART3RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LRSTR_SPEC, bool, O>;
///Field `I2C1RST` reader - I2C1 block reset Set and reset by software.
pub type I2C1RST_R = crate::BitReader<bool>;
///Field `I2C1RST` writer - I2C1 block reset Set and reset by software.
pub type I2C1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LRSTR_SPEC, bool, O>;
///Field `I2C2RST` reader - I2C2 block reset Set and reset by software.
pub type I2C2RST_R = crate::BitReader<bool>;
///Field `I2C2RST` writer - I2C2 block reset Set and reset by software.
pub type I2C2RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LRSTR_SPEC, bool, O>;
///Field `I3C1RST` reader - I3C1 block reset Set and reset by software.
pub type I3C1RST_R = crate::BitReader<bool>;
///Field `I3C1RST` writer - I3C1 block reset Set and reset by software.
pub type I3C1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LRSTR_SPEC, bool, O>;
///Field `CRSRST` reader - CRS block reset Set and reset by software.
pub type CRSRST_R = crate::BitReader<bool>;
///Field `CRSRST` writer - CRS block reset Set and reset by software.
pub type CRSRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LRSTR_SPEC, bool, O>;
impl R {
    ///Bit 0 - TIM2 block reset Set and reset by software.
    #[inline(always)]
    pub fn tim2rst(&self) -> TIM2RST_R {
        TIM2RST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TIM3 block reset Set and reset by software.
    #[inline(always)]
    pub fn tim3rst(&self) -> TIM3RST_R {
        TIM3RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 4 - TIM6 block reset Set and reset by software.
    #[inline(always)]
    pub fn tim6rst(&self) -> TIM6RST_R {
        TIM6RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TIM7 block reset Set and reset by software.
    #[inline(always)]
    pub fn tim7rst(&self) -> TIM7RST_R {
        TIM7RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 13 - OPAMP block reset Set and reset by software.
    #[inline(always)]
    pub fn opamprst(&self) -> OPAMPRST_R {
        OPAMPRST_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - SPI2 block reset Set and reset by software.
    #[inline(always)]
    pub fn spi2rst(&self) -> SPI2RST_R {
        SPI2RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - SPI3 block reset Set and reset by software.
    #[inline(always)]
    pub fn spi3rst(&self) -> SPI3RST_R {
        SPI3RST_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - COMP block reset Set and reset by software.
    #[inline(always)]
    pub fn comprst(&self) -> COMPRST_R {
        COMPRST_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - USART2 block reset Set and reset by software.
    #[inline(always)]
    pub fn usart2rst(&self) -> USART2RST_R {
        USART2RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - USART3 block reset Set and reset by software.
    #[inline(always)]
    pub fn usart3rst(&self) -> USART3RST_R {
        USART3RST_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 21 - I2C1 block reset Set and reset by software.
    #[inline(always)]
    pub fn i2c1rst(&self) -> I2C1RST_R {
        I2C1RST_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - I2C2 block reset Set and reset by software.
    #[inline(always)]
    pub fn i2c2rst(&self) -> I2C2RST_R {
        I2C2RST_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - I3C1 block reset Set and reset by software.
    #[inline(always)]
    pub fn i3c1rst(&self) -> I3C1RST_R {
        I3C1RST_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - CRS block reset Set and reset by software.
    #[inline(always)]
    pub fn crsrst(&self) -> CRSRST_R {
        CRSRST_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - TIM2 block reset Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn tim2rst(&mut self) -> TIM2RST_W<0> {
        TIM2RST_W::new(self)
    }
    ///Bit 1 - TIM3 block reset Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn tim3rst(&mut self) -> TIM3RST_W<1> {
        TIM3RST_W::new(self)
    }
    ///Bit 4 - TIM6 block reset Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn tim6rst(&mut self) -> TIM6RST_W<4> {
        TIM6RST_W::new(self)
    }
    ///Bit 5 - TIM7 block reset Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn tim7rst(&mut self) -> TIM7RST_W<5> {
        TIM7RST_W::new(self)
    }
    ///Bit 13 - OPAMP block reset Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn opamprst(&mut self) -> OPAMPRST_W<13> {
        OPAMPRST_W::new(self)
    }
    ///Bit 14 - SPI2 block reset Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn spi2rst(&mut self) -> SPI2RST_W<14> {
        SPI2RST_W::new(self)
    }
    ///Bit 15 - SPI3 block reset Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn spi3rst(&mut self) -> SPI3RST_W<15> {
        SPI3RST_W::new(self)
    }
    ///Bit 16 - COMP block reset Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn comprst(&mut self) -> COMPRST_W<16> {
        COMPRST_W::new(self)
    }
    ///Bit 17 - USART2 block reset Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn usart2rst(&mut self) -> USART2RST_W<17> {
        USART2RST_W::new(self)
    }
    ///Bit 18 - USART3 block reset Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn usart3rst(&mut self) -> USART3RST_W<18> {
        USART3RST_W::new(self)
    }
    ///Bit 21 - I2C1 block reset Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn i2c1rst(&mut self) -> I2C1RST_W<21> {
        I2C1RST_W::new(self)
    }
    ///Bit 22 - I2C2 block reset Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn i2c2rst(&mut self) -> I2C2RST_W<22> {
        I2C2RST_W::new(self)
    }
    ///Bit 23 - I3C1 block reset Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn i3c1rst(&mut self) -> I3C1RST_W<23> {
        I3C1RST_W::new(self)
    }
    ///Bit 24 - CRS block reset Set and reset by software.
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
///RCC APB1 peripheral low reset register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [apb1lrstr](index.html) module
pub struct APB1LRSTR_SPEC;
impl crate::RegisterSpec for APB1LRSTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [apb1lrstr::R](R) reader structure
impl crate::Readable for APB1LRSTR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [apb1lrstr::W](W) writer structure
impl crate::Writable for APB1LRSTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets APB1LRSTR to value 0
impl crate::Resettable for APB1LRSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
