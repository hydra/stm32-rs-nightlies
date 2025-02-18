///Register `HWCFGR1` reader
pub struct R(crate::R<HWCFGR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HWCFGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HWCFGR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HWCFGR1_SPEC>) -> Self {
        R(reader)
    }
}
///Field `NUM_DMA_STREAMS` reader - number of DMA request line multiplexer (output) channels
pub type NUM_DMA_STREAMS_R = crate::FieldReader<u8, u8>;
///Field `NUM_DMA_PERIPH_REQ` reader - number of DMA request lines from peripherals
pub type NUM_DMA_PERIPH_REQ_R = crate::FieldReader<u8, u8>;
///Field `NUM_DMA_TRIG` reader - number of synchronization inputs
pub type NUM_DMA_TRIG_R = crate::FieldReader<u8, u8>;
///Field `NUM_DMA_REQGEN` reader - number of DMA request generator channels
pub type NUM_DMA_REQGEN_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:7 - number of DMA request line multiplexer (output) channels
    #[inline(always)]
    pub fn num_dma_streams(&self) -> NUM_DMA_STREAMS_R {
        NUM_DMA_STREAMS_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - number of DMA request lines from peripherals
    #[inline(always)]
    pub fn num_dma_periph_req(&self) -> NUM_DMA_PERIPH_REQ_R {
        NUM_DMA_PERIPH_REQ_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - number of synchronization inputs
    #[inline(always)]
    pub fn num_dma_trig(&self) -> NUM_DMA_TRIG_R {
        NUM_DMA_TRIG_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - number of DMA request generator channels
    #[inline(always)]
    pub fn num_dma_reqgen(&self) -> NUM_DMA_REQGEN_R {
        NUM_DMA_REQGEN_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
///DMAMUX hardware configuration 1 register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hwcfgr1](index.html) module
pub struct HWCFGR1_SPEC;
impl crate::RegisterSpec for HWCFGR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [hwcfgr1::R](R) reader structure
impl crate::Readable for HWCFGR1_SPEC {
    type Reader = R;
}
///`reset()` method sets HWCFGR1 to value 0x0417_3907
impl crate::Resettable for HWCFGR1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0417_3907;
}
