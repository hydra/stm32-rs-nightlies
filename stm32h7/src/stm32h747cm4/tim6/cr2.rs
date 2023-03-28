///Register `CR2` reader
pub struct R(crate::R<CR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CR2` writer
pub struct W(crate::W<CR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR2_SPEC>;
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
impl From<crate::W<CR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MMS` reader - Master mode selection
pub type MMS_R = crate::FieldReader<u8, MMS_A>;
///Master mode selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MMS_A {
    ///0: Use UG bit from TIMx_EGR register
    Reset = 0,
    ///1: Use CNT bit from TIMx_CEN register
    Enable = 1,
    ///2: Use the update event
    Update = 2,
}
impl From<MMS_A> for u8 {
    #[inline(always)]
    fn from(variant: MMS_A) -> Self {
        variant as _
    }
}
impl MMS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<MMS_A> {
        match self.bits {
            0 => Some(MMS_A::Reset),
            1 => Some(MMS_A::Enable),
            2 => Some(MMS_A::Update),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Reset`
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == MMS_A::Reset
    }
    ///Checks if the value of the field is `Enable`
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MMS_A::Enable
    }
    ///Checks if the value of the field is `Update`
    #[inline(always)]
    pub fn is_update(&self) -> bool {
        *self == MMS_A::Update
    }
}
///Field `MMS` writer - Master mode selection
pub type MMS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR2_SPEC, u8, MMS_A, 3, O>;
impl<'a, const O: u8> MMS_W<'a, O> {
    ///Use UG bit from TIMx_EGR register
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(MMS_A::Reset)
    }
    ///Use CNT bit from TIMx_CEN register
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MMS_A::Enable)
    }
    ///Use the update event
    #[inline(always)]
    pub fn update(self) -> &'a mut W {
        self.variant(MMS_A::Update)
    }
}
impl R {
    ///Bits 4:6 - Master mode selection
    #[inline(always)]
    pub fn mms(&self) -> MMS_R {
        MMS_R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    ///Bits 4:6 - Master mode selection
    #[inline(always)]
    #[must_use]
    pub fn mms(&mut self) -> MMS_W<4> {
        MMS_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///control register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr2](index.html) module
pub struct CR2_SPEC;
impl crate::RegisterSpec for CR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [cr2::R](R) reader structure
impl crate::Readable for CR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cr2::W](W) writer structure
impl crate::Writable for CR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CR2 to value 0
impl crate::Resettable for CR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
