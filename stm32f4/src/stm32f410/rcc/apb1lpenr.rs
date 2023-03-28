///Register `APB1LPENR` reader
pub struct R(crate::R<APB1LPENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB1LPENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB1LPENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB1LPENR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `APB1LPENR` writer
pub struct W(crate::W<APB1LPENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB1LPENR_SPEC>;
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
impl From<crate::W<APB1LPENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB1LPENR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TIM5LPEN` reader - TIM5 clock enable during Sleep mode
pub type TIM5LPEN_R = crate::BitReader<TIM5LPEN_A>;
///TIM5 clock enable during Sleep mode
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM5LPEN_A {
    ///0: Selected module is disabled during Sleep mode
    DisabledInSleep = 0,
    ///1: Selected module is enabled during Sleep mode
    EnabledInSleep = 1,
}
impl From<TIM5LPEN_A> for bool {
    #[inline(always)]
    fn from(variant: TIM5LPEN_A) -> Self {
        variant as u8 != 0
    }
}
impl TIM5LPEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TIM5LPEN_A {
        match self.bits {
            false => TIM5LPEN_A::DisabledInSleep,
            true => TIM5LPEN_A::EnabledInSleep,
        }
    }
    ///Checks if the value of the field is `DisabledInSleep`
    #[inline(always)]
    pub fn is_disabled_in_sleep(&self) -> bool {
        *self == TIM5LPEN_A::DisabledInSleep
    }
    ///Checks if the value of the field is `EnabledInSleep`
    #[inline(always)]
    pub fn is_enabled_in_sleep(&self) -> bool {
        *self == TIM5LPEN_A::EnabledInSleep
    }
}
///Field `TIM5LPEN` writer - TIM5 clock enable during Sleep mode
pub type TIM5LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LPENR_SPEC, TIM5LPEN_A, O>;
impl<'a, const O: u8> TIM5LPEN_W<'a, O> {
    ///Selected module is disabled during Sleep mode
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM5LPEN_A::DisabledInSleep)
    }
    ///Selected module is enabled during Sleep mode
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM5LPEN_A::EnabledInSleep)
    }
}
///Field `TIM6LPEN` reader - TIM6 clock enable during Sleep mode
pub use TIM5LPEN_R as TIM6LPEN_R;
///Field `LPTIM1LPEN` reader - LPTIM1 clock enable during sleep mode
pub use TIM5LPEN_R as LPTIM1LPEN_R;
///Field `RTCAPBLPEN` reader - RTC APB clock enable during sleep mode
pub use TIM5LPEN_R as RTCAPBLPEN_R;
///Field `WWDGLPEN` reader - Window watchdog clock enable during Sleep mode
pub use TIM5LPEN_R as WWDGLPEN_R;
///Field `SPI2LPEN` reader - SPI2 clock enable during Sleep mode
pub use TIM5LPEN_R as SPI2LPEN_R;
///Field `USART2LPEN` reader - USART2 clock enable during Sleep mode
pub use TIM5LPEN_R as USART2LPEN_R;
///Field `I2C1LPEN` reader - I2C1 clock enable during Sleep mode
pub use TIM5LPEN_R as I2C1LPEN_R;
///Field `I2C2LPEN` reader - I2C2 clock enable during Sleep mode
pub use TIM5LPEN_R as I2C2LPEN_R;
///Field `FMPI2C1LPEN` reader - FMPI2C1 clock enable during Sleep
pub use TIM5LPEN_R as FMPI2C1LPEN_R;
///Field `PWRLPEN` reader - Power interface clock enable during Sleep mode
pub use TIM5LPEN_R as PWRLPEN_R;
///Field `DACLPEN` reader - DAC interface clock enable during sleep mode
pub use TIM5LPEN_R as DACLPEN_R;
///Field `TIM6LPEN` writer - TIM6 clock enable during Sleep mode
pub use TIM5LPEN_W as TIM6LPEN_W;
///Field `LPTIM1LPEN` writer - LPTIM1 clock enable during sleep mode
pub use TIM5LPEN_W as LPTIM1LPEN_W;
///Field `RTCAPBLPEN` writer - RTC APB clock enable during sleep mode
pub use TIM5LPEN_W as RTCAPBLPEN_W;
///Field `WWDGLPEN` writer - Window watchdog clock enable during Sleep mode
pub use TIM5LPEN_W as WWDGLPEN_W;
///Field `SPI2LPEN` writer - SPI2 clock enable during Sleep mode
pub use TIM5LPEN_W as SPI2LPEN_W;
///Field `USART2LPEN` writer - USART2 clock enable during Sleep mode
pub use TIM5LPEN_W as USART2LPEN_W;
///Field `I2C1LPEN` writer - I2C1 clock enable during Sleep mode
pub use TIM5LPEN_W as I2C1LPEN_W;
///Field `I2C2LPEN` writer - I2C2 clock enable during Sleep mode
pub use TIM5LPEN_W as I2C2LPEN_W;
///Field `FMPI2C1LPEN` writer - FMPI2C1 clock enable during Sleep
pub use TIM5LPEN_W as FMPI2C1LPEN_W;
///Field `PWRLPEN` writer - Power interface clock enable during Sleep mode
pub use TIM5LPEN_W as PWRLPEN_W;
///Field `DACLPEN` writer - DAC interface clock enable during sleep mode
pub use TIM5LPEN_W as DACLPEN_W;
impl R {
    ///Bit 3 - TIM5 clock enable during Sleep mode
    #[inline(always)]
    pub fn tim5lpen(&self) -> TIM5LPEN_R {
        TIM5LPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TIM6 clock enable during Sleep mode
    #[inline(always)]
    pub fn tim6lpen(&self) -> TIM6LPEN_R {
        TIM6LPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 9 - LPTIM1 clock enable during sleep mode
    #[inline(always)]
    pub fn lptim1lpen(&self) -> LPTIM1LPEN_R {
        LPTIM1LPEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - RTC APB clock enable during sleep mode
    #[inline(always)]
    pub fn rtcapblpen(&self) -> RTCAPBLPEN_R {
        RTCAPBLPEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Window watchdog clock enable during Sleep mode
    #[inline(always)]
    pub fn wwdglpen(&self) -> WWDGLPEN_R {
        WWDGLPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 14 - SPI2 clock enable during Sleep mode
    #[inline(always)]
    pub fn spi2lpen(&self) -> SPI2LPEN_R {
        SPI2LPEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 17 - USART2 clock enable during Sleep mode
    #[inline(always)]
    pub fn usart2lpen(&self) -> USART2LPEN_R {
        USART2LPEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 21 - I2C1 clock enable during Sleep mode
    #[inline(always)]
    pub fn i2c1lpen(&self) -> I2C1LPEN_R {
        I2C1LPEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - I2C2 clock enable during Sleep mode
    #[inline(always)]
    pub fn i2c2lpen(&self) -> I2C2LPEN_R {
        I2C2LPEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 24 - FMPI2C1 clock enable during Sleep
    #[inline(always)]
    pub fn fmpi2c1lpen(&self) -> FMPI2C1LPEN_R {
        FMPI2C1LPEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 28 - Power interface clock enable during Sleep mode
    #[inline(always)]
    pub fn pwrlpen(&self) -> PWRLPEN_R {
        PWRLPEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - DAC interface clock enable during sleep mode
    #[inline(always)]
    pub fn daclpen(&self) -> DACLPEN_R {
        DACLPEN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    ///Bit 3 - TIM5 clock enable during Sleep mode
    #[inline(always)]
    #[must_use]
    pub fn tim5lpen(&mut self) -> TIM5LPEN_W<3> {
        TIM5LPEN_W::new(self)
    }
    ///Bit 4 - TIM6 clock enable during Sleep mode
    #[inline(always)]
    #[must_use]
    pub fn tim6lpen(&mut self) -> TIM6LPEN_W<4> {
        TIM6LPEN_W::new(self)
    }
    ///Bit 9 - LPTIM1 clock enable during sleep mode
    #[inline(always)]
    #[must_use]
    pub fn lptim1lpen(&mut self) -> LPTIM1LPEN_W<9> {
        LPTIM1LPEN_W::new(self)
    }
    ///Bit 10 - RTC APB clock enable during sleep mode
    #[inline(always)]
    #[must_use]
    pub fn rtcapblpen(&mut self) -> RTCAPBLPEN_W<10> {
        RTCAPBLPEN_W::new(self)
    }
    ///Bit 11 - Window watchdog clock enable during Sleep mode
    #[inline(always)]
    #[must_use]
    pub fn wwdglpen(&mut self) -> WWDGLPEN_W<11> {
        WWDGLPEN_W::new(self)
    }
    ///Bit 14 - SPI2 clock enable during Sleep mode
    #[inline(always)]
    #[must_use]
    pub fn spi2lpen(&mut self) -> SPI2LPEN_W<14> {
        SPI2LPEN_W::new(self)
    }
    ///Bit 17 - USART2 clock enable during Sleep mode
    #[inline(always)]
    #[must_use]
    pub fn usart2lpen(&mut self) -> USART2LPEN_W<17> {
        USART2LPEN_W::new(self)
    }
    ///Bit 21 - I2C1 clock enable during Sleep mode
    #[inline(always)]
    #[must_use]
    pub fn i2c1lpen(&mut self) -> I2C1LPEN_W<21> {
        I2C1LPEN_W::new(self)
    }
    ///Bit 22 - I2C2 clock enable during Sleep mode
    #[inline(always)]
    #[must_use]
    pub fn i2c2lpen(&mut self) -> I2C2LPEN_W<22> {
        I2C2LPEN_W::new(self)
    }
    ///Bit 24 - FMPI2C1 clock enable during Sleep
    #[inline(always)]
    #[must_use]
    pub fn fmpi2c1lpen(&mut self) -> FMPI2C1LPEN_W<24> {
        FMPI2C1LPEN_W::new(self)
    }
    ///Bit 28 - Power interface clock enable during Sleep mode
    #[inline(always)]
    #[must_use]
    pub fn pwrlpen(&mut self) -> PWRLPEN_W<28> {
        PWRLPEN_W::new(self)
    }
    ///Bit 29 - DAC interface clock enable during sleep mode
    #[inline(always)]
    #[must_use]
    pub fn daclpen(&mut self) -> DACLPEN_W<29> {
        DACLPEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///APB1 peripheral clock enable in low power mode register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [apb1lpenr](index.html) module
pub struct APB1LPENR_SPEC;
impl crate::RegisterSpec for APB1LPENR_SPEC {
    type Ux = u32;
}
///`read()` method returns [apb1lpenr::R](R) reader structure
impl crate::Readable for APB1LPENR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [apb1lpenr::W](W) writer structure
impl crate::Writable for APB1LPENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets APB1LPENR to value 0x36fe_c9ff
impl crate::Resettable for APB1LPENR_SPEC {
    const RESET_VALUE: Self::Ux = 0x36fe_c9ff;
}
