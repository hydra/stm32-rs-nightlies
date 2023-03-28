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
///Field `MR32` reader - Interrupt Mask on external/internal line 32
pub type MR32_R = crate::BitReader<MR32_A>;
///Interrupt Mask on external/internal line 32
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MR32_A {
    ///0: Interrupt request line is masked
    Masked = 0,
    ///1: Interrupt request line is unmasked
    Unmasked = 1,
}
impl From<MR32_A> for bool {
    #[inline(always)]
    fn from(variant: MR32_A) -> Self {
        variant as u8 != 0
    }
}
impl MR32_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MR32_A {
        match self.bits {
            false => MR32_A::Masked,
            true => MR32_A::Unmasked,
        }
    }
    ///Checks if the value of the field is `Masked`
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == MR32_A::Masked
    }
    ///Checks if the value of the field is `Unmasked`
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == MR32_A::Unmasked
    }
}
///Field `MR32` writer - Interrupt Mask on external/internal line 32
pub type MR32_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR2_SPEC, MR32_A, O>;
impl<'a, const O: u8> MR32_W<'a, O> {
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR32_A::Masked)
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR32_A::Unmasked)
    }
}
///Field `MR33` reader - Interrupt Mask on external/internal line 33
pub use MR32_R as MR33_R;
///Field `MR34` reader - Interrupt Mask on external/internal line 34
pub use MR32_R as MR34_R;
///Field `MR35` reader - Interrupt Mask on external/internal line 35
pub use MR32_R as MR35_R;
///Field `MR33` writer - Interrupt Mask on external/internal line 33
pub use MR32_W as MR33_W;
///Field `MR34` writer - Interrupt Mask on external/internal line 34
pub use MR32_W as MR34_W;
///Field `MR35` writer - Interrupt Mask on external/internal line 35
pub use MR32_W as MR35_W;
impl R {
    ///Bit 0 - Interrupt Mask on external/internal line 32
    #[inline(always)]
    pub fn mr32(&self) -> MR32_R {
        MR32_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Interrupt Mask on external/internal line 33
    #[inline(always)]
    pub fn mr33(&self) -> MR33_R {
        MR33_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Interrupt Mask on external/internal line 34
    #[inline(always)]
    pub fn mr34(&self) -> MR34_R {
        MR34_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Interrupt Mask on external/internal line 35
    #[inline(always)]
    pub fn mr35(&self) -> MR35_R {
        MR35_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Interrupt Mask on external/internal line 32
    #[inline(always)]
    #[must_use]
    pub fn mr32(&mut self) -> MR32_W<0> {
        MR32_W::new(self)
    }
    ///Bit 1 - Interrupt Mask on external/internal line 33
    #[inline(always)]
    #[must_use]
    pub fn mr33(&mut self) -> MR33_W<1> {
        MR33_W::new(self)
    }
    ///Bit 2 - Interrupt Mask on external/internal line 34
    #[inline(always)]
    #[must_use]
    pub fn mr34(&mut self) -> MR34_W<2> {
        MR34_W::new(self)
    }
    ///Bit 3 - Interrupt Mask on external/internal line 35
    #[inline(always)]
    #[must_use]
    pub fn mr35(&mut self) -> MR35_W<3> {
        MR35_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Interrupt mask register
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
///`reset()` method sets IMR2 to value 0xffff_fffe
impl crate::Resettable for IMR2_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_fffe;
}
