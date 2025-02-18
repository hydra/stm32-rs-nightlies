///Register `CSGCM7R` reader
pub struct R(crate::R<CSGCM7R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSGCM7R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSGCM7R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSGCM7R_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CSGCM7R` writer
pub struct W(crate::W<CSGCM7R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSGCM7R_SPEC>;
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
impl From<crate::W<CSGCM7R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSGCM7R_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CSGCM7` reader - CSGCM7
pub type CSGCM7_R = crate::FieldReader<u32, u32>;
///Field `CSGCM7` writer - CSGCM7
pub type CSGCM7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSGCM7R_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - CSGCM7
    #[inline(always)]
    pub fn csgcm7(&self) -> CSGCM7_R {
        CSGCM7_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - CSGCM7
    #[inline(always)]
    #[must_use]
    pub fn csgcm7(&mut self) -> CSGCM7_W<0> {
        CSGCM7_W::new(self)
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
///For information about available fields see [csgcm7r](index.html) module
pub struct CSGCM7R_SPEC;
impl crate::RegisterSpec for CSGCM7R_SPEC {
    type Ux = u32;
}
///`read()` method returns [csgcm7r::R](R) reader structure
impl crate::Readable for CSGCM7R_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [csgcm7r::W](W) writer structure
impl crate::Writable for CSGCM7R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CSGCM7R to value 0
impl crate::Resettable for CSGCM7R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
