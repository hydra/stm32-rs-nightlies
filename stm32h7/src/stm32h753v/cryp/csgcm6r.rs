///Register `CSGCM6R` reader
pub struct R(crate::R<CSGCM6R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSGCM6R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSGCM6R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSGCM6R_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CSGCM6R` writer
pub struct W(crate::W<CSGCM6R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSGCM6R_SPEC>;
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
impl From<crate::W<CSGCM6R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSGCM6R_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CSGCM6` reader - CSGCM6
pub type CSGCM6_R = crate::FieldReader<u32, u32>;
///Field `CSGCM6` writer - CSGCM6
pub type CSGCM6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSGCM6R_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - CSGCM6
    #[inline(always)]
    pub fn csgcm6(&self) -> CSGCM6_R {
        CSGCM6_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - CSGCM6
    #[inline(always)]
    #[must_use]
    pub fn csgcm6(&mut self) -> CSGCM6_W<0> {
        CSGCM6_W::new(self)
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
///For information about available fields see [csgcm6r](index.html) module
pub struct CSGCM6R_SPEC;
impl crate::RegisterSpec for CSGCM6R_SPEC {
    type Ux = u32;
}
///`read()` method returns [csgcm6r::R](R) reader structure
impl crate::Readable for CSGCM6R_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [csgcm6r::W](W) writer structure
impl crate::Writable for CSGCM6R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CSGCM6R to value 0
impl crate::Resettable for CSGCM6R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
