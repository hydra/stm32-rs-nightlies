///Register `HASH_CSR18` reader
pub struct R(crate::R<HASH_CSR18_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HASH_CSR18_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HASH_CSR18_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HASH_CSR18_SPEC>) -> Self {
        R(reader)
    }
}
///Register `HASH_CSR18` writer
pub struct W(crate::W<HASH_CSR18_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HASH_CSR18_SPEC>;
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
impl From<crate::W<HASH_CSR18_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HASH_CSR18_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CS18` reader - CS18
pub type CS18_R = crate::FieldReader<u32, u32>;
///Field `CS18` writer - CS18
pub type CS18_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HASH_CSR18_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - CS18
    #[inline(always)]
    pub fn cs18(&self) -> CS18_R {
        CS18_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - CS18
    #[inline(always)]
    #[must_use]
    pub fn cs18(&mut self) -> CS18_W<0> {
        CS18_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///HASH context swap registers
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hash_csr18](index.html) module
pub struct HASH_CSR18_SPEC;
impl crate::RegisterSpec for HASH_CSR18_SPEC {
    type Ux = u32;
}
///`read()` method returns [hash_csr18::R](R) reader structure
impl crate::Readable for HASH_CSR18_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [hash_csr18::W](W) writer structure
impl crate::Writable for HASH_CSR18_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets HASH_CSR18 to value 0
impl crate::Resettable for HASH_CSR18_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
