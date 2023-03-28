///Register `RX_PAYSZ` reader
pub struct R(crate::R<RX_PAYSZ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_PAYSZ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_PAYSZ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_PAYSZ_SPEC>) -> Self {
        R(reader)
    }
}
///Field `RXPAYSZ` reader - RXPAYSZ
pub type RXPAYSZ_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:9 - RXPAYSZ
    #[inline(always)]
    pub fn rxpaysz(&self) -> RXPAYSZ_R {
        RXPAYSZ_R::new((self.bits & 0x03ff) as u16)
    }
}
///UCPD Rx Paysize Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rx_paysz](index.html) module
pub struct RX_PAYSZ_SPEC;
impl crate::RegisterSpec for RX_PAYSZ_SPEC {
    type Ux = u32;
}
///`read()` method returns [rx_paysz::R](R) reader structure
impl crate::Readable for RX_PAYSZ_SPEC {
    type Reader = R;
}
///`reset()` method sets RX_PAYSZ to value 0
impl crate::Resettable for RX_PAYSZ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
