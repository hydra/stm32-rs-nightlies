///Register `SPI_TXCRC` reader
pub struct R(crate::R<SPI_TXCRC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_TXCRC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_TXCRC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_TXCRC_SPEC>) -> Self {
        R(reader)
    }
}
///Field `TXCRC` reader - TXCRC
pub type TXCRC_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - TXCRC
    #[inline(always)]
    pub fn txcrc(&self) -> TXCRC_R {
        TXCRC_R::new(self.bits)
    }
}
///SPI transmitter CRC register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [spi_txcrc](index.html) module
pub struct SPI_TXCRC_SPEC;
impl crate::RegisterSpec for SPI_TXCRC_SPEC {
    type Ux = u32;
}
///`read()` method returns [spi_txcrc::R](R) reader structure
impl crate::Readable for SPI_TXCRC_SPEC {
    type Reader = R;
}
///`reset()` method sets SPI_TXCRC to value 0
impl crate::Resettable for SPI_TXCRC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
