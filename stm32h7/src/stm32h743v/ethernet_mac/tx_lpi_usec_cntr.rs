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
///Field `TXLPIUSC` reader - Tx LPI Microseconds Counter
pub type TXLPIUSC_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - Tx LPI Microseconds Counter
    #[inline(always)]
    pub fn txlpiusc(&self) -> TXLPIUSC_R {
        TXLPIUSC_R::new(self.bits)
    }
}
///Tx LPI microsecond timer register
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
