///Register `FDCAN_TXFQS` reader
pub struct R(crate::R<FDCAN_TXFQS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_TXFQS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_TXFQS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_TXFQS_SPEC>) -> Self {
        R(reader)
    }
}
///Field `TFFL` reader - TFFL
pub type TFFL_R = crate::FieldReader<u8, u8>;
///Field `TFGI` reader - TFGI
pub type TFGI_R = crate::FieldReader<u8, u8>;
///Field `TFQPI` reader - TFQPI
pub type TFQPI_R = crate::FieldReader<u8, u8>;
///Field `TFQF` reader - TFQF
pub type TFQF_R = crate::BitReader<bool>;
impl R {
    ///Bits 0:5 - TFFL
    #[inline(always)]
    pub fn tffl(&self) -> TFFL_R {
        TFFL_R::new((self.bits & 0x3f) as u8)
    }
    ///Bits 8:12 - TFGI
    #[inline(always)]
    pub fn tfgi(&self) -> TFGI_R {
        TFGI_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    ///Bits 16:20 - TFQPI
    #[inline(always)]
    pub fn tfqpi(&self) -> TFQPI_R {
        TFQPI_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bit 21 - TFQF
    #[inline(always)]
    pub fn tfqf(&self) -> TFQF_R {
        TFQF_R::new(((self.bits >> 21) & 1) != 0)
    }
}
///The Tx FIFO/queue status is related to the pending Tx requests listed in register FDCAN_TXBRP. Therefore the effect of add/cancellation requests may be delayed due to a running Tx scan (FDCAN_TXBRP not yet updated).
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fdcan_txfqs](index.html) module
pub struct FDCAN_TXFQS_SPEC;
impl crate::RegisterSpec for FDCAN_TXFQS_SPEC {
    type Ux = u32;
}
///`read()` method returns [fdcan_txfqs::R](R) reader structure
impl crate::Readable for FDCAN_TXFQS_SPEC {
    type Reader = R;
}
///`reset()` method sets FDCAN_TXFQS to value 0
impl crate::Resettable for FDCAN_TXFQS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
