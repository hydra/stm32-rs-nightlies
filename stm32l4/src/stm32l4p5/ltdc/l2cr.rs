///Register `L2CR` reader
pub struct R(crate::R<L2CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L2CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L2CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L2CR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `L2CR` writer
pub struct W(crate::W<L2CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<L2CR_SPEC>;
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
impl From<crate::W<L2CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<L2CR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `LEN` reader - Layer Enable
pub type LEN_R = crate::BitReader<LEN_A>;
///Layer Enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LEN_A {
    ///0: Layer disabled
    Disabled = 0,
    ///1: Layer enabled
    Enabled = 1,
}
impl From<LEN_A> for bool {
    #[inline(always)]
    fn from(variant: LEN_A) -> Self {
        variant as u8 != 0
    }
}
impl LEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LEN_A {
        match self.bits {
            false => LEN_A::Disabled,
            true => LEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LEN_A::Enabled
    }
}
///Field `LEN` writer - Layer Enable
pub type LEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, L2CR_SPEC, LEN_A, O>;
impl<'a, const O: u8> LEN_W<'a, O> {
    ///Layer disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LEN_A::Disabled)
    }
    ///Layer enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LEN_A::Enabled)
    }
}
///Field `COLKEN` reader - Color Keying Enable
pub type COLKEN_R = crate::BitReader<COLKEN_A>;
///Color Keying Enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COLKEN_A {
    ///0: Color keying disabled
    Disabled = 0,
    ///1: Color keying enabled
    Enabled = 1,
}
impl From<COLKEN_A> for bool {
    #[inline(always)]
    fn from(variant: COLKEN_A) -> Self {
        variant as u8 != 0
    }
}
impl COLKEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> COLKEN_A {
        match self.bits {
            false => COLKEN_A::Disabled,
            true => COLKEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == COLKEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == COLKEN_A::Enabled
    }
}
///Field `COLKEN` writer - Color Keying Enable
pub type COLKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, L2CR_SPEC, COLKEN_A, O>;
impl<'a, const O: u8> COLKEN_W<'a, O> {
    ///Color keying disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COLKEN_A::Disabled)
    }
    ///Color keying enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COLKEN_A::Enabled)
    }
}
///Field `CLUTEN` reader - Color Look-Up Table Enable
pub type CLUTEN_R = crate::BitReader<CLUTEN_A>;
///Color Look-Up Table Enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLUTEN_A {
    ///0: Color look-up table disabled
    Disabled = 0,
    ///1: Color look-up table enabled
    Enabled = 1,
}
impl From<CLUTEN_A> for bool {
    #[inline(always)]
    fn from(variant: CLUTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CLUTEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CLUTEN_A {
        match self.bits {
            false => CLUTEN_A::Disabled,
            true => CLUTEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CLUTEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CLUTEN_A::Enabled
    }
}
///Field `CLUTEN` writer - Color Look-Up Table Enable
pub type CLUTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, L2CR_SPEC, CLUTEN_A, O>;
impl<'a, const O: u8> CLUTEN_W<'a, O> {
    ///Color look-up table disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CLUTEN_A::Disabled)
    }
    ///Color look-up table enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CLUTEN_A::Enabled)
    }
}
impl R {
    ///Bit 0 - Layer Enable
    #[inline(always)]
    pub fn len(&self) -> LEN_R {
        LEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Color Keying Enable
    #[inline(always)]
    pub fn colken(&self) -> COLKEN_R {
        COLKEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 4 - Color Look-Up Table Enable
    #[inline(always)]
    pub fn cluten(&self) -> CLUTEN_R {
        CLUTEN_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Layer Enable
    #[inline(always)]
    #[must_use]
    pub fn len(&mut self) -> LEN_W<0> {
        LEN_W::new(self)
    }
    ///Bit 1 - Color Keying Enable
    #[inline(always)]
    #[must_use]
    pub fn colken(&mut self) -> COLKEN_W<1> {
        COLKEN_W::new(self)
    }
    ///Bit 4 - Color Look-Up Table Enable
    #[inline(always)]
    #[must_use]
    pub fn cluten(&mut self) -> CLUTEN_W<4> {
        CLUTEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///LTDC Layer Control Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [l2cr](index.html) module
pub struct L2CR_SPEC;
impl crate::RegisterSpec for L2CR_SPEC {
    type Ux = u32;
}
///`read()` method returns [l2cr::R](R) reader structure
impl crate::Readable for L2CR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [l2cr::W](W) writer structure
impl crate::Writable for L2CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets L2CR to value 0
impl crate::Resettable for L2CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
