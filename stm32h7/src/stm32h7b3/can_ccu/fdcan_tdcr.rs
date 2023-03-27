///Register `FDCAN_TDCR` reader
pub struct R(crate::R<FDCAN_TDCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_TDCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_TDCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_TDCR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `TDCF` reader - Transmitter Delay Compensation Filter Window Length
pub type TDCF_R = crate::FieldReader<u8, u8>;
///Field `TDCO` reader - Transmitter Delay Compensation Offset
pub type TDCO_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:6 - Transmitter Delay Compensation Filter Window Length
    #[inline(always)]
    pub fn tdcf(&self) -> TDCF_R {
        TDCF_R::new((self.bits & 0x7f) as u8)
    }
    ///Bits 8:14 - Transmitter Delay Compensation Offset
    #[inline(always)]
    pub fn tdco(&self) -> TDCO_R {
        TDCO_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
///FDCAN Transmitter Delay Compensation Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fdcan_tdcr](index.html) module
pub struct FDCAN_TDCR_SPEC;
impl crate::RegisterSpec for FDCAN_TDCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [fdcan_tdcr::R](R) reader structure
impl crate::Readable for FDCAN_TDCR_SPEC {
    type Reader = R;
}
///`reset()` method sets FDCAN_TDCR to value 0
impl crate::Resettable for FDCAN_TDCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
