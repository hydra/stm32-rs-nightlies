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
///Field `TIM1RST` reader - TIM1 reset
pub type TIM1RST_R = crate::BitReader<TIM1RST_A>;
///TIM1 reset
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM1RST_A {
    ///1: Reset the selected module
    Reset = 1,
}
impl From<TIM1RST_A> for bool {
    #[inline(always)]
    fn from(variant: TIM1RST_A) -> Self {
        variant as u8 != 0
    }
}
impl TIM1RST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<TIM1RST_A> {
        match self.bits {
            true => Some(TIM1RST_A::Reset),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Reset`
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == TIM1RST_A::Reset
    }
}
///Field `TIM1RST` writer - TIM1 reset
pub type TIM1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2RSTR_SPEC, TIM1RST_A, O>;
impl<'a, const O: u8> TIM1RST_W<'a, O> {
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM1RST_A::Reset)
    }
}
///Field `TIM8RST` reader - TIM8 reset
pub use TIM1RST_R as TIM8RST_R;
///Field `USART1RST` reader - USART1 reset
pub use TIM1RST_R as USART1RST_R;
///Field `USART6RST` reader - USART6 reset
pub use TIM1RST_R as USART6RST_R;
///Field `ADCRST` reader - ADC interface reset (common to all ADCs)
pub use TIM1RST_R as ADCRST_R;
///Field `SDIORST` reader - SDIO reset
pub use TIM1RST_R as SDIORST_R;
///Field `SPI1RST` reader - SPI 1 reset
pub use TIM1RST_R as SPI1RST_R;
///Field `SYSCFGRST` reader - System configuration controller reset
pub use TIM1RST_R as SYSCFGRST_R;
///Field `TIM9RST` reader - TIM9 reset
pub use TIM1RST_R as TIM9RST_R;
///Field `TIM10RST` reader - TIM10 reset
pub use TIM1RST_R as TIM10RST_R;
///Field `TIM11RST` reader - TIM11 reset
pub use TIM1RST_R as TIM11RST_R;
///Field `TIM8RST` writer - TIM8 reset
pub use TIM1RST_W as TIM8RST_W;
///Field `USART1RST` writer - USART1 reset
pub use TIM1RST_W as USART1RST_W;
///Field `USART6RST` writer - USART6 reset
pub use TIM1RST_W as USART6RST_W;
///Field `ADCRST` writer - ADC interface reset (common to all ADCs)
pub use TIM1RST_W as ADCRST_W;
///Field `SDIORST` writer - SDIO reset
pub use TIM1RST_W as SDIORST_W;
///Field `SPI1RST` writer - SPI 1 reset
pub use TIM1RST_W as SPI1RST_W;
///Field `SYSCFGRST` writer - System configuration controller reset
pub use TIM1RST_W as SYSCFGRST_W;
///Field `TIM9RST` writer - TIM9 reset
pub use TIM1RST_W as TIM9RST_W;
///Field `TIM10RST` writer - TIM10 reset
pub use TIM1RST_W as TIM10RST_W;
///Field `TIM11RST` writer - TIM11 reset
pub use TIM1RST_W as TIM11RST_W;
impl R {
    ///Bit 0 - TIM1 reset
    #[inline(always)]
    pub fn tim1rst(&self) -> TIM1RST_R {
        TIM1RST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TIM8 reset
    #[inline(always)]
    pub fn tim8rst(&self) -> TIM8RST_R {
        TIM8RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 4 - USART1 reset
    #[inline(always)]
    pub fn usart1rst(&self) -> USART1RST_R {
        USART1RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - USART6 reset
    #[inline(always)]
    pub fn usart6rst(&self) -> USART6RST_R {
        USART6RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - ADC interface reset (common to all ADCs)
    #[inline(always)]
    pub fn adcrst(&self) -> ADCRST_R {
        ADCRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 11 - SDIO reset
    #[inline(always)]
    pub fn sdiorst(&self) -> SDIORST_R {
        SDIORST_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - SPI 1 reset
    #[inline(always)]
    pub fn spi1rst(&self) -> SPI1RST_R {
        SPI1RST_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - System configuration controller reset
    #[inline(always)]
    pub fn syscfgrst(&self) -> SYSCFGRST_R {
        SYSCFGRST_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - TIM9 reset
    #[inline(always)]
    pub fn tim9rst(&self) -> TIM9RST_R {
        TIM9RST_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - TIM10 reset
    #[inline(always)]
    pub fn tim10rst(&self) -> TIM10RST_R {
        TIM10RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - TIM11 reset
    #[inline(always)]
    pub fn tim11rst(&self) -> TIM11RST_R {
        TIM11RST_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - TIM1 reset
    #[inline(always)]
    #[must_use]
    pub fn tim1rst(&mut self) -> TIM1RST_W<0> {
        TIM1RST_W::new(self)
    }
    ///Bit 1 - TIM8 reset
    #[inline(always)]
    #[must_use]
    pub fn tim8rst(&mut self) -> TIM8RST_W<1> {
        TIM8RST_W::new(self)
    }
    ///Bit 4 - USART1 reset
    #[inline(always)]
    #[must_use]
    pub fn usart1rst(&mut self) -> USART1RST_W<4> {
        USART1RST_W::new(self)
    }
    ///Bit 5 - USART6 reset
    #[inline(always)]
    #[must_use]
    pub fn usart6rst(&mut self) -> USART6RST_W<5> {
        USART6RST_W::new(self)
    }
    ///Bit 8 - ADC interface reset (common to all ADCs)
    #[inline(always)]
    #[must_use]
    pub fn adcrst(&mut self) -> ADCRST_W<8> {
        ADCRST_W::new(self)
    }
    ///Bit 11 - SDIO reset
    #[inline(always)]
    #[must_use]
    pub fn sdiorst(&mut self) -> SDIORST_W<11> {
        SDIORST_W::new(self)
    }
    ///Bit 12 - SPI 1 reset
    #[inline(always)]
    #[must_use]
    pub fn spi1rst(&mut self) -> SPI1RST_W<12> {
        SPI1RST_W::new(self)
    }
    ///Bit 14 - System configuration controller reset
    #[inline(always)]
    #[must_use]
    pub fn syscfgrst(&mut self) -> SYSCFGRST_W<14> {
        SYSCFGRST_W::new(self)
    }
    ///Bit 16 - TIM9 reset
    #[inline(always)]
    #[must_use]
    pub fn tim9rst(&mut self) -> TIM9RST_W<16> {
        TIM9RST_W::new(self)
    }
    ///Bit 17 - TIM10 reset
    #[inline(always)]
    #[must_use]
    pub fn tim10rst(&mut self) -> TIM10RST_W<17> {
        TIM10RST_W::new(self)
    }
    ///Bit 18 - TIM11 reset
    #[inline(always)]
    #[must_use]
    pub fn tim11rst(&mut self) -> TIM11RST_W<18> {
        TIM11RST_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///APB2 peripheral reset register
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
