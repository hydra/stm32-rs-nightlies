///Register `APB1LRSTR` reader
pub struct R(crate::R<APB1LRSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB1LRSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB1LRSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB1LRSTR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `APB1LRSTR` writer
pub struct W(crate::W<APB1LRSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB1LRSTR_SPEC>;
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
impl From<crate::W<APB1LRSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB1LRSTR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TIM2RST` reader - TIM block reset
pub type TIM2RST_R = crate::BitReader<TIM2RST_A>;
///TIM block reset
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
///Field `TIM2RST` writer - TIM block reset
pub type TIM2RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LRSTR_SPEC, TIM2RST_A, O>;
impl<'a, const O: u8> TIM2RST_W<'a, O> {
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM2RST_A::Reset)
    }
}
///Field `TIM3RST` reader - TIM block reset
pub use TIM2RST_R as TIM3RST_R;
///Field `TIM4RST` reader - TIM block reset
pub use TIM2RST_R as TIM4RST_R;
///Field `TIM5RST` reader - TIM block reset
pub use TIM2RST_R as TIM5RST_R;
///Field `TIM6RST` reader - TIM block reset
pub use TIM2RST_R as TIM6RST_R;
///Field `TIM7RST` reader - TIM block reset
pub use TIM2RST_R as TIM7RST_R;
///Field `TIM12RST` reader - TIM block reset
pub use TIM2RST_R as TIM12RST_R;
///Field `TIM13RST` reader - TIM block reset
pub use TIM2RST_R as TIM13RST_R;
///Field `TIM14RST` reader - TIM block reset
pub use TIM2RST_R as TIM14RST_R;
///Field `LPTIM1RST` reader - TIM block reset
pub use TIM2RST_R as LPTIM1RST_R;
///Field `SPI2RST` reader - SPI2 block reset
pub use TIM2RST_R as SPI2RST_R;
///Field `SPI3RST` reader - SPI3 block reset
pub use TIM2RST_R as SPI3RST_R;
///Field `SPDIFRXRST` reader - SPDIFRX block reset
pub use TIM2RST_R as SPDIFRXRST_R;
///Field `USART2RST` reader - USART2 block reset
pub use TIM2RST_R as USART2RST_R;
///Field `USART3RST` reader - USART3 block reset
pub use TIM2RST_R as USART3RST_R;
///Field `UART4RST` reader - UART4 block reset
pub use TIM2RST_R as UART4RST_R;
///Field `UART5RST` reader - UART5 block reset
pub use TIM2RST_R as UART5RST_R;
///Field `I2C1RST` reader - I2C1 block reset
pub use TIM2RST_R as I2C1RST_R;
///Field `I2C2RST` reader - I2C2 block reset
pub use TIM2RST_R as I2C2RST_R;
///Field `I2C3RST` reader - I2C3 block reset
pub use TIM2RST_R as I2C3RST_R;
///Field `CECRST` reader - HDMI-CEC block reset
pub use TIM2RST_R as CECRST_R;
///Field `DAC12RST` reader - DAC1 and 2 Blocks Reset
pub use TIM2RST_R as DAC12RST_R;
///Field `UART7RST` reader - UART7 block reset
pub use TIM2RST_R as UART7RST_R;
///Field `UART8RST` reader - UART8 block reset
pub use TIM2RST_R as UART8RST_R;
///Field `TIM3RST` writer - TIM block reset
pub use TIM2RST_W as TIM3RST_W;
///Field `TIM4RST` writer - TIM block reset
pub use TIM2RST_W as TIM4RST_W;
///Field `TIM5RST` writer - TIM block reset
pub use TIM2RST_W as TIM5RST_W;
///Field `TIM6RST` writer - TIM block reset
pub use TIM2RST_W as TIM6RST_W;
///Field `TIM7RST` writer - TIM block reset
pub use TIM2RST_W as TIM7RST_W;
///Field `TIM12RST` writer - TIM block reset
pub use TIM2RST_W as TIM12RST_W;
///Field `TIM13RST` writer - TIM block reset
pub use TIM2RST_W as TIM13RST_W;
///Field `TIM14RST` writer - TIM block reset
pub use TIM2RST_W as TIM14RST_W;
///Field `LPTIM1RST` writer - TIM block reset
pub use TIM2RST_W as LPTIM1RST_W;
///Field `SPI2RST` writer - SPI2 block reset
pub use TIM2RST_W as SPI2RST_W;
///Field `SPI3RST` writer - SPI3 block reset
pub use TIM2RST_W as SPI3RST_W;
///Field `SPDIFRXRST` writer - SPDIFRX block reset
pub use TIM2RST_W as SPDIFRXRST_W;
///Field `USART2RST` writer - USART2 block reset
pub use TIM2RST_W as USART2RST_W;
///Field `USART3RST` writer - USART3 block reset
pub use TIM2RST_W as USART3RST_W;
///Field `UART4RST` writer - UART4 block reset
pub use TIM2RST_W as UART4RST_W;
///Field `UART5RST` writer - UART5 block reset
pub use TIM2RST_W as UART5RST_W;
///Field `I2C1RST` writer - I2C1 block reset
pub use TIM2RST_W as I2C1RST_W;
///Field `I2C2RST` writer - I2C2 block reset
pub use TIM2RST_W as I2C2RST_W;
///Field `I2C3RST` writer - I2C3 block reset
pub use TIM2RST_W as I2C3RST_W;
///Field `CECRST` writer - HDMI-CEC block reset
pub use TIM2RST_W as CECRST_W;
///Field `DAC12RST` writer - DAC1 and 2 Blocks Reset
pub use TIM2RST_W as DAC12RST_W;
///Field `UART7RST` writer - UART7 block reset
pub use TIM2RST_W as UART7RST_W;
///Field `UART8RST` writer - UART8 block reset
pub use TIM2RST_W as UART8RST_W;
impl R {
    ///Bit 0 - TIM block reset
    #[inline(always)]
    pub fn tim2rst(&self) -> TIM2RST_R {
        TIM2RST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TIM block reset
    #[inline(always)]
    pub fn tim3rst(&self) -> TIM3RST_R {
        TIM3RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TIM block reset
    #[inline(always)]
    pub fn tim4rst(&self) -> TIM4RST_R {
        TIM4RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TIM block reset
    #[inline(always)]
    pub fn tim5rst(&self) -> TIM5RST_R {
        TIM5RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TIM block reset
    #[inline(always)]
    pub fn tim6rst(&self) -> TIM6RST_R {
        TIM6RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TIM block reset
    #[inline(always)]
    pub fn tim7rst(&self) -> TIM7RST_R {
        TIM7RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - TIM block reset
    #[inline(always)]
    pub fn tim12rst(&self) -> TIM12RST_R {
        TIM12RST_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - TIM block reset
    #[inline(always)]
    pub fn tim13rst(&self) -> TIM13RST_R {
        TIM13RST_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - TIM block reset
    #[inline(always)]
    pub fn tim14rst(&self) -> TIM14RST_R {
        TIM14RST_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - TIM block reset
    #[inline(always)]
    pub fn lptim1rst(&self) -> LPTIM1RST_R {
        LPTIM1RST_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 14 - SPI2 block reset
    #[inline(always)]
    pub fn spi2rst(&self) -> SPI2RST_R {
        SPI2RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - SPI3 block reset
    #[inline(always)]
    pub fn spi3rst(&self) -> SPI3RST_R {
        SPI3RST_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - SPDIFRX block reset
    #[inline(always)]
    pub fn spdifrxrst(&self) -> SPDIFRXRST_R {
        SPDIFRXRST_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - USART2 block reset
    #[inline(always)]
    pub fn usart2rst(&self) -> USART2RST_R {
        USART2RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - USART3 block reset
    #[inline(always)]
    pub fn usart3rst(&self) -> USART3RST_R {
        USART3RST_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - UART4 block reset
    #[inline(always)]
    pub fn uart4rst(&self) -> UART4RST_R {
        UART4RST_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - UART5 block reset
    #[inline(always)]
    pub fn uart5rst(&self) -> UART5RST_R {
        UART5RST_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - I2C1 block reset
    #[inline(always)]
    pub fn i2c1rst(&self) -> I2C1RST_R {
        I2C1RST_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - I2C2 block reset
    #[inline(always)]
    pub fn i2c2rst(&self) -> I2C2RST_R {
        I2C2RST_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - I2C3 block reset
    #[inline(always)]
    pub fn i2c3rst(&self) -> I2C3RST_R {
        I2C3RST_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 27 - HDMI-CEC block reset
    #[inline(always)]
    pub fn cecrst(&self) -> CECRST_R {
        CECRST_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 29 - DAC1 and 2 Blocks Reset
    #[inline(always)]
    pub fn dac12rst(&self) -> DAC12RST_R {
        DAC12RST_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - UART7 block reset
    #[inline(always)]
    pub fn uart7rst(&self) -> UART7RST_R {
        UART7RST_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - UART8 block reset
    #[inline(always)]
    pub fn uart8rst(&self) -> UART8RST_R {
        UART8RST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - TIM block reset
    #[inline(always)]
    #[must_use]
    pub fn tim2rst(&mut self) -> TIM2RST_W<0> {
        TIM2RST_W::new(self)
    }
    ///Bit 1 - TIM block reset
    #[inline(always)]
    #[must_use]
    pub fn tim3rst(&mut self) -> TIM3RST_W<1> {
        TIM3RST_W::new(self)
    }
    ///Bit 2 - TIM block reset
    #[inline(always)]
    #[must_use]
    pub fn tim4rst(&mut self) -> TIM4RST_W<2> {
        TIM4RST_W::new(self)
    }
    ///Bit 3 - TIM block reset
    #[inline(always)]
    #[must_use]
    pub fn tim5rst(&mut self) -> TIM5RST_W<3> {
        TIM5RST_W::new(self)
    }
    ///Bit 4 - TIM block reset
    #[inline(always)]
    #[must_use]
    pub fn tim6rst(&mut self) -> TIM6RST_W<4> {
        TIM6RST_W::new(self)
    }
    ///Bit 5 - TIM block reset
    #[inline(always)]
    #[must_use]
    pub fn tim7rst(&mut self) -> TIM7RST_W<5> {
        TIM7RST_W::new(self)
    }
    ///Bit 6 - TIM block reset
    #[inline(always)]
    #[must_use]
    pub fn tim12rst(&mut self) -> TIM12RST_W<6> {
        TIM12RST_W::new(self)
    }
    ///Bit 7 - TIM block reset
    #[inline(always)]
    #[must_use]
    pub fn tim13rst(&mut self) -> TIM13RST_W<7> {
        TIM13RST_W::new(self)
    }
    ///Bit 8 - TIM block reset
    #[inline(always)]
    #[must_use]
    pub fn tim14rst(&mut self) -> TIM14RST_W<8> {
        TIM14RST_W::new(self)
    }
    ///Bit 9 - TIM block reset
    #[inline(always)]
    #[must_use]
    pub fn lptim1rst(&mut self) -> LPTIM1RST_W<9> {
        LPTIM1RST_W::new(self)
    }
    ///Bit 14 - SPI2 block reset
    #[inline(always)]
    #[must_use]
    pub fn spi2rst(&mut self) -> SPI2RST_W<14> {
        SPI2RST_W::new(self)
    }
    ///Bit 15 - SPI3 block reset
    #[inline(always)]
    #[must_use]
    pub fn spi3rst(&mut self) -> SPI3RST_W<15> {
        SPI3RST_W::new(self)
    }
    ///Bit 16 - SPDIFRX block reset
    #[inline(always)]
    #[must_use]
    pub fn spdifrxrst(&mut self) -> SPDIFRXRST_W<16> {
        SPDIFRXRST_W::new(self)
    }
    ///Bit 17 - USART2 block reset
    #[inline(always)]
    #[must_use]
    pub fn usart2rst(&mut self) -> USART2RST_W<17> {
        USART2RST_W::new(self)
    }
    ///Bit 18 - USART3 block reset
    #[inline(always)]
    #[must_use]
    pub fn usart3rst(&mut self) -> USART3RST_W<18> {
        USART3RST_W::new(self)
    }
    ///Bit 19 - UART4 block reset
    #[inline(always)]
    #[must_use]
    pub fn uart4rst(&mut self) -> UART4RST_W<19> {
        UART4RST_W::new(self)
    }
    ///Bit 20 - UART5 block reset
    #[inline(always)]
    #[must_use]
    pub fn uart5rst(&mut self) -> UART5RST_W<20> {
        UART5RST_W::new(self)
    }
    ///Bit 21 - I2C1 block reset
    #[inline(always)]
    #[must_use]
    pub fn i2c1rst(&mut self) -> I2C1RST_W<21> {
        I2C1RST_W::new(self)
    }
    ///Bit 22 - I2C2 block reset
    #[inline(always)]
    #[must_use]
    pub fn i2c2rst(&mut self) -> I2C2RST_W<22> {
        I2C2RST_W::new(self)
    }
    ///Bit 23 - I2C3 block reset
    #[inline(always)]
    #[must_use]
    pub fn i2c3rst(&mut self) -> I2C3RST_W<23> {
        I2C3RST_W::new(self)
    }
    ///Bit 27 - HDMI-CEC block reset
    #[inline(always)]
    #[must_use]
    pub fn cecrst(&mut self) -> CECRST_W<27> {
        CECRST_W::new(self)
    }
    ///Bit 29 - DAC1 and 2 Blocks Reset
    #[inline(always)]
    #[must_use]
    pub fn dac12rst(&mut self) -> DAC12RST_W<29> {
        DAC12RST_W::new(self)
    }
    ///Bit 30 - UART7 block reset
    #[inline(always)]
    #[must_use]
    pub fn uart7rst(&mut self) -> UART7RST_W<30> {
        UART7RST_W::new(self)
    }
    ///Bit 31 - UART8 block reset
    #[inline(always)]
    #[must_use]
    pub fn uart8rst(&mut self) -> UART8RST_W<31> {
        UART8RST_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RCC APB1 Peripheral Reset Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [apb1lrstr](index.html) module
pub struct APB1LRSTR_SPEC;
impl crate::RegisterSpec for APB1LRSTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [apb1lrstr::R](R) reader structure
impl crate::Readable for APB1LRSTR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [apb1lrstr::W](W) writer structure
impl crate::Writable for APB1LRSTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets APB1LRSTR to value 0
impl crate::Resettable for APB1LRSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
