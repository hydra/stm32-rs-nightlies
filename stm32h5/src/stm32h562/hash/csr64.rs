///Register `CSR64` reader
pub struct R(crate::R<CSR64_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR64_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR64_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR64_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CSR64` writer
pub struct W(crate::W<CSR64_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSR64_SPEC>;
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
impl From<crate::W<CSR64_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSR64_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CSx` reader - Context swap 64 Refer to introduction.
pub type CSX_R = crate::FieldReader<u32, u32>;
///Field `CSx` writer - Context swap 64 Refer to introduction.
pub type CSX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSR64_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - Context swap 64 Refer to introduction.
    #[inline(always)]
    pub fn csx(&self) -> CSX_R {
        CSX_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Context swap 64 Refer to introduction.
    #[inline(always)]
    #[must_use]
    pub fn csx(&mut self) -> CSX_W<0> {
        CSX_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///HASH context swap register 64
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [csr64](index.html) module
pub struct CSR64_SPEC;
impl crate::RegisterSpec for CSR64_SPEC {
    type Ux = u32;
}
///`read()` method returns [csr64::R](R) reader structure
impl crate::Readable for CSR64_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [csr64::W](W) writer structure
impl crate::Writable for CSR64_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CSR64 to value 0x0022_0002
impl crate::Resettable for CSR64_SPEC {
    const RESET_VALUE: Self::Ux = 0x0022_0002;
}
