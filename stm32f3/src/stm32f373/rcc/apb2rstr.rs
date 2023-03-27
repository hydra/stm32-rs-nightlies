///Register `APB2RSTR` reader
pub struct R(crate::R<APB2RSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB2RSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB2RSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB2RSTR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `APB2RSTR` writer
pub struct W(crate::W<APB2RSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB2RSTR_SPEC>;
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
impl From<crate::W<APB2RSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB2RSTR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SYSCFGRST` reader - SYSCFG and COMP reset
pub type SYSCFGRST_R = crate::BitReader<SYSCFGRST_A>;
///SYSCFG and COMP reset
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYSCFGRST_A {
    ///1: Reset the selected module
    Reset = 1,
}
impl From<SYSCFGRST_A> for bool {
    #[inline(always)]
    fn from(variant: SYSCFGRST_A) -> Self {
        variant as u8 != 0
    }
}
impl SYSCFGRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<SYSCFGRST_A> {
        match self.bits {
            true => Some(SYSCFGRST_A::Reset),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Reset`
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SYSCFGRST_A::Reset
    }
}
///Field `SYSCFGRST` writer - SYSCFG and COMP reset
pub type SYSCFGRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2RSTR_SPEC, SYSCFGRST_A, O>;
impl<'a, const O: u8> SYSCFGRST_W<'a, O> {
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(SYSCFGRST_A::Reset)
    }
}
///Field `ADCRST` reader - ADC interface reset
pub use SYSCFGRST_R as ADCRST_R;
///Field `SPI1RST` reader - SPI 1 reset
pub use SYSCFGRST_R as SPI1RST_R;
///Field `USART1RST` reader - USART1 reset
pub use SYSCFGRST_R as USART1RST_R;
///Field `TIM15RST` reader - TIM15 timer reset
pub use SYSCFGRST_R as TIM15RST_R;
///Field `TIM16RST` reader - TIM16 timer reset
pub use SYSCFGRST_R as TIM16RST_R;
///Field `TIM17RST` reader - TIM17 timer reset
pub use SYSCFGRST_R as TIM17RST_R;
///Field `TIM19RST` reader - TIM19 timer reset
pub use SYSCFGRST_R as TIM19RST_R;
///Field `SDADC1RST` reader - SDADC1 (Sigma delta ADC 1) reset
pub use SYSCFGRST_R as SDADC1RST_R;
///Field `SDADC2RST` reader - SDADC2 (Sigma delta ADC 2) reset
pub use SYSCFGRST_R as SDADC2RST_R;
///Field `SDADC3RST` reader - SDADC3 (Sigma delta ADC 3) reset
pub use SYSCFGRST_R as SDADC3RST_R;
///Field `ADCRST` writer - ADC interface reset
pub use SYSCFGRST_W as ADCRST_W;
///Field `SPI1RST` writer - SPI 1 reset
pub use SYSCFGRST_W as SPI1RST_W;
///Field `USART1RST` writer - USART1 reset
pub use SYSCFGRST_W as USART1RST_W;
///Field `TIM15RST` writer - TIM15 timer reset
pub use SYSCFGRST_W as TIM15RST_W;
///Field `TIM16RST` writer - TIM16 timer reset
pub use SYSCFGRST_W as TIM16RST_W;
///Field `TIM17RST` writer - TIM17 timer reset
pub use SYSCFGRST_W as TIM17RST_W;
///Field `TIM19RST` writer - TIM19 timer reset
pub use SYSCFGRST_W as TIM19RST_W;
///Field `SDADC1RST` writer - SDADC1 (Sigma delta ADC 1) reset
pub use SYSCFGRST_W as SDADC1RST_W;
///Field `SDADC2RST` writer - SDADC2 (Sigma delta ADC 2) reset
pub use SYSCFGRST_W as SDADC2RST_W;
///Field `SDADC3RST` writer - SDADC3 (Sigma delta ADC 3) reset
pub use SYSCFGRST_W as SDADC3RST_W;
impl R {
    ///Bit 0 - SYSCFG and COMP reset
    #[inline(always)]
    pub fn syscfgrst(&self) -> SYSCFGRST_R {
        SYSCFGRST_R::new((self.bits & 1) != 0)
    }
    ///Bit 9 - ADC interface reset
    #[inline(always)]
    pub fn adcrst(&self) -> ADCRST_R {
        ADCRST_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 12 - SPI 1 reset
    #[inline(always)]
    pub fn spi1rst(&self) -> SPI1RST_R {
        SPI1RST_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - USART1 reset
    #[inline(always)]
    pub fn usart1rst(&self) -> USART1RST_R {
        USART1RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - TIM15 timer reset
    #[inline(always)]
    pub fn tim15rst(&self) -> TIM15RST_R {
        TIM15RST_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - TIM16 timer reset
    #[inline(always)]
    pub fn tim16rst(&self) -> TIM16RST_R {
        TIM16RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - TIM17 timer reset
    #[inline(always)]
    pub fn tim17rst(&self) -> TIM17RST_R {
        TIM17RST_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - TIM19 timer reset
    #[inline(always)]
    pub fn tim19rst(&self) -> TIM19RST_R {
        TIM19RST_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 24 - SDADC1 (Sigma delta ADC 1) reset
    #[inline(always)]
    pub fn sdadc1rst(&self) -> SDADC1RST_R {
        SDADC1RST_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - SDADC2 (Sigma delta ADC 2) reset
    #[inline(always)]
    pub fn sdadc2rst(&self) -> SDADC2RST_R {
        SDADC2RST_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - SDADC3 (Sigma delta ADC 3) reset
    #[inline(always)]
    pub fn sdadc3rst(&self) -> SDADC3RST_R {
        SDADC3RST_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - SYSCFG and COMP reset
    #[inline(always)]
    #[must_use]
    pub fn syscfgrst(&mut self) -> SYSCFGRST_W<0> {
        SYSCFGRST_W::new(self)
    }
    ///Bit 9 - ADC interface reset
    #[inline(always)]
    #[must_use]
    pub fn adcrst(&mut self) -> ADCRST_W<9> {
        ADCRST_W::new(self)
    }
    ///Bit 12 - SPI 1 reset
    #[inline(always)]
    #[must_use]
    pub fn spi1rst(&mut self) -> SPI1RST_W<12> {
        SPI1RST_W::new(self)
    }
    ///Bit 14 - USART1 reset
    #[inline(always)]
    #[must_use]
    pub fn usart1rst(&mut self) -> USART1RST_W<14> {
        USART1RST_W::new(self)
    }
    ///Bit 16 - TIM15 timer reset
    #[inline(always)]
    #[must_use]
    pub fn tim15rst(&mut self) -> TIM15RST_W<16> {
        TIM15RST_W::new(self)
    }
    ///Bit 17 - TIM16 timer reset
    #[inline(always)]
    #[must_use]
    pub fn tim16rst(&mut self) -> TIM16RST_W<17> {
        TIM16RST_W::new(self)
    }
    ///Bit 18 - TIM17 timer reset
    #[inline(always)]
    #[must_use]
    pub fn tim17rst(&mut self) -> TIM17RST_W<18> {
        TIM17RST_W::new(self)
    }
    ///Bit 19 - TIM19 timer reset
    #[inline(always)]
    #[must_use]
    pub fn tim19rst(&mut self) -> TIM19RST_W<19> {
        TIM19RST_W::new(self)
    }
    ///Bit 24 - SDADC1 (Sigma delta ADC 1) reset
    #[inline(always)]
    #[must_use]
    pub fn sdadc1rst(&mut self) -> SDADC1RST_W<24> {
        SDADC1RST_W::new(self)
    }
    ///Bit 25 - SDADC2 (Sigma delta ADC 2) reset
    #[inline(always)]
    #[must_use]
    pub fn sdadc2rst(&mut self) -> SDADC2RST_W<25> {
        SDADC2RST_W::new(self)
    }
    ///Bit 26 - SDADC3 (Sigma delta ADC 3) reset
    #[inline(always)]
    #[must_use]
    pub fn sdadc3rst(&mut self) -> SDADC3RST_W<26> {
        SDADC3RST_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///APB2 peripheral reset register (RCC_APB2RSTR)
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [apb2rstr](index.html) module
pub struct APB2RSTR_SPEC;
impl crate::RegisterSpec for APB2RSTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [apb2rstr::R](R) reader structure
impl crate::Readable for APB2RSTR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [apb2rstr::W](W) writer structure
impl crate::Writable for APB2RSTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets APB2RSTR to value 0
impl crate::Resettable for APB2RSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
