///Register `CSGCM0R` reader
pub struct R(crate::R<CSGCM0R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSGCM0R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSGCM0R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSGCM0R_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CSGCM0R` writer
pub struct W(crate::W<CSGCM0R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSGCM0R_SPEC>;
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
impl From<crate::W<CSGCM0R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSGCM0R_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CSGCM0` reader - CSGCM0
pub type CSGCM0_R = crate::FieldReader<u32, u32>;
///Field `CSGCM0` writer - CSGCM0
pub type CSGCM0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSGCM0R_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - CSGCM0
    #[inline(always)]
    pub fn csgcm0(&self) -> CSGCM0_R {
        CSGCM0_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - CSGCM0
    #[inline(always)]
    #[must_use]
    pub fn csgcm0(&mut self) -> CSGCM0_W<0> {
        CSGCM0_W::new(self)
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
///For information about available fields see [csgcm0r](index.html) module
pub struct CSGCM0R_SPEC;
impl crate::RegisterSpec for CSGCM0R_SPEC {
    type Ux = u32;
}
///`read()` method returns [csgcm0r::R](R) reader structure
impl crate::Readable for CSGCM0R_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [csgcm0r::W](W) writer structure
impl crate::Writable for CSGCM0R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CSGCM0R to value 0
impl crate::Resettable for CSGCM0R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
