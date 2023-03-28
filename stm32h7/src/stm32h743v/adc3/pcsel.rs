///Register `PCSEL` reader
pub struct R(crate::R<PCSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCSEL_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PCSEL` writer
pub struct W(crate::W<PCSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCSEL_SPEC>;
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
impl From<crate::W<PCSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCSEL_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PCSEL` reader - Channel x (VINP\[i\]) pre selection
pub type PCSEL_R = crate::FieldReader<u32, PCSEL_A>;
///Channel x (VINP\[i\]) pre selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum PCSEL_A {
    ///0: Input channel x is not pre-selected
    NotPreselected = 0,
    ///1: Pre-select input channel x
    Preselected = 1,
}
impl From<PCSEL_A> for u32 {
    #[inline(always)]
    fn from(variant: PCSEL_A) -> Self {
        variant as _
    }
}
impl PCSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<PCSEL_A> {
        match self.bits {
            0 => Some(PCSEL_A::NotPreselected),
            1 => Some(PCSEL_A::Preselected),
            _ => None,
        }
    }
    ///Checks if the value of the field is `NotPreselected`
    #[inline(always)]
    pub fn is_not_preselected(&self) -> bool {
        *self == PCSEL_A::NotPreselected
    }
    ///Checks if the value of the field is `Preselected`
    #[inline(always)]
    pub fn is_preselected(&self) -> bool {
        *self == PCSEL_A::Preselected
    }
}
///Field `PCSEL` writer - Channel x (VINP\[i\]) pre selection
pub type PCSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PCSEL_SPEC, u32, PCSEL_A, 20, O>;
impl<'a, const O: u8> PCSEL_W<'a, O> {
    ///Input channel x is not pre-selected
    #[inline(always)]
    pub fn not_preselected(self) -> &'a mut W {
        self.variant(PCSEL_A::NotPreselected)
    }
    ///Pre-select input channel x
    #[inline(always)]
    pub fn preselected(self) -> &'a mut W {
        self.variant(PCSEL_A::Preselected)
    }
}
impl R {
    ///Bits 0:19 - Channel x (VINP\[i\]) pre selection
    #[inline(always)]
    pub fn pcsel(&self) -> PCSEL_R {
        PCSEL_R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    ///Bits 0:19 - Channel x (VINP\[i\]) pre selection
    #[inline(always)]
    #[must_use]
    pub fn pcsel(&mut self) -> PCSEL_W<0> {
        PCSEL_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///ADC pre channel selection register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pcsel](index.html) module
pub struct PCSEL_SPEC;
impl crate::RegisterSpec for PCSEL_SPEC {
    type Ux = u32;
}
///`read()` method returns [pcsel::R](R) reader structure
impl crate::Readable for PCSEL_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [pcsel::W](W) writer structure
impl crate::Writable for PCSEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets PCSEL to value 0
impl crate::Resettable for PCSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
