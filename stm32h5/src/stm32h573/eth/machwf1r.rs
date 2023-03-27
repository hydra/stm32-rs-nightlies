///Register `MACHWF1R` reader
pub struct R(crate::R<MACHWF1R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACHWF1R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACHWF1R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACHWF1R_SPEC>) -> Self {
        R(reader)
    }
}
///Field `RXFIFOSIZE` reader - MTL Receive FIFO Size This field contains the configured value of MTL Rx FIFO in bytes expressed as Log to base 2 minus 7, that is, Log2(RXFIFO_SIZE) -7: 01100 to 11111: Reserved, must not be used
pub type RXFIFOSIZE_R = crate::FieldReader<u8, u8>;
///Field `TXFIFOSIZE` reader - MTL Transmit FIFO Size This field contains the configured value of MTL Tx FIFO in bytes expressed as Log to base 2 minus 7, that is, Log2(TXFIFO_SIZE) -7: 01011 to 11111: Reserved, must not be used
pub type TXFIFOSIZE_R = crate::FieldReader<u8, u8>;
///Field `OSTEN` reader - One-Step Timestamping Enable This bit is set to 1 when the Enable One-Step Timestamp Feature is selected.
pub type OSTEN_R = crate::BitReader<bool>;
///Field `PTOEN` reader - PTP Offload Enable This bit is set to 1 when the Enable PTP Timestamp Offload Feature is selected.
pub type PTOEN_R = crate::BitReader<bool>;
///Field `ADVTHWORD` reader - IEEE 1588 High Word Register Enable This bit is set to 1 when the Add IEEE 1588 Higher Word Register option is selected
pub type ADVTHWORD_R = crate::BitReader<bool>;
///Field `ADDR64` reader - Address width This field indicates the configured address width. Others: Reserved, must not be used
pub type ADDR64_R = crate::FieldReader<u8, u8>;
///Field `DCBEN` reader - DCB Feature Enable This bit is set to 1 when the Enable Data Center Bridging option is selected
pub type DCBEN_R = crate::BitReader<bool>;
///Field `SPHEN` reader - Split Header Feature Enable This bit is set to 1 when the Enable Split Header Structure option is selected
pub type SPHEN_R = crate::BitReader<bool>;
///Field `TSOEN` reader - TCP Segmentation Offload Enable This bit is set to 1 when the Enable TCP Segmentation Offloading for TCP/IP Packets option is selected
pub type TSOEN_R = crate::BitReader<bool>;
///Field `DBGMEMA` reader - DMA Debug Registers Enable This bit is set to 1 when the Debug Mode Enable option is selected
pub type DBGMEMA_R = crate::BitReader<bool>;
///Field `AVSEL` reader - AV Feature Enable This bit is set to 1 when the Enable Audio video bridging option is selected.
pub type AVSEL_R = crate::BitReader<bool>;
///Field `RAVSEL` reader - Rx Side Only AV Feature Enable This bit is set to 1 when the Enable Audio video bridging option on Rx Side Only is selected.
pub type RAVSEL_R = crate::BitReader<bool>;
///Field `POUOST` reader - One Step for PTP over UDP/IP Feature Enable This bit is set to 1 when the Enable one step timestamp for PTP over UDP/IP feature is selected.
pub type POUOST_R = crate::BitReader<bool>;
///Field `HASHTBLSZ` reader - Hash Table Size This field indicates the size of the Hash table:
pub type HASHTBLSZ_R = crate::FieldReader<u8, u8>;
///Field `L3L4FNUM` reader - Total number of L3 or L4 Filters This field indicates the total number of L3 or L4 filters: ..
pub type L3L4FNUM_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:4 - MTL Receive FIFO Size This field contains the configured value of MTL Rx FIFO in bytes expressed as Log to base 2 minus 7, that is, Log2(RXFIFO_SIZE) -7: 01100 to 11111: Reserved, must not be used
    #[inline(always)]
    pub fn rxfifosize(&self) -> RXFIFOSIZE_R {
        RXFIFOSIZE_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 6:10 - MTL Transmit FIFO Size This field contains the configured value of MTL Tx FIFO in bytes expressed as Log to base 2 minus 7, that is, Log2(TXFIFO_SIZE) -7: 01011 to 11111: Reserved, must not be used
    #[inline(always)]
    pub fn txfifosize(&self) -> TXFIFOSIZE_R {
        TXFIFOSIZE_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    ///Bit 11 - One-Step Timestamping Enable This bit is set to 1 when the Enable One-Step Timestamp Feature is selected.
    #[inline(always)]
    pub fn osten(&self) -> OSTEN_R {
        OSTEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - PTP Offload Enable This bit is set to 1 when the Enable PTP Timestamp Offload Feature is selected.
    #[inline(always)]
    pub fn ptoen(&self) -> PTOEN_R {
        PTOEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - IEEE 1588 High Word Register Enable This bit is set to 1 when the Add IEEE 1588 Higher Word Register option is selected
    #[inline(always)]
    pub fn advthword(&self) -> ADVTHWORD_R {
        ADVTHWORD_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bits 14:15 - Address width This field indicates the configured address width. Others: Reserved, must not be used
    #[inline(always)]
    pub fn addr64(&self) -> ADDR64_R {
        ADDR64_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bit 16 - DCB Feature Enable This bit is set to 1 when the Enable Data Center Bridging option is selected
    #[inline(always)]
    pub fn dcben(&self) -> DCBEN_R {
        DCBEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Split Header Feature Enable This bit is set to 1 when the Enable Split Header Structure option is selected
    #[inline(always)]
    pub fn sphen(&self) -> SPHEN_R {
        SPHEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - TCP Segmentation Offload Enable This bit is set to 1 when the Enable TCP Segmentation Offloading for TCP/IP Packets option is selected
    #[inline(always)]
    pub fn tsoen(&self) -> TSOEN_R {
        TSOEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - DMA Debug Registers Enable This bit is set to 1 when the Debug Mode Enable option is selected
    #[inline(always)]
    pub fn dbgmema(&self) -> DBGMEMA_R {
        DBGMEMA_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - AV Feature Enable This bit is set to 1 when the Enable Audio video bridging option is selected.
    #[inline(always)]
    pub fn avsel(&self) -> AVSEL_R {
        AVSEL_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Rx Side Only AV Feature Enable This bit is set to 1 when the Enable Audio video bridging option on Rx Side Only is selected.
    #[inline(always)]
    pub fn ravsel(&self) -> RAVSEL_R {
        RAVSEL_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 23 - One Step for PTP over UDP/IP Feature Enable This bit is set to 1 when the Enable one step timestamp for PTP over UDP/IP feature is selected.
    #[inline(always)]
    pub fn pouost(&self) -> POUOST_R {
        POUOST_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bits 24:25 - Hash Table Size This field indicates the size of the Hash table:
    #[inline(always)]
    pub fn hashtblsz(&self) -> HASHTBLSZ_R {
        HASHTBLSZ_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bits 27:30 - Total number of L3 or L4 Filters This field indicates the total number of L3 or L4 filters: ..
    #[inline(always)]
    pub fn l3l4fnum(&self) -> L3L4FNUM_R {
        L3L4FNUM_R::new(((self.bits >> 27) & 0x0f) as u8)
    }
}
///HW feature 1 register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [machwf1r](index.html) module
pub struct MACHWF1R_SPEC;
impl crate::RegisterSpec for MACHWF1R_SPEC {
    type Ux = u32;
}
///`read()` method returns [machwf1r::R](R) reader structure
impl crate::Readable for MACHWF1R_SPEC {
    type Reader = R;
}
///`reset()` method sets MACHWF1R to value 0x1104_1904
impl crate::Resettable for MACHWF1R_SPEC {
    const RESET_VALUE: Self::Ux = 0x1104_1904;
}
