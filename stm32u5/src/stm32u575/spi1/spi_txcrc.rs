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
///Field `TXCRC` reader - CRC register for transmitter When CRC calculation is enabled, the TXCRC\[31:0\]
///bits contain the computed CRC value of the subsequently transmitted bytes. CRC calculation is initialized when the CRCEN bit of SPI_CR1 is written to 1 or when a data block is transacted completely. The CRC is calculated serially using the polynomial programmed in the SPI_CRCPOLY register. The number of bits considered at calculation depends on SPI_CRCPOLY register and CRCSIZE bits settings at SPI_CFG1 register. Note: a read to this register when the communication is ongoing could return an incorrect value. Note: TXCRC\[31-16\]
///bits are reserved at instances with data size limited to 16-bit. There is no constrain when 32-bit access is applied at these addresses. Reserved bits 31-16 are always read zero while any write to them is ignored. Note: The configuration of CRCSIZE bit field is not taken into account when the content of this register is read by software. No masking is applied for unused bits at this case.
pub type TXCRC_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - CRC register for transmitter When CRC calculation is enabled, the TXCRC\[31:0\]
    ///bits contain the computed CRC value of the subsequently transmitted bytes. CRC calculation is initialized when the CRCEN bit of SPI_CR1 is written to 1 or when a data block is transacted completely. The CRC is calculated serially using the polynomial programmed in the SPI_CRCPOLY register. The number of bits considered at calculation depends on SPI_CRCPOLY register and CRCSIZE bits settings at SPI_CFG1 register. Note: a read to this register when the communication is ongoing could return an incorrect value. Note: TXCRC\[31-16\]
    ///bits are reserved at instances with data size limited to 16-bit. There is no constrain when 32-bit access is applied at these addresses. Reserved bits 31-16 are always read zero while any write to them is ignored. Note: The configuration of CRCSIZE bit field is not taken into account when the content of this register is read by software. No masking is applied for unused bits at this case.
    #[inline(always)]
    pub fn txcrc(&self) -> TXCRC_R {
        TXCRC_R::new(self.bits)
    }
}
///
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
