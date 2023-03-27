///Register `DFSDM0_AWCFR` reader
pub struct R(crate::R<DFSDM0_AWCFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFSDM0_AWCFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFSDM0_AWCFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFSDM0_AWCFR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DFSDM0_AWCFR` writer
pub struct W(crate::W<DFSDM0_AWCFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFSDM0_AWCFR_SPEC>;
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
impl From<crate::W<DFSDM0_AWCFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFSDM0_AWCFR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CLRAWLTF` reader - Clear the analog watchdog low threshold flag
pub type CLRAWLTF_R = crate::FieldReader<u8, u8>;
///Field `CLRAWLTF` writer - Clear the analog watchdog low threshold flag
pub type CLRAWLTF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DFSDM0_AWCFR_SPEC, u8, u8, 8, O>;
///Field `CLRAWHTF` reader - Clear the analog watchdog high threshold flag
pub type CLRAWHTF_R = crate::FieldReader<u8, u8>;
///Field `CLRAWHTF` writer - Clear the analog watchdog high threshold flag
pub type CLRAWHTF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DFSDM0_AWCFR_SPEC, u8, u8, 8, O>;
impl R {
    ///Bits 0:7 - Clear the analog watchdog low threshold flag
    #[inline(always)]
    pub fn clrawltf(&self) -> CLRAWLTF_R {
        CLRAWLTF_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Clear the analog watchdog high threshold flag
    #[inline(always)]
    pub fn clrawhtf(&self) -> CLRAWHTF_R {
        CLRAWHTF_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - Clear the analog watchdog low threshold flag
    #[inline(always)]
    #[must_use]
    pub fn clrawltf(&mut self) -> CLRAWLTF_W<0> {
        CLRAWLTF_W::new(self)
    }
    ///Bits 8:15 - Clear the analog watchdog high threshold flag
    #[inline(always)]
    #[must_use]
    pub fn clrawhtf(&mut self) -> CLRAWHTF_W<8> {
        CLRAWHTF_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DFSDM analog watchdog clear flag register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm0_awcfr](index.html) module
pub struct DFSDM0_AWCFR_SPEC;
impl crate::RegisterSpec for DFSDM0_AWCFR_SPEC {
    type Ux = u32;
}
///`read()` method returns [dfsdm0_awcfr::R](R) reader structure
impl crate::Readable for DFSDM0_AWCFR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dfsdm0_awcfr::W](W) writer structure
impl crate::Writable for DFSDM0_AWCFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DFSDM0_AWCFR to value 0
impl crate::Resettable for DFSDM0_AWCFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
