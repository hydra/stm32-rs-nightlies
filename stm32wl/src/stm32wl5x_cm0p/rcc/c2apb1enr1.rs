///Register `C2APB1ENR1` reader
pub struct R(crate::R<C2APB1ENR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2APB1ENR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2APB1ENR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2APB1ENR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `C2APB1ENR1` writer
pub struct W(crate::W<C2APB1ENR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2APB1ENR1_SPEC>;
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
impl From<crate::W<C2APB1ENR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2APB1ENR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TIM2EN` reader - CPU2 TIM2 timer clock enable
pub type TIM2EN_R = crate::BitReader<TIM2EN_A>;
///CPU2 TIM2 timer clock enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM2EN_A {
    ///0: Clock disabled
    Disabled = 0,
    ///1: Clock enabled
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
///Field `TIM2EN` writer - CPU2 TIM2 timer clock enable
pub type TIM2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2APB1ENR1_SPEC, TIM2EN_A, O>;
impl<'a, const O: u8> TIM2EN_W<'a, O> {
    ///Clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM2EN_A::Disabled)
    }
    ///Clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM2EN_A::Enabled)
    }
}
///Field `RTCAPBEN` reader - CPU2 RTC APB clock enable
pub use TIM2EN_R as RTCAPBEN_R;
///Field `SPI2S2EN` reader - CPU2 SPI2S2 clock enable
pub use TIM2EN_R as SPI2S2EN_R;
///Field `USART2EN` reader - CPU2 USART2 clock enable
pub use TIM2EN_R as USART2EN_R;
///Field `I2C1EN` reader - CPU2 I2C1 clocks enable
pub use TIM2EN_R as I2C1EN_R;
///Field `I2C2EN` reader - CPU2 I2C2 clocks enable
pub use TIM2EN_R as I2C2EN_R;
///Field `I2C3EN` reader - CPU2 I2C3 clocks enable
pub use TIM2EN_R as I2C3EN_R;
///Field `DAC1EN` reader - CPU2 DAC1 clock enable
pub use TIM2EN_R as DAC1EN_R;
///Field `LPTIM1EN` reader - CPU2 Low power timer 1 clocks enable
pub use TIM2EN_R as LPTIM1EN_R;
///Field `RTCAPBEN` writer - CPU2 RTC APB clock enable
pub use TIM2EN_W as RTCAPBEN_W;
///Field `SPI2S2EN` writer - CPU2 SPI2S2 clock enable
pub use TIM2EN_W as SPI2S2EN_W;
///Field `USART2EN` writer - CPU2 USART2 clock enable
pub use TIM2EN_W as USART2EN_W;
///Field `I2C1EN` writer - CPU2 I2C1 clocks enable
pub use TIM2EN_W as I2C1EN_W;
///Field `I2C2EN` writer - CPU2 I2C2 clocks enable
pub use TIM2EN_W as I2C2EN_W;
///Field `I2C3EN` writer - CPU2 I2C3 clocks enable
pub use TIM2EN_W as I2C3EN_W;
///Field `DAC1EN` writer - CPU2 DAC1 clock enable
pub use TIM2EN_W as DAC1EN_W;
///Field `LPTIM1EN` writer - CPU2 Low power timer 1 clocks enable
pub use TIM2EN_W as LPTIM1EN_W;
impl R {
    ///Bit 0 - CPU2 TIM2 timer clock enable
    #[inline(always)]
    pub fn tim2en(&self) -> TIM2EN_R {
        TIM2EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 10 - CPU2 RTC APB clock enable
    #[inline(always)]
    pub fn rtcapben(&self) -> RTCAPBEN_R {
        RTCAPBEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 14 - CPU2 SPI2S2 clock enable
    #[inline(always)]
    pub fn spi2s2en(&self) -> SPI2S2EN_R {
        SPI2S2EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 17 - CPU2 USART2 clock enable
    #[inline(always)]
    pub fn usart2en(&self) -> USART2EN_R {
        USART2EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 21 - CPU2 I2C1 clocks enable
    #[inline(always)]
    pub fn i2c1en(&self) -> I2C1EN_R {
        I2C1EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - CPU2 I2C2 clocks enable
    #[inline(always)]
    pub fn i2c2en(&self) -> I2C2EN_R {
        I2C2EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - CPU2 I2C3 clocks enable
    #[inline(always)]
    pub fn i2c3en(&self) -> I2C3EN_R {
        I2C3EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 29 - CPU2 DAC1 clock enable
    #[inline(always)]
    pub fn dac1en(&self) -> DAC1EN_R {
        DAC1EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 31 - CPU2 Low power timer 1 clocks enable
    #[inline(always)]
    pub fn lptim1en(&self) -> LPTIM1EN_R {
        LPTIM1EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - CPU2 TIM2 timer clock enable
    #[inline(always)]
    #[must_use]
    pub fn tim2en(&mut self) -> TIM2EN_W<0> {
        TIM2EN_W::new(self)
    }
    ///Bit 10 - CPU2 RTC APB clock enable
    #[inline(always)]
    #[must_use]
    pub fn rtcapben(&mut self) -> RTCAPBEN_W<10> {
        RTCAPBEN_W::new(self)
    }
    ///Bit 14 - CPU2 SPI2S2 clock enable
    #[inline(always)]
    #[must_use]
    pub fn spi2s2en(&mut self) -> SPI2S2EN_W<14> {
        SPI2S2EN_W::new(self)
    }
    ///Bit 17 - CPU2 USART2 clock enable
    #[inline(always)]
    #[must_use]
    pub fn usart2en(&mut self) -> USART2EN_W<17> {
        USART2EN_W::new(self)
    }
    ///Bit 21 - CPU2 I2C1 clocks enable
    #[inline(always)]
    #[must_use]
    pub fn i2c1en(&mut self) -> I2C1EN_W<21> {
        I2C1EN_W::new(self)
    }
    ///Bit 22 - CPU2 I2C2 clocks enable
    #[inline(always)]
    #[must_use]
    pub fn i2c2en(&mut self) -> I2C2EN_W<22> {
        I2C2EN_W::new(self)
    }
    ///Bit 23 - CPU2 I2C3 clocks enable
    #[inline(always)]
    #[must_use]
    pub fn i2c3en(&mut self) -> I2C3EN_W<23> {
        I2C3EN_W::new(self)
    }
    ///Bit 29 - CPU2 DAC1 clock enable
    #[inline(always)]
    #[must_use]
    pub fn dac1en(&mut self) -> DAC1EN_W<29> {
        DAC1EN_W::new(self)
    }
    ///Bit 31 - CPU2 Low power timer 1 clocks enable
    #[inline(always)]
    #[must_use]
    pub fn lptim1en(&mut self) -> LPTIM1EN_W<31> {
        LPTIM1EN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///CPU2 APB1 peripheral clock enable register 1 \[dual core device only\]
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [c2apb1enr1](index.html) module
pub struct C2APB1ENR1_SPEC;
impl crate::RegisterSpec for C2APB1ENR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [c2apb1enr1::R](R) reader structure
impl crate::Readable for C2APB1ENR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [c2apb1enr1::W](W) writer structure
impl crate::Writable for C2APB1ENR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets C2APB1ENR1 to value 0
impl crate::Resettable for C2APB1ENR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
