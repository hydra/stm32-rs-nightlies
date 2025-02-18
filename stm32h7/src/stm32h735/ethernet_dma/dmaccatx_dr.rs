///Register `DMACCATxDR` reader
pub struct R(crate::R<DMACCATX_DR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACCATX_DR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACCATX_DR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACCATX_DR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DMACCATxDR` writer
pub struct W(crate::W<DMACCATX_DR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMACCATX_DR_SPEC>;
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
impl From<crate::W<DMACCATX_DR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMACCATX_DR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CURTDESAPTR` reader - Application Transmit Descriptor Address Pointer
pub type CURTDESAPTR_R = crate::FieldReader<u32, u32>;
///Field `CURTDESAPTR` writer - Application Transmit Descriptor Address Pointer
pub type CURTDESAPTR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DMACCATX_DR_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - Application Transmit Descriptor Address Pointer
    #[inline(always)]
    pub fn curtdesaptr(&self) -> CURTDESAPTR_R {
        CURTDESAPTR_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Application Transmit Descriptor Address Pointer
    #[inline(always)]
    #[must_use]
    pub fn curtdesaptr(&mut self) -> CURTDESAPTR_W<0> {
        CURTDESAPTR_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Channel current application transmit descriptor register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dmaccatx_dr](index.html) module
pub struct DMACCATX_DR_SPEC;
impl crate::RegisterSpec for DMACCATX_DR_SPEC {
    type Ux = u32;
}
///`read()` method returns [dmaccatx_dr::R](R) reader structure
impl crate::Readable for DMACCATX_DR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dmaccatx_dr::W](W) writer structure
impl crate::Writable for DMACCATX_DR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DMACCATxDR to value 0
impl crate::Resettable for DMACCATX_DR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
