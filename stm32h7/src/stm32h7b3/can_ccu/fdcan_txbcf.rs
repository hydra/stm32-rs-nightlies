///Register `FDCAN_TXBCF` reader
pub struct R(crate::R<FDCAN_TXBCF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_TXBCF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_TXBCF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_TXBCF_SPEC>) -> Self {
        R(reader)
    }
}
///Field `CF` reader - Cancellation Finished
pub type CF_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - Cancellation Finished
    #[inline(always)]
    pub fn cf(&self) -> CF_R {
        CF_R::new(self.bits)
    }
}
///FDCAN Tx Buffer Cancellation Finished Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fdcan_txbcf](index.html) module
pub struct FDCAN_TXBCF_SPEC;
impl crate::RegisterSpec for FDCAN_TXBCF_SPEC {
    type Ux = u32;
}
///`read()` method returns [fdcan_txbcf::R](R) reader structure
impl crate::Readable for FDCAN_TXBCF_SPEC {
    type Reader = R;
}
///`reset()` method sets FDCAN_TXBCF to value 0
impl crate::Resettable for FDCAN_TXBCF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
