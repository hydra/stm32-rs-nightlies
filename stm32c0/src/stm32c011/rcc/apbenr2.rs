///Register `APBENR2` reader
pub struct R(crate::R<APBENR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APBENR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APBENR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APBENR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `APBENR2` writer
pub struct W(crate::W<APBENR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APBENR2_SPEC>;
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
impl From<crate::W<APBENR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APBENR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SYSCFGEN` reader - SYSCFG clock enable Set and cleared by software.
pub type SYSCFGEN_R = crate::BitReader<bool>;
///Field `SYSCFGEN` writer - SYSCFG clock enable Set and cleared by software.
pub type SYSCFGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBENR2_SPEC, bool, O>;
///Field `TIM1EN` reader - TIM1 timer clock enable Set and cleared by software.
pub type TIM1EN_R = crate::BitReader<bool>;
///Field `TIM1EN` writer - TIM1 timer clock enable Set and cleared by software.
pub type TIM1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBENR2_SPEC, bool, O>;
///Field `SPI1EN` reader - SPI1 clock enable Set and cleared by software.
pub type SPI1EN_R = crate::BitReader<bool>;
///Field `SPI1EN` writer - SPI1 clock enable Set and cleared by software.
pub type SPI1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBENR2_SPEC, bool, O>;
///Field `USART1EN` reader - USART1 clock enable Set and cleared by software.
pub type USART1EN_R = crate::BitReader<bool>;
///Field `USART1EN` writer - USART1 clock enable Set and cleared by software.
pub type USART1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBENR2_SPEC, bool, O>;
///Field `TIM14EN` reader - TIM14 timer clock enable Set and cleared by software.
pub type TIM14EN_R = crate::BitReader<bool>;
///Field `TIM14EN` writer - TIM14 timer clock enable Set and cleared by software.
pub type TIM14EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBENR2_SPEC, bool, O>;
///Field `TIM16EN` reader - TIM16 timer clock enable Set and cleared by software.
pub type TIM16EN_R = crate::BitReader<bool>;
///Field `TIM16EN` writer - TIM16 timer clock enable Set and cleared by software.
pub type TIM16EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBENR2_SPEC, bool, O>;
///Field `TIM17EN` reader - TIM16 timer clock enable Set and cleared by software.
pub type TIM17EN_R = crate::BitReader<bool>;
///Field `TIM17EN` writer - TIM16 timer clock enable Set and cleared by software.
pub type TIM17EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBENR2_SPEC, bool, O>;
///Field `ADCEN` reader - ADC clock enable Set and cleared by software.
pub type ADCEN_R = crate::BitReader<bool>;
///Field `ADCEN` writer - ADC clock enable Set and cleared by software.
pub type ADCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBENR2_SPEC, bool, O>;
impl R {
    ///Bit 0 - SYSCFG clock enable Set and cleared by software.
    #[inline(always)]
    pub fn syscfgen(&self) -> SYSCFGEN_R {
        SYSCFGEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 11 - TIM1 timer clock enable Set and cleared by software.
    #[inline(always)]
    pub fn tim1en(&self) -> TIM1EN_R {
        TIM1EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - SPI1 clock enable Set and cleared by software.
    #[inline(always)]
    pub fn spi1en(&self) -> SPI1EN_R {
        SPI1EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - USART1 clock enable Set and cleared by software.
    #[inline(always)]
    pub fn usart1en(&self) -> USART1EN_R {
        USART1EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - TIM14 timer clock enable Set and cleared by software.
    #[inline(always)]
    pub fn tim14en(&self) -> TIM14EN_R {
        TIM14EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 17 - TIM16 timer clock enable Set and cleared by software.
    #[inline(always)]
    pub fn tim16en(&self) -> TIM16EN_R {
        TIM16EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - TIM16 timer clock enable Set and cleared by software.
    #[inline(always)]
    pub fn tim17en(&self) -> TIM17EN_R {
        TIM17EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 20 - ADC clock enable Set and cleared by software.
    #[inline(always)]
    pub fn adcen(&self) -> ADCEN_R {
        ADCEN_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - SYSCFG clock enable Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn syscfgen(&mut self) -> SYSCFGEN_W<0> {
        SYSCFGEN_W::new(self)
    }
    ///Bit 11 - TIM1 timer clock enable Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn tim1en(&mut self) -> TIM1EN_W<11> {
        TIM1EN_W::new(self)
    }
    ///Bit 12 - SPI1 clock enable Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn spi1en(&mut self) -> SPI1EN_W<12> {
        SPI1EN_W::new(self)
    }
    ///Bit 14 - USART1 clock enable Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn usart1en(&mut self) -> USART1EN_W<14> {
        USART1EN_W::new(self)
    }
    ///Bit 15 - TIM14 timer clock enable Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn tim14en(&mut self) -> TIM14EN_W<15> {
        TIM14EN_W::new(self)
    }
    ///Bit 17 - TIM16 timer clock enable Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn tim16en(&mut self) -> TIM16EN_W<17> {
        TIM16EN_W::new(self)
    }
    ///Bit 18 - TIM16 timer clock enable Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn tim17en(&mut self) -> TIM17EN_W<18> {
        TIM17EN_W::new(self)
    }
    ///Bit 20 - ADC clock enable Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn adcen(&mut self) -> ADCEN_W<20> {
        ADCEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RCC APB peripheral clock enable register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [apbenr2](index.html) module
pub struct APBENR2_SPEC;
impl crate::RegisterSpec for APBENR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [apbenr2::R](R) reader structure
impl crate::Readable for APBENR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [apbenr2::W](W) writer structure
impl crate::Writable for APBENR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets APBENR2 to value 0
impl crate::Resettable for APBENR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
