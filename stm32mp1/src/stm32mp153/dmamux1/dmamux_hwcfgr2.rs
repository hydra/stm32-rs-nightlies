///Register `DMAMUX_HWCFGR2` reader
pub struct R(crate::R<DMAMUX_HWCFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAMUX_HWCFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAMUX_HWCFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAMUX_HWCFGR2_SPEC>) -> Self {
        R(reader)
    }
}
///Field `NUM_DMA_EXT_REQ` reader - NUM_DMA_EXT_REQ
pub type NUM_DMA_EXT_REQ_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:7 - NUM_DMA_EXT_REQ
    #[inline(always)]
    pub fn num_dma_ext_req(&self) -> NUM_DMA_EXT_REQ_R {
        NUM_DMA_EXT_REQ_R::new((self.bits & 0xff) as u8)
    }
}
///DMAMUX hardware configuration 2 register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dmamux_hwcfgr2](index.html) module
pub struct DMAMUX_HWCFGR2_SPEC;
impl crate::RegisterSpec for DMAMUX_HWCFGR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [dmamux_hwcfgr2::R](R) reader structure
impl crate::Readable for DMAMUX_HWCFGR2_SPEC {
    type Reader = R;
}
///`reset()` method sets DMAMUX_HWCFGR2 to value 0x08
impl crate::Resettable for DMAMUX_HWCFGR2_SPEC {
    const RESET_VALUE: Self::Ux = 0x08;
}
