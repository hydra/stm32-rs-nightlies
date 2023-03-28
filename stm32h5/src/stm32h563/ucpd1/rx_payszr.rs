///Register `RX_PAYSZR` reader
pub struct R(crate::R<RX_PAYSZR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_PAYSZR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_PAYSZR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_PAYSZR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `RXPAYSZ` reader - Rx payload size received This bitfield contains the number of bytes of a payload (including header but excluding CRC) received: each time a new data byte is received in the UCPD_RXDR register, the bitfield value increments and the RXMSGEND flag is set (and an interrupt generated if enabled). The bitfield may return a spurious value when a byte reception is ongoing (the RXMSGEND flag is low).
pub type RXPAYSZ_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:9 - Rx payload size received This bitfield contains the number of bytes of a payload (including header but excluding CRC) received: each time a new data byte is received in the UCPD_RXDR register, the bitfield value increments and the RXMSGEND flag is set (and an interrupt generated if enabled). The bitfield may return a spurious value when a byte reception is ongoing (the RXMSGEND flag is low).
    #[inline(always)]
    pub fn rxpaysz(&self) -> RXPAYSZ_R {
        RXPAYSZ_R::new((self.bits & 0x03ff) as u16)
    }
}
///UCPD Rx payload size register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rx_payszr](index.html) module
pub struct RX_PAYSZR_SPEC;
impl crate::RegisterSpec for RX_PAYSZR_SPEC {
    type Ux = u32;
}
///`read()` method returns [rx_payszr::R](R) reader structure
impl crate::Readable for RX_PAYSZR_SPEC {
    type Reader = R;
}
///`reset()` method sets RX_PAYSZR to value 0
impl crate::Resettable for RX_PAYSZR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
