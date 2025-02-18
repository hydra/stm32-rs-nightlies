///Register `CHSELR0` reader
pub struct R(crate::R<CHSELR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHSELR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHSELR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHSELR0_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CHSELR0` writer
pub struct W(crate::W<CHSELR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHSELR0_SPEC>;
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
impl From<crate::W<CHSELR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHSELR0_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CHSEL` reader - CHSEL
pub type CHSEL_R = crate::FieldReader<u32, CHSEL_A>;
///CHSEL
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum CHSEL_A {
    ///0: Input Channel is not selected for conversion
    NotSelected = 0,
    ///1: Input Channel is selected for conversion
    Selected = 1,
}
impl From<CHSEL_A> for u32 {
    #[inline(always)]
    fn from(variant: CHSEL_A) -> Self {
        variant as _
    }
}
impl CHSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<CHSEL_A> {
        match self.bits {
            0 => Some(CHSEL_A::NotSelected),
            1 => Some(CHSEL_A::Selected),
            _ => None,
        }
    }
    ///Checks if the value of the field is `NotSelected`
    #[inline(always)]
    pub fn is_not_selected(&self) -> bool {
        *self == CHSEL_A::NotSelected
    }
    ///Checks if the value of the field is `Selected`
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == CHSEL_A::Selected
    }
}
///Field `CHSEL` writer - CHSEL
pub type CHSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHSELR0_SPEC, u32, CHSEL_A, 18, O>;
impl<'a, const O: u8> CHSEL_W<'a, O> {
    ///Input Channel is not selected for conversion
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut W {
        self.variant(CHSEL_A::NotSelected)
    }
    ///Input Channel is selected for conversion
    #[inline(always)]
    pub fn selected(self) -> &'a mut W {
        self.variant(CHSEL_A::Selected)
    }
}
impl R {
    ///Bits 0:17 - CHSEL
    #[inline(always)]
    pub fn chsel(&self) -> CHSEL_R {
        CHSEL_R::new(self.bits & 0x0003_ffff)
    }
}
impl W {
    ///Bits 0:17 - CHSEL
    #[inline(always)]
    #[must_use]
    pub fn chsel(&mut self) -> CHSEL_W<0> {
        CHSEL_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///channel selection register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [chselr0](index.html) module
pub struct CHSELR0_SPEC;
impl crate::RegisterSpec for CHSELR0_SPEC {
    type Ux = u32;
}
///`read()` method returns [chselr0::R](R) reader structure
impl crate::Readable for CHSELR0_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [chselr0::W](W) writer structure
impl crate::Writable for CHSELR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CHSELR0 to value 0
impl crate::Resettable for CHSELR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
