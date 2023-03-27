///Register `OTG_DOEPDMA6` reader
pub struct R(crate::R<OTG_DOEPDMA6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_DOEPDMA6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_DOEPDMA6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_DOEPDMA6_SPEC>) -> Self {
        R(reader)
    }
}
///Register `OTG_DOEPDMA6` writer
pub struct W(crate::W<OTG_DOEPDMA6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTG_DOEPDMA6_SPEC>;
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
impl From<crate::W<OTG_DOEPDMA6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTG_DOEPDMA6_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DMAADDR` reader - DMAADDR
pub type DMAADDR_R = crate::FieldReader<u32, u32>;
///Field `DMAADDR` writer - DMAADDR
pub type DMAADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OTG_DOEPDMA6_SPEC, u32, u32, 32, O>;
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
///OTG device OUT endpoint 6 DMA address register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_doepdma6](index.html) module
pub struct OTG_DOEPDMA6_SPEC;
impl crate::RegisterSpec for OTG_DOEPDMA6_SPEC {
    type Ux = u32;
}
///`read()` method returns [otg_doepdma6::R](R) reader structure
impl crate::Readable for OTG_DOEPDMA6_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [otg_doepdma6::W](W) writer structure
impl crate::Writable for OTG_DOEPDMA6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets OTG_DOEPDMA6 to value 0
impl crate::Resettable for OTG_DOEPDMA6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
