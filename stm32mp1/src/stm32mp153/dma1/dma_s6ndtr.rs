///Register `DMA_S6NDTR` reader
pub struct R(crate::R<DMA_S6NDTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_S6NDTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_S6NDTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_S6NDTR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DMA_S6NDTR` writer
pub struct W(crate::W<DMA_S6NDTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_S6NDTR_SPEC>;
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
impl From<crate::W<DMA_S6NDTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_S6NDTR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `NDT` reader - NDT
pub type NDT_R = crate::FieldReader<u16, u16>;
///Field `NDT` writer - NDT
pub type NDT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMA_S6NDTR_SPEC, u16, u16, 16, O>;
impl R {
    ///Bits 0:15 - NDT
    #[inline(always)]
    pub fn ndt(&self) -> NDT_R {
        NDT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - NDT
    #[inline(always)]
    #[must_use]
    pub fn ndt(&mut self) -> NDT_W<0> {
        NDT_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DMA stream 6 number of data register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dma_s6ndtr](index.html) module
pub struct DMA_S6NDTR_SPEC;
impl crate::RegisterSpec for DMA_S6NDTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [dma_s6ndtr::R](R) reader structure
impl crate::Readable for DMA_S6NDTR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dma_s6ndtr::W](W) writer structure
impl crate::Writable for DMA_S6NDTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DMA_S6NDTR to value 0
impl crate::Resettable for DMA_S6NDTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
