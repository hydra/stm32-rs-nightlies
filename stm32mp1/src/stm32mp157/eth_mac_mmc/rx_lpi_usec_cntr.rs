///Register `RX_LPI_USEC_CNTR` reader
pub struct R(crate::R<RX_LPI_USEC_CNTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_LPI_USEC_CNTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_LPI_USEC_CNTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_LPI_USEC_CNTR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `RXLPIUSC` reader - RXLPIUSC
pub type RXLPIUSC_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - RXLPIUSC
    #[inline(always)]
    pub fn rxlpiusc(&self) -> RXLPIUSC_R {
        RXLPIUSC_R::new(self.bits)
    }
}
///This register provides the number of microseconds Rx LPI is sampled by Ethernet peripheral.
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rx_lpi_usec_cntr](index.html) module
pub struct RX_LPI_USEC_CNTR_SPEC;
impl crate::RegisterSpec for RX_LPI_USEC_CNTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [rx_lpi_usec_cntr::R](R) reader structure
impl crate::Readable for RX_LPI_USEC_CNTR_SPEC {
    type Reader = R;
}
///`reset()` method sets RX_LPI_USEC_CNTR to value 0
impl crate::Resettable for RX_LPI_USEC_CNTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
