///Register `HNPTXFSIZ_Host` reader
pub struct R(crate::R<HNPTXFSIZ_HOST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HNPTXFSIZ_HOST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HNPTXFSIZ_HOST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HNPTXFSIZ_HOST_SPEC>) -> Self {
        R(reader)
    }
}
///Register `HNPTXFSIZ_Host` writer
pub struct W(crate::W<HNPTXFSIZ_HOST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HNPTXFSIZ_HOST_SPEC>;
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
impl From<crate::W<HNPTXFSIZ_HOST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HNPTXFSIZ_HOST_SPEC>) -> Self {
        W(writer)
    }
}
///Field `NPTXFSA` reader - Non-periodic transmit RAM start address
pub type NPTXFSA_R = crate::FieldReader<u16, u16>;
///Field `NPTXFSA` writer - Non-periodic transmit RAM start address
pub type NPTXFSA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HNPTXFSIZ_HOST_SPEC, u16, u16, 16, O>;
///Field `NPTXFD` reader - Non-periodic TxFIFO depth
pub type NPTXFD_R = crate::FieldReader<u16, u16>;
///Field `NPTXFD` writer - Non-periodic TxFIFO depth
pub type NPTXFD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HNPTXFSIZ_HOST_SPEC, u16, u16, 16, O>;
impl R {
    ///Bits 0:15 - Non-periodic transmit RAM start address
    #[inline(always)]
    pub fn nptxfsa(&self) -> NPTXFSA_R {
        NPTXFSA_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - Non-periodic TxFIFO depth
    #[inline(always)]
    pub fn nptxfd(&self) -> NPTXFD_R {
        NPTXFD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Non-periodic transmit RAM start address
    #[inline(always)]
    #[must_use]
    pub fn nptxfsa(&mut self) -> NPTXFSA_W<0> {
        NPTXFSA_W::new(self)
    }
    ///Bits 16:31 - Non-periodic TxFIFO depth
    #[inline(always)]
    #[must_use]
    pub fn nptxfd(&mut self) -> NPTXFD_W<16> {
        NPTXFD_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///OTG_FS Host non-periodic transmit FIFO size register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hnptxfsiz_host](index.html) module
pub struct HNPTXFSIZ_HOST_SPEC;
impl crate::RegisterSpec for HNPTXFSIZ_HOST_SPEC {
    type Ux = u32;
}
///`read()` method returns [hnptxfsiz_host::R](R) reader structure
impl crate::Readable for HNPTXFSIZ_HOST_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [hnptxfsiz_host::W](W) writer structure
impl crate::Writable for HNPTXFSIZ_HOST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets HNPTXFSIZ_Host to value 0x0200
impl crate::Resettable for HNPTXFSIZ_HOST_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200;
}
