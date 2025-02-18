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
///Field `ADCEN` reader - CPU1 ADC clocks enable
pub type ADCEN_R = crate::BitReader<ADCEN_A>;
///CPU1 ADC clocks enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCEN_A {
    ///0: Clock disabled
    Disabled = 0,
    ///1: Clock enabled
    Enabled = 1,
}
impl From<ADCEN_A> for bool {
    #[inline(always)]
    fn from(variant: ADCEN_A) -> Self {
        variant as u8 != 0
    }
}
impl ADCEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ADCEN_A {
        match self.bits {
            false => ADCEN_A::Disabled,
            true => ADCEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADCEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADCEN_A::Enabled
    }
}
///Field `ADCEN` writer - CPU1 ADC clocks enable
pub type ADCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2ENR_SPEC, ADCEN_A, O>;
impl<'a, const O: u8> ADCEN_W<'a, O> {
    ///Clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADCEN_A::Disabled)
    }
    ///Clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADCEN_A::Enabled)
    }
}
///Field `TIM1EN` reader - CPU1 TIM1 timer clock enable
pub use ADCEN_R as TIM1EN_R;
///Field `SPI1EN` reader - CPU1 SPI1 clock enable
pub use ADCEN_R as SPI1EN_R;
///Field `USART1EN` reader - CPU1 USART1clocks enable
pub use ADCEN_R as USART1EN_R;
///Field `TIM16EN` reader - CPU1 TIM16 timer clock enable
pub use ADCEN_R as TIM16EN_R;
///Field `TIM17EN` reader - CPU1 TIM17 timer clock enable
pub use ADCEN_R as TIM17EN_R;
///Field `TIM1EN` writer - CPU1 TIM1 timer clock enable
pub use ADCEN_W as TIM1EN_W;
///Field `SPI1EN` writer - CPU1 SPI1 clock enable
pub use ADCEN_W as SPI1EN_W;
///Field `USART1EN` writer - CPU1 USART1clocks enable
pub use ADCEN_W as USART1EN_W;
///Field `TIM16EN` writer - CPU1 TIM16 timer clock enable
pub use ADCEN_W as TIM16EN_W;
///Field `TIM17EN` writer - CPU1 TIM17 timer clock enable
pub use ADCEN_W as TIM17EN_W;
impl R {
    ///Bit 9 - CPU1 ADC clocks enable
    #[inline(always)]
    pub fn adcen(&self) -> ADCEN_R {
        ADCEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - CPU1 TIM1 timer clock enable
    #[inline(always)]
    pub fn tim1en(&self) -> TIM1EN_R {
        TIM1EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - CPU1 SPI1 clock enable
    #[inline(always)]
    pub fn spi1en(&self) -> SPI1EN_R {
        SPI1EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - CPU1 USART1clocks enable
    #[inline(always)]
    pub fn usart1en(&self) -> USART1EN_R {
        USART1EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 17 - CPU1 TIM16 timer clock enable
    #[inline(always)]
    pub fn tim16en(&self) -> TIM16EN_R {
        TIM16EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - CPU1 TIM17 timer clock enable
    #[inline(always)]
    pub fn tim17en(&self) -> TIM17EN_R {
        TIM17EN_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    ///Bit 9 - CPU1 ADC clocks enable
    #[inline(always)]
    #[must_use]
    pub fn adcen(&mut self) -> ADCEN_W<9> {
        ADCEN_W::new(self)
    }
    ///Bit 11 - CPU1 TIM1 timer clock enable
    #[inline(always)]
    #[must_use]
    pub fn tim1en(&mut self) -> TIM1EN_W<11> {
        TIM1EN_W::new(self)
    }
    ///Bit 12 - CPU1 SPI1 clock enable
    #[inline(always)]
    #[must_use]
    pub fn spi1en(&mut self) -> SPI1EN_W<12> {
        SPI1EN_W::new(self)
    }
    ///Bit 14 - CPU1 USART1clocks enable
    #[inline(always)]
    #[must_use]
    pub fn usart1en(&mut self) -> USART1EN_W<14> {
        USART1EN_W::new(self)
    }
    ///Bit 17 - CPU1 TIM16 timer clock enable
    #[inline(always)]
    #[must_use]
    pub fn tim16en(&mut self) -> TIM16EN_W<17> {
        TIM16EN_W::new(self)
    }
    ///Bit 18 - CPU1 TIM17 timer clock enable
    #[inline(always)]
    #[must_use]
    pub fn tim17en(&mut self) -> TIM17EN_W<18> {
        TIM17EN_W::new(self)
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
