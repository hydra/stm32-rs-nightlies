///Register `HWCFGR` reader
pub struct R(crate::R<HWCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HWCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HWCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HWCFGR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `CRCCFG` reader - CRC capable at SPI mode
pub type CRCCFG_R = crate::FieldReader<u8, u8>;
///Field `I2SCFG` reader - I2S mode implementation
pub type I2SCFG_R = crate::FieldReader<u8, u8>;
///Field `I2SCKCFG` reader - I2S master clock generator at I2S mode
pub type I2SCKCFG_R = crate::FieldReader<u8, u8>;
///Field `DSCFG` reader - SPI data size configuration
pub type DSCFG_R = crate::FieldReader<u8, u8>;
///Field `NSSCFG` reader - NSS pulse feature enhancement at SPI master
pub type NSSCFG_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:3 - CRC capable at SPI mode
    #[inline(always)]
    pub fn crccfg(&self) -> CRCCFG_R {
        CRCCFG_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - I2S mode implementation
    #[inline(always)]
    pub fn i2scfg(&self) -> I2SCFG_R {
        I2SCFG_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - I2S master clock generator at I2S mode
    #[inline(always)]
    pub fn i2sckcfg(&self) -> I2SCKCFG_R {
        I2SCKCFG_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - SPI data size configuration
    #[inline(always)]
    pub fn dscfg(&self) -> DSCFG_R {
        DSCFG_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:19 - NSS pulse feature enhancement at SPI master
    #[inline(always)]
    pub fn nsscfg(&self) -> NSSCFG_R {
        NSSCFG_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
///hardware configuration register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hwcfgr](index.html) module
pub struct HWCFGR_SPEC;
impl crate::RegisterSpec for HWCFGR_SPEC {
    type Ux = u32;
}
///`read()` method returns [hwcfgr::R](R) reader structure
impl crate::Readable for HWCFGR_SPEC {
    type Reader = R;
}
///`reset()` method sets HWCFGR to value 0
impl crate::Resettable for HWCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
