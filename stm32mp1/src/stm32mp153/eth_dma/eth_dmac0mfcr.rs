///Register `ETH_DMAC0MFCR` reader
pub struct R(crate::R<ETH_DMAC0MFCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_DMAC0MFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_DMAC0MFCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_DMAC0MFCR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `MFC` reader - Dropped Packet Counters
pub type MFC_R = crate::FieldReader<u16, u16>;
///Field `MFCO` reader - Overflow status of the MFC Counter
pub type MFCO_R = crate::BitReader<bool>;
impl R {
    ///Bits 0:10 - Dropped Packet Counters
    #[inline(always)]
    pub fn mfc(&self) -> MFC_R {
        MFC_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bit 15 - Overflow status of the MFC Counter
    #[inline(always)]
    pub fn mfco(&self) -> MFCO_R {
        MFCO_R::new(((self.bits >> 15) & 1) != 0)
    }
}
///Channel missed frame count register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [eth_dmac0mfcr](index.html) module
pub struct ETH_DMAC0MFCR_SPEC;
impl crate::RegisterSpec for ETH_DMAC0MFCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [eth_dmac0mfcr::R](R) reader structure
impl crate::Readable for ETH_DMAC0MFCR_SPEC {
    type Reader = R;
}
///`reset()` method sets ETH_DMAC0MFCR to value 0
impl crate::Resettable for ETH_DMAC0MFCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
