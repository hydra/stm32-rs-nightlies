///Register `DLEN` reader
pub struct R(crate::R<DLEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DLEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DLEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DLEN_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DLEN` writer
pub struct W(crate::W<DLEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DLEN_SPEC>;
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
impl From<crate::W<DLEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DLEN_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DATALENGTH` reader - Data length value
pub type DATALENGTH_R = crate::FieldReader<u32, u32>;
///Field `DATALENGTH` writer - Data length value
pub type DATALENGTH_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, DLEN_SPEC, u32, u32, 25, O>;
impl R {
    ///Bits 0:24 - Data length value
    #[inline(always)]
    pub fn datalength(&self) -> DATALENGTH_R {
        DATALENGTH_R::new(self.bits & 0x01ff_ffff)
    }
}
impl W {
    ///Bits 0:24 - Data length value
    #[inline(always)]
    #[must_use]
    pub fn datalength(&mut self) -> DATALENGTH_W<0> {
        DATALENGTH_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///data length register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dlen](index.html) module
pub struct DLEN_SPEC;
impl crate::RegisterSpec for DLEN_SPEC {
    type Ux = u32;
}
///`read()` method returns [dlen::R](R) reader structure
impl crate::Readable for DLEN_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dlen::W](W) writer structure
impl crate::Writable for DLEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DLEN to value 0
impl crate::Resettable for DLEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
