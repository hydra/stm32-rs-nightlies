///Register `FDCAN_TXBTO` reader
pub struct R(crate::R<FDCAN_TXBTO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_TXBTO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_TXBTO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_TXBTO_SPEC>) -> Self {
        R(reader)
    }
}
///Field `TO` reader - TO
pub type TO_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - TO
    #[inline(always)]
    pub fn to(&self) -> TO_R {
        TO_R::new(self.bits)
    }
}
///FDCAN Tx buffer transmission occurred register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fdcan_txbto](index.html) module
pub struct FDCAN_TXBTO_SPEC;
impl crate::RegisterSpec for FDCAN_TXBTO_SPEC {
    type Ux = u32;
}
///`read()` method returns [fdcan_txbto::R](R) reader structure
impl crate::Readable for FDCAN_TXBTO_SPEC {
    type Reader = R;
}
///`reset()` method sets FDCAN_TXBTO to value 0
impl crate::Resettable for FDCAN_TXBTO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
