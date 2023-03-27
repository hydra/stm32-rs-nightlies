///Register `CSR33` reader
pub struct R(crate::R<CSR33_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR33_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR33_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR33_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CSR33` writer
pub struct W(crate::W<CSR33_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSR33_SPEC>;
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
impl From<crate::W<CSR33_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSR33_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CS33` reader - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction.
pub type CS33_R = crate::FieldReader<u32, u32>;
///Field `CS33` writer - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction.
pub type CS33_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSR33_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction.
    #[inline(always)]
    pub fn cs33(&self) -> CS33_R {
        CS33_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction.
    #[inline(always)]
    #[must_use]
    pub fn cs33(&mut self) -> CS33_W<0> {
        CS33_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///HASH context swap register 33
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [csr33](index.html) module
pub struct CSR33_SPEC;
impl crate::RegisterSpec for CSR33_SPEC {
    type Ux = u32;
}
///`read()` method returns [csr33::R](R) reader structure
impl crate::Readable for CSR33_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [csr33::W](W) writer structure
impl crate::Writable for CSR33_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CSR33 to value 0
impl crate::Resettable for CSR33_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
