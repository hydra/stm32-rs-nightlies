///Register `GRXFSIZ` reader
pub struct R(crate::R<GRXFSIZ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GRXFSIZ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GRXFSIZ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GRXFSIZ_SPEC>) -> Self {
        R(reader)
    }
}
///Register `GRXFSIZ` writer
pub struct W(crate::W<GRXFSIZ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GRXFSIZ_SPEC>;
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
impl From<crate::W<GRXFSIZ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GRXFSIZ_SPEC>) -> Self {
        W(writer)
    }
}
///Field `RXFD` reader - RxFIFO depth
pub type RXFD_R = crate::FieldReader<u16, u16>;
///Field `RXFD` writer - RxFIFO depth
pub type RXFD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GRXFSIZ_SPEC, u16, u16, 16, O>;
impl R {
    ///Bits 0:15 - RxFIFO depth
    #[inline(always)]
    pub fn rxfd(&self) -> RXFD_R {
        RXFD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - RxFIFO depth
    #[inline(always)]
    #[must_use]
    pub fn rxfd(&mut self) -> RXFD_W<0> {
        RXFD_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///OTG_HS Receive FIFO size register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [grxfsiz](index.html) module
pub struct GRXFSIZ_SPEC;
impl crate::RegisterSpec for GRXFSIZ_SPEC {
    type Ux = u32;
}
///`read()` method returns [grxfsiz::R](R) reader structure
impl crate::Readable for GRXFSIZ_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [grxfsiz::W](W) writer structure
impl crate::Writable for GRXFSIZ_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets GRXFSIZ to value 0x0200
impl crate::Resettable for GRXFSIZ_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200;
}
