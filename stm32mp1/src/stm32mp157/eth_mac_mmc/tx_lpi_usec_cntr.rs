///Register `TX_LPI_USEC_CNTR` reader
pub struct R(crate::R<TX_LPI_USEC_CNTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_LPI_USEC_CNTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_LPI_USEC_CNTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_LPI_USEC_CNTR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `TXLPIUSC` reader - TXLPIUSC
pub type TXLPIUSC_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - TXLPIUSC
    #[inline(always)]
    pub fn txlpiusc(&self) -> TXLPIUSC_R {
        TXLPIUSC_R::new(self.bits)
    }
}
///This register provides the number of microseconds Tx LPI is asserted by Ethernet peripheral.
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tx_lpi_usec_cntr](index.html) module
pub struct TX_LPI_USEC_CNTR_SPEC;
impl crate::RegisterSpec for TX_LPI_USEC_CNTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [tx_lpi_usec_cntr::R](R) reader structure
impl crate::Readable for TX_LPI_USEC_CNTR_SPEC {
    type Reader = R;
}
///`reset()` method sets TX_LPI_USEC_CNTR to value 0
impl crate::Resettable for TX_LPI_USEC_CNTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
