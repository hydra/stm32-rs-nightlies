///Register `APB1RSTR1` reader
pub struct R(crate::R<APB1RSTR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB1RSTR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB1RSTR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB1RSTR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `APB1RSTR1` writer
pub struct W(crate::W<APB1RSTR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB1RSTR1_SPEC>;
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
impl From<crate::W<APB1RSTR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB1RSTR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TIM2RST` reader - TIM2 timer reset
pub type TIM2RST_R = crate::BitReader<TIM2RST_A>;
///TIM2 timer reset
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM2RST_A {
    ///0: No effect
    NoEffect = 0,
    ///1: Reset TIMx
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
    pub fn variant(&self) -> TIM2RST_A {
        match self.bits {
            false => TIM2RST_A::NoEffect,
            true => TIM2RST_A::Reset,
        }
    }
    ///Checks if the value of the field is `NoEffect`
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == TIM2RST_A::NoEffect
    }
    ///Checks if the value of the field is `Reset`
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == TIM2RST_A::Reset
    }
}
///Field `TIM2RST` writer - TIM2 timer reset
pub type TIM2RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR1_SPEC, TIM2RST_A, O>;
impl<'a, const O: u8> TIM2RST_W<'a, O> {
    ///No effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TIM2RST_A::NoEffect)
    }
    ///Reset TIMx
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM2RST_A::Reset)
    }
}
///Field `TIM3RST` reader - TIM3 timer reset
pub use TIM2RST_R as TIM3RST_R;
///Field `TIM4RST` reader - TIM3 timer reset
pub use TIM2RST_R as TIM4RST_R;
///Field `TIM5RST` reader - TIM5 timer reset
pub use TIM2RST_R as TIM5RST_R;
///Field `TIM6RST` reader - TIM6 timer reset
pub use TIM2RST_R as TIM6RST_R;
///Field `TIM7RST` reader - TIM7 timer reset
pub use TIM2RST_R as TIM7RST_R;
///Field `TIM3RST` writer - TIM3 timer reset
pub use TIM2RST_W as TIM3RST_W;
///Field `TIM4RST` writer - TIM3 timer reset
pub use TIM2RST_W as TIM4RST_W;
///Field `TIM5RST` writer - TIM5 timer reset
pub use TIM2RST_W as TIM5RST_W;
///Field `TIM6RST` writer - TIM6 timer reset
pub use TIM2RST_W as TIM6RST_W;
///Field `TIM7RST` writer - TIM7 timer reset
pub use TIM2RST_W as TIM7RST_W;
///Field `SPI2RST` reader - SPI2 reset
pub type SPI2RST_R = crate::BitReader<SPI2RST_A>;
///SPI2 reset
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPI2RST_A {
    ///0: No effect
    NoEffect = 0,
    ///1: Reset SPIx
    Reset = 1,
}
impl From<SPI2RST_A> for bool {
    #[inline(always)]
    fn from(variant: SPI2RST_A) -> Self {
        variant as u8 != 0
    }
}
impl SPI2RST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SPI2RST_A {
        match self.bits {
            false => SPI2RST_A::NoEffect,
            true => SPI2RST_A::Reset,
        }
    }
    ///Checks if the value of the field is `NoEffect`
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == SPI2RST_A::NoEffect
    }
    ///Checks if the value of the field is `Reset`
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SPI2RST_A::Reset
    }
}
///Field `SPI2RST` writer - SPI2 reset
pub type SPI2RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR1_SPEC, SPI2RST_A, O>;
impl<'a, const O: u8> SPI2RST_W<'a, O> {
    ///No effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SPI2RST_A::NoEffect)
    }
    ///Reset SPIx
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(SPI2RST_A::Reset)
    }
}
///Field `SPI3RST` reader - SPI3 reset
pub use SPI2RST_R as SPI3RST_R;
///Field `SPI3RST` writer - SPI3 reset
pub use SPI2RST_W as SPI3RST_W;
///Field `USART2RST` reader - USART2 reset
pub type USART2RST_R = crate::BitReader<USART2RST_A>;
///USART2 reset
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USART2RST_A {
    ///0: No effect
    NoEffect = 0,
    ///1: Reset UARTx
    Reset = 1,
}
impl From<USART2RST_A> for bool {
    #[inline(always)]
    fn from(variant: USART2RST_A) -> Self {
        variant as u8 != 0
    }
}
impl USART2RST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> USART2RST_A {
        match self.bits {
            false => USART2RST_A::NoEffect,
            true => USART2RST_A::Reset,
        }
    }
    ///Checks if the value of the field is `NoEffect`
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == USART2RST_A::NoEffect
    }
    ///Checks if the value of the field is `Reset`
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == USART2RST_A::Reset
    }
}
///Field `USART2RST` writer - USART2 reset
pub type USART2RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR1_SPEC, USART2RST_A, O>;
impl<'a, const O: u8> USART2RST_W<'a, O> {
    ///No effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(USART2RST_A::NoEffect)
    }
    ///Reset UARTx
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(USART2RST_A::Reset)
    }
}
///Field `USART3RST` reader - USART3 reset
pub use USART2RST_R as USART3RST_R;
///Field `USART3RST` writer - USART3 reset
pub use USART2RST_W as USART3RST_W;
///Field `UART4RST` reader - UART4 reset
pub type UART4RST_R = crate::BitReader<bool>;
///Field `UART4RST` writer - UART4 reset
pub type UART4RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR1_SPEC, bool, O>;
///Field `UART5RST` reader - UART5 reset
pub type UART5RST_R = crate::BitReader<bool>;
///Field `UART5RST` writer - UART5 reset
pub type UART5RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR1_SPEC, bool, O>;
///Field `I2C1RST` reader - I2C1 reset
pub type I2C1RST_R = crate::BitReader<I2C1RST_A>;
///I2C1 reset
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C1RST_A {
    ///0: No effect
    NoEffect = 0,
    ///1: Reset I2Cx
    Reset = 1,
}
impl From<I2C1RST_A> for bool {
    #[inline(always)]
    fn from(variant: I2C1RST_A) -> Self {
        variant as u8 != 0
    }
}
impl I2C1RST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> I2C1RST_A {
        match self.bits {
            false => I2C1RST_A::NoEffect,
            true => I2C1RST_A::Reset,
        }
    }
    ///Checks if the value of the field is `NoEffect`
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == I2C1RST_A::NoEffect
    }
    ///Checks if the value of the field is `Reset`
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == I2C1RST_A::Reset
    }
}
///Field `I2C1RST` writer - I2C1 reset
pub type I2C1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR1_SPEC, I2C1RST_A, O>;
impl<'a, const O: u8> I2C1RST_W<'a, O> {
    ///No effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(I2C1RST_A::NoEffect)
    }
    ///Reset I2Cx
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(I2C1RST_A::Reset)
    }
}
///Field `I2C2RST` reader - I2C2 reset
pub use I2C1RST_R as I2C2RST_R;
///Field `I2C3RST` reader - I2C3 reset
pub use I2C1RST_R as I2C3RST_R;
///Field `I2C2RST` writer - I2C2 reset
pub use I2C1RST_W as I2C2RST_W;
///Field `I2C3RST` writer - I2C3 reset
pub use I2C1RST_W as I2C3RST_W;
///Field `CRSRST` reader - CRS reset
pub type CRSRST_R = crate::BitReader<CRSRST_A>;
///CRS reset
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRSRST_A {
    ///0: No effect
    NoEffect = 0,
    ///1: Reset CRS
    Reset = 1,
}
impl From<CRSRST_A> for bool {
    #[inline(always)]
    fn from(variant: CRSRST_A) -> Self {
        variant as u8 != 0
    }
}
impl CRSRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CRSRST_A {
        match self.bits {
            false => CRSRST_A::NoEffect,
            true => CRSRST_A::Reset,
        }
    }
    ///Checks if the value of the field is `NoEffect`
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == CRSRST_A::NoEffect
    }
    ///Checks if the value of the field is `Reset`
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == CRSRST_A::Reset
    }
}
///Field `CRSRST` writer - CRS reset
pub type CRSRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR1_SPEC, CRSRST_A, O>;
impl<'a, const O: u8> CRSRST_W<'a, O> {
    ///No effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(CRSRST_A::NoEffect)
    }
    ///Reset CRS
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(CRSRST_A::Reset)
    }
}
///Field `CAN1RST` reader - CAN1 reset
pub type CAN1RST_R = crate::BitReader<CAN1RST_A>;
///CAN1 reset
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAN1RST_A {
    ///0: No effect
    NoEffect = 0,
    ///1: Reset CAN1
    Reset = 1,
}
impl From<CAN1RST_A> for bool {
    #[inline(always)]
    fn from(variant: CAN1RST_A) -> Self {
        variant as u8 != 0
    }
}
impl CAN1RST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CAN1RST_A {
        match self.bits {
            false => CAN1RST_A::NoEffect,
            true => CAN1RST_A::Reset,
        }
    }
    ///Checks if the value of the field is `NoEffect`
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == CAN1RST_A::NoEffect
    }
    ///Checks if the value of the field is `Reset`
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == CAN1RST_A::Reset
    }
}
///Field `CAN1RST` writer - CAN1 reset
pub type CAN1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR1_SPEC, CAN1RST_A, O>;
impl<'a, const O: u8> CAN1RST_W<'a, O> {
    ///No effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(CAN1RST_A::NoEffect)
    }
    ///Reset CAN1
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(CAN1RST_A::Reset)
    }
}
///Field `PWRRST` reader - Power interface reset
pub type PWRRST_R = crate::BitReader<PWRRST_A>;
///Power interface reset
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWRRST_A {
    ///0: No effect
    NoEffect = 0,
    ///1: Reset PWR
    Reset = 1,
}
impl From<PWRRST_A> for bool {
    #[inline(always)]
    fn from(variant: PWRRST_A) -> Self {
        variant as u8 != 0
    }
}
impl PWRRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PWRRST_A {
        match self.bits {
            false => PWRRST_A::NoEffect,
            true => PWRRST_A::Reset,
        }
    }
    ///Checks if the value of the field is `NoEffect`
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == PWRRST_A::NoEffect
    }
    ///Checks if the value of the field is `Reset`
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == PWRRST_A::Reset
    }
}
///Field `PWRRST` writer - Power interface reset
pub type PWRRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR1_SPEC, PWRRST_A, O>;
impl<'a, const O: u8> PWRRST_W<'a, O> {
    ///No effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(PWRRST_A::NoEffect)
    }
    ///Reset PWR
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(PWRRST_A::Reset)
    }
}
///Field `DAC1RST` reader - DAC1 interface reset
pub type DAC1RST_R = crate::BitReader<DAC1RST_A>;
///DAC1 interface reset
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DAC1RST_A {
    ///0: No effect
    NoEffect = 0,
    ///1: Reset DAC1
    Reset = 1,
}
impl From<DAC1RST_A> for bool {
    #[inline(always)]
    fn from(variant: DAC1RST_A) -> Self {
        variant as u8 != 0
    }
}
impl DAC1RST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DAC1RST_A {
        match self.bits {
            false => DAC1RST_A::NoEffect,
            true => DAC1RST_A::Reset,
        }
    }
    ///Checks if the value of the field is `NoEffect`
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == DAC1RST_A::NoEffect
    }
    ///Checks if the value of the field is `Reset`
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == DAC1RST_A::Reset
    }
}
///Field `DAC1RST` writer - DAC1 interface reset
pub type DAC1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR1_SPEC, DAC1RST_A, O>;
impl<'a, const O: u8> DAC1RST_W<'a, O> {
    ///No effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(DAC1RST_A::NoEffect)
    }
    ///Reset DAC1
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(DAC1RST_A::Reset)
    }
}
///Field `OPAMPRST` reader - OPAMP interface reset
pub type OPAMPRST_R = crate::BitReader<OPAMPRST_A>;
///OPAMP interface reset
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPAMPRST_A {
    ///0: No effect
    NoEffect = 0,
    ///1: Reset OPAMP
    Reset = 1,
}
impl From<OPAMPRST_A> for bool {
    #[inline(always)]
    fn from(variant: OPAMPRST_A) -> Self {
        variant as u8 != 0
    }
}
impl OPAMPRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OPAMPRST_A {
        match self.bits {
            false => OPAMPRST_A::NoEffect,
            true => OPAMPRST_A::Reset,
        }
    }
    ///Checks if the value of the field is `NoEffect`
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == OPAMPRST_A::NoEffect
    }
    ///Checks if the value of the field is `Reset`
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == OPAMPRST_A::Reset
    }
}
///Field `OPAMPRST` writer - OPAMP interface reset
pub type OPAMPRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR1_SPEC, OPAMPRST_A, O>;
impl<'a, const O: u8> OPAMPRST_W<'a, O> {
    ///No effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(OPAMPRST_A::NoEffect)
    }
    ///Reset OPAMP
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(OPAMPRST_A::Reset)
    }
}
///Field `LPTIM1RST` reader - Low Power Timer 1 reset
pub type LPTIM1RST_R = crate::BitReader<LPTIM1RST_A>;
///Low Power Timer 1 reset
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPTIM1RST_A {
    ///0: No effect
    NoEffect = 0,
    ///1: Reset LPTIM1
    Reset = 1,
}
impl From<LPTIM1RST_A> for bool {
    #[inline(always)]
    fn from(variant: LPTIM1RST_A) -> Self {
        variant as u8 != 0
    }
}
impl LPTIM1RST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LPTIM1RST_A {
        match self.bits {
            false => LPTIM1RST_A::NoEffect,
            true => LPTIM1RST_A::Reset,
        }
    }
    ///Checks if the value of the field is `NoEffect`
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == LPTIM1RST_A::NoEffect
    }
    ///Checks if the value of the field is `Reset`
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == LPTIM1RST_A::Reset
    }
}
///Field `LPTIM1RST` writer - Low Power Timer 1 reset
pub type LPTIM1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR1_SPEC, LPTIM1RST_A, O>;
impl<'a, const O: u8> LPTIM1RST_W<'a, O> {
    ///No effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(LPTIM1RST_A::NoEffect)
    }
    ///Reset LPTIM1
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(LPTIM1RST_A::Reset)
    }
}
impl R {
    ///Bit 0 - TIM2 timer reset
    #[inline(always)]
    pub fn tim2rst(&self) -> TIM2RST_R {
        TIM2RST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TIM3 timer reset
    #[inline(always)]
    pub fn tim3rst(&self) -> TIM3RST_R {
        TIM3RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TIM3 timer reset
    #[inline(always)]
    pub fn tim4rst(&self) -> TIM4RST_R {
        TIM4RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TIM5 timer reset
    #[inline(always)]
    pub fn tim5rst(&self) -> TIM5RST_R {
        TIM5RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TIM6 timer reset
    #[inline(always)]
    pub fn tim6rst(&self) -> TIM6RST_R {
        TIM6RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TIM7 timer reset
    #[inline(always)]
    pub fn tim7rst(&self) -> TIM7RST_R {
        TIM7RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 14 - SPI2 reset
    #[inline(always)]
    pub fn spi2rst(&self) -> SPI2RST_R {
        SPI2RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - SPI3 reset
    #[inline(always)]
    pub fn spi3rst(&self) -> SPI3RST_R {
        SPI3RST_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 17 - USART2 reset
    #[inline(always)]
    pub fn usart2rst(&self) -> USART2RST_R {
        USART2RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - USART3 reset
    #[inline(always)]
    pub fn usart3rst(&self) -> USART3RST_R {
        USART3RST_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - UART4 reset
    #[inline(always)]
    pub fn uart4rst(&self) -> UART4RST_R {
        UART4RST_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - UART5 reset
    #[inline(always)]
    pub fn uart5rst(&self) -> UART5RST_R {
        UART5RST_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - I2C1 reset
    #[inline(always)]
    pub fn i2c1rst(&self) -> I2C1RST_R {
        I2C1RST_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - I2C2 reset
    #[inline(always)]
    pub fn i2c2rst(&self) -> I2C2RST_R {
        I2C2RST_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - I2C3 reset
    #[inline(always)]
    pub fn i2c3rst(&self) -> I2C3RST_R {
        I2C3RST_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - CRS reset
    #[inline(always)]
    pub fn crsrst(&self) -> CRSRST_R {
        CRSRST_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - CAN1 reset
    #[inline(always)]
    pub fn can1rst(&self) -> CAN1RST_R {
        CAN1RST_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 28 - Power interface reset
    #[inline(always)]
    pub fn pwrrst(&self) -> PWRRST_R {
        PWRRST_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - DAC1 interface reset
    #[inline(always)]
    pub fn dac1rst(&self) -> DAC1RST_R {
        DAC1RST_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - OPAMP interface reset
    #[inline(always)]
    pub fn opamprst(&self) -> OPAMPRST_R {
        OPAMPRST_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Low Power Timer 1 reset
    #[inline(always)]
    pub fn lptim1rst(&self) -> LPTIM1RST_R {
        LPTIM1RST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - TIM2 timer reset
    #[inline(always)]
    #[must_use]
    pub fn tim2rst(&mut self) -> TIM2RST_W<0> {
        TIM2RST_W::new(self)
    }
    ///Bit 1 - TIM3 timer reset
    #[inline(always)]
    #[must_use]
    pub fn tim3rst(&mut self) -> TIM3RST_W<1> {
        TIM3RST_W::new(self)
    }
    ///Bit 2 - TIM3 timer reset
    #[inline(always)]
    #[must_use]
    pub fn tim4rst(&mut self) -> TIM4RST_W<2> {
        TIM4RST_W::new(self)
    }
    ///Bit 3 - TIM5 timer reset
    #[inline(always)]
    #[must_use]
    pub fn tim5rst(&mut self) -> TIM5RST_W<3> {
        TIM5RST_W::new(self)
    }
    ///Bit 4 - TIM6 timer reset
    #[inline(always)]
    #[must_use]
    pub fn tim6rst(&mut self) -> TIM6RST_W<4> {
        TIM6RST_W::new(self)
    }
    ///Bit 5 - TIM7 timer reset
    #[inline(always)]
    #[must_use]
    pub fn tim7rst(&mut self) -> TIM7RST_W<5> {
        TIM7RST_W::new(self)
    }
    ///Bit 14 - SPI2 reset
    #[inline(always)]
    #[must_use]
    pub fn spi2rst(&mut self) -> SPI2RST_W<14> {
        SPI2RST_W::new(self)
    }
    ///Bit 15 - SPI3 reset
    #[inline(always)]
    #[must_use]
    pub fn spi3rst(&mut self) -> SPI3RST_W<15> {
        SPI3RST_W::new(self)
    }
    ///Bit 17 - USART2 reset
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
    ///Bit 19 - UART4 reset
    #[inline(always)]
    #[must_use]
    pub fn uart4rst(&mut self) -> UART4RST_W<19> {
        UART4RST_W::new(self)
    }
    ///Bit 20 - UART5 reset
    #[inline(always)]
    #[must_use]
    pub fn uart5rst(&mut self) -> UART5RST_W<20> {
        UART5RST_W::new(self)
    }
    ///Bit 21 - I2C1 reset
    #[inline(always)]
    #[must_use]
    pub fn i2c1rst(&mut self) -> I2C1RST_W<21> {
        I2C1RST_W::new(self)
    }
    ///Bit 22 - I2C2 reset
    #[inline(always)]
    #[must_use]
    pub fn i2c2rst(&mut self) -> I2C2RST_W<22> {
        I2C2RST_W::new(self)
    }
    ///Bit 23 - I2C3 reset
    #[inline(always)]
    #[must_use]
    pub fn i2c3rst(&mut self) -> I2C3RST_W<23> {
        I2C3RST_W::new(self)
    }
    ///Bit 24 - CRS reset
    #[inline(always)]
    #[must_use]
    pub fn crsrst(&mut self) -> CRSRST_W<24> {
        CRSRST_W::new(self)
    }
    ///Bit 25 - CAN1 reset
    #[inline(always)]
    #[must_use]
    pub fn can1rst(&mut self) -> CAN1RST_W<25> {
        CAN1RST_W::new(self)
    }
    ///Bit 28 - Power interface reset
    #[inline(always)]
    #[must_use]
    pub fn pwrrst(&mut self) -> PWRRST_W<28> {
        PWRRST_W::new(self)
    }
    ///Bit 29 - DAC1 interface reset
    #[inline(always)]
    #[must_use]
    pub fn dac1rst(&mut self) -> DAC1RST_W<29> {
        DAC1RST_W::new(self)
    }
    ///Bit 30 - OPAMP interface reset
    #[inline(always)]
    #[must_use]
    pub fn opamprst(&mut self) -> OPAMPRST_W<30> {
        OPAMPRST_W::new(self)
    }
    ///Bit 31 - Low Power Timer 1 reset
    #[inline(always)]
    #[must_use]
    pub fn lptim1rst(&mut self) -> LPTIM1RST_W<31> {
        LPTIM1RST_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///APB1 peripheral reset register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [apb1rstr1](index.html) module
pub struct APB1RSTR1_SPEC;
impl crate::RegisterSpec for APB1RSTR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [apb1rstr1::R](R) reader structure
impl crate::Readable for APB1RSTR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [apb1rstr1::W](W) writer structure
impl crate::Writable for APB1RSTR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets APB1RSTR1 to value 0
impl crate::Resettable for APB1RSTR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
