///Register `TXBTO` reader
pub struct R(crate::R<TXBTO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXBTO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXBTO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXBTO_SPEC>) -> Self {
        R(reader)
    }
}
///Field `TO` reader - TO
pub type TO_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:2 - TO
    #[inline(always)]
    pub fn to(&self) -> TO_R {
        TO_R::new((self.bits & 7) as u8)
    }
}
///FDCAN Tx Buffer Transmission Occurred Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [txbto](index.html) module
pub struct TXBTO_SPEC;
impl crate::RegisterSpec for TXBTO_SPEC {
    type Ux = u32;
}
///`read()` method returns [txbto::R](R) reader structure
impl crate::Readable for TXBTO_SPEC {
    type Reader = R;
}
///`reset()` method sets TXBTO to value 0
impl crate::Resettable for TXBTO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
