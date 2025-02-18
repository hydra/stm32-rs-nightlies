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
///Field `FMCRST` reader - Flexible memory controller reset
pub type FMCRST_R = crate::BitReader<FMCRST_A>;
///Flexible memory controller reset
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FMCRST_A {
    ///1: Reset the selected module
    Reset = 1,
}
impl From<FMCRST_A> for bool {
    #[inline(always)]
    fn from(variant: FMCRST_A) -> Self {
        variant as u8 != 0
    }
}
impl FMCRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<FMCRST_A> {
        match self.bits {
            true => Some(FMCRST_A::Reset),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Reset`
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == FMCRST_A::Reset
    }
}
///Field `FMCRST` writer - Flexible memory controller reset
pub type FMCRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB3RSTR_SPEC, FMCRST_A, O>;
impl<'a, const O: u8> FMCRST_W<'a, O> {
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(FMCRST_A::Reset)
    }
}
///Field `OSPI1RST` reader - OSPI1RST
pub use FMCRST_R as OSPI1RST_R;
///Field `OSPI1RST` writer - OSPI1RST
pub use FMCRST_W as OSPI1RST_W;
impl R {
    ///Bit 0 - Flexible memory controller reset
    #[inline(always)]
    pub fn fmcrst(&self) -> FMCRST_R {
        FMCRST_R::new((self.bits & 1) != 0)
    }
    ///Bit 8 - OSPI1RST
    #[inline(always)]
    pub fn ospi1rst(&self) -> OSPI1RST_R {
        OSPI1RST_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Flexible memory controller reset
    #[inline(always)]
    #[must_use]
    pub fn fmcrst(&mut self) -> FMCRST_W<0> {
        FMCRST_W::new(self)
    }
    ///Bit 8 - OSPI1RST
    #[inline(always)]
    #[must_use]
    pub fn ospi1rst(&mut self) -> OSPI1RST_W<8> {
        OSPI1RST_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///AHB3 peripheral reset register
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
