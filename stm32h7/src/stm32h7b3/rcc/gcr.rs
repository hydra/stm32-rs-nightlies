///Register `GCR` reader
pub struct R(crate::R<GCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `GCR` writer
pub struct W(crate::W<GCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GCR_SPEC>;
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
impl From<crate::W<GCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `WW1RSC` reader - WWDG1 reset scope control
pub type WW1RSC_R = crate::BitReader<WW1RSC_A>;
///WWDG1 reset scope control
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WW1RSC_A {
    ///0: Clear WWDG1 scope control
    Clear = 0,
    ///1: Set WWDG1 scope control
    Set = 1,
}
impl From<WW1RSC_A> for bool {
    #[inline(always)]
    fn from(variant: WW1RSC_A) -> Self {
        variant as u8 != 0
    }
}
impl WW1RSC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> WW1RSC_A {
        match self.bits {
            false => WW1RSC_A::Clear,
            true => WW1RSC_A::Set,
        }
    }
    ///Checks if the value of the field is `Clear`
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == WW1RSC_A::Clear
    }
    ///Checks if the value of the field is `Set`
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == WW1RSC_A::Set
    }
}
///Field `WW1RSC` writer - WWDG1 reset scope control
pub type WW1RSC_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCR_SPEC, WW1RSC_A, O>;
impl<'a, const O: u8> WW1RSC_W<'a, O> {
    ///Clear WWDG1 scope control
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(WW1RSC_A::Clear)
    }
    ///Set WWDG1 scope control
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(WW1RSC_A::Set)
    }
}
impl R {
    ///Bit 0 - WWDG1 reset scope control
    #[inline(always)]
    pub fn ww1rsc(&self) -> WW1RSC_R {
        WW1RSC_R::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - WWDG1 reset scope control
    #[inline(always)]
    #[must_use]
    pub fn ww1rsc(&mut self) -> WW1RSC_W<0> {
        WW1RSC_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Global Control Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [gcr](index.html) module
pub struct GCR_SPEC;
impl crate::RegisterSpec for GCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [gcr::R](R) reader structure
impl crate::Readable for GCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [gcr::W](W) writer structure
impl crate::Writable for GCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets GCR to value 0
impl crate::Resettable for GCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
