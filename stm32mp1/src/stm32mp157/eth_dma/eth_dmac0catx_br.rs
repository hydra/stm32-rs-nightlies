///Register `ETH_DMAC0CATxBR` reader
pub struct R(crate::R<ETH_DMAC0CATX_BR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_DMAC0CATX_BR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_DMAC0CATX_BR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_DMAC0CATX_BR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `CURTBUFAPTR` reader - Application Transmit Buffer Address Pointer
pub type CURTBUFAPTR_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - Application Transmit Buffer Address Pointer
    #[inline(always)]
    pub fn curtbufaptr(&self) -> CURTBUFAPTR_R {
        CURTBUFAPTR_R::new(self.bits)
    }
}
///Channel 0 current application transmit buffer register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [eth_dmac0catx_br](index.html) module
pub struct ETH_DMAC0CATX_BR_SPEC;
impl crate::RegisterSpec for ETH_DMAC0CATX_BR_SPEC {
    type Ux = u32;
}
///`read()` method returns [eth_dmac0catx_br::R](R) reader structure
impl crate::Readable for ETH_DMAC0CATX_BR_SPEC {
    type Reader = R;
}
///`reset()` method sets ETH_DMAC0CATxBR to value 0
impl crate::Resettable for ETH_DMAC0CATX_BR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
