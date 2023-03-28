///Register `ETH_MTLRxQ0DR` reader
pub struct R(crate::R<ETH_MTLRX_Q0DR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MTLRX_Q0DR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MTLRX_Q0DR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MTLRX_Q0DR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `RWCSTS` reader - RWCSTS
pub type RWCSTS_R = crate::BitReader<bool>;
///Field `RRCSTS` reader - RRCSTS
pub type RRCSTS_R = crate::FieldReader<u8, u8>;
///Field `RXQSTS` reader - RXQSTS
pub type RXQSTS_R = crate::FieldReader<u8, u8>;
///Field `PRXQ` reader - PRXQ
pub type PRXQ_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bit 0 - RWCSTS
    #[inline(always)]
    pub fn rwcsts(&self) -> RWCSTS_R {
        RWCSTS_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - RRCSTS
    #[inline(always)]
    pub fn rrcsts(&self) -> RRCSTS_R {
        RRCSTS_R::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bits 4:5 - RXQSTS
    #[inline(always)]
    pub fn rxqsts(&self) -> RXQSTS_R {
        RXQSTS_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 16:29 - PRXQ
    #[inline(always)]
    pub fn prxq(&self) -> PRXQ_R {
        PRXQ_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
///Rx queue i debug register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [eth_mtlrx_q0dr](index.html) module
pub struct ETH_MTLRX_Q0DR_SPEC;
impl crate::RegisterSpec for ETH_MTLRX_Q0DR_SPEC {
    type Ux = u32;
}
///`read()` method returns [eth_mtlrx_q0dr::R](R) reader structure
impl crate::Readable for ETH_MTLRX_Q0DR_SPEC {
    type Reader = R;
}
///`reset()` method sets ETH_MTLRxQ0DR to value 0
impl crate::Resettable for ETH_MTLRX_Q0DR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
