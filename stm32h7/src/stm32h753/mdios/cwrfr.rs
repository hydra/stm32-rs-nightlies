///Register `CWRFR` reader
pub struct R(crate::R<CWRFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CWRFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CWRFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CWRFR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CWRFR` writer
pub struct W(crate::W<CWRFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CWRFR_SPEC>;
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
impl From<crate::W<CWRFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CWRFR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CWRF` reader - Clear the write flag
pub type CWRF_R = crate::FieldReader<u32, u32>;
///Field `CWRF` writer - Clear the write flag
pub type CWRF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CWRFR_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - Clear the write flag
    #[inline(always)]
    pub fn cwrf(&self) -> CWRF_R {
        CWRF_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Clear the write flag
    #[inline(always)]
    #[must_use]
    pub fn cwrf(&mut self) -> CWRF_W<0> {
        CWRF_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///MDIOS clear write flag register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cwrfr](index.html) module
pub struct CWRFR_SPEC;
impl crate::RegisterSpec for CWRFR_SPEC {
    type Ux = u32;
}
///`read()` method returns [cwrfr::R](R) reader structure
impl crate::Readable for CWRFR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cwrfr::W](W) writer structure
impl crate::Writable for CWRFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CWRFR to value 0
impl crate::Resettable for CWRFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
