///Register `SPI_I2S_HWCFGR` reader
pub struct R(crate::R<SPI_I2S_HWCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_I2S_HWCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_I2S_HWCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_I2S_HWCFGR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `TXFCFG` reader - TXFCFG
pub type TXFCFG_R = crate::FieldReader<u8, u8>;
///Field `RXFCFG` reader - RXFCFG
pub type RXFCFG_R = crate::FieldReader<u8, u8>;
///Field `CRCCFG` reader - CRCCFG
pub type CRCCFG_R = crate::FieldReader<u8, u8>;
///Field `I2SCFG` reader - I2SCFG
pub type I2SCFG_R = crate::FieldReader<u8, u8>;
///Field `DSCFG` reader - DSCFG
pub type DSCFG_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:3 - TXFCFG
    #[inline(always)]
    pub fn txfcfg(&self) -> TXFCFG_R {
        TXFCFG_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - RXFCFG
    #[inline(always)]
    pub fn rxfcfg(&self) -> RXFCFG_R {
        RXFCFG_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - CRCCFG
    #[inline(always)]
    pub fn crccfg(&self) -> CRCCFG_R {
        CRCCFG_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - I2SCFG
    #[inline(always)]
    pub fn i2scfg(&self) -> I2SCFG_R {
        I2SCFG_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:19 - DSCFG
    #[inline(always)]
    pub fn dscfg(&self) -> DSCFG_R {
        DSCFG_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
///SPI/I2S hardware configuration register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [spi_i2s_hwcfgr](index.html) module
pub struct SPI_I2S_HWCFGR_SPEC;
impl crate::RegisterSpec for SPI_I2S_HWCFGR_SPEC {
    type Ux = u32;
}
///`read()` method returns [spi_i2s_hwcfgr::R](R) reader structure
impl crate::Readable for SPI_I2S_HWCFGR_SPEC {
    type Reader = R;
}
///`reset()` method sets SPI_I2S_HWCFGR to value 0
impl crate::Resettable for SPI_I2S_HWCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
