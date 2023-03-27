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
///Field `SYSCFGRST` reader - SYSCFGRST
pub type SYSCFGRST_R = crate::BitReader<SYSCFGRSTW_A>;
///SYSCFGRST
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYSCFGRSTW_A {
    ///1: Reset the module
    Reset = 1,
}
impl From<SYSCFGRSTW_A> for bool {
    #[inline(always)]
    fn from(variant: SYSCFGRSTW_A) -> Self {
        variant as u8 != 0
    }
}
impl SYSCFGRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<SYSCFGRSTW_A> {
        match self.bits {
            true => Some(SYSCFGRSTW_A::Reset),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Reset`
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SYSCFGRSTW_A::Reset
    }
}
///Field `SYSCFGRST` writer - SYSCFGRST
pub type SYSCFGRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2RSTR_SPEC, SYSCFGRSTW_A, O>;
impl<'a, const O: u8> SYSCFGRST_W<'a, O> {
    ///Reset the module
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(SYSCFGRSTW_A::Reset)
    }
}
///Field `TIM9RST` reader - TIM9RST
pub use SYSCFGRST_R as TIM9RST_R;
///Field `TM10RST` reader - TM10RST
pub use SYSCFGRST_R as TM10RST_R;
///Field `TM11RST` reader - TM11RST
pub use SYSCFGRST_R as TM11RST_R;
///Field `ADC1RST` reader - ADC1RST
pub use SYSCFGRST_R as ADC1RST_R;
///Field `SDIORST` reader - SDIORST
pub use SYSCFGRST_R as SDIORST_R;
///Field `SPI1RST` reader - SPI1RST
pub use SYSCFGRST_R as SPI1RST_R;
///Field `USART1RST` reader - USART1RST
pub use SYSCFGRST_R as USART1RST_R;
///Field `TIM9RST` writer - TIM9RST
pub use SYSCFGRST_W as TIM9RST_W;
///Field `TM10RST` writer - TM10RST
pub use SYSCFGRST_W as TM10RST_W;
///Field `TM11RST` writer - TM11RST
pub use SYSCFGRST_W as TM11RST_W;
///Field `ADC1RST` writer - ADC1RST
pub use SYSCFGRST_W as ADC1RST_W;
///Field `SDIORST` writer - SDIORST
pub use SYSCFGRST_W as SDIORST_W;
///Field `SPI1RST` writer - SPI1RST
pub use SYSCFGRST_W as SPI1RST_W;
///Field `USART1RST` writer - USART1RST
pub use SYSCFGRST_W as USART1RST_W;
impl R {
    ///Bit 0 - SYSCFGRST
    #[inline(always)]
    pub fn syscfgrst(&self) -> SYSCFGRST_R {
        SYSCFGRST_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - TIM9RST
    #[inline(always)]
    pub fn tim9rst(&self) -> TIM9RST_R {
        TIM9RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TM10RST
    #[inline(always)]
    pub fn tm10rst(&self) -> TM10RST_R {
        TM10RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TM11RST
    #[inline(always)]
    pub fn tm11rst(&self) -> TM11RST_R {
        TM11RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 9 - ADC1RST
    #[inline(always)]
    pub fn adc1rst(&self) -> ADC1RST_R {
        ADC1RST_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - SDIORST
    #[inline(always)]
    pub fn sdiorst(&self) -> SDIORST_R {
        SDIORST_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - SPI1RST
    #[inline(always)]
    pub fn spi1rst(&self) -> SPI1RST_R {
        SPI1RST_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - USART1RST
    #[inline(always)]
    pub fn usart1rst(&self) -> USART1RST_R {
        USART1RST_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - SYSCFGRST
    #[inline(always)]
    #[must_use]
    pub fn syscfgrst(&mut self) -> SYSCFGRST_W<0> {
        SYSCFGRST_W::new(self)
    }
    ///Bit 2 - TIM9RST
    #[inline(always)]
    #[must_use]
    pub fn tim9rst(&mut self) -> TIM9RST_W<2> {
        TIM9RST_W::new(self)
    }
    ///Bit 3 - TM10RST
    #[inline(always)]
    #[must_use]
    pub fn tm10rst(&mut self) -> TM10RST_W<3> {
        TM10RST_W::new(self)
    }
    ///Bit 4 - TM11RST
    #[inline(always)]
    #[must_use]
    pub fn tm11rst(&mut self) -> TM11RST_W<4> {
        TM11RST_W::new(self)
    }
    ///Bit 9 - ADC1RST
    #[inline(always)]
    #[must_use]
    pub fn adc1rst(&mut self) -> ADC1RST_W<9> {
        ADC1RST_W::new(self)
    }
    ///Bit 11 - SDIORST
    #[inline(always)]
    #[must_use]
    pub fn sdiorst(&mut self) -> SDIORST_W<11> {
        SDIORST_W::new(self)
    }
    ///Bit 12 - SPI1RST
    #[inline(always)]
    #[must_use]
    pub fn spi1rst(&mut self) -> SPI1RST_W<12> {
        SPI1RST_W::new(self)
    }
    ///Bit 14 - USART1RST
    #[inline(always)]
    #[must_use]
    pub fn usart1rst(&mut self) -> USART1RST_W<14> {
        USART1RST_W::new(self)
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
