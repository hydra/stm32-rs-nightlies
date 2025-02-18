///Register `RDATA` reader
pub struct R(crate::R<RDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RDATA_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RDATA` writer
pub struct W(crate::W<RDATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RDATA_SPEC>;
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
impl From<crate::W<RDATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RDATA_SPEC>) -> Self {
        W(writer)
    }
}
///Field `RES` reader - Read data (contents of the Y output buffer at the address indicated by the READ pointer)
pub type RES_R = crate::FieldReader<u16, u16>;
///Field `RES` writer - Read data (contents of the Y output buffer at the address indicated by the READ pointer)
pub type RES_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RDATA_SPEC, u16, u16, 16, O>;
impl R {
    ///Bits 0:15 - Read data (contents of the Y output buffer at the address indicated by the READ pointer)
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Read data (contents of the Y output buffer at the address indicated by the READ pointer)
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> RES_W<0> {
        RES_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Read data register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rdata](index.html) module
pub struct RDATA_SPEC;
impl crate::RegisterSpec for RDATA_SPEC {
    type Ux = u32;
}
///`read()` method returns [rdata::R](R) reader structure
impl crate::Readable for RDATA_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rdata::W](W) writer structure
impl crate::Writable for RDATA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RDATA to value 0
impl crate::Resettable for RDATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
