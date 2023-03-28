///Register `EGR` reader
pub struct R(crate::R<EGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EGR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `EGR` writer
pub struct W(crate::W<EGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EGR_SPEC>;
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
impl From<crate::W<EGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EGR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `UG` reader - Update generation
pub type UG_R = crate::BitReader<UG_A>;
///Update generation
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UG_A {
    ///1: Re-initializes the timer counter and generates an update of the registers.
    Update = 1,
}
impl From<UG_A> for bool {
    #[inline(always)]
    fn from(variant: UG_A) -> Self {
        variant as u8 != 0
    }
}
impl UG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<UG_A> {
        match self.bits {
            true => Some(UG_A::Update),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Update`
    #[inline(always)]
    pub fn is_update(&self) -> bool {
        *self == UG_A::Update
    }
}
///Field `UG` writer - Update generation
pub type UG_W<'a, const O: u8> = crate::BitWriter<'a, u32, EGR_SPEC, UG_A, O>;
impl<'a, const O: u8> UG_W<'a, O> {
    ///Re-initializes the timer counter and generates an update of the registers.
    #[inline(always)]
    pub fn update(self) -> &'a mut W {
        self.variant(UG_A::Update)
    }
}
///Field `CC1G` reader - Capture/Compare 1 generation
pub type CC1G_R = crate::BitReader<bool>;
///Field `CC1G` writer - Capture/Compare 1 generation
pub type CC1G_W<'a, const O: u8> = crate::BitWriter<'a, u32, EGR_SPEC, bool, O>;
impl R {
    ///Bit 0 - Update generation
    #[inline(always)]
    pub fn ug(&self) -> UG_R {
        UG_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Capture/Compare 1 generation
    #[inline(always)]
    pub fn cc1g(&self) -> CC1G_R {
        CC1G_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Update generation
    #[inline(always)]
    #[must_use]
    pub fn ug(&mut self) -> UG_W<0> {
        UG_W::new(self)
    }
    ///Bit 1 - Capture/Compare 1 generation
    #[inline(always)]
    #[must_use]
    pub fn cc1g(&mut self) -> CC1G_W<1> {
        CC1G_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///event generation register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [egr](index.html) module
pub struct EGR_SPEC;
impl crate::RegisterSpec for EGR_SPEC {
    type Ux = u32;
}
///`read()` method returns [egr::R](R) reader structure
impl crate::Readable for EGR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [egr::W](W) writer structure
impl crate::Writable for EGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets EGR to value 0
impl crate::Resettable for EGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
