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
///Field `RXFIFOSIZE` reader - MTL Receive FIFO Size
pub type RXFIFOSIZE_R = crate::FieldReader<u8, u8>;
///Field `TXFIFOSIZE` reader - MTL Transmit FIFO Size
pub type TXFIFOSIZE_R = crate::FieldReader<u8, u8>;
///Field `OSTEN` reader - One-Step Timestamping Enable
pub type OSTEN_R = crate::BitReader<bool>;
///Field `PTOEN` reader - PTP Offload Enable
pub type PTOEN_R = crate::BitReader<bool>;
///Field `ADVTHWORD` reader - IEEE 1588 High Word Register Enable
pub type ADVTHWORD_R = crate::BitReader<bool>;
///Field `DCBEN` reader - DCB Feature Enable
pub type DCBEN_R = crate::BitReader<bool>;
///Field `SPHEN` reader - Split Header Feature Enable
pub type SPHEN_R = crate::BitReader<bool>;
///Field `TSOEN` reader - TCP Segmentation Offload Enable
pub type TSOEN_R = crate::BitReader<bool>;
///Field `DBGMEMA` reader - DMA Debug Registers Enable
pub type DBGMEMA_R = crate::BitReader<bool>;
///Field `AVSEL` reader - AV Feature Enable
pub type AVSEL_R = crate::BitReader<bool>;
///Field `HASHTBLSZ` reader - Hash Table Size
pub type HASHTBLSZ_R = crate::FieldReader<u8, u8>;
///Field `L3L4FNUM` reader - Total number of L3 or L4 Filters
pub type L3L4FNUM_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:4 - MTL Receive FIFO Size
    #[inline(always)]
    pub fn rxfifosize(&self) -> RXFIFOSIZE_R {
        RXFIFOSIZE_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 6:10 - MTL Transmit FIFO Size
    #[inline(always)]
    pub fn txfifosize(&self) -> TXFIFOSIZE_R {
        TXFIFOSIZE_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    ///Bit 11 - One-Step Timestamping Enable
    #[inline(always)]
    pub fn osten(&self) -> OSTEN_R {
        OSTEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - PTP Offload Enable
    #[inline(always)]
    pub fn ptoen(&self) -> PTOEN_R {
        PTOEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - IEEE 1588 High Word Register Enable
    #[inline(always)]
    pub fn advthword(&self) -> ADVTHWORD_R {
        ADVTHWORD_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 16 - DCB Feature Enable
    #[inline(always)]
    pub fn dcben(&self) -> DCBEN_R {
        DCBEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Split Header Feature Enable
    #[inline(always)]
    pub fn sphen(&self) -> SPHEN_R {
        SPHEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - TCP Segmentation Offload Enable
    #[inline(always)]
    pub fn tsoen(&self) -> TSOEN_R {
        TSOEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - DMA Debug Registers Enable
    #[inline(always)]
    pub fn dbgmema(&self) -> DBGMEMA_R {
        DBGMEMA_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - AV Feature Enable
    #[inline(always)]
    pub fn avsel(&self) -> AVSEL_R {
        AVSEL_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bits 24:25 - Hash Table Size
    #[inline(always)]
    pub fn hashtblsz(&self) -> HASHTBLSZ_R {
        HASHTBLSZ_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bits 27:30 - Total number of L3 or L4 Filters
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
///`reset()` method sets MACHWF1R to value 0x1184_1904
impl crate::Resettable for MACHWF1R_SPEC {
    const RESET_VALUE: Self::Ux = 0x1184_1904;
}
