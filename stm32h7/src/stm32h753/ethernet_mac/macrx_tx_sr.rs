///Register `MACRxTxSR` reader
pub struct R(crate::R<MACRX_TX_SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACRX_TX_SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACRX_TX_SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACRX_TX_SR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `TJT` reader - Transmit Jabber Timeout
pub type TJT_R = crate::BitReader<bool>;
///Field `NCARR` reader - No Carrier
pub type NCARR_R = crate::BitReader<bool>;
///Field `LCARR` reader - Loss of Carrier
pub type LCARR_R = crate::BitReader<bool>;
///Field `EXDEF` reader - Excessive Deferral
pub type EXDEF_R = crate::BitReader<bool>;
///Field `LCOL` reader - Late Collision
pub type LCOL_R = crate::BitReader<bool>;
///Field `EXCOL` reader - Excessive Collisions
pub type EXCOL_R = crate::BitReader<bool>;
///Field `RWT` reader - Receive Watchdog Timeout
pub type RWT_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - Transmit Jabber Timeout
    #[inline(always)]
    pub fn tjt(&self) -> TJT_R {
        TJT_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - No Carrier
    #[inline(always)]
    pub fn ncarr(&self) -> NCARR_R {
        NCARR_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Loss of Carrier
    #[inline(always)]
    pub fn lcarr(&self) -> LCARR_R {
        LCARR_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Excessive Deferral
    #[inline(always)]
    pub fn exdef(&self) -> EXDEF_R {
        EXDEF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Late Collision
    #[inline(always)]
    pub fn lcol(&self) -> LCOL_R {
        LCOL_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Excessive Collisions
    #[inline(always)]
    pub fn excol(&self) -> EXCOL_R {
        EXCOL_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - Receive Watchdog Timeout
    #[inline(always)]
    pub fn rwt(&self) -> RWT_R {
        RWT_R::new(((self.bits >> 8) & 1) != 0)
    }
}
///Rx Tx status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [macrx_tx_sr](index.html) module
pub struct MACRX_TX_SR_SPEC;
impl crate::RegisterSpec for MACRX_TX_SR_SPEC {
    type Ux = u32;
}
///`read()` method returns [macrx_tx_sr::R](R) reader structure
impl crate::Readable for MACRX_TX_SR_SPEC {
    type Reader = R;
}
///`reset()` method sets MACRxTxSR to value 0
impl crate::Resettable for MACRX_TX_SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
