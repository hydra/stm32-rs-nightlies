///Register `ETH_DMAC0CARxDR` reader
pub struct R(crate::R<ETH_DMAC0CARX_DR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_DMAC0CARX_DR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_DMAC0CARX_DR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_DMAC0CARX_DR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `CURRDESAPTR` reader - Application Transmit Descriptor Address Pointer
pub type CURRDESAPTR_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - Application Transmit Descriptor Address Pointer
    #[inline(always)]
    pub fn currdesaptr(&self) -> CURRDESAPTR_R {
        CURRDESAPTR_R::new(self.bits)
    }
}
///Channel 0 current application receive descriptor register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [eth_dmac0carx_dr](index.html) module
pub struct ETH_DMAC0CARX_DR_SPEC;
impl crate::RegisterSpec for ETH_DMAC0CARX_DR_SPEC {
    type Ux = u32;
}
///`read()` method returns [eth_dmac0carx_dr::R](R) reader structure
impl crate::Readable for ETH_DMAC0CARX_DR_SPEC {
    type Reader = R;
}
///`reset()` method sets ETH_DMAC0CARxDR to value 0
impl crate::Resettable for ETH_DMAC0CARX_DR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
