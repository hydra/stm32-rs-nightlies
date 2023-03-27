///Register `STA` reader
pub struct R(crate::R<STA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STA_SPEC>) -> Self {
        R(reader)
    }
}
///Field `CCRCFAIL` reader - Command response received (CRC check failed). Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
pub type CCRCFAIL_R = crate::BitReader<CCRCFAIL_A>;
///Command response received (CRC check failed). Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCRCFAIL_A {
    ///0: Command response received, crc check passed
    NotFailed = 0,
    ///1: Command response received, crc check failed
    Failed = 1,
}
impl From<CCRCFAIL_A> for bool {
    #[inline(always)]
    fn from(variant: CCRCFAIL_A) -> Self {
        variant as u8 != 0
    }
}
impl CCRCFAIL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CCRCFAIL_A {
        match self.bits {
            false => CCRCFAIL_A::NotFailed,
            true => CCRCFAIL_A::Failed,
        }
    }
    ///Checks if the value of the field is `NotFailed`
    #[inline(always)]
    pub fn is_not_failed(&self) -> bool {
        *self == CCRCFAIL_A::NotFailed
    }
    ///Checks if the value of the field is `Failed`
    #[inline(always)]
    pub fn is_failed(&self) -> bool {
        *self == CCRCFAIL_A::Failed
    }
}
///Field `DCRCFAIL` reader - Data block sent/received (CRC check failed). Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
pub type DCRCFAIL_R = crate::BitReader<DCRCFAIL_A>;
///Data block sent/received (CRC check failed). Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCRCFAIL_A {
    ///0: No Data block sent/received crc check fail
    NotFailed = 0,
    ///1: Data block sent/received crc failed
    Failed = 1,
}
impl From<DCRCFAIL_A> for bool {
    #[inline(always)]
    fn from(variant: DCRCFAIL_A) -> Self {
        variant as u8 != 0
    }
}
impl DCRCFAIL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DCRCFAIL_A {
        match self.bits {
            false => DCRCFAIL_A::NotFailed,
            true => DCRCFAIL_A::Failed,
        }
    }
    ///Checks if the value of the field is `NotFailed`
    #[inline(always)]
    pub fn is_not_failed(&self) -> bool {
        *self == DCRCFAIL_A::NotFailed
    }
    ///Checks if the value of the field is `Failed`
    #[inline(always)]
    pub fn is_failed(&self) -> bool {
        *self == DCRCFAIL_A::Failed
    }
}
///Field `CTIMEOUT` reader - Command response timeout. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR. The Command Timeout period has a fixed value of 64 SDMMC_CK clock periods.
pub type CTIMEOUT_R = crate::BitReader<CTIMEOUT_A>;
///Command response timeout. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR. The Command Timeout period has a fixed value of 64 SDMMC_CK clock periods.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTIMEOUT_A {
    ///0: No Command timeout
    NoTimeout = 0,
    ///1: Command timeout
    Timeout = 1,
}
impl From<CTIMEOUT_A> for bool {
    #[inline(always)]
    fn from(variant: CTIMEOUT_A) -> Self {
        variant as u8 != 0
    }
}
impl CTIMEOUT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CTIMEOUT_A {
        match self.bits {
            false => CTIMEOUT_A::NoTimeout,
            true => CTIMEOUT_A::Timeout,
        }
    }
    ///Checks if the value of the field is `NoTimeout`
    #[inline(always)]
    pub fn is_no_timeout(&self) -> bool {
        *self == CTIMEOUT_A::NoTimeout
    }
    ///Checks if the value of the field is `Timeout`
    #[inline(always)]
    pub fn is_timeout(&self) -> bool {
        *self == CTIMEOUT_A::Timeout
    }
}
///Field `DTIMEOUT` reader - Data timeout. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
pub type DTIMEOUT_R = crate::BitReader<DTIMEOUT_A>;
///Data timeout. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTIMEOUT_A {
    ///0: No data timeout
    NoTimeout = 0,
    ///1: Data timeout
    Timeout = 1,
}
impl From<DTIMEOUT_A> for bool {
    #[inline(always)]
    fn from(variant: DTIMEOUT_A) -> Self {
        variant as u8 != 0
    }
}
impl DTIMEOUT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DTIMEOUT_A {
        match self.bits {
            false => DTIMEOUT_A::NoTimeout,
            true => DTIMEOUT_A::Timeout,
        }
    }
    ///Checks if the value of the field is `NoTimeout`
    #[inline(always)]
    pub fn is_no_timeout(&self) -> bool {
        *self == DTIMEOUT_A::NoTimeout
    }
    ///Checks if the value of the field is `Timeout`
    #[inline(always)]
    pub fn is_timeout(&self) -> bool {
        *self == DTIMEOUT_A::Timeout
    }
}
///Field `TXUNDERR` reader - Transmit FIFO underrun error or IDMA read transfer error. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
pub type TXUNDERR_R = crate::BitReader<TXUNDERR_A>;
///Transmit FIFO underrun error or IDMA read transfer error. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXUNDERR_A {
    ///0: No transmit FIFO underrun error
    NoUnderrun = 0,
    ///1: Transmit FIFO underrun error
    Underrun = 1,
}
impl From<TXUNDERR_A> for bool {
    #[inline(always)]
    fn from(variant: TXUNDERR_A) -> Self {
        variant as u8 != 0
    }
}
impl TXUNDERR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TXUNDERR_A {
        match self.bits {
            false => TXUNDERR_A::NoUnderrun,
            true => TXUNDERR_A::Underrun,
        }
    }
    ///Checks if the value of the field is `NoUnderrun`
    #[inline(always)]
    pub fn is_no_underrun(&self) -> bool {
        *self == TXUNDERR_A::NoUnderrun
    }
    ///Checks if the value of the field is `Underrun`
    #[inline(always)]
    pub fn is_underrun(&self) -> bool {
        *self == TXUNDERR_A::Underrun
    }
}
///Field `RXOVERR` reader - Received FIFO overrun error or IDMA write transfer error. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
pub type RXOVERR_R = crate::BitReader<RXOVERR_A>;
///Received FIFO overrun error or IDMA write transfer error. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXOVERR_A {
    ///0: No FIFO overrun error
    NoOverrun = 0,
    ///1: Receive FIFO overrun error
    Overrun = 1,
}
impl From<RXOVERR_A> for bool {
    #[inline(always)]
    fn from(variant: RXOVERR_A) -> Self {
        variant as u8 != 0
    }
}
impl RXOVERR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RXOVERR_A {
        match self.bits {
            false => RXOVERR_A::NoOverrun,
            true => RXOVERR_A::Overrun,
        }
    }
    ///Checks if the value of the field is `NoOverrun`
    #[inline(always)]
    pub fn is_no_overrun(&self) -> bool {
        *self == RXOVERR_A::NoOverrun
    }
    ///Checks if the value of the field is `Overrun`
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == RXOVERR_A::Overrun
    }
}
///Field `CMDREND` reader - Command response received (CRC check passed, or no CRC). Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
pub type CMDREND_R = crate::BitReader<CMDREND_A>;
///Command response received (CRC check passed, or no CRC). Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMDREND_A {
    ///0: Command not done
    NotDone = 0,
    ///1: Command response received (CRC check passed)
    Done = 1,
}
impl From<CMDREND_A> for bool {
    #[inline(always)]
    fn from(variant: CMDREND_A) -> Self {
        variant as u8 != 0
    }
}
impl CMDREND_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CMDREND_A {
        match self.bits {
            false => CMDREND_A::NotDone,
            true => CMDREND_A::Done,
        }
    }
    ///Checks if the value of the field is `NotDone`
    #[inline(always)]
    pub fn is_not_done(&self) -> bool {
        *self == CMDREND_A::NotDone
    }
    ///Checks if the value of the field is `Done`
    #[inline(always)]
    pub fn is_done(&self) -> bool {
        *self == CMDREND_A::Done
    }
}
///Field `CMDSENT` reader - Command sent (no response required). Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
pub type CMDSENT_R = crate::BitReader<CMDSENT_A>;
///Command sent (no response required). Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMDSENT_A {
    ///0: Command not sent
    NotSent = 0,
    ///1: Command sent (no response required)
    Sent = 1,
}
impl From<CMDSENT_A> for bool {
    #[inline(always)]
    fn from(variant: CMDSENT_A) -> Self {
        variant as u8 != 0
    }
}
impl CMDSENT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CMDSENT_A {
        match self.bits {
            false => CMDSENT_A::NotSent,
            true => CMDSENT_A::Sent,
        }
    }
    ///Checks if the value of the field is `NotSent`
    #[inline(always)]
    pub fn is_not_sent(&self) -> bool {
        *self == CMDSENT_A::NotSent
    }
    ///Checks if the value of the field is `Sent`
    #[inline(always)]
    pub fn is_sent(&self) -> bool {
        *self == CMDSENT_A::Sent
    }
}
///Field `DATAEND` reader - Data transfer ended correctly. (data counter, DATACOUNT is zero and no errors occur). Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
pub type DATAEND_R = crate::BitReader<DATAEND_A>;
///Data transfer ended correctly. (data counter, DATACOUNT is zero and no errors occur). Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DATAEND_A {
    ///0: Not done
    NotDone = 0,
    ///1: Data end (DCOUNT, is zero)
    Done = 1,
}
impl From<DATAEND_A> for bool {
    #[inline(always)]
    fn from(variant: DATAEND_A) -> Self {
        variant as u8 != 0
    }
}
impl DATAEND_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DATAEND_A {
        match self.bits {
            false => DATAEND_A::NotDone,
            true => DATAEND_A::Done,
        }
    }
    ///Checks if the value of the field is `NotDone`
    #[inline(always)]
    pub fn is_not_done(&self) -> bool {
        *self == DATAEND_A::NotDone
    }
    ///Checks if the value of the field is `Done`
    #[inline(always)]
    pub fn is_done(&self) -> bool {
        *self == DATAEND_A::Done
    }
}
///Field `DBCKEND` reader - Data block sent/received. (CRC check passed) and DPSM moves to the READWAIT state. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
pub type DBCKEND_R = crate::BitReader<DBCKEND_A>;
///Data block sent/received. (CRC check passed) and DPSM moves to the READWAIT state. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBCKEND_A {
    ///0: Data block not sent/received (CRC check failed)
    NotTransferred = 0,
    ///1: Data block sent/received (CRC check passed)
    Transferred = 1,
}
impl From<DBCKEND_A> for bool {
    #[inline(always)]
    fn from(variant: DBCKEND_A) -> Self {
        variant as u8 != 0
    }
}
impl DBCKEND_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DBCKEND_A {
        match self.bits {
            false => DBCKEND_A::NotTransferred,
            true => DBCKEND_A::Transferred,
        }
    }
    ///Checks if the value of the field is `NotTransferred`
    #[inline(always)]
    pub fn is_not_transferred(&self) -> bool {
        *self == DBCKEND_A::NotTransferred
    }
    ///Checks if the value of the field is `Transferred`
    #[inline(always)]
    pub fn is_transferred(&self) -> bool {
        *self == DBCKEND_A::Transferred
    }
}
///Field `CMDACT` reader - Data transfer aborted by CMD12. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
pub type CMDACT_R = crate::BitReader<CMDACT_A>;
///Data transfer aborted by CMD12. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMDACT_A {
    ///0: Command transfer not in progress
    NotInProgress = 0,
    ///1: Command tranfer in progress
    InProgress = 1,
}
impl From<CMDACT_A> for bool {
    #[inline(always)]
    fn from(variant: CMDACT_A) -> Self {
        variant as u8 != 0
    }
}
impl CMDACT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CMDACT_A {
        match self.bits {
            false => CMDACT_A::NotInProgress,
            true => CMDACT_A::InProgress,
        }
    }
    ///Checks if the value of the field is `NotInProgress`
    #[inline(always)]
    pub fn is_not_in_progress(&self) -> bool {
        *self == CMDACT_A::NotInProgress
    }
    ///Checks if the value of the field is `InProgress`
    #[inline(always)]
    pub fn is_in_progress(&self) -> bool {
        *self == CMDACT_A::InProgress
    }
}
///Field `TXACT` reader - Data path state machine active, i.e. not in Idle state. This is a hardware status flag only, does not generate an interrupt.
pub type TXACT_R = crate::BitReader<TXACT_A>;
///Data path state machine active, i.e. not in Idle state. This is a hardware status flag only, does not generate an interrupt.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXACT_A {
    ///0: Data transmit is not in progress
    NotInProgress = 0,
    ///1: Data transmit in progress
    InProgress = 1,
}
impl From<TXACT_A> for bool {
    #[inline(always)]
    fn from(variant: TXACT_A) -> Self {
        variant as u8 != 0
    }
}
impl TXACT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TXACT_A {
        match self.bits {
            false => TXACT_A::NotInProgress,
            true => TXACT_A::InProgress,
        }
    }
    ///Checks if the value of the field is `NotInProgress`
    #[inline(always)]
    pub fn is_not_in_progress(&self) -> bool {
        *self == TXACT_A::NotInProgress
    }
    ///Checks if the value of the field is `InProgress`
    #[inline(always)]
    pub fn is_in_progress(&self) -> bool {
        *self == TXACT_A::InProgress
    }
}
///Field `RXACT` reader - Command path state machine active, i.e. not in Idle state. This is a hardware status flag only, does not generate an interrupt.
pub type RXACT_R = crate::BitReader<RXACT_A>;
///Command path state machine active, i.e. not in Idle state. This is a hardware status flag only, does not generate an interrupt.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXACT_A {
    ///0: Data receive not in progress
    NotInProgress = 0,
    ///1: Data receive in progress
    InProgress = 1,
}
impl From<RXACT_A> for bool {
    #[inline(always)]
    fn from(variant: RXACT_A) -> Self {
        variant as u8 != 0
    }
}
impl RXACT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RXACT_A {
        match self.bits {
            false => RXACT_A::NotInProgress,
            true => RXACT_A::InProgress,
        }
    }
    ///Checks if the value of the field is `NotInProgress`
    #[inline(always)]
    pub fn is_not_in_progress(&self) -> bool {
        *self == RXACT_A::NotInProgress
    }
    ///Checks if the value of the field is `InProgress`
    #[inline(always)]
    pub fn is_in_progress(&self) -> bool {
        *self == RXACT_A::InProgress
    }
}
///Field `TXFIFOHE` reader - Transmit FIFO half empty At least half the number of words can be written into the FIFO. This bit is cleared when the FIFO becomes half+1 full.
pub type TXFIFOHE_R = crate::BitReader<TXFIFOHE_A>;
///Transmit FIFO half empty At least half the number of words can be written into the FIFO. This bit is cleared when the FIFO becomes half+1 full.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXFIFOHE_A {
    ///0: Transmit FIFO not half empty
    NotHalfEmpty = 0,
    ///1: Transmit FIFO half empty. At least 8 words can be written into the FIFO
    HalfEmpty = 1,
}
impl From<TXFIFOHE_A> for bool {
    #[inline(always)]
    fn from(variant: TXFIFOHE_A) -> Self {
        variant as u8 != 0
    }
}
impl TXFIFOHE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TXFIFOHE_A {
        match self.bits {
            false => TXFIFOHE_A::NotHalfEmpty,
            true => TXFIFOHE_A::HalfEmpty,
        }
    }
    ///Checks if the value of the field is `NotHalfEmpty`
    #[inline(always)]
    pub fn is_not_half_empty(&self) -> bool {
        *self == TXFIFOHE_A::NotHalfEmpty
    }
    ///Checks if the value of the field is `HalfEmpty`
    #[inline(always)]
    pub fn is_half_empty(&self) -> bool {
        *self == TXFIFOHE_A::HalfEmpty
    }
}
///Field `RXFIFOHF` reader - Receive FIFO half full There are at least half the number of words in the FIFO. This bit is cleared when the FIFO becomes half+1 empty.
pub type RXFIFOHF_R = crate::BitReader<RXFIFOHF_A>;
///Receive FIFO half full There are at least half the number of words in the FIFO. This bit is cleared when the FIFO becomes half+1 empty.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXFIFOHF_A {
    ///0: Receive FIFO not half full
    NotHalfFull = 0,
    ///1: Receive FIFO half full. At least 8 words in the FIFO
    HalfFull = 1,
}
impl From<RXFIFOHF_A> for bool {
    #[inline(always)]
    fn from(variant: RXFIFOHF_A) -> Self {
        variant as u8 != 0
    }
}
impl RXFIFOHF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RXFIFOHF_A {
        match self.bits {
            false => RXFIFOHF_A::NotHalfFull,
            true => RXFIFOHF_A::HalfFull,
        }
    }
    ///Checks if the value of the field is `NotHalfFull`
    #[inline(always)]
    pub fn is_not_half_full(&self) -> bool {
        *self == RXFIFOHF_A::NotHalfFull
    }
    ///Checks if the value of the field is `HalfFull`
    #[inline(always)]
    pub fn is_half_full(&self) -> bool {
        *self == RXFIFOHF_A::HalfFull
    }
}
///Field `TXFIFOF` reader - Transmit FIFO full This is a hardware status flag only, does not generate an interrupt. This bit is cleared when one FIFO location becomes empty.
pub type TXFIFOF_R = crate::BitReader<TXFIFOF_A>;
///Transmit FIFO full This is a hardware status flag only, does not generate an interrupt. This bit is cleared when one FIFO location becomes empty.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXFIFOF_A {
    ///0: Transmit FIFO not full
    NotFull = 0,
    ///1: Transmit FIFO full
    Full = 1,
}
impl From<TXFIFOF_A> for bool {
    #[inline(always)]
    fn from(variant: TXFIFOF_A) -> Self {
        variant as u8 != 0
    }
}
impl TXFIFOF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TXFIFOF_A {
        match self.bits {
            false => TXFIFOF_A::NotFull,
            true => TXFIFOF_A::Full,
        }
    }
    ///Checks if the value of the field is `NotFull`
    #[inline(always)]
    pub fn is_not_full(&self) -> bool {
        *self == TXFIFOF_A::NotFull
    }
    ///Checks if the value of the field is `Full`
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == TXFIFOF_A::Full
    }
}
///Field `RXFIFOF` reader - Receive FIFO full This bit is cleared when one FIFO location becomes empty.
pub type RXFIFOF_R = crate::BitReader<RXFIFOF_A>;
///Receive FIFO full This bit is cleared when one FIFO location becomes empty.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXFIFOF_A {
    ///0: Transmit FIFO not full
    NotFull = 0,
    ///1: Receive FIFO full. When HW Flow Control is enabled, RXFIFOF signals becomes activated 2 words before the FIFO is full.
    Full = 1,
}
impl From<RXFIFOF_A> for bool {
    #[inline(always)]
    fn from(variant: RXFIFOF_A) -> Self {
        variant as u8 != 0
    }
}
impl RXFIFOF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RXFIFOF_A {
        match self.bits {
            false => RXFIFOF_A::NotFull,
            true => RXFIFOF_A::Full,
        }
    }
    ///Checks if the value of the field is `NotFull`
    #[inline(always)]
    pub fn is_not_full(&self) -> bool {
        *self == RXFIFOF_A::NotFull
    }
    ///Checks if the value of the field is `Full`
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == RXFIFOF_A::Full
    }
}
///Field `TXFIFOE` reader - Transmit FIFO empty This bit is cleared when one FIFO location becomes full.
pub type TXFIFOE_R = crate::BitReader<TXFIFOE_A>;
///Transmit FIFO empty This bit is cleared when one FIFO location becomes full.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXFIFOE_A {
    ///0: Transmit FIFO not empty
    NotEmpty = 0,
    ///1: Transmit FIFO empty. When HW Flow Control is enabled, TXFIFOE signals becomes activated when the FIFO contains 2 words.
    Empty = 1,
}
impl From<TXFIFOE_A> for bool {
    #[inline(always)]
    fn from(variant: TXFIFOE_A) -> Self {
        variant as u8 != 0
    }
}
impl TXFIFOE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TXFIFOE_A {
        match self.bits {
            false => TXFIFOE_A::NotEmpty,
            true => TXFIFOE_A::Empty,
        }
    }
    ///Checks if the value of the field is `NotEmpty`
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == TXFIFOE_A::NotEmpty
    }
    ///Checks if the value of the field is `Empty`
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == TXFIFOE_A::Empty
    }
}
///Field `RXFIFOE` reader - Receive FIFO empty This is a hardware status flag only, does not generate an interrupt. This bit is cleared when one FIFO location becomes full.
pub type RXFIFOE_R = crate::BitReader<RXFIFOE_A>;
///Receive FIFO empty This is a hardware status flag only, does not generate an interrupt. This bit is cleared when one FIFO location becomes full.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXFIFOE_A {
    ///0: Receive FIFO not empty
    NotEmpty = 0,
    ///1: Receive FIFO empty
    Empty = 1,
}
impl From<RXFIFOE_A> for bool {
    #[inline(always)]
    fn from(variant: RXFIFOE_A) -> Self {
        variant as u8 != 0
    }
}
impl RXFIFOE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RXFIFOE_A {
        match self.bits {
            false => RXFIFOE_A::NotEmpty,
            true => RXFIFOE_A::Empty,
        }
    }
    ///Checks if the value of the field is `NotEmpty`
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == RXFIFOE_A::NotEmpty
    }
    ///Checks if the value of the field is `Empty`
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == RXFIFOE_A::Empty
    }
}
///Field `TXDAVL` reader - Inverted value of SDMMC_D0 line (Busy), sampled at the end of a CMD response and a second time 2 SDMMC_CK cycles after the CMD response. This bit is reset to not busy when the SDMMCD0 line changes from busy to not busy. This bit does not signal busy due to data transfer. This is a hardware status flag only, it does not generate an interrupt.
pub type TXDAVL_R = crate::BitReader<TXDAVL_A>;
///Inverted value of SDMMC_D0 line (Busy), sampled at the end of a CMD response and a second time 2 SDMMC_CK cycles after the CMD response. This bit is reset to not busy when the SDMMCD0 line changes from busy to not busy. This bit does not signal busy due to data transfer. This is a hardware status flag only, it does not generate an interrupt.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXDAVL_A {
    ///0: Data not available in transmit FIFO
    NotAvailable = 0,
    ///1: Data available in transmit FIFO
    Available = 1,
}
impl From<TXDAVL_A> for bool {
    #[inline(always)]
    fn from(variant: TXDAVL_A) -> Self {
        variant as u8 != 0
    }
}
impl TXDAVL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TXDAVL_A {
        match self.bits {
            false => TXDAVL_A::NotAvailable,
            true => TXDAVL_A::Available,
        }
    }
    ///Checks if the value of the field is `NotAvailable`
    #[inline(always)]
    pub fn is_not_available(&self) -> bool {
        *self == TXDAVL_A::NotAvailable
    }
    ///Checks if the value of the field is `Available`
    #[inline(always)]
    pub fn is_available(&self) -> bool {
        *self == TXDAVL_A::Available
    }
}
///Field `RXDAVL` reader - end of SDMMC_D0 Busy following a CMD response detected. This indicates only end of busy following a CMD response. This bit does not signal busy due to data transfer. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
pub type RXDAVL_R = crate::BitReader<RXDAVL_A>;
///end of SDMMC_D0 Busy following a CMD response detected. This indicates only end of busy following a CMD response. This bit does not signal busy due to data transfer. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXDAVL_A {
    ///0: Data not available in receive FIFO
    NotAvailable = 0,
    ///1: Data available in receive FIFO
    Available = 1,
}
impl From<RXDAVL_A> for bool {
    #[inline(always)]
    fn from(variant: RXDAVL_A) -> Self {
        variant as u8 != 0
    }
}
impl RXDAVL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RXDAVL_A {
        match self.bits {
            false => RXDAVL_A::NotAvailable,
            true => RXDAVL_A::Available,
        }
    }
    ///Checks if the value of the field is `NotAvailable`
    #[inline(always)]
    pub fn is_not_available(&self) -> bool {
        *self == RXDAVL_A::NotAvailable
    }
    ///Checks if the value of the field is `Available`
    #[inline(always)]
    pub fn is_available(&self) -> bool {
        *self == RXDAVL_A::Available
    }
}
///Field `SDIOIT` reader - SDIO interrupt received. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
pub type SDIOIT_R = crate::BitReader<SDIOIT_A>;
///SDIO interrupt received. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDIOIT_A {
    ///0: SDIO interrupt not receieved
    NotReceived = 0,
    ///1: SDIO interrupt received
    Received = 1,
}
impl From<SDIOIT_A> for bool {
    #[inline(always)]
    fn from(variant: SDIOIT_A) -> Self {
        variant as u8 != 0
    }
}
impl SDIOIT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SDIOIT_A {
        match self.bits {
            false => SDIOIT_A::NotReceived,
            true => SDIOIT_A::Received,
        }
    }
    ///Checks if the value of the field is `NotReceived`
    #[inline(always)]
    pub fn is_not_received(&self) -> bool {
        *self == SDIOIT_A::NotReceived
    }
    ///Checks if the value of the field is `Received`
    #[inline(always)]
    pub fn is_received(&self) -> bool {
        *self == SDIOIT_A::Received
    }
}
impl R {
    ///Bit 0 - Command response received (CRC check failed). Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
    #[inline(always)]
    pub fn ccrcfail(&self) -> CCRCFAIL_R {
        CCRCFAIL_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Data block sent/received (CRC check failed). Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
    #[inline(always)]
    pub fn dcrcfail(&self) -> DCRCFAIL_R {
        DCRCFAIL_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Command response timeout. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR. The Command Timeout period has a fixed value of 64 SDMMC_CK clock periods.
    #[inline(always)]
    pub fn ctimeout(&self) -> CTIMEOUT_R {
        CTIMEOUT_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Data timeout. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
    #[inline(always)]
    pub fn dtimeout(&self) -> DTIMEOUT_R {
        DTIMEOUT_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Transmit FIFO underrun error or IDMA read transfer error. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
    #[inline(always)]
    pub fn txunderr(&self) -> TXUNDERR_R {
        TXUNDERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Received FIFO overrun error or IDMA write transfer error. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
    #[inline(always)]
    pub fn rxoverr(&self) -> RXOVERR_R {
        RXOVERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Command response received (CRC check passed, or no CRC). Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
    #[inline(always)]
    pub fn cmdrend(&self) -> CMDREND_R {
        CMDREND_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Command sent (no response required). Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
    #[inline(always)]
    pub fn cmdsent(&self) -> CMDSENT_R {
        CMDSENT_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Data transfer ended correctly. (data counter, DATACOUNT is zero and no errors occur). Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
    #[inline(always)]
    pub fn dataend(&self) -> DATAEND_R {
        DATAEND_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 10 - Data block sent/received. (CRC check passed) and DPSM moves to the READWAIT state. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
    #[inline(always)]
    pub fn dbckend(&self) -> DBCKEND_R {
        DBCKEND_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Data transfer aborted by CMD12. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
    #[inline(always)]
    pub fn cmdact(&self) -> CMDACT_R {
        CMDACT_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Data path state machine active, i.e. not in Idle state. This is a hardware status flag only, does not generate an interrupt.
    #[inline(always)]
    pub fn txact(&self) -> TXACT_R {
        TXACT_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Command path state machine active, i.e. not in Idle state. This is a hardware status flag only, does not generate an interrupt.
    #[inline(always)]
    pub fn rxact(&self) -> RXACT_R {
        RXACT_R::new(((self.bits >> 13) & 1) != 0)
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
    ///Bit 20 - Inverted value of SDMMC_D0 line (Busy), sampled at the end of a CMD response and a second time 2 SDMMC_CK cycles after the CMD response. This bit is reset to not busy when the SDMMCD0 line changes from busy to not busy. This bit does not signal busy due to data transfer. This is a hardware status flag only, it does not generate an interrupt.
    #[inline(always)]
    pub fn txdavl(&self) -> TXDAVL_R {
        TXDAVL_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - end of SDMMC_D0 Busy following a CMD response detected. This indicates only end of busy following a CMD response. This bit does not signal busy due to data transfer. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
    #[inline(always)]
    pub fn rxdavl(&self) -> RXDAVL_R {
        RXDAVL_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - SDIO interrupt received. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
    #[inline(always)]
    pub fn sdioit(&self) -> SDIOIT_R {
        SDIOIT_R::new(((self.bits >> 22) & 1) != 0)
    }
}
///The SDMMC_STAR register is a read-only register. It contains two types of flag:Static flags (bits \[29,21,11:0\]): these bits remain asserted until they are cleared by writing to the SDMMC interrupt Clear register (see SDMMC_ICR)Dynamic flags (bits \[20:12\]): these bits change state depending on the state of the underlying logic (for example, FIFO full and empty flags are asserted and de-asserted as data while written to the FIFO)
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sta](index.html) module
pub struct STA_SPEC;
impl crate::RegisterSpec for STA_SPEC {
    type Ux = u32;
}
///`read()` method returns [sta::R](R) reader structure
impl crate::Readable for STA_SPEC {
    type Reader = R;
}
///`reset()` method sets STA to value 0
impl crate::Resettable for STA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
