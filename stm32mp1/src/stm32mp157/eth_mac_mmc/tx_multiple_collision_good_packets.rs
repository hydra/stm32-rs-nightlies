///Register `TX_MULTIPLE_COLLISION_GOOD_PACKETS` reader
pub struct R(crate::R<TX_MULTIPLE_COLLISION_GOOD_PACKETS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_MULTIPLE_COLLISION_GOOD_PACKETS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_MULTIPLE_COLLISION_GOOD_PACKETS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_MULTIPLE_COLLISION_GOOD_PACKETS_SPEC>) -> Self {
        R(reader)
    }
}
///Field `TXMULTCOLG` reader - TXMULTCOLG
pub type TXMULTCOLG_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - TXMULTCOLG
    #[inline(always)]
    pub fn txmultcolg(&self) -> TXMULTCOLG_R {
        TXMULTCOLG_R::new(self.bits)
    }
}
///This register provides the number of successfully transmitted packets by Ethernet peripheral after multiple collisions in the half-duplex mode.
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tx_multiple_collision_good_packets](index.html) module
pub struct TX_MULTIPLE_COLLISION_GOOD_PACKETS_SPEC;
impl crate::RegisterSpec for TX_MULTIPLE_COLLISION_GOOD_PACKETS_SPEC {
    type Ux = u32;
}
///`read()` method returns [tx_multiple_collision_good_packets::R](R) reader structure
impl crate::Readable for TX_MULTIPLE_COLLISION_GOOD_PACKETS_SPEC {
    type Reader = R;
}
///`reset()` method sets TX_MULTIPLE_COLLISION_GOOD_PACKETS to value 0
impl crate::Resettable for TX_MULTIPLE_COLLISION_GOOD_PACKETS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
