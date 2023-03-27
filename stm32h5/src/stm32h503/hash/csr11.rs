///Register `CSR11` reader
pub struct R(crate::R<CSR11_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR11_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR11_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR11_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CSR11` writer
pub struct W(crate::W<CSR11_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSR11_SPEC>;
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
impl From<crate::W<CSR11_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSR11_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CS11` reader - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction.
pub type CS11_R = crate::FieldReader<u32, u32>;
///Field `CS11` writer - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction.
pub type CS11_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSR11_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction.
    #[inline(always)]
    pub fn cs11(&self) -> CS11_R {
        CS11_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction.
    #[inline(always)]
    #[must_use]
    pub fn cs11(&mut self) -> CS11_W<0> {
        CS11_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///HASH context swap register 11
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [csr11](index.html) module
pub struct CSR11_SPEC;
impl crate::RegisterSpec for CSR11_SPEC {
    type Ux = u32;
}
///`read()` method returns [csr11::R](R) reader structure
impl crate::Readable for CSR11_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [csr11::W](W) writer structure
impl crate::Writable for CSR11_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CSR11 to value 0
impl crate::Resettable for CSR11_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
