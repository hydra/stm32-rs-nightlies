///Register `RCC_APB1ENR1` reader
pub struct R(crate::R<RCC_APB1ENR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_APB1ENR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_APB1ENR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_APB1ENR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RCC_APB1ENR1` writer
pub struct W(crate::W<RCC_APB1ENR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_APB1ENR1_SPEC>;
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
impl From<crate::W<RCC_APB1ENR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_APB1ENR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TIM2EN` reader - TIM2 clock enable Set and cleared by software.
pub type TIM2EN_R = crate::BitReader<bool>;
///Field `TIM2EN` writer - TIM2 clock enable Set and cleared by software.
pub type TIM2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB1ENR1_SPEC, bool, O>;
///Field `TIM3EN` reader - TIM3 clock enable Set and cleared by software.
pub type TIM3EN_R = crate::BitReader<bool>;
///Field `TIM3EN` writer - TIM3 clock enable Set and cleared by software.
pub type TIM3EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB1ENR1_SPEC, bool, O>;
///Field `TIM4EN` reader - TIM4 clock enable Set and cleared by software.
pub type TIM4EN_R = crate::BitReader<bool>;
///Field `TIM4EN` writer - TIM4 clock enable Set and cleared by software.
pub type TIM4EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB1ENR1_SPEC, bool, O>;
///Field `TIM5EN` reader - TIM5 clock enable Set and cleared by software.
pub type TIM5EN_R = crate::BitReader<bool>;
///Field `TIM5EN` writer - TIM5 clock enable Set and cleared by software.
pub type TIM5EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB1ENR1_SPEC, bool, O>;
///Field `TIM6EN` reader - TIM6 clock enable Set and cleared by software.
pub type TIM6EN_R = crate::BitReader<bool>;
///Field `TIM6EN` writer - TIM6 clock enable Set and cleared by software.
pub type TIM6EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB1ENR1_SPEC, bool, O>;
///Field `TIM7EN` reader - TIM7 clock enable Set and cleared by software.
pub type TIM7EN_R = crate::BitReader<bool>;
///Field `TIM7EN` writer - TIM7 clock enable Set and cleared by software.
pub type TIM7EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB1ENR1_SPEC, bool, O>;
///Field `WWDGEN` reader - WWDG clock enable Set by software to enable the window watchdog clock. Reset by hardware system reset. This bit can also be set by hardware if the WWDG_SW option bit is reset.
pub type WWDGEN_R = crate::BitReader<bool>;
///Field `WWDGEN` writer - WWDG clock enable Set by software to enable the window watchdog clock. Reset by hardware system reset. This bit can also be set by hardware if the WWDG_SW option bit is reset.
pub type WWDGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB1ENR1_SPEC, bool, O>;
///Field `SPI2EN` reader - SPI2 clock enable Set and cleared by software.
pub type SPI2EN_R = crate::BitReader<bool>;
///Field `SPI2EN` writer - SPI2 clock enable Set and cleared by software.
pub type SPI2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB1ENR1_SPEC, bool, O>;
///Field `USART2EN` reader - USART2 clock enable Set and cleared by software.
pub type USART2EN_R = crate::BitReader<bool>;
///Field `USART2EN` writer - USART2 clock enable Set and cleared by software.
pub type USART2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB1ENR1_SPEC, bool, O>;
///Field `USART3EN` reader - USART3 clock enable Set and cleared by software.
pub type USART3EN_R = crate::BitReader<bool>;
///Field `USART3EN` writer - USART3 clock enable Set and cleared by software.
pub type USART3EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB1ENR1_SPEC, bool, O>;
///Field `UART4EN` reader - UART4 clock enable Set and cleared by software.
pub type UART4EN_R = crate::BitReader<bool>;
///Field `UART4EN` writer - UART4 clock enable Set and cleared by software.
pub type UART4EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB1ENR1_SPEC, bool, O>;
///Field `UART5EN` reader - UART5 clock enable Set and cleared by software.
pub type UART5EN_R = crate::BitReader<bool>;
///Field `UART5EN` writer - UART5 clock enable Set and cleared by software.
pub type UART5EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB1ENR1_SPEC, bool, O>;
///Field `I2C1EN` reader - I2C1 clock enable Set and cleared by software.
pub type I2C1EN_R = crate::BitReader<bool>;
///Field `I2C1EN` writer - I2C1 clock enable Set and cleared by software.
pub type I2C1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB1ENR1_SPEC, bool, O>;
///Field `I2C2EN` reader - I2C2 clock enable Set and cleared by software.
pub type I2C2EN_R = crate::BitReader<bool>;
///Field `I2C2EN` writer - I2C2 clock enable Set and cleared by software.
pub type I2C2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB1ENR1_SPEC, bool, O>;
///Field `CRSEN` reader - CRS clock enable Set and cleared by software.
pub type CRSEN_R = crate::BitReader<bool>;
///Field `CRSEN` writer - CRS clock enable Set and cleared by software.
pub type CRSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB1ENR1_SPEC, bool, O>;
impl R {
    ///Bit 0 - TIM2 clock enable Set and cleared by software.
    #[inline(always)]
    pub fn tim2en(&self) -> TIM2EN_R {
        TIM2EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TIM3 clock enable Set and cleared by software.
    #[inline(always)]
    pub fn tim3en(&self) -> TIM3EN_R {
        TIM3EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TIM4 clock enable Set and cleared by software.
    #[inline(always)]
    pub fn tim4en(&self) -> TIM4EN_R {
        TIM4EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TIM5 clock enable Set and cleared by software.
    #[inline(always)]
    pub fn tim5en(&self) -> TIM5EN_R {
        TIM5EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TIM6 clock enable Set and cleared by software.
    #[inline(always)]
    pub fn tim6en(&self) -> TIM6EN_R {
        TIM6EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TIM7 clock enable Set and cleared by software.
    #[inline(always)]
    pub fn tim7en(&self) -> TIM7EN_R {
        TIM7EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 11 - WWDG clock enable Set by software to enable the window watchdog clock. Reset by hardware system reset. This bit can also be set by hardware if the WWDG_SW option bit is reset.
    #[inline(always)]
    pub fn wwdgen(&self) -> WWDGEN_R {
        WWDGEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 14 - SPI2 clock enable Set and cleared by software.
    #[inline(always)]
    pub fn spi2en(&self) -> SPI2EN_R {
        SPI2EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 17 - USART2 clock enable Set and cleared by software.
    #[inline(always)]
    pub fn usart2en(&self) -> USART2EN_R {
        USART2EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - USART3 clock enable Set and cleared by software.
    #[inline(always)]
    pub fn usart3en(&self) -> USART3EN_R {
        USART3EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - UART4 clock enable Set and cleared by software.
    #[inline(always)]
    pub fn uart4en(&self) -> UART4EN_R {
        UART4EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - UART5 clock enable Set and cleared by software.
    #[inline(always)]
    pub fn uart5en(&self) -> UART5EN_R {
        UART5EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - I2C1 clock enable Set and cleared by software.
    #[inline(always)]
    pub fn i2c1en(&self) -> I2C1EN_R {
        I2C1EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - I2C2 clock enable Set and cleared by software.
    #[inline(always)]
    pub fn i2c2en(&self) -> I2C2EN_R {
        I2C2EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 24 - CRS clock enable Set and cleared by software.
    #[inline(always)]
    pub fn crsen(&self) -> CRSEN_R {
        CRSEN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - TIM2 clock enable Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn tim2en(&mut self) -> TIM2EN_W<0> {
        TIM2EN_W::new(self)
    }
    ///Bit 1 - TIM3 clock enable Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn tim3en(&mut self) -> TIM3EN_W<1> {
        TIM3EN_W::new(self)
    }
    ///Bit 2 - TIM4 clock enable Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn tim4en(&mut self) -> TIM4EN_W<2> {
        TIM4EN_W::new(self)
    }
    ///Bit 3 - TIM5 clock enable Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn tim5en(&mut self) -> TIM5EN_W<3> {
        TIM5EN_W::new(self)
    }
    ///Bit 4 - TIM6 clock enable Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn tim6en(&mut self) -> TIM6EN_W<4> {
        TIM6EN_W::new(self)
    }
    ///Bit 5 - TIM7 clock enable Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn tim7en(&mut self) -> TIM7EN_W<5> {
        TIM7EN_W::new(self)
    }
    ///Bit 11 - WWDG clock enable Set by software to enable the window watchdog clock. Reset by hardware system reset. This bit can also be set by hardware if the WWDG_SW option bit is reset.
    #[inline(always)]
    #[must_use]
    pub fn wwdgen(&mut self) -> WWDGEN_W<11> {
        WWDGEN_W::new(self)
    }
    ///Bit 14 - SPI2 clock enable Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn spi2en(&mut self) -> SPI2EN_W<14> {
        SPI2EN_W::new(self)
    }
    ///Bit 17 - USART2 clock enable Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn usart2en(&mut self) -> USART2EN_W<17> {
        USART2EN_W::new(self)
    }
    ///Bit 18 - USART3 clock enable Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn usart3en(&mut self) -> USART3EN_W<18> {
        USART3EN_W::new(self)
    }
    ///Bit 19 - UART4 clock enable Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn uart4en(&mut self) -> UART4EN_W<19> {
        UART4EN_W::new(self)
    }
    ///Bit 20 - UART5 clock enable Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn uart5en(&mut self) -> UART5EN_W<20> {
        UART5EN_W::new(self)
    }
    ///Bit 21 - I2C1 clock enable Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn i2c1en(&mut self) -> I2C1EN_W<21> {
        I2C1EN_W::new(self)
    }
    ///Bit 22 - I2C2 clock enable Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn i2c2en(&mut self) -> I2C2EN_W<22> {
        I2C2EN_W::new(self)
    }
    ///Bit 24 - CRS clock enable Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn crsen(&mut self) -> CRSEN_W<24> {
        CRSEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RCC APB1 peripheral clock enable register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rcc_apb1enr1](index.html) module
pub struct RCC_APB1ENR1_SPEC;
impl crate::RegisterSpec for RCC_APB1ENR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [rcc_apb1enr1::R](R) reader structure
impl crate::Readable for RCC_APB1ENR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rcc_apb1enr1::W](W) writer structure
impl crate::Writable for RCC_APB1ENR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RCC_APB1ENR1 to value 0
impl crate::Resettable for RCC_APB1ENR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
