///Register `CSR0` reader
pub struct R(crate::R<CSR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR0_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CSR0` writer
pub struct W(crate::W<CSR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSR0_SPEC>;
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
impl From<crate::W<CSR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSR0_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CSR0` reader - CSR0
pub type CSR0_R = crate::FieldReader<u32, u32>;
///Field `CSR0` writer - CSR0
pub type CSR0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSR0_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - CSR0
    #[inline(always)]
    pub fn csr0(&self) -> CSR0_R {
        CSR0_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - CSR0
    #[inline(always)]
    #[must_use]
    pub fn csr0(&mut self) -> CSR0_W<0> {
        CSR0_W::new(self)
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
///For information about available fields see [csr0](index.html) module
pub struct CSR0_SPEC;
impl crate::RegisterSpec for CSR0_SPEC {
    type Ux = u32;
}
///`read()` method returns [csr0::R](R) reader structure
impl crate::Readable for CSR0_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [csr0::W](W) writer structure
impl crate::Writable for CSR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CSR0 to value 0
impl crate::Resettable for CSR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
