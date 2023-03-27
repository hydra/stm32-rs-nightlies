///Register `I2C_RXDR` reader
pub struct R(crate::R<I2C_RXDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_RXDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_RXDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_RXDR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `RXDATA` reader - RXDATA
pub type RXDATA_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:7 - RXDATA
    #[inline(always)]
    pub fn rxdata(&self) -> RXDATA_R {
        RXDATA_R::new((self.bits & 0xff) as u8)
    }
}
///Access: No wait states
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [i2c_rxdr](index.html) module
pub struct I2C_RXDR_SPEC;
impl crate::RegisterSpec for I2C_RXDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [i2c_rxdr::R](R) reader structure
impl crate::Readable for I2C_RXDR_SPEC {
    type Reader = R;
}
///`reset()` method sets I2C_RXDR to value 0
impl crate::Resettable for I2C_RXDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
