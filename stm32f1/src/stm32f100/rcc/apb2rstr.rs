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
///Field `AFIORST` reader - Alternate function I/O reset
pub type AFIORST_R = crate::BitReader<AFIORST_A>;
///Alternate function I/O reset
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AFIORST_A {
    ///1: Reset the selected module
    Reset = 1,
}
impl From<AFIORST_A> for bool {
    #[inline(always)]
    fn from(variant: AFIORST_A) -> Self {
        variant as u8 != 0
    }
}
impl AFIORST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<AFIORST_A> {
        match self.bits {
            true => Some(AFIORST_A::Reset),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Reset`
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == AFIORST_A::Reset
    }
}
///Field `AFIORST` writer - Alternate function I/O reset
pub type AFIORST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2RSTR_SPEC, AFIORST_A, O>;
impl<'a, const O: u8> AFIORST_W<'a, O> {
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(AFIORST_A::Reset)
    }
}
///Field `IOPARST` reader - IO port A reset
pub use AFIORST_R as IOPARST_R;
///Field `IOPBRST` reader - IO port B reset
pub use AFIORST_R as IOPBRST_R;
///Field `IOPCRST` reader - IO port C reset
pub use AFIORST_R as IOPCRST_R;
///Field `IOPDRST` reader - IO port D reset
pub use AFIORST_R as IOPDRST_R;
///Field `IOPERST` reader - IO port E reset
pub use AFIORST_R as IOPERST_R;
///Field `IOPFRST` reader - IO port F reset
pub use AFIORST_R as IOPFRST_R;
///Field `IOPGRST` reader - IO port G reset
pub use AFIORST_R as IOPGRST_R;
///Field `ADC1RST` reader - ADC 1 interface reset
pub use AFIORST_R as ADC1RST_R;
///Field `TIM1RST` reader - TIM1 timer reset
pub use AFIORST_R as TIM1RST_R;
///Field `SPI1RST` reader - SPI 1 reset
pub use AFIORST_R as SPI1RST_R;
///Field `USART1RST` reader - USART1 reset
pub use AFIORST_R as USART1RST_R;
///Field `TIM15RST` reader - TIM15 timer reset
pub use AFIORST_R as TIM15RST_R;
///Field `TIM16RST` reader - TIM16 timer reset
pub use AFIORST_R as TIM16RST_R;
///Field `TIM17RST` reader - TIM17 timer reset
pub use AFIORST_R as TIM17RST_R;
///Field `IOPARST` writer - IO port A reset
pub use AFIORST_W as IOPARST_W;
///Field `IOPBRST` writer - IO port B reset
pub use AFIORST_W as IOPBRST_W;
///Field `IOPCRST` writer - IO port C reset
pub use AFIORST_W as IOPCRST_W;
///Field `IOPDRST` writer - IO port D reset
pub use AFIORST_W as IOPDRST_W;
///Field `IOPERST` writer - IO port E reset
pub use AFIORST_W as IOPERST_W;
///Field `IOPFRST` writer - IO port F reset
pub use AFIORST_W as IOPFRST_W;
///Field `IOPGRST` writer - IO port G reset
pub use AFIORST_W as IOPGRST_W;
///Field `ADC1RST` writer - ADC 1 interface reset
pub use AFIORST_W as ADC1RST_W;
///Field `TIM1RST` writer - TIM1 timer reset
pub use AFIORST_W as TIM1RST_W;
///Field `SPI1RST` writer - SPI 1 reset
pub use AFIORST_W as SPI1RST_W;
///Field `USART1RST` writer - USART1 reset
pub use AFIORST_W as USART1RST_W;
///Field `TIM15RST` writer - TIM15 timer reset
pub use AFIORST_W as TIM15RST_W;
///Field `TIM16RST` writer - TIM16 timer reset
pub use AFIORST_W as TIM16RST_W;
///Field `TIM17RST` writer - TIM17 timer reset
pub use AFIORST_W as TIM17RST_W;
impl R {
    ///Bit 0 - Alternate function I/O reset
    #[inline(always)]
    pub fn afiorst(&self) -> AFIORST_R {
        AFIORST_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - IO port A reset
    #[inline(always)]
    pub fn ioparst(&self) -> IOPARST_R {
        IOPARST_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - IO port B reset
    #[inline(always)]
    pub fn iopbrst(&self) -> IOPBRST_R {
        IOPBRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - IO port C reset
    #[inline(always)]
    pub fn iopcrst(&self) -> IOPCRST_R {
        IOPCRST_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - IO port D reset
    #[inline(always)]
    pub fn iopdrst(&self) -> IOPDRST_R {
        IOPDRST_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - IO port E reset
    #[inline(always)]
    pub fn ioperst(&self) -> IOPERST_R {
        IOPERST_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - IO port F reset
    #[inline(always)]
    pub fn iopfrst(&self) -> IOPFRST_R {
        IOPFRST_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - IO port G reset
    #[inline(always)]
    pub fn iopgrst(&self) -> IOPGRST_R {
        IOPGRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - ADC 1 interface reset
    #[inline(always)]
    pub fn adc1rst(&self) -> ADC1RST_R {
        ADC1RST_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - TIM1 timer reset
    #[inline(always)]
    pub fn tim1rst(&self) -> TIM1RST_R {
        TIM1RST_R::new(((self.bits >> 11) & 1) != 0)
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
}
impl W {
    ///Bit 0 - Alternate function I/O reset
    #[inline(always)]
    #[must_use]
    pub fn afiorst(&mut self) -> AFIORST_W<0> {
        AFIORST_W::new(self)
    }
    ///Bit 2 - IO port A reset
    #[inline(always)]
    #[must_use]
    pub fn ioparst(&mut self) -> IOPARST_W<2> {
        IOPARST_W::new(self)
    }
    ///Bit 3 - IO port B reset
    #[inline(always)]
    #[must_use]
    pub fn iopbrst(&mut self) -> IOPBRST_W<3> {
        IOPBRST_W::new(self)
    }
    ///Bit 4 - IO port C reset
    #[inline(always)]
    #[must_use]
    pub fn iopcrst(&mut self) -> IOPCRST_W<4> {
        IOPCRST_W::new(self)
    }
    ///Bit 5 - IO port D reset
    #[inline(always)]
    #[must_use]
    pub fn iopdrst(&mut self) -> IOPDRST_W<5> {
        IOPDRST_W::new(self)
    }
    ///Bit 6 - IO port E reset
    #[inline(always)]
    #[must_use]
    pub fn ioperst(&mut self) -> IOPERST_W<6> {
        IOPERST_W::new(self)
    }
    ///Bit 7 - IO port F reset
    #[inline(always)]
    #[must_use]
    pub fn iopfrst(&mut self) -> IOPFRST_W<7> {
        IOPFRST_W::new(self)
    }
    ///Bit 8 - IO port G reset
    #[inline(always)]
    #[must_use]
    pub fn iopgrst(&mut self) -> IOPGRST_W<8> {
        IOPGRST_W::new(self)
    }
    ///Bit 9 - ADC 1 interface reset
    #[inline(always)]
    #[must_use]
    pub fn adc1rst(&mut self) -> ADC1RST_W<9> {
        ADC1RST_W::new(self)
    }
    ///Bit 11 - TIM1 timer reset
    #[inline(always)]
    #[must_use]
    pub fn tim1rst(&mut self) -> TIM1RST_W<11> {
        TIM1RST_W::new(self)
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
