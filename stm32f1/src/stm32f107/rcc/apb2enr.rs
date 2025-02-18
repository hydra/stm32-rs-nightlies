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
///Field `AFIOEN` reader - Alternate function I/O clock enable
pub type AFIOEN_R = crate::BitReader<AFIOEN_A>;
///Alternate function I/O clock enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AFIOEN_A {
    ///0: The selected clock is disabled
    Disabled = 0,
    ///1: The selected clock is enabled
    Enabled = 1,
}
impl From<AFIOEN_A> for bool {
    #[inline(always)]
    fn from(variant: AFIOEN_A) -> Self {
        variant as u8 != 0
    }
}
impl AFIOEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> AFIOEN_A {
        match self.bits {
            false => AFIOEN_A::Disabled,
            true => AFIOEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AFIOEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AFIOEN_A::Enabled
    }
}
///Field `AFIOEN` writer - Alternate function I/O clock enable
pub type AFIOEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2ENR_SPEC, AFIOEN_A, O>;
impl<'a, const O: u8> AFIOEN_W<'a, O> {
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AFIOEN_A::Disabled)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AFIOEN_A::Enabled)
    }
}
///Field `IOPAEN` reader - I/O port A clock enable
pub use AFIOEN_R as IOPAEN_R;
///Field `IOPBEN` reader - I/O port B clock enable
pub use AFIOEN_R as IOPBEN_R;
///Field `IOPCEN` reader - I/O port C clock enable
pub use AFIOEN_R as IOPCEN_R;
///Field `IOPDEN` reader - I/O port D clock enable
pub use AFIOEN_R as IOPDEN_R;
///Field `IOPEEN` reader - I/O port E clock enable
pub use AFIOEN_R as IOPEEN_R;
///Field `ADC1EN` reader - ADC 1 interface clock enable
pub use AFIOEN_R as ADC1EN_R;
///Field `ADC2EN` reader - ADC 2 interface clock enable
pub use AFIOEN_R as ADC2EN_R;
///Field `TIM1EN` reader - TIM1 Timer clock enable
pub use AFIOEN_R as TIM1EN_R;
///Field `SPI1EN` reader - SPI 1 clock enable
pub use AFIOEN_R as SPI1EN_R;
///Field `USART1EN` reader - USART1 clock enable
pub use AFIOEN_R as USART1EN_R;
///Field `IOPAEN` writer - I/O port A clock enable
pub use AFIOEN_W as IOPAEN_W;
///Field `IOPBEN` writer - I/O port B clock enable
pub use AFIOEN_W as IOPBEN_W;
///Field `IOPCEN` writer - I/O port C clock enable
pub use AFIOEN_W as IOPCEN_W;
///Field `IOPDEN` writer - I/O port D clock enable
pub use AFIOEN_W as IOPDEN_W;
///Field `IOPEEN` writer - I/O port E clock enable
pub use AFIOEN_W as IOPEEN_W;
///Field `ADC1EN` writer - ADC 1 interface clock enable
pub use AFIOEN_W as ADC1EN_W;
///Field `ADC2EN` writer - ADC 2 interface clock enable
pub use AFIOEN_W as ADC2EN_W;
///Field `TIM1EN` writer - TIM1 Timer clock enable
pub use AFIOEN_W as TIM1EN_W;
///Field `SPI1EN` writer - SPI 1 clock enable
pub use AFIOEN_W as SPI1EN_W;
///Field `USART1EN` writer - USART1 clock enable
pub use AFIOEN_W as USART1EN_W;
impl R {
    ///Bit 0 - Alternate function I/O clock enable
    #[inline(always)]
    pub fn afioen(&self) -> AFIOEN_R {
        AFIOEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - I/O port A clock enable
    #[inline(always)]
    pub fn iopaen(&self) -> IOPAEN_R {
        IOPAEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - I/O port B clock enable
    #[inline(always)]
    pub fn iopben(&self) -> IOPBEN_R {
        IOPBEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - I/O port C clock enable
    #[inline(always)]
    pub fn iopcen(&self) -> IOPCEN_R {
        IOPCEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - I/O port D clock enable
    #[inline(always)]
    pub fn iopden(&self) -> IOPDEN_R {
        IOPDEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - I/O port E clock enable
    #[inline(always)]
    pub fn iopeen(&self) -> IOPEEN_R {
        IOPEEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 9 - ADC 1 interface clock enable
    #[inline(always)]
    pub fn adc1en(&self) -> ADC1EN_R {
        ADC1EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - ADC 2 interface clock enable
    #[inline(always)]
    pub fn adc2en(&self) -> ADC2EN_R {
        ADC2EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - TIM1 Timer clock enable
    #[inline(always)]
    pub fn tim1en(&self) -> TIM1EN_R {
        TIM1EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - SPI 1 clock enable
    #[inline(always)]
    pub fn spi1en(&self) -> SPI1EN_R {
        SPI1EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - USART1 clock enable
    #[inline(always)]
    pub fn usart1en(&self) -> USART1EN_R {
        USART1EN_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Alternate function I/O clock enable
    #[inline(always)]
    #[must_use]
    pub fn afioen(&mut self) -> AFIOEN_W<0> {
        AFIOEN_W::new(self)
    }
    ///Bit 2 - I/O port A clock enable
    #[inline(always)]
    #[must_use]
    pub fn iopaen(&mut self) -> IOPAEN_W<2> {
        IOPAEN_W::new(self)
    }
    ///Bit 3 - I/O port B clock enable
    #[inline(always)]
    #[must_use]
    pub fn iopben(&mut self) -> IOPBEN_W<3> {
        IOPBEN_W::new(self)
    }
    ///Bit 4 - I/O port C clock enable
    #[inline(always)]
    #[must_use]
    pub fn iopcen(&mut self) -> IOPCEN_W<4> {
        IOPCEN_W::new(self)
    }
    ///Bit 5 - I/O port D clock enable
    #[inline(always)]
    #[must_use]
    pub fn iopden(&mut self) -> IOPDEN_W<5> {
        IOPDEN_W::new(self)
    }
    ///Bit 6 - I/O port E clock enable
    #[inline(always)]
    #[must_use]
    pub fn iopeen(&mut self) -> IOPEEN_W<6> {
        IOPEEN_W::new(self)
    }
    ///Bit 9 - ADC 1 interface clock enable
    #[inline(always)]
    #[must_use]
    pub fn adc1en(&mut self) -> ADC1EN_W<9> {
        ADC1EN_W::new(self)
    }
    ///Bit 10 - ADC 2 interface clock enable
    #[inline(always)]
    #[must_use]
    pub fn adc2en(&mut self) -> ADC2EN_W<10> {
        ADC2EN_W::new(self)
    }
    ///Bit 11 - TIM1 Timer clock enable
    #[inline(always)]
    #[must_use]
    pub fn tim1en(&mut self) -> TIM1EN_W<11> {
        TIM1EN_W::new(self)
    }
    ///Bit 12 - SPI 1 clock enable
    #[inline(always)]
    #[must_use]
    pub fn spi1en(&mut self) -> SPI1EN_W<12> {
        SPI1EN_W::new(self)
    }
    ///Bit 14 - USART1 clock enable
    #[inline(always)]
    #[must_use]
    pub fn usart1en(&mut self) -> USART1EN_W<14> {
        USART1EN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///APB2 peripheral clock enable register (RCC_APB2ENR)
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
