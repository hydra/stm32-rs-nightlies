///Register `S3M0AR` reader
pub struct R(crate::R<S3M0AR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<S3M0AR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<S3M0AR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<S3M0AR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `S3M0AR` writer
pub struct W(crate::W<S3M0AR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<S3M0AR_SPEC>;
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
impl From<crate::W<S3M0AR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<S3M0AR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `M0A` reader - Memory 0 address
pub type M0A_R = crate::FieldReader<u32, u32>;
///Field `M0A` writer - Memory 0 address
pub type M0A_W<'a, const O: u8> = crate::FieldWriter<'a, u32, S3M0AR_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - Memory 0 address
    #[inline(always)]
    pub fn m0a(&self) -> M0A_R {
        M0A_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Memory 0 address
    #[inline(always)]
    #[must_use]
    pub fn m0a(&mut self) -> M0A_W<0> {
        M0A_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///stream x memory 0 address register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [s3m0ar](index.html) module
pub struct S3M0AR_SPEC;
impl crate::RegisterSpec for S3M0AR_SPEC {
    type Ux = u32;
}
///`read()` method returns [s3m0ar::R](R) reader structure
impl crate::Readable for S3M0AR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [s3m0ar::W](W) writer structure
impl crate::Writable for S3M0AR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets S3M0AR to value 0
impl crate::Resettable for S3M0AR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
