///Register `HASH_CSR8` reader
pub struct R(crate::R<HASH_CSR8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HASH_CSR8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HASH_CSR8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HASH_CSR8_SPEC>) -> Self {
        R(reader)
    }
}
///Register `HASH_CSR8` writer
pub struct W(crate::W<HASH_CSR8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HASH_CSR8_SPEC>;
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
impl From<crate::W<HASH_CSR8_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HASH_CSR8_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CS8` reader - CS8
pub type CS8_R = crate::FieldReader<u32, u32>;
///Field `CS8` writer - CS8
pub type CS8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HASH_CSR8_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - CS8
    #[inline(always)]
    pub fn cs8(&self) -> CS8_R {
        CS8_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - CS8
    #[inline(always)]
    #[must_use]
    pub fn cs8(&mut self) -> CS8_W<0> {
        CS8_W::new(self)
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
///For information about available fields see [hash_csr8](index.html) module
pub struct HASH_CSR8_SPEC;
impl crate::RegisterSpec for HASH_CSR8_SPEC {
    type Ux = u32;
}
///`read()` method returns [hash_csr8::R](R) reader structure
impl crate::Readable for HASH_CSR8_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [hash_csr8::W](W) writer structure
impl crate::Writable for HASH_CSR8_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets HASH_CSR8 to value 0
impl crate::Resettable for HASH_CSR8_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
