///Register `RXD` reader
pub struct R(crate::R<RXD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXD_SPEC>) -> Self {
        R(reader)
    }
}
///Field `RXD` reader - Rx data
pub type RXD_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:7 - Rx data
    #[inline(always)]
    pub fn rxd(&self) -> RXD_R {
        RXD_R::new((self.bits & 0xff) as u8)
    }
}
///CEC Rx data register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rxd](index.html) module
pub struct RXD_SPEC;
impl crate::RegisterSpec for RXD_SPEC {
    type Ux = u32;
}
///`read()` method returns [rxd::R](R) reader structure
impl crate::Readable for RXD_SPEC {
    type Reader = R;
}
///`reset()` method sets RXD to value 0
impl crate::Resettable for RXD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
