///Register `CRH` reader
pub struct R(crate::R<CRH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRH_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CRH` writer
pub struct W(crate::W<CRH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRH_SPEC>;
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
impl From<crate::W<CRH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRH_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SECIE` reader - Second interrupt Enable
pub type SECIE_R = crate::BitReader<SECIE_A>;
///Second interrupt Enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SECIE_A {
    ///0: Second interrupt is masked
    Disabled = 0,
    ///1: Second interrupt is enabled
    Enabled = 1,
}
impl From<SECIE_A> for bool {
    #[inline(always)]
    fn from(variant: SECIE_A) -> Self {
        variant as u8 != 0
    }
}
impl SECIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SECIE_A {
        match self.bits {
            false => SECIE_A::Disabled,
            true => SECIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SECIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SECIE_A::Enabled
    }
}
///Field `SECIE` writer - Second interrupt Enable
pub type SECIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRH_SPEC, SECIE_A, O>;
impl<'a, const O: u8> SECIE_W<'a, O> {
    ///Second interrupt is masked
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SECIE_A::Disabled)
    }
    ///Second interrupt is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SECIE_A::Enabled)
    }
}
///Field `ALRIE` reader - Alarm interrupt Enable
pub type ALRIE_R = crate::BitReader<ALRIE_A>;
///Alarm interrupt Enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALRIE_A {
    ///0: Alarm interrupt is masked
    Disabled = 0,
    ///1: Alarm interrupt is enabled
    Enabled = 1,
}
impl From<ALRIE_A> for bool {
    #[inline(always)]
    fn from(variant: ALRIE_A) -> Self {
        variant as u8 != 0
    }
}
impl ALRIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ALRIE_A {
        match self.bits {
            false => ALRIE_A::Disabled,
            true => ALRIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ALRIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ALRIE_A::Enabled
    }
}
///Field `ALRIE` writer - Alarm interrupt Enable
pub type ALRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRH_SPEC, ALRIE_A, O>;
impl<'a, const O: u8> ALRIE_W<'a, O> {
    ///Alarm interrupt is masked
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ALRIE_A::Disabled)
    }
    ///Alarm interrupt is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ALRIE_A::Enabled)
    }
}
///Field `OWIE` reader - Overflow interrupt Enable
pub type OWIE_R = crate::BitReader<OWIE_A>;
///Overflow interrupt Enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OWIE_A {
    ///0: Overflow interrupt is masked
    Disabled = 0,
    ///1: Overflow interrupt is enabled
    Enabled = 1,
}
impl From<OWIE_A> for bool {
    #[inline(always)]
    fn from(variant: OWIE_A) -> Self {
        variant as u8 != 0
    }
}
impl OWIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OWIE_A {
        match self.bits {
            false => OWIE_A::Disabled,
            true => OWIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OWIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OWIE_A::Enabled
    }
}
///Field `OWIE` writer - Overflow interrupt Enable
pub type OWIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRH_SPEC, OWIE_A, O>;
impl<'a, const O: u8> OWIE_W<'a, O> {
    ///Overflow interrupt is masked
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OWIE_A::Disabled)
    }
    ///Overflow interrupt is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OWIE_A::Enabled)
    }
}
impl R {
    ///Bit 0 - Second interrupt Enable
    #[inline(always)]
    pub fn secie(&self) -> SECIE_R {
        SECIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Alarm interrupt Enable
    #[inline(always)]
    pub fn alrie(&self) -> ALRIE_R {
        ALRIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Overflow interrupt Enable
    #[inline(always)]
    pub fn owie(&self) -> OWIE_R {
        OWIE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Second interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn secie(&mut self) -> SECIE_W<0> {
        SECIE_W::new(self)
    }
    ///Bit 1 - Alarm interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn alrie(&mut self) -> ALRIE_W<1> {
        ALRIE_W::new(self)
    }
    ///Bit 2 - Overflow interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn owie(&mut self) -> OWIE_W<2> {
        OWIE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RTC Control Register High
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [crh](index.html) module
pub struct CRH_SPEC;
impl crate::RegisterSpec for CRH_SPEC {
    type Ux = u32;
}
///`read()` method returns [crh::R](R) reader structure
impl crate::Readable for CRH_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [crh::W](W) writer structure
impl crate::Writable for CRH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CRH to value 0
impl crate::Resettable for CRH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
