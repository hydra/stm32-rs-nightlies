///Register `PR2` reader
pub struct R(crate::R<PR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PR2` writer
pub struct W(crate::W<PR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PR2_SPEC>;
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
impl From<crate::W<PR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PIF32` reader - Pending bit 32
pub type PIF32_R = crate::BitReader<PIF32R_A>;
///Pending bit 32
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIF32R_A {
    ///0: No trigger request occurred
    NotPending = 0,
    ///1: Selected trigger request occurred
    Pending = 1,
}
impl From<PIF32R_A> for bool {
    #[inline(always)]
    fn from(variant: PIF32R_A) -> Self {
        variant as u8 != 0
    }
}
impl PIF32_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PIF32R_A {
        match self.bits {
            false => PIF32R_A::NotPending,
            true => PIF32R_A::Pending,
        }
    }
    ///Checks if the value of the field is `NotPending`
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == PIF32R_A::NotPending
    }
    ///Checks if the value of the field is `Pending`
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == PIF32R_A::Pending
    }
}
///Pending bit 32
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIF32W_AW {
    ///1: Clears pending bit
    Clear = 1,
}
impl From<PIF32W_AW> for bool {
    #[inline(always)]
    fn from(variant: PIF32W_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `PIF32` writer - Pending bit 32
pub type PIF32_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, PR2_SPEC, PIF32W_AW, O>;
impl<'a, const O: u8> PIF32_W<'a, O> {
    ///Clears pending bit
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PIF32W_AW::Clear)
    }
}
///Field `PIF33` reader - Pending bit 33
pub use PIF32_R as PIF33_R;
///Field `PIF40` reader - Pending bit 40
pub use PIF32_R as PIF40_R;
///Field `PIF41` reader - Pending bit 41
pub use PIF32_R as PIF41_R;
///Field `PIF33` writer - Pending bit 33
pub use PIF32_W as PIF33_W;
///Field `PIF40` writer - Pending bit 40
pub use PIF32_W as PIF40_W;
///Field `PIF41` writer - Pending bit 41
pub use PIF32_W as PIF41_W;
impl R {
    ///Bit 0 - Pending bit 32
    #[inline(always)]
    pub fn pif32(&self) -> PIF32_R {
        PIF32_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Pending bit 33
    #[inline(always)]
    pub fn pif33(&self) -> PIF33_R {
        PIF33_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 8 - Pending bit 40
    #[inline(always)]
    pub fn pif40(&self) -> PIF40_R {
        PIF40_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Pending bit 41
    #[inline(always)]
    pub fn pif41(&self) -> PIF41_R {
        PIF41_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Pending bit 32
    #[inline(always)]
    #[must_use]
    pub fn pif32(&mut self) -> PIF32_W<0> {
        PIF32_W::new(self)
    }
    ///Bit 1 - Pending bit 33
    #[inline(always)]
    #[must_use]
    pub fn pif33(&mut self) -> PIF33_W<1> {
        PIF33_W::new(self)
    }
    ///Bit 8 - Pending bit 40
    #[inline(always)]
    #[must_use]
    pub fn pif40(&mut self) -> PIF40_W<8> {
        PIF40_W::new(self)
    }
    ///Bit 9 - Pending bit 41
    #[inline(always)]
    #[must_use]
    pub fn pif41(&mut self) -> PIF41_W<9> {
        PIF41_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Pending register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pr2](index.html) module
pub struct PR2_SPEC;
impl crate::RegisterSpec for PR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [pr2::R](R) reader structure
impl crate::Readable for PR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [pr2::W](W) writer structure
impl crate::Writable for PR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x0303;
}
///`reset()` method sets PR2 to value 0
impl crate::Resettable for PR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
