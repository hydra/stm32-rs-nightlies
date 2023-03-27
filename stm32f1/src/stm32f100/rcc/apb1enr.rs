///Register `APB1ENR` reader
pub struct R(crate::R<APB1ENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB1ENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB1ENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB1ENR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `APB1ENR` writer
pub struct W(crate::W<APB1ENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB1ENR_SPEC>;
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
impl From<crate::W<APB1ENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB1ENR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TIM2EN` reader - Timer 2 clock enable
pub type TIM2EN_R = crate::BitReader<TIM2EN_A>;
///Timer 2 clock enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM2EN_A {
    ///0: The selected clock is disabled
    Disabled = 0,
    ///1: The selected clock is enabled
    Enabled = 1,
}
impl From<TIM2EN_A> for bool {
    #[inline(always)]
    fn from(variant: TIM2EN_A) -> Self {
        variant as u8 != 0
    }
}
impl TIM2EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TIM2EN_A {
        match self.bits {
            false => TIM2EN_A::Disabled,
            true => TIM2EN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TIM2EN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TIM2EN_A::Enabled
    }
}
///Field `TIM2EN` writer - Timer 2 clock enable
pub type TIM2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1ENR_SPEC, TIM2EN_A, O>;
impl<'a, const O: u8> TIM2EN_W<'a, O> {
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM2EN_A::Disabled)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM2EN_A::Enabled)
    }
}
///Field `TIM3EN` reader - Timer 3 clock enable
pub use TIM2EN_R as TIM3EN_R;
///Field `TIM4EN` reader - Timer 4 clock enable
pub use TIM2EN_R as TIM4EN_R;
///Field `TIM5EN` reader - Timer 5 clock enable
pub use TIM2EN_R as TIM5EN_R;
///Field `TIM6EN` reader - Timer 6 clock enable
pub use TIM2EN_R as TIM6EN_R;
///Field `TIM7EN` reader - Timer 7 clock enable
pub use TIM2EN_R as TIM7EN_R;
///Field `TIM12EN` reader - Timer 12 clock enable
pub use TIM2EN_R as TIM12EN_R;
///Field `TIM13EN` reader - Timer 13 clock enable
pub use TIM2EN_R as TIM13EN_R;
///Field `TIM14EN` reader - Timer 14 clock enable
pub use TIM2EN_R as TIM14EN_R;
///Field `WWDGEN` reader - Window watchdog clock enable
pub use TIM2EN_R as WWDGEN_R;
///Field `SPI2EN` reader - SPI 2 clock enable
pub use TIM2EN_R as SPI2EN_R;
///Field `SPI3EN` reader - SPI 3 clock enable
pub use TIM2EN_R as SPI3EN_R;
///Field `USART2EN` reader - USART 2 clock enable
pub use TIM2EN_R as USART2EN_R;
///Field `USART3EN` reader - USART 3 clock enable
pub use TIM2EN_R as USART3EN_R;
///Field `UART4EN` reader - UART 4 clock enable
pub use TIM2EN_R as UART4EN_R;
///Field `UART5EN` reader - UART 5 clock enable
pub use TIM2EN_R as UART5EN_R;
///Field `I2C1EN` reader - I2C 1 clock enable
pub use TIM2EN_R as I2C1EN_R;
///Field `I2C2EN` reader - I2C 2 clock enable
pub use TIM2EN_R as I2C2EN_R;
///Field `BKPEN` reader - Backup interface clock enable
pub use TIM2EN_R as BKPEN_R;
///Field `PWREN` reader - Power interface clock enable
pub use TIM2EN_R as PWREN_R;
///Field `DACEN` reader - DAC interface clock enable
pub use TIM2EN_R as DACEN_R;
///Field `CECEN` reader - CEC clock enable
pub use TIM2EN_R as CECEN_R;
///Field `TIM3EN` writer - Timer 3 clock enable
pub use TIM2EN_W as TIM3EN_W;
///Field `TIM4EN` writer - Timer 4 clock enable
pub use TIM2EN_W as TIM4EN_W;
///Field `TIM5EN` writer - Timer 5 clock enable
pub use TIM2EN_W as TIM5EN_W;
///Field `TIM6EN` writer - Timer 6 clock enable
pub use TIM2EN_W as TIM6EN_W;
///Field `TIM7EN` writer - Timer 7 clock enable
pub use TIM2EN_W as TIM7EN_W;
///Field `TIM12EN` writer - Timer 12 clock enable
pub use TIM2EN_W as TIM12EN_W;
///Field `TIM13EN` writer - Timer 13 clock enable
pub use TIM2EN_W as TIM13EN_W;
///Field `TIM14EN` writer - Timer 14 clock enable
pub use TIM2EN_W as TIM14EN_W;
///Field `WWDGEN` writer - Window watchdog clock enable
pub use TIM2EN_W as WWDGEN_W;
///Field `SPI2EN` writer - SPI 2 clock enable
pub use TIM2EN_W as SPI2EN_W;
///Field `SPI3EN` writer - SPI 3 clock enable
pub use TIM2EN_W as SPI3EN_W;
///Field `USART2EN` writer - USART 2 clock enable
pub use TIM2EN_W as USART2EN_W;
///Field `USART3EN` writer - USART 3 clock enable
pub use TIM2EN_W as USART3EN_W;
///Field `UART4EN` writer - UART 4 clock enable
pub use TIM2EN_W as UART4EN_W;
///Field `UART5EN` writer - UART 5 clock enable
pub use TIM2EN_W as UART5EN_W;
///Field `I2C1EN` writer - I2C 1 clock enable
pub use TIM2EN_W as I2C1EN_W;
///Field `I2C2EN` writer - I2C 2 clock enable
pub use TIM2EN_W as I2C2EN_W;
///Field `BKPEN` writer - Backup interface clock enable
pub use TIM2EN_W as BKPEN_W;
///Field `PWREN` writer - Power interface clock enable
pub use TIM2EN_W as PWREN_W;
///Field `DACEN` writer - DAC interface clock enable
pub use TIM2EN_W as DACEN_W;
///Field `CECEN` writer - CEC clock enable
pub use TIM2EN_W as CECEN_W;
impl R {
    ///Bit 0 - Timer 2 clock enable
    #[inline(always)]
    pub fn tim2en(&self) -> TIM2EN_R {
        TIM2EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Timer 3 clock enable
    #[inline(always)]
    pub fn tim3en(&self) -> TIM3EN_R {
        TIM3EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Timer 4 clock enable
    #[inline(always)]
    pub fn tim4en(&self) -> TIM4EN_R {
        TIM4EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Timer 5 clock enable
    #[inline(always)]
    pub fn tim5en(&self) -> TIM5EN_R {
        TIM5EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Timer 6 clock enable
    #[inline(always)]
    pub fn tim6en(&self) -> TIM6EN_R {
        TIM6EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Timer 7 clock enable
    #[inline(always)]
    pub fn tim7en(&self) -> TIM7EN_R {
        TIM7EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Timer 12 clock enable
    #[inline(always)]
    pub fn tim12en(&self) -> TIM12EN_R {
        TIM12EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Timer 13 clock enable
    #[inline(always)]
    pub fn tim13en(&self) -> TIM13EN_R {
        TIM13EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Timer 14 clock enable
    #[inline(always)]
    pub fn tim14en(&self) -> TIM14EN_R {
        TIM14EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 11 - Window watchdog clock enable
    #[inline(always)]
    pub fn wwdgen(&self) -> WWDGEN_R {
        WWDGEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 14 - SPI 2 clock enable
    #[inline(always)]
    pub fn spi2en(&self) -> SPI2EN_R {
        SPI2EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - SPI 3 clock enable
    #[inline(always)]
    pub fn spi3en(&self) -> SPI3EN_R {
        SPI3EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 17 - USART 2 clock enable
    #[inline(always)]
    pub fn usart2en(&self) -> USART2EN_R {
        USART2EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - USART 3 clock enable
    #[inline(always)]
    pub fn usart3en(&self) -> USART3EN_R {
        USART3EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - UART 4 clock enable
    #[inline(always)]
    pub fn uart4en(&self) -> UART4EN_R {
        UART4EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - UART 5 clock enable
    #[inline(always)]
    pub fn uart5en(&self) -> UART5EN_R {
        UART5EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - I2C 1 clock enable
    #[inline(always)]
    pub fn i2c1en(&self) -> I2C1EN_R {
        I2C1EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - I2C 2 clock enable
    #[inline(always)]
    pub fn i2c2en(&self) -> I2C2EN_R {
        I2C2EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 27 - Backup interface clock enable
    #[inline(always)]
    pub fn bkpen(&self) -> BKPEN_R {
        BKPEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Power interface clock enable
    #[inline(always)]
    pub fn pwren(&self) -> PWREN_R {
        PWREN_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - DAC interface clock enable
    #[inline(always)]
    pub fn dacen(&self) -> DACEN_R {
        DACEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - CEC clock enable
    #[inline(always)]
    pub fn cecen(&self) -> CECEN_R {
        CECEN_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Timer 2 clock enable
    #[inline(always)]
    #[must_use]
    pub fn tim2en(&mut self) -> TIM2EN_W<0> {
        TIM2EN_W::new(self)
    }
    ///Bit 1 - Timer 3 clock enable
    #[inline(always)]
    #[must_use]
    pub fn tim3en(&mut self) -> TIM3EN_W<1> {
        TIM3EN_W::new(self)
    }
    ///Bit 2 - Timer 4 clock enable
    #[inline(always)]
    #[must_use]
    pub fn tim4en(&mut self) -> TIM4EN_W<2> {
        TIM4EN_W::new(self)
    }
    ///Bit 3 - Timer 5 clock enable
    #[inline(always)]
    #[must_use]
    pub fn tim5en(&mut self) -> TIM5EN_W<3> {
        TIM5EN_W::new(self)
    }
    ///Bit 4 - Timer 6 clock enable
    #[inline(always)]
    #[must_use]
    pub fn tim6en(&mut self) -> TIM6EN_W<4> {
        TIM6EN_W::new(self)
    }
    ///Bit 5 - Timer 7 clock enable
    #[inline(always)]
    #[must_use]
    pub fn tim7en(&mut self) -> TIM7EN_W<5> {
        TIM7EN_W::new(self)
    }
    ///Bit 6 - Timer 12 clock enable
    #[inline(always)]
    #[must_use]
    pub fn tim12en(&mut self) -> TIM12EN_W<6> {
        TIM12EN_W::new(self)
    }
    ///Bit 7 - Timer 13 clock enable
    #[inline(always)]
    #[must_use]
    pub fn tim13en(&mut self) -> TIM13EN_W<7> {
        TIM13EN_W::new(self)
    }
    ///Bit 8 - Timer 14 clock enable
    #[inline(always)]
    #[must_use]
    pub fn tim14en(&mut self) -> TIM14EN_W<8> {
        TIM14EN_W::new(self)
    }
    ///Bit 11 - Window watchdog clock enable
    #[inline(always)]
    #[must_use]
    pub fn wwdgen(&mut self) -> WWDGEN_W<11> {
        WWDGEN_W::new(self)
    }
    ///Bit 14 - SPI 2 clock enable
    #[inline(always)]
    #[must_use]
    pub fn spi2en(&mut self) -> SPI2EN_W<14> {
        SPI2EN_W::new(self)
    }
    ///Bit 15 - SPI 3 clock enable
    #[inline(always)]
    #[must_use]
    pub fn spi3en(&mut self) -> SPI3EN_W<15> {
        SPI3EN_W::new(self)
    }
    ///Bit 17 - USART 2 clock enable
    #[inline(always)]
    #[must_use]
    pub fn usart2en(&mut self) -> USART2EN_W<17> {
        USART2EN_W::new(self)
    }
    ///Bit 18 - USART 3 clock enable
    #[inline(always)]
    #[must_use]
    pub fn usart3en(&mut self) -> USART3EN_W<18> {
        USART3EN_W::new(self)
    }
    ///Bit 19 - UART 4 clock enable
    #[inline(always)]
    #[must_use]
    pub fn uart4en(&mut self) -> UART4EN_W<19> {
        UART4EN_W::new(self)
    }
    ///Bit 20 - UART 5 clock enable
    #[inline(always)]
    #[must_use]
    pub fn uart5en(&mut self) -> UART5EN_W<20> {
        UART5EN_W::new(self)
    }
    ///Bit 21 - I2C 1 clock enable
    #[inline(always)]
    #[must_use]
    pub fn i2c1en(&mut self) -> I2C1EN_W<21> {
        I2C1EN_W::new(self)
    }
    ///Bit 22 - I2C 2 clock enable
    #[inline(always)]
    #[must_use]
    pub fn i2c2en(&mut self) -> I2C2EN_W<22> {
        I2C2EN_W::new(self)
    }
    ///Bit 27 - Backup interface clock enable
    #[inline(always)]
    #[must_use]
    pub fn bkpen(&mut self) -> BKPEN_W<27> {
        BKPEN_W::new(self)
    }
    ///Bit 28 - Power interface clock enable
    #[inline(always)]
    #[must_use]
    pub fn pwren(&mut self) -> PWREN_W<28> {
        PWREN_W::new(self)
    }
    ///Bit 29 - DAC interface clock enable
    #[inline(always)]
    #[must_use]
    pub fn dacen(&mut self) -> DACEN_W<29> {
        DACEN_W::new(self)
    }
    ///Bit 30 - CEC clock enable
    #[inline(always)]
    #[must_use]
    pub fn cecen(&mut self) -> CECEN_W<30> {
        CECEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///APB1 peripheral clock enable register (RCC_APB1ENR)
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [apb1enr](index.html) module
pub struct APB1ENR_SPEC;
impl crate::RegisterSpec for APB1ENR_SPEC {
    type Ux = u32;
}
///`read()` method returns [apb1enr::R](R) reader structure
impl crate::Readable for APB1ENR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [apb1enr::W](W) writer structure
impl crate::Writable for APB1ENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets APB1ENR to value 0
impl crate::Resettable for APB1ENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
