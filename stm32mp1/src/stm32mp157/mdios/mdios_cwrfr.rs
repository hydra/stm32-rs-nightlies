///Register `MDIOS_CWRFR` reader
pub struct R(crate::R<MDIOS_CWRFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDIOS_CWRFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDIOS_CWRFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDIOS_CWRFR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MDIOS_CWRFR` writer
pub struct W(crate::W<MDIOS_CWRFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MDIOS_CWRFR_SPEC>;
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
impl From<crate::W<MDIOS_CWRFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MDIOS_CWRFR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CWRF` reader - CWRF
pub type CWRF_R = crate::FieldReader<u32, u32>;
///Field `CWRF` writer - CWRF
pub type CWRF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MDIOS_CWRFR_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - CWRF
    #[inline(always)]
    pub fn cwrf(&self) -> CWRF_R {
        CWRF_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - CWRF
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
///For information about available fields see [mdios_cwrfr](index.html) module
pub struct MDIOS_CWRFR_SPEC;
impl crate::RegisterSpec for MDIOS_CWRFR_SPEC {
    type Ux = u32;
}
///`read()` method returns [mdios_cwrfr::R](R) reader structure
impl crate::Readable for MDIOS_CWRFR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [mdios_cwrfr::W](W) writer structure
impl crate::Writable for MDIOS_CWRFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets MDIOS_CWRFR to value 0
impl crate::Resettable for MDIOS_CWRFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
