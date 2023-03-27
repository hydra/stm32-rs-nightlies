///Register `STAR` reader
pub struct R(crate::R<STAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STAR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `CCRCFAIL` reader - Command response received (CRC check failed) Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
pub type CCRCFAIL_R = crate::BitReader<bool>;
///Field `DCRCFAIL` reader - Data block sent/received (CRC check failed) Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
pub type DCRCFAIL_R = crate::BitReader<bool>;
///Field `CTIMEOUT` reader - Command response timeout Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR. The Command Timeout period has a fixed value of 64 SDMMC_CK clock periods.
pub type CTIMEOUT_R = crate::BitReader<bool>;
///Field `DTIMEOUT` reader - Data timeout Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
pub type DTIMEOUT_R = crate::BitReader<bool>;
///Field `TXUNDERR` reader - Transmit FIFO underrun error (masked by hardware when IDMA is enabled) Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
pub type TXUNDERR_R = crate::BitReader<bool>;
///Field `RXOVERR` reader - Received FIFO overrun error (masked by hardware when IDMA is enabled) Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
pub type RXOVERR_R = crate::BitReader<bool>;
///Field `CMDREND` reader - Command response received (CRC check passed, or no CRC) Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
pub type CMDREND_R = crate::BitReader<bool>;
///Field `CMDSENT` reader - Command sent (no response required) Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
pub type CMDSENT_R = crate::BitReader<bool>;
///Field `DATAEND` reader - Data transfer ended correctly DATAEND is set if data counter DATACOUNT is zero and no errors occur, and no transmit data transfer hold. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
pub type DATAEND_R = crate::BitReader<bool>;
///Field `DHOLD` reader - Data transfer Hold Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
pub type DHOLD_R = crate::BitReader<bool>;
///Field `DBCKEND` reader - Data block sent/received DBCKEND is set when: - CRC check passed and DPSM moves to the R_W state or - IDMAEN = 0 and transmit data transfer hold and DATACOUNT >0 and DPSM moves to Wait_S. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
pub type DBCKEND_R = crate::BitReader<bool>;
///Field `DABORT` reader - Data transfer aborted by CMD12 Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
pub type DABORT_R = crate::BitReader<bool>;
///Field `DPSMACT` reader - Data path state machine active, i.e. not in Idle state This is a hardware status flag only, does not generate an interrupt.
pub type DPSMACT_R = crate::BitReader<bool>;
///Field `CPSMACT` reader - Command path state machine active, i.e. not in Idle state This is a hardware status flag only, does not generate an interrupt.
pub type CPSMACT_R = crate::BitReader<bool>;
///Field `TXFIFOHE` reader - Transmit FIFO half empty At least half the number of words can be written into the FIFO. This bit is cleared when the FIFO becomes half+1 full.
pub type TXFIFOHE_R = crate::BitReader<bool>;
///Field `RXFIFOHF` reader - Receive FIFO half full There are at least half the number of words in the FIFO. This bit is cleared when the FIFO becomes half+1 empty.
pub type RXFIFOHF_R = crate::BitReader<bool>;
///Field `TXFIFOF` reader - Transmit FIFO full This is a hardware status flag only, does not generate an interrupt. This bit is cleared when one FIFO location becomes empty.
pub type TXFIFOF_R = crate::BitReader<bool>;
///Field `RXFIFOF` reader - Receive FIFO full This bit is cleared when one FIFO location becomes empty.
pub type RXFIFOF_R = crate::BitReader<bool>;
///Field `TXFIFOE` reader - Transmit FIFO empty This bit is cleared when one FIFO location becomes full.
pub type TXFIFOE_R = crate::BitReader<bool>;
///Field `RXFIFOE` reader - Receive FIFO empty This is a hardware status flag only, does not generate an interrupt. This bit is cleared when one FIFO location becomes full.
pub type RXFIFOE_R = crate::BitReader<bool>;
///Field `BUSYD0` reader - Inverted value of SDMMC_D0 line (Busy), sampled at the end of a CMD response and a second time 2 SDMMC_CK cycles after the CMD response This bit is reset to not busy when the SDMMCD0 line changes from busy to not busy. This bit does not signal busy due to data transfer. This is a hardware status flag only, it does not generate an interrupt.
pub type BUSYD0_R = crate::BitReader<bool>;
///Field `BUSYD0END` reader - end of SDMMC_D0 Busy following a CMD response detected This indicates only end of busy following a CMD response. This bit does not signal busy due to data transfer. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
pub type BUSYD0END_R = crate::BitReader<bool>;
///Field `SDIOIT` reader - SDIO interrupt received The interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
pub type SDIOIT_R = crate::BitReader<bool>;
///Field `ACKFAIL` reader - Boot acknowledgment received (boot acknowledgment check fail) The interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
pub type ACKFAIL_R = crate::BitReader<bool>;
///Field `ACKTIMEOUT` reader - Boot acknowledgment timeout The interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
pub type ACKTIMEOUT_R = crate::BitReader<bool>;
///Field `VSWEND` reader - Voltage switch critical timing section completion The interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
pub type VSWEND_R = crate::BitReader<bool>;
///Field `CKSTOP` reader - SDMMC_CK stopped in Voltage switch procedure The interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
pub type CKSTOP_R = crate::BitReader<bool>;
///Field `IDMATE` reader - IDMA transfer error The interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
pub type IDMATE_R = crate::BitReader<bool>;
///Field `IDMABTC` reader - IDMA buffer transfer complete The interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
pub type IDMABTC_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - Command response received (CRC check failed) Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
    #[inline(always)]
    pub fn ccrcfail(&self) -> CCRCFAIL_R {
        CCRCFAIL_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Data block sent/received (CRC check failed) Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
    #[inline(always)]
    pub fn dcrcfail(&self) -> DCRCFAIL_R {
        DCRCFAIL_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Command response timeout Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR. The Command Timeout period has a fixed value of 64 SDMMC_CK clock periods.
    #[inline(always)]
    pub fn ctimeout(&self) -> CTIMEOUT_R {
        CTIMEOUT_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Data timeout Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
    #[inline(always)]
    pub fn dtimeout(&self) -> DTIMEOUT_R {
        DTIMEOUT_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Transmit FIFO underrun error (masked by hardware when IDMA is enabled) Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
    #[inline(always)]
    pub fn txunderr(&self) -> TXUNDERR_R {
        TXUNDERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Received FIFO overrun error (masked by hardware when IDMA is enabled) Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
    #[inline(always)]
    pub fn rxoverr(&self) -> RXOVERR_R {
        RXOVERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Command response received (CRC check passed, or no CRC) Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
    #[inline(always)]
    pub fn cmdrend(&self) -> CMDREND_R {
        CMDREND_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Command sent (no response required) Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
    #[inline(always)]
    pub fn cmdsent(&self) -> CMDSENT_R {
        CMDSENT_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Data transfer ended correctly DATAEND is set if data counter DATACOUNT is zero and no errors occur, and no transmit data transfer hold. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
    #[inline(always)]
    pub fn dataend(&self) -> DATAEND_R {
        DATAEND_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Data transfer Hold Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
    #[inline(always)]
    pub fn dhold(&self) -> DHOLD_R {
        DHOLD_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Data block sent/received DBCKEND is set when: - CRC check passed and DPSM moves to the R_W state or - IDMAEN = 0 and transmit data transfer hold and DATACOUNT >0 and DPSM moves to Wait_S. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
    #[inline(always)]
    pub fn dbckend(&self) -> DBCKEND_R {
        DBCKEND_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Data transfer aborted by CMD12 Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
    #[inline(always)]
    pub fn dabort(&self) -> DABORT_R {
        DABORT_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Data path state machine active, i.e. not in Idle state This is a hardware status flag only, does not generate an interrupt.
    #[inline(always)]
    pub fn dpsmact(&self) -> DPSMACT_R {
        DPSMACT_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Command path state machine active, i.e. not in Idle state This is a hardware status flag only, does not generate an interrupt.
    #[inline(always)]
    pub fn cpsmact(&self) -> CPSMACT_R {
        CPSMACT_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Transmit FIFO half empty At least half the number of words can be written into the FIFO. This bit is cleared when the FIFO becomes half+1 full.
    #[inline(always)]
    pub fn txfifohe(&self) -> TXFIFOHE_R {
        TXFIFOHE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Receive FIFO half full There are at least half the number of words in the FIFO. This bit is cleared when the FIFO becomes half+1 empty.
    #[inline(always)]
    pub fn rxfifohf(&self) -> RXFIFOHF_R {
        RXFIFOHF_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Transmit FIFO full This is a hardware status flag only, does not generate an interrupt. This bit is cleared when one FIFO location becomes empty.
    #[inline(always)]
    pub fn txfifof(&self) -> TXFIFOF_R {
        TXFIFOF_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Receive FIFO full This bit is cleared when one FIFO location becomes empty.
    #[inline(always)]
    pub fn rxfifof(&self) -> RXFIFOF_R {
        RXFIFOF_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Transmit FIFO empty This bit is cleared when one FIFO location becomes full.
    #[inline(always)]
    pub fn txfifoe(&self) -> TXFIFOE_R {
        TXFIFOE_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Receive FIFO empty This is a hardware status flag only, does not generate an interrupt. This bit is cleared when one FIFO location becomes full.
    #[inline(always)]
    pub fn rxfifoe(&self) -> RXFIFOE_R {
        RXFIFOE_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Inverted value of SDMMC_D0 line (Busy), sampled at the end of a CMD response and a second time 2 SDMMC_CK cycles after the CMD response This bit is reset to not busy when the SDMMCD0 line changes from busy to not busy. This bit does not signal busy due to data transfer. This is a hardware status flag only, it does not generate an interrupt.
    #[inline(always)]
    pub fn busyd0(&self) -> BUSYD0_R {
        BUSYD0_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - end of SDMMC_D0 Busy following a CMD response detected This indicates only end of busy following a CMD response. This bit does not signal busy due to data transfer. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
    #[inline(always)]
    pub fn busyd0end(&self) -> BUSYD0END_R {
        BUSYD0END_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - SDIO interrupt received The interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
    #[inline(always)]
    pub fn sdioit(&self) -> SDIOIT_R {
        SDIOIT_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Boot acknowledgment received (boot acknowledgment check fail) The interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
    #[inline(always)]
    pub fn ackfail(&self) -> ACKFAIL_R {
        ACKFAIL_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Boot acknowledgment timeout The interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
    #[inline(always)]
    pub fn acktimeout(&self) -> ACKTIMEOUT_R {
        ACKTIMEOUT_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Voltage switch critical timing section completion The interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
    #[inline(always)]
    pub fn vswend(&self) -> VSWEND_R {
        VSWEND_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - SDMMC_CK stopped in Voltage switch procedure The interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
    #[inline(always)]
    pub fn ckstop(&self) -> CKSTOP_R {
        CKSTOP_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - IDMA transfer error The interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
    #[inline(always)]
    pub fn idmate(&self) -> IDMATE_R {
        IDMATE_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - IDMA buffer transfer complete The interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
    #[inline(always)]
    pub fn idmabtc(&self) -> IDMABTC_R {
        IDMABTC_R::new(((self.bits >> 28) & 1) != 0)
    }
}
///SDMMC status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [star](index.html) module
pub struct STAR_SPEC;
impl crate::RegisterSpec for STAR_SPEC {
    type Ux = u32;
}
///`read()` method returns [star::R](R) reader structure
impl crate::Readable for STAR_SPEC {
    type Reader = R;
}
///`reset()` method sets STAR to value 0
impl crate::Resettable for STAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
