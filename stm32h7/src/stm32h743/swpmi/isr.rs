///Register `ISR` reader
pub struct R(crate::R<ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `RXBFF` reader - Receive buffer full flag
pub type RXBFF_R = crate::BitReader<bool>;
///Field `TXBEF` reader - Transmit buffer empty flag
pub type TXBEF_R = crate::BitReader<bool>;
///Field `RXBERF` reader - Receive CRC error flag
pub type RXBERF_R = crate::BitReader<bool>;
///Field `RXOVRF` reader - Receive overrun error flag
pub type RXOVRF_R = crate::BitReader<bool>;
///Field `TXUNRF` reader - Transmit underrun error flag
pub type TXUNRF_R = crate::BitReader<bool>;
///Field `RXNE` reader - Receive data register not empty
pub type RXNE_R = crate::BitReader<bool>;
///Field `TXE` reader - Transmit data register empty
pub type TXE_R = crate::BitReader<bool>;
///Field `TCF` reader - Transfer complete flag
pub type TCF_R = crate::BitReader<bool>;
///Field `SRF` reader - Slave resume flag
pub type SRF_R = crate::BitReader<bool>;
///Field `SUSP` reader - SUSPEND flag
pub type SUSP_R = crate::BitReader<bool>;
///Field `DEACTF` reader - DEACTIVATED flag
pub type DEACTF_R = crate::BitReader<bool>;
///Field `RDYF` reader - transceiver ready flag
pub type RDYF_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - Receive buffer full flag
    #[inline(always)]
    pub fn rxbff(&self) -> RXBFF_R {
        RXBFF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Transmit buffer empty flag
    #[inline(always)]
    pub fn txbef(&self) -> TXBEF_R {
        TXBEF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Receive CRC error flag
    #[inline(always)]
    pub fn rxberf(&self) -> RXBERF_R {
        RXBERF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Receive overrun error flag
    #[inline(always)]
    pub fn rxovrf(&self) -> RXOVRF_R {
        RXOVRF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Transmit underrun error flag
    #[inline(always)]
    pub fn txunrf(&self) -> TXUNRF_R {
        TXUNRF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Receive data register not empty
    #[inline(always)]
    pub fn rxne(&self) -> RXNE_R {
        RXNE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Transmit data register empty
    #[inline(always)]
    pub fn txe(&self) -> TXE_R {
        TXE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Transfer complete flag
    #[inline(always)]
    pub fn tcf(&self) -> TCF_R {
        TCF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Slave resume flag
    #[inline(always)]
    pub fn srf(&self) -> SRF_R {
        SRF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - SUSPEND flag
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - DEACTIVATED flag
    #[inline(always)]
    pub fn deactf(&self) -> DEACTF_R {
        DEACTF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - transceiver ready flag
    #[inline(always)]
    pub fn rdyf(&self) -> RDYF_R {
        RDYF_R::new(((self.bits >> 11) & 1) != 0)
    }
}
///SWPMI Interrupt and Status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [isr](index.html) module
pub struct ISR_SPEC;
impl crate::RegisterSpec for ISR_SPEC {
    type Ux = u32;
}
///`read()` method returns [isr::R](R) reader structure
impl crate::Readable for ISR_SPEC {
    type Reader = R;
}
///`reset()` method sets ISR to value 0x02c2
impl crate::Resettable for ISR_SPEC {
    const RESET_VALUE: Self::Ux = 0x02c2;
}
