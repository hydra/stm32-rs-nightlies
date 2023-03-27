///Register `CCIPR2` reader
pub struct R(crate::R<CCIPR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCIPR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCIPR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCIPR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CCIPR2` writer
pub struct W(crate::W<CCIPR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCIPR2_SPEC>;
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
impl From<crate::W<CCIPR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCIPR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `I2C4SEL` reader - I2C4 clock source selection
pub type I2C4SEL_R = crate::FieldReader<u8, I2C4SEL_A>;
///I2C4 clock source selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2C4SEL_A {
    ///0: PCLK clock selected as I2C clock
    Pclk = 0,
    ///1: System clock (SYSCLK) selected as I2C clock
    System = 1,
    ///2: HSI16 clock selected as I2C clock
    Hsi16 = 2,
}
impl From<I2C4SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: I2C4SEL_A) -> Self {
        variant as _
    }
}
impl I2C4SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<I2C4SEL_A> {
        match self.bits {
            0 => Some(I2C4SEL_A::Pclk),
            1 => Some(I2C4SEL_A::System),
            2 => Some(I2C4SEL_A::Hsi16),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Pclk`
    #[inline(always)]
    pub fn is_pclk(&self) -> bool {
        *self == I2C4SEL_A::Pclk
    }
    ///Checks if the value of the field is `System`
    #[inline(always)]
    pub fn is_system(&self) -> bool {
        *self == I2C4SEL_A::System
    }
    ///Checks if the value of the field is `Hsi16`
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == I2C4SEL_A::Hsi16
    }
}
///Field `I2C4SEL` writer - I2C4 clock source selection
pub type I2C4SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCIPR2_SPEC, u8, I2C4SEL_A, 2, O>;
impl<'a, const O: u8> I2C4SEL_W<'a, O> {
    ///PCLK clock selected as I2C clock
    #[inline(always)]
    pub fn pclk(self) -> &'a mut W {
        self.variant(I2C4SEL_A::Pclk)
    }
    ///System clock (SYSCLK) selected as I2C clock
    #[inline(always)]
    pub fn system(self) -> &'a mut W {
        self.variant(I2C4SEL_A::System)
    }
    ///HSI16 clock selected as I2C clock
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut W {
        self.variant(I2C4SEL_A::Hsi16)
    }
}
///Field `QSPISEL` reader - Octospi clock source selection
pub type QSPISEL_R = crate::FieldReader<u8, QSPISEL_A>;
///Octospi clock source selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum QSPISEL_A {
    ///0: System clock selected as QUADSPI kernel clock
    System = 0,
    ///1: HSI16 clock selected as QUADSPI kernel clock
    Hsi16 = 1,
    ///2: PLL 'Q' clock selected as QUADSPI kernel clock
    Pllq = 2,
}
impl From<QSPISEL_A> for u8 {
    #[inline(always)]
    fn from(variant: QSPISEL_A) -> Self {
        variant as _
    }
}
impl QSPISEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<QSPISEL_A> {
        match self.bits {
            0 => Some(QSPISEL_A::System),
            1 => Some(QSPISEL_A::Hsi16),
            2 => Some(QSPISEL_A::Pllq),
            _ => None,
        }
    }
    ///Checks if the value of the field is `System`
    #[inline(always)]
    pub fn is_system(&self) -> bool {
        *self == QSPISEL_A::System
    }
    ///Checks if the value of the field is `Hsi16`
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == QSPISEL_A::Hsi16
    }
    ///Checks if the value of the field is `Pllq`
    #[inline(always)]
    pub fn is_pllq(&self) -> bool {
        *self == QSPISEL_A::Pllq
    }
}
///Field `QSPISEL` writer - Octospi clock source selection
pub type QSPISEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCIPR2_SPEC, u8, QSPISEL_A, 2, O>;
impl<'a, const O: u8> QSPISEL_W<'a, O> {
    ///System clock selected as QUADSPI kernel clock
    #[inline(always)]
    pub fn system(self) -> &'a mut W {
        self.variant(QSPISEL_A::System)
    }
    ///HSI16 clock selected as QUADSPI kernel clock
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut W {
        self.variant(QSPISEL_A::Hsi16)
    }
    ///PLL 'Q' clock selected as QUADSPI kernel clock
    #[inline(always)]
    pub fn pllq(self) -> &'a mut W {
        self.variant(QSPISEL_A::Pllq)
    }
}
impl R {
    ///Bits 0:1 - I2C4 clock source selection
    #[inline(always)]
    pub fn i2c4sel(&self) -> I2C4SEL_R {
        I2C4SEL_R::new((self.bits & 3) as u8)
    }
    ///Bits 20:21 - Octospi clock source selection
    #[inline(always)]
    pub fn qspisel(&self) -> QSPISEL_R {
        QSPISEL_R::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    ///Bits 0:1 - I2C4 clock source selection
    #[inline(always)]
    #[must_use]
    pub fn i2c4sel(&mut self) -> I2C4SEL_W<0> {
        I2C4SEL_W::new(self)
    }
    ///Bits 20:21 - Octospi clock source selection
    #[inline(always)]
    #[must_use]
    pub fn qspisel(&mut self) -> QSPISEL_W<20> {
        QSPISEL_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Peripherals independent clock configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ccipr2](index.html) module
pub struct CCIPR2_SPEC;
impl crate::RegisterSpec for CCIPR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [ccipr2::R](R) reader structure
impl crate::Readable for CCIPR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ccipr2::W](W) writer structure
impl crate::Writable for CCIPR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CCIPR2 to value 0
impl crate::Resettable for CCIPR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
