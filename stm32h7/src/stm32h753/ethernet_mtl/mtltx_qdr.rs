///Register `MTLTxQDR` reader
pub struct R(crate::R<MTLTX_QDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MTLTX_QDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MTLTX_QDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MTLTX_QDR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `TXQPAUSED` reader - Transmit Queue in Pause
pub type TXQPAUSED_R = crate::BitReader<bool>;
///Field `TRCSTS` reader - MTL Tx Queue Read Controller Status
pub type TRCSTS_R = crate::FieldReader<u8, u8>;
///Field `TWCSTS` reader - MTL Tx Queue Write Controller Status
pub type TWCSTS_R = crate::BitReader<bool>;
///Field `TXQSTS` reader - MTL Tx Queue Not Empty Status
pub type TXQSTS_R = crate::BitReader<bool>;
///Field `TXSTSFSTS` reader - MTL Tx Status FIFO Full Status
pub type TXSTSFSTS_R = crate::BitReader<bool>;
///Field `PTXQ` reader - Number of Packets in the Transmit Queue
pub type PTXQ_R = crate::FieldReader<u8, u8>;
///Field `STXSTSF` reader - Number of Status Words in Tx Status FIFO of Queue
pub type STXSTSF_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bit 0 - Transmit Queue in Pause
    #[inline(always)]
    pub fn txqpaused(&self) -> TXQPAUSED_R {
        TXQPAUSED_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - MTL Tx Queue Read Controller Status
    #[inline(always)]
    pub fn trcsts(&self) -> TRCSTS_R {
        TRCSTS_R::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bit 3 - MTL Tx Queue Write Controller Status
    #[inline(always)]
    pub fn twcsts(&self) -> TWCSTS_R {
        TWCSTS_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - MTL Tx Queue Not Empty Status
    #[inline(always)]
    pub fn txqsts(&self) -> TXQSTS_R {
        TXQSTS_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - MTL Tx Status FIFO Full Status
    #[inline(always)]
    pub fn txstsfsts(&self) -> TXSTSFSTS_R {
        TXSTSFSTS_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 16:18 - Number of Packets in the Transmit Queue
    #[inline(always)]
    pub fn ptxq(&self) -> PTXQ_R {
        PTXQ_R::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bits 20:22 - Number of Status Words in Tx Status FIFO of Queue
    #[inline(always)]
    pub fn stxstsf(&self) -> STXSTSF_R {
        STXSTSF_R::new(((self.bits >> 20) & 7) as u8)
    }
}
///Tx queue debug Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mtltx_qdr](index.html) module
pub struct MTLTX_QDR_SPEC;
impl crate::RegisterSpec for MTLTX_QDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [mtltx_qdr::R](R) reader structure
impl crate::Readable for MTLTX_QDR_SPEC {
    type Reader = R;
}
///`reset()` method sets MTLTxQDR to value 0
impl crate::Resettable for MTLTX_QDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
