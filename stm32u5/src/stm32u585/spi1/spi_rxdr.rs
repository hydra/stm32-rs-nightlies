///Register `SPI_RXDR` reader
pub struct R(crate::R<SPI_RXDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_RXDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_RXDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_RXDR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `RXDR` reader - receive data register The register serves as an interface with RxFIFO. When it is read, RxFIFO is accessed. Note: data is always right-aligned. Unused bits are read as zero when the register is read. Writing to the register is ignored. Note: DR can be accessed byte-wise (8-bit access): in this case only one data-byte is read by single access halfword-wise (16 bit access) in this case 2 data-bytes or 1 halfword-data can be read by single access word-wise (32 bit access). In this case 4 data-bytes or 2 halfword-data or word-data can be read by single access. Read access of this register less than the configured data size is forbidden.
pub type RXDR_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - receive data register The register serves as an interface with RxFIFO. When it is read, RxFIFO is accessed. Note: data is always right-aligned. Unused bits are read as zero when the register is read. Writing to the register is ignored. Note: DR can be accessed byte-wise (8-bit access): in this case only one data-byte is read by single access halfword-wise (16 bit access) in this case 2 data-bytes or 1 halfword-data can be read by single access word-wise (32 bit access). In this case 4 data-bytes or 2 halfword-data or word-data can be read by single access. Read access of this register less than the configured data size is forbidden.
    #[inline(always)]
    pub fn rxdr(&self) -> RXDR_R {
        RXDR_R::new(self.bits)
    }
}
///
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [spi_rxdr](index.html) module
pub struct SPI_RXDR_SPEC;
impl crate::RegisterSpec for SPI_RXDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [spi_rxdr::R](R) reader structure
impl crate::Readable for SPI_RXDR_SPEC {
    type Reader = R;
}
///`reset()` method sets SPI_RXDR to value 0
impl crate::Resettable for SPI_RXDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
