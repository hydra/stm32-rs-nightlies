///Register `IMR2` reader
pub struct R(crate::R<IMR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `IMR2` writer
pub struct W(crate::W<IMR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IMR2_SPEC>;
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
impl From<crate::W<IMR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IMR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `IM32` reader - CPU wakeup with interrupt mask on event input
pub type IM32_R = crate::BitReader<IM32_A>;
///CPU wakeup with interrupt mask on event input
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IM32_A {
    ///0: Interrupt request line is masked
    Masked = 0,
    ///1: Interrupt request line is unmasked
    Unmasked = 1,
}
impl From<IM32_A> for bool {
    #[inline(always)]
    fn from(variant: IM32_A) -> Self {
        variant as u8 != 0
    }
}
impl IM32_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> IM32_A {
        match self.bits {
            false => IM32_A::Masked,
            true => IM32_A::Unmasked,
        }
    }
    ///Checks if the value of the field is `Masked`
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == IM32_A::Masked
    }
    ///Checks if the value of the field is `Unmasked`
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == IM32_A::Unmasked
    }
}
///Field `IM32` writer - CPU wakeup with interrupt mask on event input
pub type IM32_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR2_SPEC, IM32_A, O>;
impl<'a, const O: u8> IM32_W<'a, O> {
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(IM32_A::Masked)
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(IM32_A::Unmasked)
    }
}
///Field `IM33` reader - CPU wakeup with interrupt mask on event input
pub use IM32_R as IM33_R;
///Field `IM33` writer - CPU wakeup with interrupt mask on event input
pub use IM32_W as IM33_W;
impl R {
    ///Bit 0 - CPU wakeup with interrupt mask on event input
    #[inline(always)]
    pub fn im32(&self) -> IM32_R {
        IM32_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CPU wakeup with interrupt mask on event input
    #[inline(always)]
    pub fn im33(&self) -> IM33_R {
        IM33_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - CPU wakeup with interrupt mask on event input
    #[inline(always)]
    #[must_use]
    pub fn im32(&mut self) -> IM32_W<0> {
        IM32_W::new(self)
    }
    ///Bit 1 - CPU wakeup with interrupt mask on event input
    #[inline(always)]
    #[must_use]
    pub fn im33(&mut self) -> IM33_W<1> {
        IM33_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///EXTI CPU wakeup with interrupt mask register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [imr2](index.html) module
pub struct IMR2_SPEC;
impl crate::RegisterSpec for IMR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [imr2::R](R) reader structure
impl crate::Readable for IMR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [imr2::W](W) writer structure
impl crate::Writable for IMR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets IMR2 to value 0xffff_ffff
impl crate::Resettable for IMR2_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
