///Register `MMC_RX_INTERRUPT` reader
pub struct R(crate::R<MMC_RX_INTERRUPT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMC_RX_INTERRUPT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMC_RX_INTERRUPT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMC_RX_INTERRUPT_SPEC>) -> Self {
        R(reader)
    }
}
///Field `RXCRCERPIS` reader - MMC Receive CRC Error Packet Counter Interrupt Status
pub type RXCRCERPIS_R = crate::BitReader<bool>;
///Field `RXALGNERPIS` reader - MMC Receive Alignment Error Packet Counter Interrupt Status
pub type RXALGNERPIS_R = crate::BitReader<bool>;
///Field `RXUCGPIS` reader - MMC Receive Unicast Good Packet Counter Interrupt Status
pub type RXUCGPIS_R = crate::BitReader<bool>;
///Field `RXLPIUSCIS` reader - MMC Receive LPI microsecond counter interrupt status
pub type RXLPIUSCIS_R = crate::BitReader<bool>;
///Field `RXLPITRCIS` reader - MMC Receive LPI transition counter interrupt status
pub type RXLPITRCIS_R = crate::BitReader<bool>;
impl R {
    ///Bit 5 - MMC Receive CRC Error Packet Counter Interrupt Status
    #[inline(always)]
    pub fn rxcrcerpis(&self) -> RXCRCERPIS_R {
        RXCRCERPIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - MMC Receive Alignment Error Packet Counter Interrupt Status
    #[inline(always)]
    pub fn rxalgnerpis(&self) -> RXALGNERPIS_R {
        RXALGNERPIS_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 17 - MMC Receive Unicast Good Packet Counter Interrupt Status
    #[inline(always)]
    pub fn rxucgpis(&self) -> RXUCGPIS_R {
        RXUCGPIS_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 26 - MMC Receive LPI microsecond counter interrupt status
    #[inline(always)]
    pub fn rxlpiuscis(&self) -> RXLPIUSCIS_R {
        RXLPIUSCIS_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - MMC Receive LPI transition counter interrupt status
    #[inline(always)]
    pub fn rxlpitrcis(&self) -> RXLPITRCIS_R {
        RXLPITRCIS_R::new(((self.bits >> 27) & 1) != 0)
    }
}
///MMC Rx interrupt register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mmc_rx_interrupt](index.html) module
pub struct MMC_RX_INTERRUPT_SPEC;
impl crate::RegisterSpec for MMC_RX_INTERRUPT_SPEC {
    type Ux = u32;
}
///`read()` method returns [mmc_rx_interrupt::R](R) reader structure
impl crate::Readable for MMC_RX_INTERRUPT_SPEC {
    type Reader = R;
}
///`reset()` method sets MMC_RX_INTERRUPT to value 0
impl crate::Resettable for MMC_RX_INTERRUPT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
