///Register `VLCCR` reader
pub struct R(crate::R<VLCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VLCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VLCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VLCCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `VLCCR` writer
pub struct W(crate::W<VLCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VLCCR_SPEC>;
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
impl From<crate::W<VLCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VLCCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `HLINE` reader - Horizontal line duration
pub type HLINE_R = crate::FieldReader<u16, u16>;
///Field `HLINE` writer - Horizontal line duration
pub type HLINE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VLCCR_SPEC, u16, u16, 15, O>;
impl R {
    ///Bits 0:14 - Horizontal line duration
    #[inline(always)]
    pub fn hline(&self) -> HLINE_R {
        HLINE_R::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    ///Bits 0:14 - Horizontal line duration
    #[inline(always)]
    #[must_use]
    pub fn hline(&mut self) -> HLINE_W<0> {
        HLINE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DSI Host video line current configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [vlccr](index.html) module
pub struct VLCCR_SPEC;
impl crate::RegisterSpec for VLCCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [vlccr::R](R) reader structure
impl crate::Readable for VLCCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [vlccr::W](W) writer structure
impl crate::Writable for VLCCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets VLCCR to value 0
impl crate::Resettable for VLCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
