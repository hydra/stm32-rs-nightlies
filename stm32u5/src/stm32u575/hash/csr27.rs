///Register `CSR27` reader
pub struct R(crate::R<CSR27_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR27_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR27_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR27_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CSR27` writer
pub struct W(crate::W<CSR27_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSR27_SPEC>;
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
impl From<crate::W<CSR27_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSR27_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CSR27` reader - CSR27
pub type CSR27_R = crate::FieldReader<u32, u32>;
///Field `CSR27` writer - CSR27
pub type CSR27_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSR27_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - CSR27
    #[inline(always)]
    pub fn csr27(&self) -> CSR27_R {
        CSR27_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - CSR27
    #[inline(always)]
    #[must_use]
    pub fn csr27(&mut self) -> CSR27_W<0> {
        CSR27_W::new(self)
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
///For information about available fields see [csr27](index.html) module
pub struct CSR27_SPEC;
impl crate::RegisterSpec for CSR27_SPEC {
    type Ux = u32;
}
///`read()` method returns [csr27::R](R) reader structure
impl crate::Readable for CSR27_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [csr27::W](W) writer structure
impl crate::Writable for CSR27_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CSR27 to value 0
impl crate::Resettable for CSR27_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
