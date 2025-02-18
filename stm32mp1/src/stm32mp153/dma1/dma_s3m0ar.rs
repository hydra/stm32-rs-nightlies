///Register `DMA_S3M0AR` reader
pub struct R(crate::R<DMA_S3M0AR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_S3M0AR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_S3M0AR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_S3M0AR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DMA_S3M0AR` writer
pub struct W(crate::W<DMA_S3M0AR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_S3M0AR_SPEC>;
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
impl From<crate::W<DMA_S3M0AR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_S3M0AR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `M0A` reader - M0A
pub type M0A_R = crate::FieldReader<u32, u32>;
///Field `M0A` writer - M0A
pub type M0A_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMA_S3M0AR_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - M0A
    #[inline(always)]
    pub fn m0a(&self) -> M0A_R {
        M0A_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - M0A
    #[inline(always)]
    #[must_use]
    pub fn m0a(&mut self) -> M0A_W<0> {
        M0A_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DMA stream 3 memory 0 address register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dma_s3m0ar](index.html) module
pub struct DMA_S3M0AR_SPEC;
impl crate::RegisterSpec for DMA_S3M0AR_SPEC {
    type Ux = u32;
}
///`read()` method returns [dma_s3m0ar::R](R) reader structure
impl crate::Readable for DMA_S3M0AR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dma_s3m0ar::W](W) writer structure
impl crate::Writable for DMA_S3M0AR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DMA_S3M0AR to value 0
impl crate::Resettable for DMA_S3M0AR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
