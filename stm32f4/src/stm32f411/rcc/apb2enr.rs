///Register `APB2ENR` reader
pub struct R(crate::R<APB2ENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB2ENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB2ENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB2ENR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `APB2ENR` writer
pub struct W(crate::W<APB2ENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB2ENR_SPEC>;
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
impl From<crate::W<APB2ENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB2ENR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TIM1EN` reader - TIM1 clock enable
pub type TIM1EN_R = crate::BitReader<TIM1EN_A>;
///TIM1 clock enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM1EN_A {
    ///0: The selected clock is disabled
    Disabled = 0,
    ///1: The selected clock is enabled
    Enabled = 1,
}
impl From<TIM1EN_A> for bool {
    #[inline(always)]
    fn from(variant: TIM1EN_A) -> Self {
        variant as u8 != 0
    }
}
impl TIM1EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TIM1EN_A {
        match self.bits {
            false => TIM1EN_A::Disabled,
            true => TIM1EN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TIM1EN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TIM1EN_A::Enabled
    }
}
///Field `TIM1EN` writer - TIM1 clock enable
pub type TIM1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2ENR_SPEC, TIM1EN_A, O>;
impl<'a, const O: u8> TIM1EN_W<'a, O> {
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM1EN_A::Disabled)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM1EN_A::Enabled)
    }
}
///Field `USART1EN` reader - USART1 clock enable
pub use TIM1EN_R as USART1EN_R;
///Field `USART6EN` reader - USART6 clock enable
pub use TIM1EN_R as USART6EN_R;
///Field `ADC1EN` reader - ADC1 clock enable
pub use TIM1EN_R as ADC1EN_R;
///Field `SDIOEN` reader - SDIO clock enable
pub use TIM1EN_R as SDIOEN_R;
///Field `SPI1EN` reader - SPI1 clock enable
pub use TIM1EN_R as SPI1EN_R;
///Field `SPI4EN` reader - SPI4 clock enable
pub use TIM1EN_R as SPI4EN_R;
///Field `SYSCFGEN` reader - System configuration controller clock enable
pub use TIM1EN_R as SYSCFGEN_R;
///Field `TIM9EN` reader - TIM9 clock enable
pub use TIM1EN_R as TIM9EN_R;
///Field `TIM10EN` reader - TIM10 clock enable
pub use TIM1EN_R as TIM10EN_R;
///Field `TIM11EN` reader - TIM11 clock enable
pub use TIM1EN_R as TIM11EN_R;
///Field `SPI5EN` reader - SPI5 clock enable
pub use TIM1EN_R as SPI5EN_R;
///Field `USART1EN` writer - USART1 clock enable
pub use TIM1EN_W as USART1EN_W;
///Field `USART6EN` writer - USART6 clock enable
pub use TIM1EN_W as USART6EN_W;
///Field `ADC1EN` writer - ADC1 clock enable
pub use TIM1EN_W as ADC1EN_W;
///Field `SDIOEN` writer - SDIO clock enable
pub use TIM1EN_W as SDIOEN_W;
///Field `SPI1EN` writer - SPI1 clock enable
pub use TIM1EN_W as SPI1EN_W;
///Field `SPI4EN` writer - SPI4 clock enable
pub use TIM1EN_W as SPI4EN_W;
///Field `SYSCFGEN` writer - System configuration controller clock enable
pub use TIM1EN_W as SYSCFGEN_W;
///Field `TIM9EN` writer - TIM9 clock enable
pub use TIM1EN_W as TIM9EN_W;
///Field `TIM10EN` writer - TIM10 clock enable
pub use TIM1EN_W as TIM10EN_W;
///Field `TIM11EN` writer - TIM11 clock enable
pub use TIM1EN_W as TIM11EN_W;
///Field `SPI5EN` writer - SPI5 clock enable
pub use TIM1EN_W as SPI5EN_W;
impl R {
    ///Bit 0 - TIM1 clock enable
    #[inline(always)]
    pub fn tim1en(&self) -> TIM1EN_R {
        TIM1EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - USART1 clock enable
    #[inline(always)]
    pub fn usart1en(&self) -> USART1EN_R {
        USART1EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - USART6 clock enable
    #[inline(always)]
    pub fn usart6en(&self) -> USART6EN_R {
        USART6EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - ADC1 clock enable
    #[inline(always)]
    pub fn adc1en(&self) -> ADC1EN_R {
        ADC1EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 11 - SDIO clock enable
    #[inline(always)]
    pub fn sdioen(&self) -> SDIOEN_R {
        SDIOEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - SPI1 clock enable
    #[inline(always)]
    pub fn spi1en(&self) -> SPI1EN_R {
        SPI1EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - SPI4 clock enable
    #[inline(always)]
    pub fn spi4en(&self) -> SPI4EN_R {
        SPI4EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - System configuration controller clock enable
    #[inline(always)]
    pub fn syscfgen(&self) -> SYSCFGEN_R {
        SYSCFGEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - TIM9 clock enable
    #[inline(always)]
    pub fn tim9en(&self) -> TIM9EN_R {
        TIM9EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - TIM10 clock enable
    #[inline(always)]
    pub fn tim10en(&self) -> TIM10EN_R {
        TIM10EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - TIM11 clock enable
    #[inline(always)]
    pub fn tim11en(&self) -> TIM11EN_R {
        TIM11EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 20 - SPI5 clock enable
    #[inline(always)]
    pub fn spi5en(&self) -> SPI5EN_R {
        SPI5EN_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - TIM1 clock enable
    #[inline(always)]
    #[must_use]
    pub fn tim1en(&mut self) -> TIM1EN_W<0> {
        TIM1EN_W::new(self)
    }
    ///Bit 4 - USART1 clock enable
    #[inline(always)]
    #[must_use]
    pub fn usart1en(&mut self) -> USART1EN_W<4> {
        USART1EN_W::new(self)
    }
    ///Bit 5 - USART6 clock enable
    #[inline(always)]
    #[must_use]
    pub fn usart6en(&mut self) -> USART6EN_W<5> {
        USART6EN_W::new(self)
    }
    ///Bit 8 - ADC1 clock enable
    #[inline(always)]
    #[must_use]
    pub fn adc1en(&mut self) -> ADC1EN_W<8> {
        ADC1EN_W::new(self)
    }
    ///Bit 11 - SDIO clock enable
    #[inline(always)]
    #[must_use]
    pub fn sdioen(&mut self) -> SDIOEN_W<11> {
        SDIOEN_W::new(self)
    }
    ///Bit 12 - SPI1 clock enable
    #[inline(always)]
    #[must_use]
    pub fn spi1en(&mut self) -> SPI1EN_W<12> {
        SPI1EN_W::new(self)
    }
    ///Bit 13 - SPI4 clock enable
    #[inline(always)]
    #[must_use]
    pub fn spi4en(&mut self) -> SPI4EN_W<13> {
        SPI4EN_W::new(self)
    }
    ///Bit 14 - System configuration controller clock enable
    #[inline(always)]
    #[must_use]
    pub fn syscfgen(&mut self) -> SYSCFGEN_W<14> {
        SYSCFGEN_W::new(self)
    }
    ///Bit 16 - TIM9 clock enable
    #[inline(always)]
    #[must_use]
    pub fn tim9en(&mut self) -> TIM9EN_W<16> {
        TIM9EN_W::new(self)
    }
    ///Bit 17 - TIM10 clock enable
    #[inline(always)]
    #[must_use]
    pub fn tim10en(&mut self) -> TIM10EN_W<17> {
        TIM10EN_W::new(self)
    }
    ///Bit 18 - TIM11 clock enable
    #[inline(always)]
    #[must_use]
    pub fn tim11en(&mut self) -> TIM11EN_W<18> {
        TIM11EN_W::new(self)
    }
    ///Bit 20 - SPI5 clock enable
    #[inline(always)]
    #[must_use]
    pub fn spi5en(&mut self) -> SPI5EN_W<20> {
        SPI5EN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///APB2 peripheral clock enable register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [apb2enr](index.html) module
pub struct APB2ENR_SPEC;
impl crate::RegisterSpec for APB2ENR_SPEC {
    type Ux = u32;
}
///`read()` method returns [apb2enr::R](R) reader structure
impl crate::Readable for APB2ENR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [apb2enr::W](W) writer structure
impl crate::Writable for APB2ENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets APB2ENR to value 0
impl crate::Resettable for APB2ENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
