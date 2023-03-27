///Register `ETH_MACRxTxSR` reader
pub struct R(crate::R<ETH_MACRX_TX_SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACRX_TX_SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACRX_TX_SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACRX_TX_SR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `TJT` reader - TJT
pub type TJT_R = crate::BitReader<bool>;
///Field `NCARR` reader - NCARR
pub type NCARR_R = crate::BitReader<bool>;
///Field `LCARR` reader - LCARR
pub type LCARR_R = crate::BitReader<bool>;
///Field `EXDEF` reader - EXDEF
pub type EXDEF_R = crate::BitReader<bool>;
///Field `LCOL` reader - LCOL
pub type LCOL_R = crate::BitReader<bool>;
///Field `EXCOL` reader - EXCOL
pub type EXCOL_R = crate::BitReader<bool>;
///Field `RWT` reader - RWT
pub type RWT_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - TJT
    #[inline(always)]
    pub fn tjt(&self) -> TJT_R {
        TJT_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - NCARR
    #[inline(always)]
    pub fn ncarr(&self) -> NCARR_R {
        NCARR_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - LCARR
    #[inline(always)]
    pub fn lcarr(&self) -> LCARR_R {
        LCARR_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - EXDEF
    #[inline(always)]
    pub fn exdef(&self) -> EXDEF_R {
        EXDEF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - LCOL
    #[inline(always)]
    pub fn lcol(&self) -> LCOL_R {
        LCOL_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - EXCOL
    #[inline(always)]
    pub fn excol(&self) -> EXCOL_R {
        EXCOL_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - RWT
    #[inline(always)]
    pub fn rwt(&self) -> RWT_R {
        RWT_R::new(((self.bits >> 8) & 1) != 0)
    }
}
///The Receive Transmit Status register contains the Receive and Transmit Error status.
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [eth_macrx_tx_sr](index.html) module
pub struct ETH_MACRX_TX_SR_SPEC;
impl crate::RegisterSpec for ETH_MACRX_TX_SR_SPEC {
    type Ux = u32;
}
///`read()` method returns [eth_macrx_tx_sr::R](R) reader structure
impl crate::Readable for ETH_MACRX_TX_SR_SPEC {
    type Reader = R;
}
///`reset()` method sets ETH_MACRxTxSR to value 0
impl crate::Resettable for ETH_MACRX_TX_SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
