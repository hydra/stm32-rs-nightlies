///Register `APB4LPENR` reader
pub struct R(crate::R<APB4LPENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB4LPENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB4LPENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB4LPENR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `APB4LPENR` writer
pub struct W(crate::W<APB4LPENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB4LPENR_SPEC>;
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
impl From<crate::W<APB4LPENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB4LPENR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SYSCFGLPEN` reader - SYSCFG peripheral clock enable during CSleep mode
pub type SYSCFGLPEN_R = crate::BitReader<SYSCFGLPEN_A>;
///SYSCFG peripheral clock enable during CSleep mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYSCFGLPEN_A {
    ///0: The selected clock is disabled during csleep mode
    Disabled = 0,
    ///1: The selected clock is enabled during csleep mode
    Enabled = 1,
}
impl From<SYSCFGLPEN_A> for bool {
    #[inline(always)]
    fn from(variant: SYSCFGLPEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SYSCFGLPEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SYSCFGLPEN_A {
        match self.bits {
            false => SYSCFGLPEN_A::Disabled,
            true => SYSCFGLPEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SYSCFGLPEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SYSCFGLPEN_A::Enabled
    }
}
///Field `SYSCFGLPEN` writer - SYSCFG peripheral clock enable during CSleep mode
pub type SYSCFGLPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB4LPENR_SPEC, SYSCFGLPEN_A, O>;
impl<'a, const O: u8> SYSCFGLPEN_W<'a, O> {
    ///The selected clock is disabled during csleep mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SYSCFGLPEN_A::Disabled)
    }
    ///The selected clock is enabled during csleep mode
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SYSCFGLPEN_A::Enabled)
    }
}
///Field `LPUART1LPEN` reader - LPUART1 Peripheral Clocks Enable During CSleep Mode
pub use SYSCFGLPEN_R as LPUART1LPEN_R;
///Field `SPI6LPEN` reader - SPI6 Peripheral Clocks Enable During CSleep Mode
pub use SYSCFGLPEN_R as SPI6LPEN_R;
///Field `I2C4LPEN` reader - I2C4 Peripheral Clocks Enable During CSleep Mode
pub use SYSCFGLPEN_R as I2C4LPEN_R;
///Field `LPTIM2LPEN` reader - LPTIM2 Peripheral Clocks Enable During CSleep Mode
pub use SYSCFGLPEN_R as LPTIM2LPEN_R;
///Field `LPTIM3LPEN` reader - LPTIM3 Peripheral Clocks Enable During CSleep Mode
pub use SYSCFGLPEN_R as LPTIM3LPEN_R;
///Field `LPTIM4LPEN` reader - LPTIM4 Peripheral Clocks Enable During CSleep Mode
pub use SYSCFGLPEN_R as LPTIM4LPEN_R;
///Field `LPTIM5LPEN` reader - LPTIM5 Peripheral Clocks Enable During CSleep Mode
pub use SYSCFGLPEN_R as LPTIM5LPEN_R;
///Field `COMP12LPEN` reader - COMP1/2 peripheral clock enable during CSleep mode
pub use SYSCFGLPEN_R as COMP12LPEN_R;
///Field `VREFLPEN` reader - VREF peripheral clock enable during CSleep mode
pub use SYSCFGLPEN_R as VREFLPEN_R;
///Field `RTCAPBLPEN` reader - RTC APB Clock Enable During CSleep Mode
pub use SYSCFGLPEN_R as RTCAPBLPEN_R;
///Field `SAI4LPEN` reader - SAI4 Peripheral Clocks Enable During CSleep Mode
pub use SYSCFGLPEN_R as SAI4LPEN_R;
///Field `LPUART1LPEN` writer - LPUART1 Peripheral Clocks Enable During CSleep Mode
pub use SYSCFGLPEN_W as LPUART1LPEN_W;
///Field `SPI6LPEN` writer - SPI6 Peripheral Clocks Enable During CSleep Mode
pub use SYSCFGLPEN_W as SPI6LPEN_W;
///Field `I2C4LPEN` writer - I2C4 Peripheral Clocks Enable During CSleep Mode
pub use SYSCFGLPEN_W as I2C4LPEN_W;
///Field `LPTIM2LPEN` writer - LPTIM2 Peripheral Clocks Enable During CSleep Mode
pub use SYSCFGLPEN_W as LPTIM2LPEN_W;
///Field `LPTIM3LPEN` writer - LPTIM3 Peripheral Clocks Enable During CSleep Mode
pub use SYSCFGLPEN_W as LPTIM3LPEN_W;
///Field `LPTIM4LPEN` writer - LPTIM4 Peripheral Clocks Enable During CSleep Mode
pub use SYSCFGLPEN_W as LPTIM4LPEN_W;
///Field `LPTIM5LPEN` writer - LPTIM5 Peripheral Clocks Enable During CSleep Mode
pub use SYSCFGLPEN_W as LPTIM5LPEN_W;
///Field `COMP12LPEN` writer - COMP1/2 peripheral clock enable during CSleep mode
pub use SYSCFGLPEN_W as COMP12LPEN_W;
///Field `VREFLPEN` writer - VREF peripheral clock enable during CSleep mode
pub use SYSCFGLPEN_W as VREFLPEN_W;
///Field `RTCAPBLPEN` writer - RTC APB Clock Enable During CSleep Mode
pub use SYSCFGLPEN_W as RTCAPBLPEN_W;
///Field `SAI4LPEN` writer - SAI4 Peripheral Clocks Enable During CSleep Mode
pub use SYSCFGLPEN_W as SAI4LPEN_W;
impl R {
    ///Bit 1 - SYSCFG peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn syscfglpen(&self) -> SYSCFGLPEN_R {
        SYSCFGLPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - LPUART1 Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    pub fn lpuart1lpen(&self) -> LPUART1LPEN_R {
        LPUART1LPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 5 - SPI6 Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    pub fn spi6lpen(&self) -> SPI6LPEN_R {
        SPI6LPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 7 - I2C4 Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    pub fn i2c4lpen(&self) -> I2C4LPEN_R {
        I2C4LPEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 9 - LPTIM2 Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    pub fn lptim2lpen(&self) -> LPTIM2LPEN_R {
        LPTIM2LPEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - LPTIM3 Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    pub fn lptim3lpen(&self) -> LPTIM3LPEN_R {
        LPTIM3LPEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - LPTIM4 Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    pub fn lptim4lpen(&self) -> LPTIM4LPEN_R {
        LPTIM4LPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - LPTIM5 Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    pub fn lptim5lpen(&self) -> LPTIM5LPEN_R {
        LPTIM5LPEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - COMP1/2 peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn comp12lpen(&self) -> COMP12LPEN_R {
        COMP12LPEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - VREF peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn vreflpen(&self) -> VREFLPEN_R {
        VREFLPEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - RTC APB Clock Enable During CSleep Mode
    #[inline(always)]
    pub fn rtcapblpen(&self) -> RTCAPBLPEN_R {
        RTCAPBLPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 21 - SAI4 Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    pub fn sai4lpen(&self) -> SAI4LPEN_R {
        SAI4LPEN_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    ///Bit 1 - SYSCFG peripheral clock enable during CSleep mode
    #[inline(always)]
    #[must_use]
    pub fn syscfglpen(&mut self) -> SYSCFGLPEN_W<1> {
        SYSCFGLPEN_W::new(self)
    }
    ///Bit 3 - LPUART1 Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    #[must_use]
    pub fn lpuart1lpen(&mut self) -> LPUART1LPEN_W<3> {
        LPUART1LPEN_W::new(self)
    }
    ///Bit 5 - SPI6 Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    #[must_use]
    pub fn spi6lpen(&mut self) -> SPI6LPEN_W<5> {
        SPI6LPEN_W::new(self)
    }
    ///Bit 7 - I2C4 Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    #[must_use]
    pub fn i2c4lpen(&mut self) -> I2C4LPEN_W<7> {
        I2C4LPEN_W::new(self)
    }
    ///Bit 9 - LPTIM2 Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    #[must_use]
    pub fn lptim2lpen(&mut self) -> LPTIM2LPEN_W<9> {
        LPTIM2LPEN_W::new(self)
    }
    ///Bit 10 - LPTIM3 Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    #[must_use]
    pub fn lptim3lpen(&mut self) -> LPTIM3LPEN_W<10> {
        LPTIM3LPEN_W::new(self)
    }
    ///Bit 11 - LPTIM4 Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    #[must_use]
    pub fn lptim4lpen(&mut self) -> LPTIM4LPEN_W<11> {
        LPTIM4LPEN_W::new(self)
    }
    ///Bit 12 - LPTIM5 Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    #[must_use]
    pub fn lptim5lpen(&mut self) -> LPTIM5LPEN_W<12> {
        LPTIM5LPEN_W::new(self)
    }
    ///Bit 14 - COMP1/2 peripheral clock enable during CSleep mode
    #[inline(always)]
    #[must_use]
    pub fn comp12lpen(&mut self) -> COMP12LPEN_W<14> {
        COMP12LPEN_W::new(self)
    }
    ///Bit 15 - VREF peripheral clock enable during CSleep mode
    #[inline(always)]
    #[must_use]
    pub fn vreflpen(&mut self) -> VREFLPEN_W<15> {
        VREFLPEN_W::new(self)
    }
    ///Bit 16 - RTC APB Clock Enable During CSleep Mode
    #[inline(always)]
    #[must_use]
    pub fn rtcapblpen(&mut self) -> RTCAPBLPEN_W<16> {
        RTCAPBLPEN_W::new(self)
    }
    ///Bit 21 - SAI4 Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    #[must_use]
    pub fn sai4lpen(&mut self) -> SAI4LPEN_W<21> {
        SAI4LPEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RCC APB4 Sleep Clock Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [apb4lpenr](index.html) module
pub struct APB4LPENR_SPEC;
impl crate::RegisterSpec for APB4LPENR_SPEC {
    type Ux = u32;
}
///`read()` method returns [apb4lpenr::R](R) reader structure
impl crate::Readable for APB4LPENR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [apb4lpenr::W](W) writer structure
impl crate::Writable for APB4LPENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets APB4LPENR to value 0
impl crate::Resettable for APB4LPENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
