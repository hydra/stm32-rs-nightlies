///Register `RX_CRC_ERROR_PACKETS` reader
pub struct R(crate::R<RX_CRC_ERROR_PACKETS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_CRC_ERROR_PACKETS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_CRC_ERROR_PACKETS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_CRC_ERROR_PACKETS_SPEC>) -> Self {
        R(reader)
    }
}
///Field `RXCRCERR` reader - Rx CRC Error Packets
pub type RXCRCERR_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - Rx CRC Error Packets
    #[inline(always)]
    pub fn rxcrcerr(&self) -> RXCRCERR_R {
        RXCRCERR_R::new(self.bits)
    }
}
///Rx CRC error packets register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rx_crc_error_packets](index.html) module
pub struct RX_CRC_ERROR_PACKETS_SPEC;
impl crate::RegisterSpec for RX_CRC_ERROR_PACKETS_SPEC {
    type Ux = u32;
}
///`read()` method returns [rx_crc_error_packets::R](R) reader structure
impl crate::Readable for RX_CRC_ERROR_PACKETS_SPEC {
    type Reader = R;
}
///`reset()` method sets RX_CRC_ERROR_PACKETS to value 0
impl crate::Resettable for RX_CRC_ERROR_PACKETS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
