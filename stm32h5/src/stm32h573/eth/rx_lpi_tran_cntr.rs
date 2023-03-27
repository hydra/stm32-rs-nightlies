///Register `RX_LPI_TRAN_CNTR` reader
pub struct R(crate::R<RX_LPI_TRAN_CNTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_LPI_TRAN_CNTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_LPI_TRAN_CNTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_LPI_TRAN_CNTR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `RXLPITRC` reader - Rx LPI Transition counter This field indicates the number of times Rx LPI Entry has occurred.
pub type RXLPITRC_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - Rx LPI Transition counter This field indicates the number of times Rx LPI Entry has occurred.
    #[inline(always)]
    pub fn rxlpitrc(&self) -> RXLPITRC_R {
        RXLPITRC_R::new(self.bits)
    }
}
///Rx LPI transition counter register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rx_lpi_tran_cntr](index.html) module
pub struct RX_LPI_TRAN_CNTR_SPEC;
impl crate::RegisterSpec for RX_LPI_TRAN_CNTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [rx_lpi_tran_cntr::R](R) reader structure
impl crate::Readable for RX_LPI_TRAN_CNTR_SPEC {
    type Reader = R;
}
///`reset()` method sets RX_LPI_TRAN_CNTR to value 0
impl crate::Resettable for RX_LPI_TRAN_CNTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
