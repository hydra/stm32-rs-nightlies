///Register `CSGCM%sR` reader
pub struct R(crate::R<CSGCMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSGCMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSGCMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSGCMR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CSGCM%sR` writer
pub struct W(crate::W<CSGCMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSGCMR_SPEC>;
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
impl From<crate::W<CSGCMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSGCMR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CSGCMR` reader - CSGCM0R
pub type CSGCMR_R = crate::FieldReader<u32, u32>;
///Field `CSGCMR` writer - CSGCM0R
pub type CSGCMR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSGCMR_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - CSGCM0R
    #[inline(always)]
    pub fn csgcmr(&self) -> CSGCMR_R {
        CSGCMR_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - CSGCM0R
    #[inline(always)]
    #[must_use]
    pub fn csgcmr(&mut self) -> CSGCMR_W<0> {
        CSGCMR_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///context swap register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [csgcmr](index.html) module
pub struct CSGCMR_SPEC;
impl crate::RegisterSpec for CSGCMR_SPEC {
    type Ux = u32;
}
///`read()` method returns [csgcmr::R](R) reader structure
impl crate::Readable for CSGCMR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [csgcmr::W](W) writer structure
impl crate::Writable for CSGCMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CSGCM%sR to value 0
impl crate::Resettable for CSGCMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
