///Register `TX_LPI_TRAN_CNTR` reader
pub struct R(crate::R<TX_LPI_TRAN_CNTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_LPI_TRAN_CNTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_LPI_TRAN_CNTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_LPI_TRAN_CNTR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `TXLPITRC` reader - TXLPITRC
pub type TXLPITRC_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - TXLPITRC
    #[inline(always)]
    pub fn txlpitrc(&self) -> TXLPITRC_R {
        TXLPITRC_R::new(self.bits)
    }
}
///This register provides the number of times Ethernet peripheral has entered Tx LPI.
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tx_lpi_tran_cntr](index.html) module
pub struct TX_LPI_TRAN_CNTR_SPEC;
impl crate::RegisterSpec for TX_LPI_TRAN_CNTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [tx_lpi_tran_cntr::R](R) reader structure
impl crate::Readable for TX_LPI_TRAN_CNTR_SPEC {
    type Reader = R;
}
///`reset()` method sets TX_LPI_TRAN_CNTR to value 0
impl crate::Resettable for TX_LPI_TRAN_CNTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
