///Register `CSR51` reader
pub struct R(crate::R<CSR51_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR51_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR51_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR51_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CSR51` writer
pub struct W(crate::W<CSR51_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSR51_SPEC>;
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
impl From<crate::W<CSR51_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSR51_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CSR51` reader - CSR51
pub type CSR51_R = crate::FieldReader<u32, u32>;
///Field `CSR51` writer - CSR51
pub type CSR51_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSR51_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - CSR51
    #[inline(always)]
    pub fn csr51(&self) -> CSR51_R {
        CSR51_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - CSR51
    #[inline(always)]
    #[must_use]
    pub fn csr51(&mut self) -> CSR51_W<0> {
        CSR51_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///context swap registers
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [csr51](index.html) module
pub struct CSR51_SPEC;
impl crate::RegisterSpec for CSR51_SPEC {
    type Ux = u32;
}
///`read()` method returns [csr51::R](R) reader structure
impl crate::Readable for CSR51_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [csr51::W](W) writer structure
impl crate::Writable for CSR51_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CSR51 to value 0
impl crate::Resettable for CSR51_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
