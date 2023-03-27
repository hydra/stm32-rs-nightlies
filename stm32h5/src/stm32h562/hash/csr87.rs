///Register `CSR87` reader
pub struct R(crate::R<CSR87_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR87_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR87_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR87_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CSR87` writer
pub struct W(crate::W<CSR87_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSR87_SPEC>;
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
impl From<crate::W<CSR87_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSR87_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CSx` reader - Context swap 87 Refer to introduction.
pub type CSX_R = crate::FieldReader<u32, u32>;
///Field `CSx` writer - Context swap 87 Refer to introduction.
pub type CSX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSR87_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - Context swap 87 Refer to introduction.
    #[inline(always)]
    pub fn csx(&self) -> CSX_R {
        CSX_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Context swap 87 Refer to introduction.
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
///HASH context swap register 87
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [csr87](index.html) module
pub struct CSR87_SPEC;
impl crate::RegisterSpec for CSR87_SPEC {
    type Ux = u32;
}
///`read()` method returns [csr87::R](R) reader structure
impl crate::Readable for CSR87_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [csr87::W](W) writer structure
impl crate::Writable for CSR87_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CSR87 to value 0x0022_0002
impl crate::Resettable for CSR87_SPEC {
    const RESET_VALUE: Self::Ux = 0x0022_0002;
}
