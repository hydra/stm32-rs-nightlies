///Register `AHB3RSTR` reader
pub struct R(crate::R<AHB3RSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB3RSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB3RSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB3RSTR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `AHB3RSTR` writer
pub struct W(crate::W<AHB3RSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB3RSTR_SPEC>;
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
impl From<crate::W<AHB3RSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB3RSTR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `FSMCRST` reader - Flexible memory controller module reset
pub type FSMCRST_R = crate::BitReader<FSMCRST_A>;
///Flexible memory controller module reset
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FSMCRST_A {
    ///1: Reset the selected module
    Reset = 1,
}
impl From<FSMCRST_A> for bool {
    #[inline(always)]
    fn from(variant: FSMCRST_A) -> Self {
        variant as u8 != 0
    }
}
impl FSMCRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<FSMCRST_A> {
        match self.bits {
            true => Some(FSMCRST_A::Reset),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Reset`
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == FSMCRST_A::Reset
    }
}
///Field `FSMCRST` writer - Flexible memory controller module reset
pub type FSMCRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB3RSTR_SPEC, FSMCRST_A, O>;
impl<'a, const O: u8> FSMCRST_W<'a, O> {
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(FSMCRST_A::Reset)
    }
}
///Field `QSPIRST` reader - QUADSPI module reset
pub use FSMCRST_R as QSPIRST_R;
///Field `QSPIRST` writer - QUADSPI module reset
pub use FSMCRST_W as QSPIRST_W;
impl R {
    ///Bit 0 - Flexible memory controller module reset
    #[inline(always)]
    pub fn fsmcrst(&self) -> FSMCRST_R {
        FSMCRST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - QUADSPI module reset
    #[inline(always)]
    pub fn qspirst(&self) -> QSPIRST_R {
        QSPIRST_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Flexible memory controller module reset
    #[inline(always)]
    #[must_use]
    pub fn fsmcrst(&mut self) -> FSMCRST_W<0> {
        FSMCRST_W::new(self)
    }
    ///Bit 1 - QUADSPI module reset
    #[inline(always)]
    #[must_use]
    pub fn qspirst(&mut self) -> QSPIRST_W<1> {
        QSPIRST_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///peripheral reset register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ahb3rstr](index.html) module
pub struct AHB3RSTR_SPEC;
impl crate::RegisterSpec for AHB3RSTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ahb3rstr::R](R) reader structure
impl crate::Readable for AHB3RSTR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ahb3rstr::W](W) writer structure
impl crate::Writable for AHB3RSTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets AHB3RSTR to value 0
impl crate::Resettable for AHB3RSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
