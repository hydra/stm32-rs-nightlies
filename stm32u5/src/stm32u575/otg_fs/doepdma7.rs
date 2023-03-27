///Register `DOEPDMA7` reader
pub struct R(crate::R<DOEPDMA7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOEPDMA7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOEPDMA7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOEPDMA7_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DOEPDMA7` writer
pub struct W(crate::W<DOEPDMA7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOEPDMA7_SPEC>;
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
impl From<crate::W<DOEPDMA7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOEPDMA7_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DMAADDR` reader - DMAADDR
pub type DMAADDR_R = crate::FieldReader<u32, u32>;
///Field `DMAADDR` writer - DMAADDR
pub type DMAADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DOEPDMA7_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - DMAADDR
    #[inline(always)]
    pub fn dmaaddr(&self) -> DMAADDR_R {
        DMAADDR_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - DMAADDR
    #[inline(always)]
    #[must_use]
    pub fn dmaaddr(&mut self) -> DMAADDR_W<0> {
        DMAADDR_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///OTG device OUT endpoint 7 DMA address register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [doepdma7](index.html) module
pub struct DOEPDMA7_SPEC;
impl crate::RegisterSpec for DOEPDMA7_SPEC {
    type Ux = u32;
}
///`read()` method returns [doepdma7::R](R) reader structure
impl crate::Readable for DOEPDMA7_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [doepdma7::W](W) writer structure
impl crate::Writable for DOEPDMA7_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DOEPDMA7 to value 0
impl crate::Resettable for DOEPDMA7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
