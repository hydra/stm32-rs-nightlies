///Register `AF1` reader
pub struct R(crate::R<AF1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AF1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AF1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `AF1` writer
pub struct W(crate::W<AF1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AF1_SPEC>;
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
impl From<crate::W<AF1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AF1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ETRSEL` reader - External trigger source selection
pub type ETRSEL_R = crate::FieldReader<u8, ETRSEL_A>;
///External trigger source selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ETRSEL_A {
    ///0: ETR legacy mode
    Legacy = 0,
    ///1: COMP1 output
    Comp1 = 1,
    ///2: COMP2 output
    Comp2 = 2,
}
impl From<ETRSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: ETRSEL_A) -> Self {
        variant as _
    }
}
impl ETRSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<ETRSEL_A> {
        match self.bits {
            0 => Some(ETRSEL_A::Legacy),
            1 => Some(ETRSEL_A::Comp1),
            2 => Some(ETRSEL_A::Comp2),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Legacy`
    #[inline(always)]
    pub fn is_legacy(&self) -> bool {
        *self == ETRSEL_A::Legacy
    }
    ///Checks if the value of the field is `Comp1`
    #[inline(always)]
    pub fn is_comp1(&self) -> bool {
        *self == ETRSEL_A::Comp1
    }
    ///Checks if the value of the field is `Comp2`
    #[inline(always)]
    pub fn is_comp2(&self) -> bool {
        *self == ETRSEL_A::Comp2
    }
}
///Field `ETRSEL` writer - External trigger source selection
pub type ETRSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AF1_SPEC, u8, ETRSEL_A, 4, O>;
impl<'a, const O: u8> ETRSEL_W<'a, O> {
    ///ETR legacy mode
    #[inline(always)]
    pub fn legacy(self) -> &'a mut W {
        self.variant(ETRSEL_A::Legacy)
    }
    ///COMP1 output
    #[inline(always)]
    pub fn comp1(self) -> &'a mut W {
        self.variant(ETRSEL_A::Comp1)
    }
    ///COMP2 output
    #[inline(always)]
    pub fn comp2(self) -> &'a mut W {
        self.variant(ETRSEL_A::Comp2)
    }
}
impl R {
    ///Bits 14:17 - External trigger source selection
    #[inline(always)]
    pub fn etrsel(&self) -> ETRSEL_R {
        ETRSEL_R::new(((self.bits >> 14) & 0x0f) as u8)
    }
}
impl W {
    ///Bits 14:17 - External trigger source selection
    #[inline(always)]
    #[must_use]
    pub fn etrsel(&mut self) -> ETRSEL_W<14> {
        ETRSEL_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TIM2 alternate function option register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [af1](index.html) module
pub struct AF1_SPEC;
impl crate::RegisterSpec for AF1_SPEC {
    type Ux = u32;
}
///`read()` method returns [af1::R](R) reader structure
impl crate::Readable for AF1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [af1::W](W) writer structure
impl crate::Writable for AF1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets AF1 to value 0
impl crate::Resettable for AF1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
