///Register `CFGR3` reader
pub struct R(crate::R<CFGR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR3_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CFGR3` writer
pub struct W(crate::W<CFGR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR3_SPEC>;
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
impl From<crate::W<CFGR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR3_SPEC>) -> Self {
        W(writer)
    }
}
///Field `USART1SW` reader - USART1 clock source selection
pub type USART1SW_R = crate::FieldReader<u8, USART1SW_A>;
///USART1 clock source selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USART1SW_A {
    ///0: PCLK selected as USART clock source
    Pclk = 0,
    ///1: SYSCLK selected as USART clock source
    Sysclk = 1,
    ///2: LSE selected as USART clock source
    Lse = 2,
    ///3: HSI selected as USART clock source
    Hsi = 3,
}
impl From<USART1SW_A> for u8 {
    #[inline(always)]
    fn from(variant: USART1SW_A) -> Self {
        variant as _
    }
}
impl USART1SW_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> USART1SW_A {
        match self.bits {
            0 => USART1SW_A::Pclk,
            1 => USART1SW_A::Sysclk,
            2 => USART1SW_A::Lse,
            3 => USART1SW_A::Hsi,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Pclk`
    #[inline(always)]
    pub fn is_pclk(&self) -> bool {
        *self == USART1SW_A::Pclk
    }
    ///Checks if the value of the field is `Sysclk`
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == USART1SW_A::Sysclk
    }
    ///Checks if the value of the field is `Lse`
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == USART1SW_A::Lse
    }
    ///Checks if the value of the field is `Hsi`
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == USART1SW_A::Hsi
    }
}
///Field `USART1SW` writer - USART1 clock source selection
pub type USART1SW_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CFGR3_SPEC, u8, USART1SW_A, 2, O>;
impl<'a, const O: u8> USART1SW_W<'a, O> {
    ///PCLK selected as USART clock source
    #[inline(always)]
    pub fn pclk(self) -> &'a mut W {
        self.variant(USART1SW_A::Pclk)
    }
    ///SYSCLK selected as USART clock source
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(USART1SW_A::Sysclk)
    }
    ///LSE selected as USART clock source
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(USART1SW_A::Lse)
    }
    ///HSI selected as USART clock source
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(USART1SW_A::Hsi)
    }
}
///Field `I2C1SW` reader - I2C1 clock source selection
pub type I2C1SW_R = crate::BitReader<I2C1SW_A>;
///I2C1 clock source selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C1SW_A {
    ///0: HSI clock selected as I2C clock source
    Hsi = 0,
    ///1: SYSCLK clock selected as I2C clock source
    Sysclk = 1,
}
impl From<I2C1SW_A> for bool {
    #[inline(always)]
    fn from(variant: I2C1SW_A) -> Self {
        variant as u8 != 0
    }
}
impl I2C1SW_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> I2C1SW_A {
        match self.bits {
            false => I2C1SW_A::Hsi,
            true => I2C1SW_A::Sysclk,
        }
    }
    ///Checks if the value of the field is `Hsi`
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == I2C1SW_A::Hsi
    }
    ///Checks if the value of the field is `Sysclk`
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == I2C1SW_A::Sysclk
    }
}
///Field `I2C1SW` writer - I2C1 clock source selection
pub type I2C1SW_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR3_SPEC, I2C1SW_A, O>;
impl<'a, const O: u8> I2C1SW_W<'a, O> {
    ///HSI clock selected as I2C clock source
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(I2C1SW_A::Hsi)
    }
    ///SYSCLK clock selected as I2C clock source
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(I2C1SW_A::Sysclk)
    }
}
///Field `I2C2SW` reader - I2C2 clock source selection
pub use I2C1SW_R as I2C2SW_R;
///Field `I2C3SW` reader - I2C3 clock source selection
pub use I2C1SW_R as I2C3SW_R;
///Field `I2C2SW` writer - I2C2 clock source selection
pub use I2C1SW_W as I2C2SW_W;
///Field `I2C3SW` writer - I2C3 clock source selection
pub use I2C1SW_W as I2C3SW_W;
///Field `TIM1SW` reader - Timer1 clock source selection
pub type TIM1SW_R = crate::BitReader<TIM1SW_A>;
///Timer1 clock source selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM1SW_A {
    ///0: PCLK2 clock (doubled frequency when prescaled)
    Pclk2 = 0,
    ///1: PLL vco output (running up to 144 MHz)
    Pll = 1,
}
impl From<TIM1SW_A> for bool {
    #[inline(always)]
    fn from(variant: TIM1SW_A) -> Self {
        variant as u8 != 0
    }
}
impl TIM1SW_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TIM1SW_A {
        match self.bits {
            false => TIM1SW_A::Pclk2,
            true => TIM1SW_A::Pll,
        }
    }
    ///Checks if the value of the field is `Pclk2`
    #[inline(always)]
    pub fn is_pclk2(&self) -> bool {
        *self == TIM1SW_A::Pclk2
    }
    ///Checks if the value of the field is `Pll`
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == TIM1SW_A::Pll
    }
}
///Field `TIM1SW` writer - Timer1 clock source selection
pub type TIM1SW_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR3_SPEC, TIM1SW_A, O>;
impl<'a, const O: u8> TIM1SW_W<'a, O> {
    ///PCLK2 clock (doubled frequency when prescaled)
    #[inline(always)]
    pub fn pclk2(self) -> &'a mut W {
        self.variant(TIM1SW_A::Pclk2)
    }
    ///PLL vco output (running up to 144 MHz)
    #[inline(always)]
    pub fn pll(self) -> &'a mut W {
        self.variant(TIM1SW_A::Pll)
    }
}
///Field `TIM15SW` reader - Timer15 clock source selection
pub use TIM1SW_R as TIM15SW_R;
///Field `TIM16SW` reader - Timer16 clock source selection
pub use TIM1SW_R as TIM16SW_R;
///Field `TIM17SW` reader - Timer17 clock source selection
pub use TIM1SW_R as TIM17SW_R;
///Field `TIM2SW` reader - Timer2 clock source selection
pub use TIM1SW_R as TIM2SW_R;
///Field `TIM34SW` reader - Timer34 clock source selection
pub use TIM1SW_R as TIM34SW_R;
///Field `TIM15SW` writer - Timer15 clock source selection
pub use TIM1SW_W as TIM15SW_W;
///Field `TIM16SW` writer - Timer16 clock source selection
pub use TIM1SW_W as TIM16SW_W;
///Field `TIM17SW` writer - Timer17 clock source selection
pub use TIM1SW_W as TIM17SW_W;
///Field `TIM2SW` writer - Timer2 clock source selection
pub use TIM1SW_W as TIM2SW_W;
///Field `TIM34SW` writer - Timer34 clock source selection
pub use TIM1SW_W as TIM34SW_W;
///Field `USART2SW` reader - USART2 clock source selection
pub use USART1SW_R as USART2SW_R;
///Field `USART3SW` reader - USART3 clock source selection
pub use USART1SW_R as USART3SW_R;
///Field `UART4SW` reader - UART4 clock source selection
pub use USART1SW_R as UART4SW_R;
///Field `UART5SW` reader - UART5 clock source selection
pub use USART1SW_R as UART5SW_R;
///Field `USART2SW` writer - USART2 clock source selection
pub use USART1SW_W as USART2SW_W;
///Field `USART3SW` writer - USART3 clock source selection
pub use USART1SW_W as USART3SW_W;
///Field `UART4SW` writer - UART4 clock source selection
pub use USART1SW_W as UART4SW_W;
///Field `UART5SW` writer - UART5 clock source selection
pub use USART1SW_W as UART5SW_W;
impl R {
    ///Bits 0:1 - USART1 clock source selection
    #[inline(always)]
    pub fn usart1sw(&self) -> USART1SW_R {
        USART1SW_R::new((self.bits & 3) as u8)
    }
    ///Bit 4 - I2C1 clock source selection
    #[inline(always)]
    pub fn i2c1sw(&self) -> I2C1SW_R {
        I2C1SW_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - I2C2 clock source selection
    #[inline(always)]
    pub fn i2c2sw(&self) -> I2C2SW_R {
        I2C2SW_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - I2C3 clock source selection
    #[inline(always)]
    pub fn i2c3sw(&self) -> I2C3SW_R {
        I2C3SW_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - Timer1 clock source selection
    #[inline(always)]
    pub fn tim1sw(&self) -> TIM1SW_R {
        TIM1SW_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 10 - Timer15 clock source selection
    #[inline(always)]
    pub fn tim15sw(&self) -> TIM15SW_R {
        TIM15SW_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Timer16 clock source selection
    #[inline(always)]
    pub fn tim16sw(&self) -> TIM16SW_R {
        TIM16SW_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 13 - Timer17 clock source selection
    #[inline(always)]
    pub fn tim17sw(&self) -> TIM17SW_R {
        TIM17SW_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bits 16:17 - USART2 clock source selection
    #[inline(always)]
    pub fn usart2sw(&self) -> USART2SW_R {
        USART2SW_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:19 - USART3 clock source selection
    #[inline(always)]
    pub fn usart3sw(&self) -> USART3SW_R {
        USART3SW_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bits 20:21 - UART4 clock source selection
    #[inline(always)]
    pub fn uart4sw(&self) -> UART4SW_R {
        UART4SW_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bits 22:23 - UART5 clock source selection
    #[inline(always)]
    pub fn uart5sw(&self) -> UART5SW_R {
        UART5SW_R::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bit 24 - Timer2 clock source selection
    #[inline(always)]
    pub fn tim2sw(&self) -> TIM2SW_R {
        TIM2SW_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Timer34 clock source selection
    #[inline(always)]
    pub fn tim34sw(&self) -> TIM34SW_R {
        TIM34SW_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    ///Bits 0:1 - USART1 clock source selection
    #[inline(always)]
    #[must_use]
    pub fn usart1sw(&mut self) -> USART1SW_W<0> {
        USART1SW_W::new(self)
    }
    ///Bit 4 - I2C1 clock source selection
    #[inline(always)]
    #[must_use]
    pub fn i2c1sw(&mut self) -> I2C1SW_W<4> {
        I2C1SW_W::new(self)
    }
    ///Bit 5 - I2C2 clock source selection
    #[inline(always)]
    #[must_use]
    pub fn i2c2sw(&mut self) -> I2C2SW_W<5> {
        I2C2SW_W::new(self)
    }
    ///Bit 6 - I2C3 clock source selection
    #[inline(always)]
    #[must_use]
    pub fn i2c3sw(&mut self) -> I2C3SW_W<6> {
        I2C3SW_W::new(self)
    }
    ///Bit 8 - Timer1 clock source selection
    #[inline(always)]
    #[must_use]
    pub fn tim1sw(&mut self) -> TIM1SW_W<8> {
        TIM1SW_W::new(self)
    }
    ///Bit 10 - Timer15 clock source selection
    #[inline(always)]
    #[must_use]
    pub fn tim15sw(&mut self) -> TIM15SW_W<10> {
        TIM15SW_W::new(self)
    }
    ///Bit 11 - Timer16 clock source selection
    #[inline(always)]
    #[must_use]
    pub fn tim16sw(&mut self) -> TIM16SW_W<11> {
        TIM16SW_W::new(self)
    }
    ///Bit 13 - Timer17 clock source selection
    #[inline(always)]
    #[must_use]
    pub fn tim17sw(&mut self) -> TIM17SW_W<13> {
        TIM17SW_W::new(self)
    }
    ///Bits 16:17 - USART2 clock source selection
    #[inline(always)]
    #[must_use]
    pub fn usart2sw(&mut self) -> USART2SW_W<16> {
        USART2SW_W::new(self)
    }
    ///Bits 18:19 - USART3 clock source selection
    #[inline(always)]
    #[must_use]
    pub fn usart3sw(&mut self) -> USART3SW_W<18> {
        USART3SW_W::new(self)
    }
    ///Bits 20:21 - UART4 clock source selection
    #[inline(always)]
    #[must_use]
    pub fn uart4sw(&mut self) -> UART4SW_W<20> {
        UART4SW_W::new(self)
    }
    ///Bits 22:23 - UART5 clock source selection
    #[inline(always)]
    #[must_use]
    pub fn uart5sw(&mut self) -> UART5SW_W<22> {
        UART5SW_W::new(self)
    }
    ///Bit 24 - Timer2 clock source selection
    #[inline(always)]
    #[must_use]
    pub fn tim2sw(&mut self) -> TIM2SW_W<24> {
        TIM2SW_W::new(self)
    }
    ///Bit 25 - Timer34 clock source selection
    #[inline(always)]
    #[must_use]
    pub fn tim34sw(&mut self) -> TIM34SW_W<25> {
        TIM34SW_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Clock configuration register 3
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cfgr3](index.html) module
pub struct CFGR3_SPEC;
impl crate::RegisterSpec for CFGR3_SPEC {
    type Ux = u32;
}
///`read()` method returns [cfgr3::R](R) reader structure
impl crate::Readable for CFGR3_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cfgr3::W](W) writer structure
impl crate::Writable for CFGR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CFGR3 to value 0
impl crate::Resettable for CFGR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
