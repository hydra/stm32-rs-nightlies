///Register `VPCCR` reader
pub struct R(crate::R<VPCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VPCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VPCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VPCCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `VPCCR` writer
pub struct W(crate::W<VPCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VPCCR_SPEC>;
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
impl From<crate::W<VPCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VPCCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `VPSIZE` reader - Video packet size
pub type VPSIZE_R = crate::FieldReader<u16, u16>;
///Field `VPSIZE` writer - Video packet size
pub type VPSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VPCCR_SPEC, u16, u16, 14, O>;
impl R {
    ///Bits 0:13 - Video packet size
    #[inline(always)]
    pub fn vpsize(&self) -> VPSIZE_R {
        VPSIZE_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    ///Bits 0:13 - Video packet size
    #[inline(always)]
    #[must_use]
    pub fn vpsize(&mut self) -> VPSIZE_W<0> {
        VPSIZE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DSI Host video packet current configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [vpccr](index.html) module
pub struct VPCCR_SPEC;
impl crate::RegisterSpec for VPCCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [vpccr::R](R) reader structure
impl crate::Readable for VPCCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [vpccr::W](W) writer structure
impl crate::Writable for VPCCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets VPCCR to value 0
impl crate::Resettable for VPCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
