///Register `SPI_RXCRC` reader
pub struct R(crate::R<SPI_RXCRC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_RXCRC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_RXCRC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_RXCRC_SPEC>) -> Self {
        R(reader)
    }
}
///Field `RXCRC` reader - RXCRC
pub type RXCRC_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - RXCRC
    #[inline(always)]
    pub fn rxcrc(&self) -> RXCRC_R {
        RXCRC_R::new(self.bits)
    }
}
///SPI receiver CRC register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [spi_rxcrc](index.html) module
pub struct SPI_RXCRC_SPEC;
impl crate::RegisterSpec for SPI_RXCRC_SPEC {
    type Ux = u32;
}
///`read()` method returns [spi_rxcrc::R](R) reader structure
impl crate::Readable for SPI_RXCRC_SPEC {
    type Reader = R;
}
///`reset()` method sets SPI_RXCRC to value 0
impl crate::Resettable for SPI_RXCRC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
