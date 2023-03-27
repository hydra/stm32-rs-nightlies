///Register `FDCAN_TXBRP` reader
pub struct R(crate::R<FDCAN_TXBRP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_TXBRP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_TXBRP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_TXBRP_SPEC>) -> Self {
        R(reader)
    }
}
///Field `TRP` reader - Transmission Request Pending
pub type TRP_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - Transmission Request Pending
    #[inline(always)]
    pub fn trp(&self) -> TRP_R {
        TRP_R::new(self.bits)
    }
}
///FDCAN Tx Buffer Request Pending Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fdcan_txbrp](index.html) module
pub struct FDCAN_TXBRP_SPEC;
impl crate::RegisterSpec for FDCAN_TXBRP_SPEC {
    type Ux = u32;
}
///`read()` method returns [fdcan_txbrp::R](R) reader structure
impl crate::Readable for FDCAN_TXBRP_SPEC {
    type Reader = R;
}
///`reset()` method sets FDCAN_TXBRP to value 0
impl crate::Resettable for FDCAN_TXBRP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
