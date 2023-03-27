///Register `RCC_APB1SMENR1` reader
pub struct R(crate::R<RCC_APB1SMENR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_APB1SMENR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_APB1SMENR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_APB1SMENR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RCC_APB1SMENR1` writer
pub struct W(crate::W<RCC_APB1SMENR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_APB1SMENR1_SPEC>;
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
impl From<crate::W<RCC_APB1SMENR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_APB1SMENR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TIM2SMEN` reader - TIM2 clocks enable during Sleep and Stop modes Set and cleared by software.
pub type TIM2SMEN_R = crate::BitReader<bool>;
///Field `TIM2SMEN` writer - TIM2 clocks enable during Sleep and Stop modes Set and cleared by software.
pub type TIM2SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB1SMENR1_SPEC, bool, O>;
///Field `TIM3SMEN` reader - TIM3 clocks enable during Sleep and Stop modes Set and cleared by software.
pub type TIM3SMEN_R = crate::BitReader<bool>;
///Field `TIM3SMEN` writer - TIM3 clocks enable during Sleep and Stop modes Set and cleared by software.
pub type TIM3SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB1SMENR1_SPEC, bool, O>;
///Field `TIM4SMEN` reader - TIM4 clocks enable during Sleep and Stop modes Set and cleared by software.
pub type TIM4SMEN_R = crate::BitReader<bool>;
///Field `TIM4SMEN` writer - TIM4 clocks enable during Sleep and Stop modes Set and cleared by software.
pub type TIM4SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB1SMENR1_SPEC, bool, O>;
///Field `TIM5SMEN` reader - TIM5 clocks enable during Sleep and Stop modes Set and cleared by software.
pub type TIM5SMEN_R = crate::BitReader<bool>;
///Field `TIM5SMEN` writer - TIM5 clocks enable during Sleep and Stop modes Set and cleared by software.
pub type TIM5SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB1SMENR1_SPEC, bool, O>;
///Field `TIM6SMEN` reader - TIM6 clocks enable during Sleep and Stop modes Set and cleared by software.
pub type TIM6SMEN_R = crate::BitReader<bool>;
///Field `TIM6SMEN` writer - TIM6 clocks enable during Sleep and Stop modes Set and cleared by software.
pub type TIM6SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB1SMENR1_SPEC, bool, O>;
///Field `TIM7SMEN` reader - TIM7 clocks enable during Sleep and Stop modes Set and cleared by software.
pub type TIM7SMEN_R = crate::BitReader<bool>;
///Field `TIM7SMEN` writer - TIM7 clocks enable during Sleep and Stop modes Set and cleared by software.
pub type TIM7SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB1SMENR1_SPEC, bool, O>;
///Field `WWDGSMEN` reader - Window watchdog clocks enable during Sleep and Stop modes Set and cleared by software. This bit is forced to 1 by hardware when the hardware WWDG option is activated.
pub type WWDGSMEN_R = crate::BitReader<bool>;
///Field `WWDGSMEN` writer - Window watchdog clocks enable during Sleep and Stop modes Set and cleared by software. This bit is forced to 1 by hardware when the hardware WWDG option is activated.
pub type WWDGSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB1SMENR1_SPEC, bool, O>;
///Field `SPI2SMEN` reader - SPI2 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type SPI2SMEN_R = crate::BitReader<bool>;
///Field `SPI2SMEN` writer - SPI2 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type SPI2SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB1SMENR1_SPEC, bool, O>;
///Field `USART2SMEN` reader - USART2 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type USART2SMEN_R = crate::BitReader<bool>;
///Field `USART2SMEN` writer - USART2 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type USART2SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB1SMENR1_SPEC, bool, O>;
///Field `USART3SMEN` reader - USART3 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type USART3SMEN_R = crate::BitReader<bool>;
///Field `USART3SMEN` writer - USART3 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type USART3SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB1SMENR1_SPEC, bool, O>;
///Field `UART4SMEN` reader - UART4 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type UART4SMEN_R = crate::BitReader<bool>;
///Field `UART4SMEN` writer - UART4 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type UART4SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB1SMENR1_SPEC, bool, O>;
///Field `UART5SMEN` reader - UART5 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type UART5SMEN_R = crate::BitReader<bool>;
///Field `UART5SMEN` writer - UART5 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type UART5SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB1SMENR1_SPEC, bool, O>;
///Field `I2C1SMEN` reader - I2C1 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type I2C1SMEN_R = crate::BitReader<bool>;
///Field `I2C1SMEN` writer - I2C1 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type I2C1SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB1SMENR1_SPEC, bool, O>;
///Field `I2C2SMEN` reader - I2C2 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type I2C2SMEN_R = crate::BitReader<bool>;
///Field `I2C2SMEN` writer - I2C2 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type I2C2SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB1SMENR1_SPEC, bool, O>;
///Field `CRSSMEN` reader - CRS clock enable during Sleep and Stop modes Set and cleared by software.
pub type CRSSMEN_R = crate::BitReader<bool>;
///Field `CRSSMEN` writer - CRS clock enable during Sleep and Stop modes Set and cleared by software.
pub type CRSSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB1SMENR1_SPEC, bool, O>;
impl R {
    ///Bit 0 - TIM2 clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn tim2smen(&self) -> TIM2SMEN_R {
        TIM2SMEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TIM3 clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn tim3smen(&self) -> TIM3SMEN_R {
        TIM3SMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TIM4 clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn tim4smen(&self) -> TIM4SMEN_R {
        TIM4SMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TIM5 clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn tim5smen(&self) -> TIM5SMEN_R {
        TIM5SMEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TIM6 clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn tim6smen(&self) -> TIM6SMEN_R {
        TIM6SMEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TIM7 clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn tim7smen(&self) -> TIM7SMEN_R {
        TIM7SMEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 11 - Window watchdog clocks enable during Sleep and Stop modes Set and cleared by software. This bit is forced to 1 by hardware when the hardware WWDG option is activated.
    #[inline(always)]
    pub fn wwdgsmen(&self) -> WWDGSMEN_R {
        WWDGSMEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 14 - SPI2 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn spi2smen(&self) -> SPI2SMEN_R {
        SPI2SMEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 17 - USART2 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn usart2smen(&self) -> USART2SMEN_R {
        USART2SMEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - USART3 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn usart3smen(&self) -> USART3SMEN_R {
        USART3SMEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - UART4 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn uart4smen(&self) -> UART4SMEN_R {
        UART4SMEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - UART5 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn uart5smen(&self) -> UART5SMEN_R {
        UART5SMEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - I2C1 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn i2c1smen(&self) -> I2C1SMEN_R {
        I2C1SMEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - I2C2 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn i2c2smen(&self) -> I2C2SMEN_R {
        I2C2SMEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 24 - CRS clock enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn crssmen(&self) -> CRSSMEN_R {
        CRSSMEN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - TIM2 clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn tim2smen(&mut self) -> TIM2SMEN_W<0> {
        TIM2SMEN_W::new(self)
    }
    ///Bit 1 - TIM3 clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn tim3smen(&mut self) -> TIM3SMEN_W<1> {
        TIM3SMEN_W::new(self)
    }
    ///Bit 2 - TIM4 clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn tim4smen(&mut self) -> TIM4SMEN_W<2> {
        TIM4SMEN_W::new(self)
    }
    ///Bit 3 - TIM5 clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn tim5smen(&mut self) -> TIM5SMEN_W<3> {
        TIM5SMEN_W::new(self)
    }
    ///Bit 4 - TIM6 clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn tim6smen(&mut self) -> TIM6SMEN_W<4> {
        TIM6SMEN_W::new(self)
    }
    ///Bit 5 - TIM7 clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn tim7smen(&mut self) -> TIM7SMEN_W<5> {
        TIM7SMEN_W::new(self)
    }
    ///Bit 11 - Window watchdog clocks enable during Sleep and Stop modes Set and cleared by software. This bit is forced to 1 by hardware when the hardware WWDG option is activated.
    #[inline(always)]
    #[must_use]
    pub fn wwdgsmen(&mut self) -> WWDGSMEN_W<11> {
        WWDGSMEN_W::new(self)
    }
    ///Bit 14 - SPI2 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    #[must_use]
    pub fn spi2smen(&mut self) -> SPI2SMEN_W<14> {
        SPI2SMEN_W::new(self)
    }
    ///Bit 17 - USART2 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    #[must_use]
    pub fn usart2smen(&mut self) -> USART2SMEN_W<17> {
        USART2SMEN_W::new(self)
    }
    ///Bit 18 - USART3 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    #[must_use]
    pub fn usart3smen(&mut self) -> USART3SMEN_W<18> {
        USART3SMEN_W::new(self)
    }
    ///Bit 19 - UART4 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    #[must_use]
    pub fn uart4smen(&mut self) -> UART4SMEN_W<19> {
        UART4SMEN_W::new(self)
    }
    ///Bit 20 - UART5 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    #[must_use]
    pub fn uart5smen(&mut self) -> UART5SMEN_W<20> {
        UART5SMEN_W::new(self)
    }
    ///Bit 21 - I2C1 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    #[must_use]
    pub fn i2c1smen(&mut self) -> I2C1SMEN_W<21> {
        I2C1SMEN_W::new(self)
    }
    ///Bit 22 - I2C2 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    #[must_use]
    pub fn i2c2smen(&mut self) -> I2C2SMEN_W<22> {
        I2C2SMEN_W::new(self)
    }
    ///Bit 24 - CRS clock enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn crssmen(&mut self) -> CRSSMEN_W<24> {
        CRSSMEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RCC APB1 peripheral clocks enable in Sleep and Stop modes register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rcc_apb1smenr1](index.html) module
pub struct RCC_APB1SMENR1_SPEC;
impl crate::RegisterSpec for RCC_APB1SMENR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [rcc_apb1smenr1::R](R) reader structure
impl crate::Readable for RCC_APB1SMENR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rcc_apb1smenr1::W](W) writer structure
impl crate::Writable for RCC_APB1SMENR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RCC_APB1SMENR1 to value 0xffff_ffff
impl crate::Resettable for RCC_APB1SMENR1_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
