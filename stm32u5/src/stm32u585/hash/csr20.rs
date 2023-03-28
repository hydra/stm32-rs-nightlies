///Register `CSR20` reader
pub struct R(crate::R<CSR20_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR20_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR20_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR20_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CSR20` writer
pub struct W(crate::W<CSR20_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSR20_SPEC>;
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
impl From<crate::W<CSR20_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSR20_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CSR20` reader - CSR20
pub type CSR20_R = crate::FieldReader<u32, u32>;
///Field `CSR20` writer - CSR20
pub type CSR20_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSR20_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - CSR20
    #[inline(always)]
    pub fn csr20(&self) -> CSR20_R {
        CSR20_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - CSR20
    #[inline(always)]
    #[must_use]
    pub fn csr20(&mut self) -> CSR20_W<0> {
        CSR20_W::new(self)
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
///For information about available fields see [csr20](index.html) module
pub struct CSR20_SPEC;
impl crate::RegisterSpec for CSR20_SPEC {
    type Ux = u32;
}
///`read()` method returns [csr20::R](R) reader structure
impl crate::Readable for CSR20_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [csr20::W](W) writer structure
impl crate::Writable for CSR20_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CSR20 to value 0
impl crate::Resettable for CSR20_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
