///Register `VVPBCCR` reader
pub struct R(crate::R<VVPBCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VVPBCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VVPBCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VVPBCCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `VVPBCCR` writer
pub struct W(crate::W<VVPBCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VVPBCCR_SPEC>;
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
impl From<crate::W<VVPBCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VVPBCCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `VBP` reader - Vertical back
pub type VBP_R = crate::FieldReader<u16, u16>;
///Field `VBP` writer - Vertical back
pub type VBP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VVPBCCR_SPEC, u16, u16, 10, O>;
impl R {
    ///Bits 0:9 - Vertical back
    #[inline(always)]
    pub fn vbp(&self) -> VBP_R {
        VBP_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    ///Bits 0:9 - Vertical back
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
///DSI Host video VBP current configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [vvpbccr](index.html) module
pub struct VVPBCCR_SPEC;
impl crate::RegisterSpec for VVPBCCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [vvpbccr::R](R) reader structure
impl crate::Readable for VVPBCCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [vvpbccr::W](W) writer structure
impl crate::Writable for VVPBCCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets VVPBCCR to value 0
impl crate::Resettable for VVPBCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
