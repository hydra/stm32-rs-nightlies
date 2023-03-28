///Register `CCIPR` reader
pub struct R(crate::R<CCIPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCIPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCIPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCIPR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CCIPR` writer
pub struct W(crate::W<CCIPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCIPR_SPEC>;
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
impl From<crate::W<CCIPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCIPR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `USART1SEL` reader - USART1 clock source selection bits
pub type USART1SEL_R = crate::FieldReader<u8, USART1SEL_A>;
///USART1 clock source selection bits
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USART1SEL_A {
    ///0: APB clock selected as peripheral clock
    Apb = 0,
    ///1: System clock selected as peripheral clock
    System = 1,
    ///2: HSI16 clock selected as peripheral clock
    Hsi16 = 2,
    ///3: LSE clock selected as peripheral clock
    Lse = 3,
}
impl From<USART1SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: USART1SEL_A) -> Self {
        variant as _
    }
}
impl USART1SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> USART1SEL_A {
        match self.bits {
            0 => USART1SEL_A::Apb,
            1 => USART1SEL_A::System,
            2 => USART1SEL_A::Hsi16,
            3 => USART1SEL_A::Lse,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Apb`
    #[inline(always)]
    pub fn is_apb(&self) -> bool {
        *self == USART1SEL_A::Apb
    }
    ///Checks if the value of the field is `System`
    #[inline(always)]
    pub fn is_system(&self) -> bool {
        *self == USART1SEL_A::System
    }
    ///Checks if the value of the field is `Hsi16`
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == USART1SEL_A::Hsi16
    }
    ///Checks if the value of the field is `Lse`
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == USART1SEL_A::Lse
    }
}
///Field `USART1SEL` writer - USART1 clock source selection bits
pub type USART1SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CCIPR_SPEC, u8, USART1SEL_A, 2, O>;
impl<'a, const O: u8> USART1SEL_W<'a, O> {
    ///APB clock selected as peripheral clock
    #[inline(always)]
    pub fn apb(self) -> &'a mut W {
        self.variant(USART1SEL_A::Apb)
    }
    ///System clock selected as peripheral clock
    #[inline(always)]
    pub fn system(self) -> &'a mut W {
        self.variant(USART1SEL_A::System)
    }
    ///HSI16 clock selected as peripheral clock
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut W {
        self.variant(USART1SEL_A::Hsi16)
    }
    ///LSE clock selected as peripheral clock
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(USART1SEL_A::Lse)
    }
}
///Field `USART2SEL` reader - USART2 clock source selection bits
pub use USART1SEL_R as USART2SEL_R;
///Field `LPUART1SEL` reader - LPUART1 clock source selection bits
pub use USART1SEL_R as LPUART1SEL_R;
///Field `USART2SEL` writer - USART2 clock source selection bits
pub use USART1SEL_W as USART2SEL_W;
///Field `LPUART1SEL` writer - LPUART1 clock source selection bits
pub use USART1SEL_W as LPUART1SEL_W;
///Field `I2C1SEL` reader - I2C1 clock source selection bits
pub type I2C1SEL_R = crate::FieldReader<u8, I2C1SEL_A>;
///I2C1 clock source selection bits
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2C1SEL_A {
    ///0: APB clock selected as peripheral clock
    Apb = 0,
    ///1: System clock selected as peripheral clock
    System = 1,
    ///2: HSI16 clock selected as peripheral clock
    Hsi16 = 2,
}
impl From<I2C1SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: I2C1SEL_A) -> Self {
        variant as _
    }
}
impl I2C1SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<I2C1SEL_A> {
        match self.bits {
            0 => Some(I2C1SEL_A::Apb),
            1 => Some(I2C1SEL_A::System),
            2 => Some(I2C1SEL_A::Hsi16),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Apb`
    #[inline(always)]
    pub fn is_apb(&self) -> bool {
        *self == I2C1SEL_A::Apb
    }
    ///Checks if the value of the field is `System`
    #[inline(always)]
    pub fn is_system(&self) -> bool {
        *self == I2C1SEL_A::System
    }
    ///Checks if the value of the field is `Hsi16`
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == I2C1SEL_A::Hsi16
    }
}
///Field `I2C1SEL` writer - I2C1 clock source selection bits
pub type I2C1SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCIPR_SPEC, u8, I2C1SEL_A, 2, O>;
impl<'a, const O: u8> I2C1SEL_W<'a, O> {
    ///APB clock selected as peripheral clock
    #[inline(always)]
    pub fn apb(self) -> &'a mut W {
        self.variant(I2C1SEL_A::Apb)
    }
    ///System clock selected as peripheral clock
    #[inline(always)]
    pub fn system(self) -> &'a mut W {
        self.variant(I2C1SEL_A::System)
    }
    ///HSI16 clock selected as peripheral clock
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut W {
        self.variant(I2C1SEL_A::Hsi16)
    }
}
///Field `I2C3SEL` reader - I2C3 clock source selection bits
pub use I2C1SEL_R as I2C3SEL_R;
///Field `I2C3SEL` writer - I2C3 clock source selection bits
pub use I2C1SEL_W as I2C3SEL_W;
///Field `LPTIM1SEL` reader - Low Power Timer clock source selection bits
pub type LPTIM1SEL_R = crate::FieldReader<u8, LPTIM1SEL_A>;
///Low Power Timer clock source selection bits
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LPTIM1SEL_A {
    ///0: APB clock selected as Timer clock
    Apb = 0,
    ///1: LSI clock selected as Timer clock
    Lsi = 1,
    ///2: HSI16 clock selected as Timer clock
    Hsi16 = 2,
    ///3: LSE clock selected as Timer clock
    Lse = 3,
}
impl From<LPTIM1SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: LPTIM1SEL_A) -> Self {
        variant as _
    }
}
impl LPTIM1SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LPTIM1SEL_A {
        match self.bits {
            0 => LPTIM1SEL_A::Apb,
            1 => LPTIM1SEL_A::Lsi,
            2 => LPTIM1SEL_A::Hsi16,
            3 => LPTIM1SEL_A::Lse,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Apb`
    #[inline(always)]
    pub fn is_apb(&self) -> bool {
        *self == LPTIM1SEL_A::Apb
    }
    ///Checks if the value of the field is `Lsi`
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == LPTIM1SEL_A::Lsi
    }
    ///Checks if the value of the field is `Hsi16`
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == LPTIM1SEL_A::Hsi16
    }
    ///Checks if the value of the field is `Lse`
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == LPTIM1SEL_A::Lse
    }
}
///Field `LPTIM1SEL` writer - Low Power Timer clock source selection bits
pub type LPTIM1SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CCIPR_SPEC, u8, LPTIM1SEL_A, 2, O>;
impl<'a, const O: u8> LPTIM1SEL_W<'a, O> {
    ///APB clock selected as Timer clock
    #[inline(always)]
    pub fn apb(self) -> &'a mut W {
        self.variant(LPTIM1SEL_A::Apb)
    }
    ///LSI clock selected as Timer clock
    #[inline(always)]
    pub fn lsi(self) -> &'a mut W {
        self.variant(LPTIM1SEL_A::Lsi)
    }
    ///HSI16 clock selected as Timer clock
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut W {
        self.variant(LPTIM1SEL_A::Hsi16)
    }
    ///LSE clock selected as Timer clock
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(LPTIM1SEL_A::Lse)
    }
}
impl R {
    ///Bits 0:1 - USART1 clock source selection bits
    #[inline(always)]
    pub fn usart1sel(&self) -> USART1SEL_R {
        USART1SEL_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - USART2 clock source selection bits
    #[inline(always)]
    pub fn usart2sel(&self) -> USART2SEL_R {
        USART2SEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 10:11 - LPUART1 clock source selection bits
    #[inline(always)]
    pub fn lpuart1sel(&self) -> LPUART1SEL_R {
        LPUART1SEL_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:13 - I2C1 clock source selection bits
    #[inline(always)]
    pub fn i2c1sel(&self) -> I2C1SEL_R {
        I2C1SEL_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 16:17 - I2C3 clock source selection bits
    #[inline(always)]
    pub fn i2c3sel(&self) -> I2C3SEL_R {
        I2C3SEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:19 - Low Power Timer clock source selection bits
    #[inline(always)]
    pub fn lptim1sel(&self) -> LPTIM1SEL_R {
        LPTIM1SEL_R::new(((self.bits >> 18) & 3) as u8)
    }
}
impl W {
    ///Bits 0:1 - USART1 clock source selection bits
    #[inline(always)]
    #[must_use]
    pub fn usart1sel(&mut self) -> USART1SEL_W<0> {
        USART1SEL_W::new(self)
    }
    ///Bits 2:3 - USART2 clock source selection bits
    #[inline(always)]
    #[must_use]
    pub fn usart2sel(&mut self) -> USART2SEL_W<2> {
        USART2SEL_W::new(self)
    }
    ///Bits 10:11 - LPUART1 clock source selection bits
    #[inline(always)]
    #[must_use]
    pub fn lpuart1sel(&mut self) -> LPUART1SEL_W<10> {
        LPUART1SEL_W::new(self)
    }
    ///Bits 12:13 - I2C1 clock source selection bits
    #[inline(always)]
    #[must_use]
    pub fn i2c1sel(&mut self) -> I2C1SEL_W<12> {
        I2C1SEL_W::new(self)
    }
    ///Bits 16:17 - I2C3 clock source selection bits
    #[inline(always)]
    #[must_use]
    pub fn i2c3sel(&mut self) -> I2C3SEL_W<16> {
        I2C3SEL_W::new(self)
    }
    ///Bits 18:19 - Low Power Timer clock source selection bits
    #[inline(always)]
    #[must_use]
    pub fn lptim1sel(&mut self) -> LPTIM1SEL_W<18> {
        LPTIM1SEL_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Clock configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ccipr](index.html) module
pub struct CCIPR_SPEC;
impl crate::RegisterSpec for CCIPR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ccipr::R](R) reader structure
impl crate::Readable for CCIPR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ccipr::W](W) writer structure
impl crate::Writable for CCIPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CCIPR to value 0
impl crate::Resettable for CCIPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
