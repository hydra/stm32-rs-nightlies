///Register `IER1` reader
pub struct R(crate::R<IER1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IER1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IER1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IER1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `IER1` writer
pub struct W(crate::W<IER1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IER1_SPEC>;
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
impl From<crate::W<IER1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IER1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TIM2IE` reader - TIM2IE
pub type TIM2IE_R = crate::BitReader<bool>;
///Field `TIM2IE` writer - TIM2IE
pub type TIM2IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
///Field `TIM3IE` reader - TIM3IE
pub type TIM3IE_R = crate::BitReader<bool>;
///Field `TIM3IE` writer - TIM3IE
pub type TIM3IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
///Field `TIM4IE` reader - TIM4IE
pub type TIM4IE_R = crate::BitReader<bool>;
///Field `TIM4IE` writer - TIM4IE
pub type TIM4IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
///Field `TIM5IE` reader - TIM5IE
pub type TIM5IE_R = crate::BitReader<bool>;
///Field `TIM5IE` writer - TIM5IE
pub type TIM5IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
///Field `TIM6IE` reader - TIM6IE
pub type TIM6IE_R = crate::BitReader<bool>;
///Field `TIM6IE` writer - TIM6IE
pub type TIM6IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
///Field `TIM7IE` reader - TIM7IE
pub type TIM7IE_R = crate::BitReader<bool>;
///Field `TIM7IE` writer - TIM7IE
pub type TIM7IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
///Field `WWDGIE` reader - WWDGIE
pub type WWDGIE_R = crate::BitReader<bool>;
///Field `WWDGIE` writer - WWDGIE
pub type WWDGIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
///Field `IWDGIE` reader - IWDGIE
pub type IWDGIE_R = crate::BitReader<bool>;
///Field `IWDGIE` writer - IWDGIE
pub type IWDGIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
///Field `SPI2IE` reader - SPI2IE
pub type SPI2IE_R = crate::BitReader<bool>;
///Field `SPI2IE` writer - SPI2IE
pub type SPI2IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
///Field `USART2IE` reader - illegal access interrupt enable for USART2
pub type USART2IE_R = crate::BitReader<bool>;
///Field `USART2IE` writer - illegal access interrupt enable for USART2
pub type USART2IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
///Field `USART3IE` reader - illegal access interrupt enable for USART3
pub type USART3IE_R = crate::BitReader<bool>;
///Field `USART3IE` writer - illegal access interrupt enable for USART3
pub type USART3IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
///Field `USART4IE` reader - illegal access interrupt enable for UART4
pub type USART4IE_R = crate::BitReader<bool>;
///Field `USART4IE` writer - illegal access interrupt enable for UART4
pub type USART4IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
///Field `UART5IE` reader - illegal access interrupt enable for UART5
pub type UART5IE_R = crate::BitReader<bool>;
///Field `UART5IE` writer - illegal access interrupt enable for UART5
pub type UART5IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
///Field `I2C1IE` reader - illegal access interrupt enable for I2C1
pub type I2C1IE_R = crate::BitReader<bool>;
///Field `I2C1IE` writer - illegal access interrupt enable for I2C1
pub type I2C1IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
///Field `I2C2IE` reader - illegal access interrupt enable for I2C2
pub type I2C2IE_R = crate::BitReader<bool>;
///Field `I2C2IE` writer - illegal access interrupt enable for I2C2
pub type I2C2IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
///Field `CRSIE` reader - illegal access interrupt enable for CRS
pub type CRSIE_R = crate::BitReader<bool>;
///Field `CRSIE` writer - illegal access interrupt enable for CRS
pub type CRSIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
///Field `I2C4IE` reader - illegal access interrupt enable for I2C4
pub type I2C4IE_R = crate::BitReader<bool>;
///Field `I2C4IE` writer - illegal access interrupt enable for I2C4
pub type I2C4IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
///Field `LPTIM2IE` reader - illegal access interrupt enable for LPTIM2
pub type LPTIM2IE_R = crate::BitReader<bool>;
///Field `LPTIM2IE` writer - illegal access interrupt enable for LPTIM2
pub type LPTIM2IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
///Field `FDCAN1IE` reader - illegal access interrupt enable for FDCAN1
pub type FDCAN1IE_R = crate::BitReader<bool>;
///Field `FDCAN1IE` writer - illegal access interrupt enable for FDCAN1
pub type FDCAN1IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
///Field `UCPD1IE` reader - illegal access interrupt enable for UCPD1
pub type UCPD1IE_R = crate::BitReader<bool>;
///Field `UCPD1IE` writer - illegal access interrupt enable for UCPD1
pub type UCPD1IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
impl R {
    ///Bit 0 - TIM2IE
    #[inline(always)]
    pub fn tim2ie(&self) -> TIM2IE_R {
        TIM2IE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TIM3IE
    #[inline(always)]
    pub fn tim3ie(&self) -> TIM3IE_R {
        TIM3IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TIM4IE
    #[inline(always)]
    pub fn tim4ie(&self) -> TIM4IE_R {
        TIM4IE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TIM5IE
    #[inline(always)]
    pub fn tim5ie(&self) -> TIM5IE_R {
        TIM5IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TIM6IE
    #[inline(always)]
    pub fn tim6ie(&self) -> TIM6IE_R {
        TIM6IE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TIM7IE
    #[inline(always)]
    pub fn tim7ie(&self) -> TIM7IE_R {
        TIM7IE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - WWDGIE
    #[inline(always)]
    pub fn wwdgie(&self) -> WWDGIE_R {
        WWDGIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - IWDGIE
    #[inline(always)]
    pub fn iwdgie(&self) -> IWDGIE_R {
        IWDGIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - SPI2IE
    #[inline(always)]
    pub fn spi2ie(&self) -> SPI2IE_R {
        SPI2IE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - illegal access interrupt enable for USART2
    #[inline(always)]
    pub fn usart2ie(&self) -> USART2IE_R {
        USART2IE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - illegal access interrupt enable for USART3
    #[inline(always)]
    pub fn usart3ie(&self) -> USART3IE_R {
        USART3IE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - illegal access interrupt enable for UART4
    #[inline(always)]
    pub fn usart4ie(&self) -> USART4IE_R {
        USART4IE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - illegal access interrupt enable for UART5
    #[inline(always)]
    pub fn uart5ie(&self) -> UART5IE_R {
        UART5IE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - illegal access interrupt enable for I2C1
    #[inline(always)]
    pub fn i2c1ie(&self) -> I2C1IE_R {
        I2C1IE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - illegal access interrupt enable for I2C2
    #[inline(always)]
    pub fn i2c2ie(&self) -> I2C2IE_R {
        I2C2IE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - illegal access interrupt enable for CRS
    #[inline(always)]
    pub fn crsie(&self) -> CRSIE_R {
        CRSIE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - illegal access interrupt enable for I2C4
    #[inline(always)]
    pub fn i2c4ie(&self) -> I2C4IE_R {
        I2C4IE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - illegal access interrupt enable for LPTIM2
    #[inline(always)]
    pub fn lptim2ie(&self) -> LPTIM2IE_R {
        LPTIM2IE_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - illegal access interrupt enable for FDCAN1
    #[inline(always)]
    pub fn fdcan1ie(&self) -> FDCAN1IE_R {
        FDCAN1IE_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - illegal access interrupt enable for UCPD1
    #[inline(always)]
    pub fn ucpd1ie(&self) -> UCPD1IE_R {
        UCPD1IE_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - TIM2IE
    #[inline(always)]
    #[must_use]
    pub fn tim2ie(&mut self) -> TIM2IE_W<0> {
        TIM2IE_W::new(self)
    }
    ///Bit 1 - TIM3IE
    #[inline(always)]
    #[must_use]
    pub fn tim3ie(&mut self) -> TIM3IE_W<1> {
        TIM3IE_W::new(self)
    }
    ///Bit 2 - TIM4IE
    #[inline(always)]
    #[must_use]
    pub fn tim4ie(&mut self) -> TIM4IE_W<2> {
        TIM4IE_W::new(self)
    }
    ///Bit 3 - TIM5IE
    #[inline(always)]
    #[must_use]
    pub fn tim5ie(&mut self) -> TIM5IE_W<3> {
        TIM5IE_W::new(self)
    }
    ///Bit 4 - TIM6IE
    #[inline(always)]
    #[must_use]
    pub fn tim6ie(&mut self) -> TIM6IE_W<4> {
        TIM6IE_W::new(self)
    }
    ///Bit 5 - TIM7IE
    #[inline(always)]
    #[must_use]
    pub fn tim7ie(&mut self) -> TIM7IE_W<5> {
        TIM7IE_W::new(self)
    }
    ///Bit 6 - WWDGIE
    #[inline(always)]
    #[must_use]
    pub fn wwdgie(&mut self) -> WWDGIE_W<6> {
        WWDGIE_W::new(self)
    }
    ///Bit 7 - IWDGIE
    #[inline(always)]
    #[must_use]
    pub fn iwdgie(&mut self) -> IWDGIE_W<7> {
        IWDGIE_W::new(self)
    }
    ///Bit 8 - SPI2IE
    #[inline(always)]
    #[must_use]
    pub fn spi2ie(&mut self) -> SPI2IE_W<8> {
        SPI2IE_W::new(self)
    }
    ///Bit 9 - illegal access interrupt enable for USART2
    #[inline(always)]
    #[must_use]
    pub fn usart2ie(&mut self) -> USART2IE_W<9> {
        USART2IE_W::new(self)
    }
    ///Bit 10 - illegal access interrupt enable for USART3
    #[inline(always)]
    #[must_use]
    pub fn usart3ie(&mut self) -> USART3IE_W<10> {
        USART3IE_W::new(self)
    }
    ///Bit 11 - illegal access interrupt enable for UART4
    #[inline(always)]
    #[must_use]
    pub fn usart4ie(&mut self) -> USART4IE_W<11> {
        USART4IE_W::new(self)
    }
    ///Bit 12 - illegal access interrupt enable for UART5
    #[inline(always)]
    #[must_use]
    pub fn uart5ie(&mut self) -> UART5IE_W<12> {
        UART5IE_W::new(self)
    }
    ///Bit 13 - illegal access interrupt enable for I2C1
    #[inline(always)]
    #[must_use]
    pub fn i2c1ie(&mut self) -> I2C1IE_W<13> {
        I2C1IE_W::new(self)
    }
    ///Bit 14 - illegal access interrupt enable for I2C2
    #[inline(always)]
    #[must_use]
    pub fn i2c2ie(&mut self) -> I2C2IE_W<14> {
        I2C2IE_W::new(self)
    }
    ///Bit 15 - illegal access interrupt enable for CRS
    #[inline(always)]
    #[must_use]
    pub fn crsie(&mut self) -> CRSIE_W<15> {
        CRSIE_W::new(self)
    }
    ///Bit 16 - illegal access interrupt enable for I2C4
    #[inline(always)]
    #[must_use]
    pub fn i2c4ie(&mut self) -> I2C4IE_W<16> {
        I2C4IE_W::new(self)
    }
    ///Bit 17 - illegal access interrupt enable for LPTIM2
    #[inline(always)]
    #[must_use]
    pub fn lptim2ie(&mut self) -> LPTIM2IE_W<17> {
        LPTIM2IE_W::new(self)
    }
    ///Bit 18 - illegal access interrupt enable for FDCAN1
    #[inline(always)]
    #[must_use]
    pub fn fdcan1ie(&mut self) -> FDCAN1IE_W<18> {
        FDCAN1IE_W::new(self)
    }
    ///Bit 19 - illegal access interrupt enable for UCPD1
    #[inline(always)]
    #[must_use]
    pub fn ucpd1ie(&mut self) -> UCPD1IE_W<19> {
        UCPD1IE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TZIC interrupt enable register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ier1](index.html) module
pub struct IER1_SPEC;
impl crate::RegisterSpec for IER1_SPEC {
    type Ux = u32;
}
///`read()` method returns [ier1::R](R) reader structure
impl crate::Readable for IER1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ier1::W](W) writer structure
impl crate::Writable for IER1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets IER1 to value 0
impl crate::Resettable for IER1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
