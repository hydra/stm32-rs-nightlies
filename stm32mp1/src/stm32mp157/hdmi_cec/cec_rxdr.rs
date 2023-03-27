///Register `CEC_RXDR` reader
pub struct R(crate::R<CEC_RXDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CEC_RXDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CEC_RXDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CEC_RXDR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `RXD` reader - RXD
pub type RXD_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:7 - RXD
    #[inline(always)]
    pub fn rxd(&self) -> RXD_R {
        RXD_R::new((self.bits & 0xff) as u8)
    }
}
///CEC Rx data register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cec_rxdr](index.html) module
pub struct CEC_RXDR_SPEC;
impl crate::RegisterSpec for CEC_RXDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [cec_rxdr::R](R) reader structure
impl crate::Readable for CEC_RXDR_SPEC {
    type Reader = R;
}
///`reset()` method sets CEC_RXDR to value 0
impl crate::Resettable for CEC_RXDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
