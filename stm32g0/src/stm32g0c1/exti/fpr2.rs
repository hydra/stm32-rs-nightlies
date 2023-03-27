///Register `FPR2` reader
pub struct R(crate::R<FPR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FPR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FPR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FPR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FPR2` writer
pub struct W(crate::W<FPR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FPR2_SPEC>;
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
impl From<crate::W<FPR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FPR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `FPIF2` reader - Falling edge event pending for configurable line 34
pub type FPIF2_R = crate::BitReader<FPIF2R_A>;
///Falling edge event pending for configurable line 34
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPIF2R_A {
    ///0: No trigger request occurred
    NotPending = 0,
    ///1: Selected trigger request occurred
    Pending = 1,
}
impl From<FPIF2R_A> for bool {
    #[inline(always)]
    fn from(variant: FPIF2R_A) -> Self {
        variant as u8 != 0
    }
}
impl FPIF2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FPIF2R_A {
        match self.bits {
            false => FPIF2R_A::NotPending,
            true => FPIF2R_A::Pending,
        }
    }
    ///Checks if the value of the field is `NotPending`
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == FPIF2R_A::NotPending
    }
    ///Checks if the value of the field is `Pending`
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == FPIF2R_A::Pending
    }
}
///Falling edge event pending for configurable line 34
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPIF2W_AW {
    ///1: Clears pending bit
    Clear = 1,
}
impl From<FPIF2W_AW> for bool {
    #[inline(always)]
    fn from(variant: FPIF2W_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `FPIF2` writer - Falling edge event pending for configurable line 34
pub type FPIF2_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, FPR2_SPEC, FPIF2W_AW, O>;
impl<'a, const O: u8> FPIF2_W<'a, O> {
    ///Clears pending bit
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(FPIF2W_AW::Clear)
    }
}
impl R {
    ///Bit 2 - Falling edge event pending for configurable line 34
    #[inline(always)]
    pub fn fpif2(&self) -> FPIF2_R {
        FPIF2_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    ///Bit 2 - Falling edge event pending for configurable line 34
    #[inline(always)]
    #[must_use]
    pub fn fpif2(&mut self) -> FPIF2_W<2> {
        FPIF2_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///EXTI falling edge pending register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fpr2](index.html) module
pub struct FPR2_SPEC;
impl crate::RegisterSpec for FPR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [fpr2::R](R) reader structure
impl crate::Readable for FPR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [fpr2::W](W) writer structure
impl crate::Writable for FPR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x04;
}
///`reset()` method sets FPR2 to value 0
impl crate::Resettable for FPR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
