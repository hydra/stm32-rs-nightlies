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
///Field `CK48MSEL` reader - SDIO/USBFS clock selection
pub type CK48MSEL_R = crate::BitReader<CK48MSEL_A>;
///SDIO/USBFS clock selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CK48MSEL_A {
    ///0: 48MHz clock from PLL is selected
    Pll = 0,
    ///1: 48MHz clock from PLLSAI is selected
    Pllsai = 1,
}
impl From<CK48MSEL_A> for bool {
    #[inline(always)]
    fn from(variant: CK48MSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl CK48MSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CK48MSEL_A {
        match self.bits {
            false => CK48MSEL_A::Pll,
            true => CK48MSEL_A::Pllsai,
        }
    }
    ///Checks if the value of the field is `Pll`
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == CK48MSEL_A::Pll
    }
    ///Checks if the value of the field is `Pllsai`
    #[inline(always)]
    pub fn is_pllsai(&self) -> bool {
        *self == CK48MSEL_A::Pllsai
    }
}
///Field `CK48MSEL` writer - SDIO/USBFS clock selection
pub type CK48MSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCKCFGR2_SPEC, CK48MSEL_A, O>;
impl<'a, const O: u8> CK48MSEL_W<'a, O> {
    ///48MHz clock from PLL is selected
    #[inline(always)]
    pub fn pll(self) -> &'a mut W {
        self.variant(CK48MSEL_A::Pll)
    }
    ///48MHz clock from PLLSAI is selected
    #[inline(always)]
    pub fn pllsai(self) -> &'a mut W {
        self.variant(CK48MSEL_A::Pllsai)
    }
}
///Field `SDIOSEL` reader - SDIO clock selection
pub type SDIOSEL_R = crate::BitReader<SDIOSEL_A>;
///SDIO clock selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDIOSEL_A {
    ///0: 48 MHz clock is selected as SD clock
    Ck48m = 0,
    ///1: System clock is selected as SD clock
    Sysclk = 1,
}
impl From<SDIOSEL_A> for bool {
    #[inline(always)]
    fn from(variant: SDIOSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl SDIOSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SDIOSEL_A {
        match self.bits {
            false => SDIOSEL_A::Ck48m,
            true => SDIOSEL_A::Sysclk,
        }
    }
    ///Checks if the value of the field is `Ck48m`
    #[inline(always)]
    pub fn is_ck48m(&self) -> bool {
        *self == SDIOSEL_A::Ck48m
    }
    ///Checks if the value of the field is `Sysclk`
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == SDIOSEL_A::Sysclk
    }
}
///Field `SDIOSEL` writer - SDIO clock selection
pub type SDIOSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCKCFGR2_SPEC, SDIOSEL_A, O>;
impl<'a, const O: u8> SDIOSEL_W<'a, O> {
    ///48 MHz clock is selected as SD clock
    #[inline(always)]
    pub fn ck48m(self) -> &'a mut W {
        self.variant(SDIOSEL_A::Ck48m)
    }
    ///System clock is selected as SD clock
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(SDIOSEL_A::Sysclk)
    }
}
impl R {
    ///Bits 22:23 - FMPI2C1 kernel clock source selection
    #[inline(always)]
    pub fn fmpi2c1sel(&self) -> FMPI2C1SEL_R {
        FMPI2C1SEL_R::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bit 27 - SDIO/USBFS clock selection
    #[inline(always)]
    pub fn ck48msel(&self) -> CK48MSEL_R {
        CK48MSEL_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - SDIO clock selection
    #[inline(always)]
    pub fn sdiosel(&self) -> SDIOSEL_R {
        SDIOSEL_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    ///Bits 22:23 - FMPI2C1 kernel clock source selection
    #[inline(always)]
    #[must_use]
    pub fn fmpi2c1sel(&mut self) -> FMPI2C1SEL_W<22> {
        FMPI2C1SEL_W::new(self)
    }
    ///Bit 27 - SDIO/USBFS clock selection
    #[inline(always)]
    #[must_use]
    pub fn ck48msel(&mut self) -> CK48MSEL_W<27> {
        CK48MSEL_W::new(self)
    }
    ///Bit 28 - SDIO clock selection
    #[inline(always)]
    #[must_use]
    pub fn sdiosel(&mut self) -> SDIOSEL_W<28> {
        SDIOSEL_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Dedicated Clock Configuration Register
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
