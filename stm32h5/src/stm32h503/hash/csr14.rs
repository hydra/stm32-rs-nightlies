///Register `CSR14` reader
pub struct R(crate::R<CSR14_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR14_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR14_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR14_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CSR14` writer
pub struct W(crate::W<CSR14_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSR14_SPEC>;
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
impl From<crate::W<CSR14_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSR14_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CS14` reader - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction.
pub type CS14_R = crate::FieldReader<u32, u32>;
///Field `CS14` writer - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction.
pub type CS14_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSR14_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction.
    #[inline(always)]
    pub fn cs14(&self) -> CS14_R {
        CS14_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction.
    #[inline(always)]
    #[must_use]
    pub fn cs14(&mut self) -> CS14_W<0> {
        CS14_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///HASH context swap register 14
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [csr14](index.html) module
pub struct CSR14_SPEC;
impl crate::RegisterSpec for CSR14_SPEC {
    type Ux = u32;
}
///`read()` method returns [csr14::R](R) reader structure
impl crate::Readable for CSR14_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [csr14::W](W) writer structure
impl crate::Writable for CSR14_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CSR14 to value 0
impl crate::Resettable for CSR14_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
