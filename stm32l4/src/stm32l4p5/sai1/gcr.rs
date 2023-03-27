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
///Field `SYNCIN` reader - Synchronization inputs
pub type SYNCIN_R = crate::FieldReader<u8, u8>;
///Field `SYNCIN` writer - Synchronization inputs
pub type SYNCIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GCR_SPEC, u8, u8, 2, O>;
///Field `SYNCOUT` reader - Synchronization outputs
pub type SYNCOUT_R = crate::FieldReader<u8, SYNCOUT_A>;
///Synchronization outputs
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SYNCOUT_A {
    ///0: No synchronization output signals. SYNCOUT\[1:0\]
    ///should be configured as “No synchronization output signals” when audio block is configured as SPDIF
    Disabled = 0,
    ///1: Block A used for further synchronization for others SAI
    BlockA = 1,
    ///2: Block B used for further synchronization for others SAI
    BlockB = 2,
}
impl From<SYNCOUT_A> for u8 {
    #[inline(always)]
    fn from(variant: SYNCOUT_A) -> Self {
        variant as _
    }
}
impl SYNCOUT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<SYNCOUT_A> {
        match self.bits {
            0 => Some(SYNCOUT_A::Disabled),
            1 => Some(SYNCOUT_A::BlockA),
            2 => Some(SYNCOUT_A::BlockB),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SYNCOUT_A::Disabled
    }
    ///Checks if the value of the field is `BlockA`
    #[inline(always)]
    pub fn is_block_a(&self) -> bool {
        *self == SYNCOUT_A::BlockA
    }
    ///Checks if the value of the field is `BlockB`
    #[inline(always)]
    pub fn is_block_b(&self) -> bool {
        *self == SYNCOUT_A::BlockB
    }
}
///Field `SYNCOUT` writer - Synchronization outputs
pub type SYNCOUT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GCR_SPEC, u8, SYNCOUT_A, 2, O>;
impl<'a, const O: u8> SYNCOUT_W<'a, O> {
    ///No synchronization output signals. SYNCOUT\[1:0\]
    ///should be configured as “No synchronization output signals” when audio block is configured as SPDIF
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SYNCOUT_A::Disabled)
    }
    ///Block A used for further synchronization for others SAI
    #[inline(always)]
    pub fn block_a(self) -> &'a mut W {
        self.variant(SYNCOUT_A::BlockA)
    }
    ///Block B used for further synchronization for others SAI
    #[inline(always)]
    pub fn block_b(self) -> &'a mut W {
        self.variant(SYNCOUT_A::BlockB)
    }
}
impl R {
    ///Bits 0:1 - Synchronization inputs
    #[inline(always)]
    pub fn syncin(&self) -> SYNCIN_R {
        SYNCIN_R::new((self.bits & 3) as u8)
    }
    ///Bits 4:5 - Synchronization outputs
    #[inline(always)]
    pub fn syncout(&self) -> SYNCOUT_R {
        SYNCOUT_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    ///Bits 0:1 - Synchronization inputs
    #[inline(always)]
    #[must_use]
    pub fn syncin(&mut self) -> SYNCIN_W<0> {
        SYNCIN_W::new(self)
    }
    ///Bits 4:5 - Synchronization outputs
    #[inline(always)]
    #[must_use]
    pub fn syncout(&mut self) -> SYNCOUT_W<4> {
        SYNCOUT_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Global configuration register
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
