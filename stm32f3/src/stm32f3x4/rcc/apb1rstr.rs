///Register `APB1RSTR` reader
pub struct R(crate::R<APB1RSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB1RSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB1RSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB1RSTR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `APB1RSTR` writer
pub struct W(crate::W<APB1RSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB1RSTR_SPEC>;
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
impl From<crate::W<APB1RSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB1RSTR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TIM2RST` reader - Timer 2 reset
pub type TIM2RST_R = crate::BitReader<TIM2RST_A>;
///Timer 2 reset
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM2RST_A {
    ///1: Reset the selected module
    Reset = 1,
}
impl From<TIM2RST_A> for bool {
    #[inline(always)]
    fn from(variant: TIM2RST_A) -> Self {
        variant as u8 != 0
    }
}
impl TIM2RST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<TIM2RST_A> {
        match self.bits {
            true => Some(TIM2RST_A::Reset),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Reset`
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == TIM2RST_A::Reset
    }
}
///Field `TIM2RST` writer - Timer 2 reset
pub type TIM2RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR_SPEC, TIM2RST_A, O>;
impl<'a, const O: u8> TIM2RST_W<'a, O> {
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM2RST_A::Reset)
    }
}
///Field `TIM3RST` reader - Timer 3 reset
pub use TIM2RST_R as TIM3RST_R;
///Field `TIM6RST` reader - Timer 6 reset
pub use TIM2RST_R as TIM6RST_R;
///Field `TIM7RST` reader - Timer 7 reset
pub use TIM2RST_R as TIM7RST_R;
///Field `WWDGRST` reader - Window watchdog reset
pub use TIM2RST_R as WWDGRST_R;
///Field `USART2RST` reader - USART 2 reset
pub use TIM2RST_R as USART2RST_R;
///Field `USART3RST` reader - USART3 reset
pub use TIM2RST_R as USART3RST_R;
///Field `I2C1RST` reader - I2C1 reset
pub use TIM2RST_R as I2C1RST_R;
///Field `CANRST` reader - CAN reset
pub use TIM2RST_R as CANRST_R;
///Field `DAC2RST` reader - DAC2 interface reset
pub use TIM2RST_R as DAC2RST_R;
///Field `PWRRST` reader - Power interface reset
pub use TIM2RST_R as PWRRST_R;
///Field `DAC1RST` reader - DAC interface reset
pub use TIM2RST_R as DAC1RST_R;
///Field `TIM3RST` writer - Timer 3 reset
pub use TIM2RST_W as TIM3RST_W;
///Field `TIM6RST` writer - Timer 6 reset
pub use TIM2RST_W as TIM6RST_W;
///Field `TIM7RST` writer - Timer 7 reset
pub use TIM2RST_W as TIM7RST_W;
///Field `WWDGRST` writer - Window watchdog reset
pub use TIM2RST_W as WWDGRST_W;
///Field `USART2RST` writer - USART 2 reset
pub use TIM2RST_W as USART2RST_W;
///Field `USART3RST` writer - USART3 reset
pub use TIM2RST_W as USART3RST_W;
///Field `I2C1RST` writer - I2C1 reset
pub use TIM2RST_W as I2C1RST_W;
///Field `CANRST` writer - CAN reset
pub use TIM2RST_W as CANRST_W;
///Field `DAC2RST` writer - DAC2 interface reset
pub use TIM2RST_W as DAC2RST_W;
///Field `PWRRST` writer - Power interface reset
pub use TIM2RST_W as PWRRST_W;
///Field `DAC1RST` writer - DAC interface reset
pub use TIM2RST_W as DAC1RST_W;
impl R {
    ///Bit 0 - Timer 2 reset
    #[inline(always)]
    pub fn tim2rst(&self) -> TIM2RST_R {
        TIM2RST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Timer 3 reset
    #[inline(always)]
    pub fn tim3rst(&self) -> TIM3RST_R {
        TIM3RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 4 - Timer 6 reset
    #[inline(always)]
    pub fn tim6rst(&self) -> TIM6RST_R {
        TIM6RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Timer 7 reset
    #[inline(always)]
    pub fn tim7rst(&self) -> TIM7RST_R {
        TIM7RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 11 - Window watchdog reset
    #[inline(always)]
    pub fn wwdgrst(&self) -> WWDGRST_R {
        WWDGRST_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 17 - USART 2 reset
    #[inline(always)]
    pub fn usart2rst(&self) -> USART2RST_R {
        USART2RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - USART3 reset
    #[inline(always)]
    pub fn usart3rst(&self) -> USART3RST_R {
        USART3RST_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 21 - I2C1 reset
    #[inline(always)]
    pub fn i2c1rst(&self) -> I2C1RST_R {
        I2C1RST_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 25 - CAN reset
    #[inline(always)]
    pub fn canrst(&self) -> CANRST_R {
        CANRST_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - DAC2 interface reset
    #[inline(always)]
    pub fn dac2rst(&self) -> DAC2RST_R {
        DAC2RST_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 28 - Power interface reset
    #[inline(always)]
    pub fn pwrrst(&self) -> PWRRST_R {
        PWRRST_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - DAC interface reset
    #[inline(always)]
    pub fn dac1rst(&self) -> DAC1RST_R {
        DAC1RST_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Timer 2 reset
    #[inline(always)]
    #[must_use]
    pub fn tim2rst(&mut self) -> TIM2RST_W<0> {
        TIM2RST_W::new(self)
    }
    ///Bit 1 - Timer 3 reset
    #[inline(always)]
    #[must_use]
    pub fn tim3rst(&mut self) -> TIM3RST_W<1> {
        TIM3RST_W::new(self)
    }
    ///Bit 4 - Timer 6 reset
    #[inline(always)]
    #[must_use]
    pub fn tim6rst(&mut self) -> TIM6RST_W<4> {
        TIM6RST_W::new(self)
    }
    ///Bit 5 - Timer 7 reset
    #[inline(always)]
    #[must_use]
    pub fn tim7rst(&mut self) -> TIM7RST_W<5> {
        TIM7RST_W::new(self)
    }
    ///Bit 11 - Window watchdog reset
    #[inline(always)]
    #[must_use]
    pub fn wwdgrst(&mut self) -> WWDGRST_W<11> {
        WWDGRST_W::new(self)
    }
    ///Bit 17 - USART 2 reset
    #[inline(always)]
    #[must_use]
    pub fn usart2rst(&mut self) -> USART2RST_W<17> {
        USART2RST_W::new(self)
    }
    ///Bit 18 - USART3 reset
    #[inline(always)]
    #[must_use]
    pub fn usart3rst(&mut self) -> USART3RST_W<18> {
        USART3RST_W::new(self)
    }
    ///Bit 21 - I2C1 reset
    #[inline(always)]
    #[must_use]
    pub fn i2c1rst(&mut self) -> I2C1RST_W<21> {
        I2C1RST_W::new(self)
    }
    ///Bit 25 - CAN reset
    #[inline(always)]
    #[must_use]
    pub fn canrst(&mut self) -> CANRST_W<25> {
        CANRST_W::new(self)
    }
    ///Bit 26 - DAC2 interface reset
    #[inline(always)]
    #[must_use]
    pub fn dac2rst(&mut self) -> DAC2RST_W<26> {
        DAC2RST_W::new(self)
    }
    ///Bit 28 - Power interface reset
    #[inline(always)]
    #[must_use]
    pub fn pwrrst(&mut self) -> PWRRST_W<28> {
        PWRRST_W::new(self)
    }
    ///Bit 29 - DAC interface reset
    #[inline(always)]
    #[must_use]
    pub fn dac1rst(&mut self) -> DAC1RST_W<29> {
        DAC1RST_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///APB1 peripheral reset register (RCC_APB1RSTR)
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [apb1rstr](index.html) module
pub struct APB1RSTR_SPEC;
impl crate::RegisterSpec for APB1RSTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [apb1rstr::R](R) reader structure
impl crate::Readable for APB1RSTR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [apb1rstr::W](W) writer structure
impl crate::Writable for APB1RSTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets APB1RSTR to value 0
impl crate::Resettable for APB1RSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
