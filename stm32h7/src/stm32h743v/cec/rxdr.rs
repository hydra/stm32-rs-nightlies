///Register `RXDR` reader
pub struct R(crate::R<RXDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXDR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `RXD` reader - Rx Data register. RXD is read-only and contains the last data byte which has been received from the CEC line.
pub type RXD_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:7 - Rx Data register. RXD is read-only and contains the last data byte which has been received from the CEC line.
    #[inline(always)]
    pub fn rxd(&self) -> RXD_R {
        RXD_R::new((self.bits & 0xff) as u8)
    }
}
///CEC Rx Data Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rxdr](index.html) module
pub struct RXDR_SPEC;
impl crate::RegisterSpec for RXDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [rxdr::R](R) reader structure
impl crate::Readable for RXDR_SPEC {
    type Reader = R;
}
///`reset()` method sets RXDR to value 0
impl crate::Resettable for RXDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
