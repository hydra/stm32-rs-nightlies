///Register `VLCR` reader
pub struct R(crate::R<VLCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VLCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VLCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VLCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `VLCR` writer
pub struct W(crate::W<VLCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VLCR_SPEC>;
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
impl From<crate::W<VLCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VLCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `HLINE` reader - HLINE
pub type HLINE_R = crate::FieldReader<u16, u16>;
///Field `HLINE` writer - HLINE
pub type HLINE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VLCR_SPEC, u16, u16, 15, O>;
impl R {
    ///Bits 0:14 - HLINE
    #[inline(always)]
    pub fn hline(&self) -> HLINE_R {
        HLINE_R::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    ///Bits 0:14 - HLINE
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
///DSI Host video line configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [vlcr](index.html) module
pub struct VLCR_SPEC;
impl crate::RegisterSpec for VLCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [vlcr::R](R) reader structure
impl crate::Readable for VLCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [vlcr::W](W) writer structure
impl crate::Writable for VLCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets VLCR to value 0
impl crate::Resettable for VLCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
