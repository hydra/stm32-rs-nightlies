///Register `DOEPDMA5` reader
pub struct R(crate::R<DOEPDMA5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOEPDMA5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOEPDMA5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOEPDMA5_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DOEPDMA5` writer
pub struct W(crate::W<DOEPDMA5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOEPDMA5_SPEC>;
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
impl From<crate::W<DOEPDMA5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOEPDMA5_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DMAADDR` reader - DMAADDR
pub type DMAADDR_R = crate::FieldReader<u32, u32>;
///Field `DMAADDR` writer - DMAADDR
pub type DMAADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DOEPDMA5_SPEC, u32, u32, 32, O>;
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
///OTG device OUT endpoint 5 DMA address register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [doepdma5](index.html) module
pub struct DOEPDMA5_SPEC;
impl crate::RegisterSpec for DOEPDMA5_SPEC {
    type Ux = u32;
}
///`read()` method returns [doepdma5::R](R) reader structure
impl crate::Readable for DOEPDMA5_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [doepdma5::W](W) writer structure
impl crate::Writable for DOEPDMA5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DOEPDMA5 to value 0
impl crate::Resettable for DOEPDMA5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
