///Register `VVBPCR` reader
pub struct R(crate::R<VVBPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VVBPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VVBPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VVBPCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `VVBPCR` writer
pub struct W(crate::W<VVBPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VVBPCR_SPEC>;
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
impl From<crate::W<VVBPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VVBPCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `VBP` reader - Vertical Back-Porch duration
pub type VBP_R = crate::FieldReader<u16, u16>;
///Field `VBP` writer - Vertical Back-Porch duration
pub type VBP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VVBPCR_SPEC, u16, u16, 10, O>;
impl R {
    ///Bits 0:9 - Vertical Back-Porch duration
    #[inline(always)]
    pub fn vbp(&self) -> VBP_R {
        VBP_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    ///Bits 0:9 - Vertical Back-Porch duration
    #[inline(always)]
    #[must_use]
    pub fn vbp(&mut self) -> VBP_W<0> {
        VBP_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DSI Host Video VBP Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [vvbpcr](index.html) module
pub struct VVBPCR_SPEC;
impl crate::RegisterSpec for VVBPCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [vvbpcr::R](R) reader structure
impl crate::Readable for VVBPCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [vvbpcr::W](W) writer structure
impl crate::Writable for VVBPCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets VVBPCR to value 0
impl crate::Resettable for VVBPCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
