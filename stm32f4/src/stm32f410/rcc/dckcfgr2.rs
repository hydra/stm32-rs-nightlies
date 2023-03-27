///Register `DCKCFGR2` reader
pub struct R(crate::R<DCKCFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCKCFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCKCFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCKCFGR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DCKCFGR2` writer
pub struct W(crate::W<DCKCFGR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCKCFGR2_SPEC>;
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
impl From<crate::W<DCKCFGR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCKCFGR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `FMPI2C1SEL` reader - FMPI2C1 kernel clock source selection
pub type FMPI2C1SEL_R = crate::FieldReader<u8, FMPI2C1SEL_A>;
///FMPI2C1 kernel clock source selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FMPI2C1SEL_A {
    ///0: APB clock selected as I2C clock
    Apb = 0,
    ///1: System clock selected as I2C clock
    Sysclk = 1,
    ///2: HSI clock selected as I2C clock
    Hsi = 2,
}
impl From<FMPI2C1SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FMPI2C1SEL_A) -> Self {
        variant as _
    }
}
impl FMPI2C1SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<FMPI2C1SEL_A> {
        match self.bits {
            0 => Some(FMPI2C1SEL_A::Apb),
            1 => Some(FMPI2C1SEL_A::Sysclk),
            2 => Some(FMPI2C1SEL_A::Hsi),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Apb`
    #[inline(always)]
    pub fn is_apb(&self) -> bool {
        *self == FMPI2C1SEL_A::Apb
    }
    ///Checks if the value of the field is `Sysclk`
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == FMPI2C1SEL_A::Sysclk
    }
    ///Checks if the value of the field is `Hsi`
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == FMPI2C1SEL_A::Hsi
    }
}
///Field `FMPI2C1SEL` writer - FMPI2C1 kernel clock source selection
pub type FMPI2C1SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DCKCFGR2_SPEC, u8, FMPI2C1SEL_A, 2, O>;
impl<'a, const O: u8> FMPI2C1SEL_W<'a, O> {
    ///APB clock selected as I2C clock
    #[inline(always)]
    pub fn apb(self) -> &'a mut W {
        self.variant(FMPI2C1SEL_A::Apb)
    }
    ///System clock selected as I2C clock
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(FMPI2C1SEL_A::Sysclk)
    }
    ///HSI clock selected as I2C clock
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(FMPI2C1SEL_A::Hsi)
    }
}
///Field `LPTIM1SEL` reader - LPTIM1SEL
pub type LPTIM1SEL_R = crate::FieldReader<u8, LPTIM1SEL_A>;
///LPTIM1SEL
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LPTIM1SEL_A {
    ///0: APB1 clock (PCLK1) selected as LPTILM1 clock
    Apb1 = 0,
    ///1: LSI clock is selected as LPTILM1 clock
    Lsi = 1,
    ///2: HSI clock is selected as LPTILM1 clock
    Hsi = 2,
    ///3: LSE clock is selected as LPTILM1 clock
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
            0 => LPTIM1SEL_A::Apb1,
            1 => LPTIM1SEL_A::Lsi,
            2 => LPTIM1SEL_A::Hsi,
            3 => LPTIM1SEL_A::Lse,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Apb1`
    #[inline(always)]
    pub fn is_apb1(&self) -> bool {
        *self == LPTIM1SEL_A::Apb1
    }
    ///Checks if the value of the field is `Lsi`
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == LPTIM1SEL_A::Lsi
    }
    ///Checks if the value of the field is `Hsi`
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == LPTIM1SEL_A::Hsi
    }
    ///Checks if the value of the field is `Lse`
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == LPTIM1SEL_A::Lse
    }
}
///Field `LPTIM1SEL` writer - LPTIM1SEL
pub type LPTIM1SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, DCKCFGR2_SPEC, u8, LPTIM1SEL_A, 2, O>;
impl<'a, const O: u8> LPTIM1SEL_W<'a, O> {
    ///APB1 clock (PCLK1) selected as LPTILM1 clock
    #[inline(always)]
    pub fn apb1(self) -> &'a mut W {
        self.variant(LPTIM1SEL_A::Apb1)
    }
    ///LSI clock is selected as LPTILM1 clock
    #[inline(always)]
    pub fn lsi(self) -> &'a mut W {
        self.variant(LPTIM1SEL_A::Lsi)
    }
    ///HSI clock is selected as LPTILM1 clock
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(LPTIM1SEL_A::Hsi)
    }
    ///LSE clock is selected as LPTILM1 clock
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(LPTIM1SEL_A::Lse)
    }
}
impl R {
    ///Bits 22:23 - FMPI2C1 kernel clock source selection
    #[inline(always)]
    pub fn fmpi2c1sel(&self) -> FMPI2C1SEL_R {
        FMPI2C1SEL_R::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bits 30:31 - LPTIM1SEL
    #[inline(always)]
    pub fn lptim1sel(&self) -> LPTIM1SEL_R {
        LPTIM1SEL_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    ///Bits 22:23 - FMPI2C1 kernel clock source selection
    #[inline(always)]
    #[must_use]
    pub fn fmpi2c1sel(&mut self) -> FMPI2C1SEL_W<22> {
        FMPI2C1SEL_W::new(self)
    }
    ///Bits 30:31 - LPTIM1SEL
    #[inline(always)]
    #[must_use]
    pub fn lptim1sel(&mut self) -> LPTIM1SEL_W<30> {
        LPTIM1SEL_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DCKCFGR2 register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dckcfgr2](index.html) module
pub struct DCKCFGR2_SPEC;
impl crate::RegisterSpec for DCKCFGR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [dckcfgr2::R](R) reader structure
impl crate::Readable for DCKCFGR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dckcfgr2::W](W) writer structure
impl crate::Writable for DCKCFGR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DCKCFGR2 to value 0
impl crate::Resettable for DCKCFGR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
