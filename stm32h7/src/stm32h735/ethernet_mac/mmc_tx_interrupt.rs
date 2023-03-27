///Register `MMC_TX_INTERRUPT` reader
pub struct R(crate::R<MMC_TX_INTERRUPT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMC_TX_INTERRUPT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMC_TX_INTERRUPT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMC_TX_INTERRUPT_SPEC>) -> Self {
        R(reader)
    }
}
///Field `TXSCOLGPIS` reader - MMC Transmit Single Collision Good Packet Counter Interrupt Status
pub type TXSCOLGPIS_R = crate::BitReader<bool>;
///Field `TXMCOLGPIS` reader - MMC Transmit Multiple Collision Good Packet Counter Interrupt Status
pub type TXMCOLGPIS_R = crate::BitReader<bool>;
///Field `TXGPKTIS` reader - MMC Transmit Good Packet Counter Interrupt Status
pub type TXGPKTIS_R = crate::BitReader<bool>;
///Field `TXLPIUSCIS` reader - MMC Transmit LPI microsecond counter interrupt status
pub type TXLPIUSCIS_R = crate::BitReader<bool>;
///Field `TXLPITRCIS` reader - MMC Transmit LPI transition counter interrupt status
pub type TXLPITRCIS_R = crate::BitReader<bool>;
impl R {
    ///Bit 14 - MMC Transmit Single Collision Good Packet Counter Interrupt Status
    #[inline(always)]
    pub fn txscolgpis(&self) -> TXSCOLGPIS_R {
        TXSCOLGPIS_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - MMC Transmit Multiple Collision Good Packet Counter Interrupt Status
    #[inline(always)]
    pub fn txmcolgpis(&self) -> TXMCOLGPIS_R {
        TXMCOLGPIS_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 21 - MMC Transmit Good Packet Counter Interrupt Status
    #[inline(always)]
    pub fn txgpktis(&self) -> TXGPKTIS_R {
        TXGPKTIS_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 26 - MMC Transmit LPI microsecond counter interrupt status
    #[inline(always)]
    pub fn txlpiuscis(&self) -> TXLPIUSCIS_R {
        TXLPIUSCIS_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - MMC Transmit LPI transition counter interrupt status
    #[inline(always)]
    pub fn txlpitrcis(&self) -> TXLPITRCIS_R {
        TXLPITRCIS_R::new(((self.bits >> 27) & 1) != 0)
    }
}
///MMC Tx interrupt register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mmc_tx_interrupt](index.html) module
pub struct MMC_TX_INTERRUPT_SPEC;
impl crate::RegisterSpec for MMC_TX_INTERRUPT_SPEC {
    type Ux = u32;
}
///`read()` method returns [mmc_tx_interrupt::R](R) reader structure
impl crate::Readable for MMC_TX_INTERRUPT_SPEC {
    type Reader = R;
}
///`reset()` method sets MMC_TX_INTERRUPT to value 0
impl crate::Resettable for MMC_TX_INTERRUPT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
