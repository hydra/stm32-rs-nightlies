///Register `TX_PACKET_COUNT_GOOD` reader
pub struct R(crate::R<TX_PACKET_COUNT_GOOD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_PACKET_COUNT_GOOD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_PACKET_COUNT_GOOD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_PACKET_COUNT_GOOD_SPEC>) -> Self {
        R(reader)
    }
}
///Field `TXPKTG` reader - TXPKTG
pub type TXPKTG_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - TXPKTG
    #[inline(always)]
    pub fn txpktg(&self) -> TXPKTG_R {
        TXPKTG_R::new(self.bits)
    }
}
///This register provides the number of good packets transmitted by Ethernet peripheral.
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tx_packet_count_good](index.html) module
pub struct TX_PACKET_COUNT_GOOD_SPEC;
impl crate::RegisterSpec for TX_PACKET_COUNT_GOOD_SPEC {
    type Ux = u32;
}
///`read()` method returns [tx_packet_count_good::R](R) reader structure
impl crate::Readable for TX_PACKET_COUNT_GOOD_SPEC {
    type Reader = R;
}
///`reset()` method sets TX_PACKET_COUNT_GOOD to value 0
impl crate::Resettable for TX_PACKET_COUNT_GOOD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
