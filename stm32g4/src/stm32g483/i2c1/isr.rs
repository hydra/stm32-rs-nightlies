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
///Register `ISR` writer
pub struct W(crate::W<ISR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<ISR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ISR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TXE` reader - Transmit data register empty (transmitters)
pub type TXE_R = crate::BitReader<TXER_A>;
///Transmit data register empty (transmitters)
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXER_A {
    ///0: TXDR register not empty
    NotEmpty = 0,
    ///1: TXDR register empty
    Empty = 1,
}
impl From<TXER_A> for bool {
    #[inline(always)]
    fn from(variant: TXER_A) -> Self {
        variant as u8 != 0
    }
}
impl TXE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TXER_A {
        match self.bits {
            false => TXER_A::NotEmpty,
            true => TXER_A::Empty,
        }
    }
    ///Checks if the value of the field is `NotEmpty`
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == TXER_A::NotEmpty
    }
    ///Checks if the value of the field is `Empty`
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == TXER_A::Empty
    }
}
///Transmit data register empty (transmitters)
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXEW_AW {
    ///1: Flush the transmit data register
    Flush = 1,
}
impl From<TXEW_AW> for bool {
    #[inline(always)]
    fn from(variant: TXEW_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `TXE` writer - Transmit data register empty (transmitters)
pub type TXE_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, ISR_SPEC, TXEW_AW, O>;
impl<'a, const O: u8> TXE_W<'a, O> {
    ///Flush the transmit data register
    #[inline(always)]
    pub fn flush(self) -> &'a mut W {
        self.variant(TXEW_AW::Flush)
    }
}
///Field `TXIS` reader - Transmit interrupt status (transmitters)
pub type TXIS_R = crate::BitReader<TXISR_A>;
///Transmit interrupt status (transmitters)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXISR_A {
    ///0: The TXDR register is not empty
    NotEmpty = 0,
    ///1: The TXDR register is empty and the data to be transmitted must be written in the TXDR register
    Empty = 1,
}
impl From<TXISR_A> for bool {
    #[inline(always)]
    fn from(variant: TXISR_A) -> Self {
        variant as u8 != 0
    }
}
impl TXIS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TXISR_A {
        match self.bits {
            false => TXISR_A::NotEmpty,
            true => TXISR_A::Empty,
        }
    }
    ///Checks if the value of the field is `NotEmpty`
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == TXISR_A::NotEmpty
    }
    ///Checks if the value of the field is `Empty`
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == TXISR_A::Empty
    }
}
///Transmit interrupt status (transmitters)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXISW_AW {
    ///1: Generate a TXIS event
    Trigger = 1,
}
impl From<TXISW_AW> for bool {
    #[inline(always)]
    fn from(variant: TXISW_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `TXIS` writer - Transmit interrupt status (transmitters)
pub type TXIS_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, ISR_SPEC, TXISW_AW, O>;
impl<'a, const O: u8> TXIS_W<'a, O> {
    ///Generate a TXIS event
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TXISW_AW::Trigger)
    }
}
///Field `RXNE` reader - Receive data register not empty (receivers)
pub type RXNE_R = crate::BitReader<RXNE_A>;
///Receive data register not empty (receivers)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXNE_A {
    ///0: The RXDR register is empty
    Empty = 0,
    ///1: Received data is copied into the RXDR register, and is ready to be read
    NotEmpty = 1,
}
impl From<RXNE_A> for bool {
    #[inline(always)]
    fn from(variant: RXNE_A) -> Self {
        variant as u8 != 0
    }
}
impl RXNE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RXNE_A {
        match self.bits {
            false => RXNE_A::Empty,
            true => RXNE_A::NotEmpty,
        }
    }
    ///Checks if the value of the field is `Empty`
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == RXNE_A::Empty
    }
    ///Checks if the value of the field is `NotEmpty`
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == RXNE_A::NotEmpty
    }
}
///Field `ADDR` reader - Address matched (slave mode)
pub type ADDR_R = crate::BitReader<ADDR_A>;
///Address matched (slave mode)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDR_A {
    ///0: Adress mismatched or not received
    NotMatch = 0,
    ///1: Received slave address matched with one of the enabled slave addresses
    Match = 1,
}
impl From<ADDR_A> for bool {
    #[inline(always)]
    fn from(variant: ADDR_A) -> Self {
        variant as u8 != 0
    }
}
impl ADDR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ADDR_A {
        match self.bits {
            false => ADDR_A::NotMatch,
            true => ADDR_A::Match,
        }
    }
    ///Checks if the value of the field is `NotMatch`
    #[inline(always)]
    pub fn is_not_match(&self) -> bool {
        *self == ADDR_A::NotMatch
    }
    ///Checks if the value of the field is `Match`
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == ADDR_A::Match
    }
}
///Field `NACKF` reader - Not acknowledge received flag
pub type NACKF_R = crate::BitReader<NACKF_A>;
///Not acknowledge received flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NACKF_A {
    ///0: No NACK has been received
    NoNack = 0,
    ///1: NACK has been received
    Nack = 1,
}
impl From<NACKF_A> for bool {
    #[inline(always)]
    fn from(variant: NACKF_A) -> Self {
        variant as u8 != 0
    }
}
impl NACKF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> NACKF_A {
        match self.bits {
            false => NACKF_A::NoNack,
            true => NACKF_A::Nack,
        }
    }
    ///Checks if the value of the field is `NoNack`
    #[inline(always)]
    pub fn is_no_nack(&self) -> bool {
        *self == NACKF_A::NoNack
    }
    ///Checks if the value of the field is `Nack`
    #[inline(always)]
    pub fn is_nack(&self) -> bool {
        *self == NACKF_A::Nack
    }
}
///Field `STOPF` reader - Stop detection flag
pub type STOPF_R = crate::BitReader<STOPF_A>;
///Stop detection flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOPF_A {
    ///0: No Stop condition detected
    NoStop = 0,
    ///1: Stop condition detected
    Stop = 1,
}
impl From<STOPF_A> for bool {
    #[inline(always)]
    fn from(variant: STOPF_A) -> Self {
        variant as u8 != 0
    }
}
impl STOPF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> STOPF_A {
        match self.bits {
            false => STOPF_A::NoStop,
            true => STOPF_A::Stop,
        }
    }
    ///Checks if the value of the field is `NoStop`
    #[inline(always)]
    pub fn is_no_stop(&self) -> bool {
        *self == STOPF_A::NoStop
    }
    ///Checks if the value of the field is `Stop`
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == STOPF_A::Stop
    }
}
///Field `TC` reader - Transfer Complete (master mode)
pub type TC_R = crate::BitReader<TC_A>;
///Transfer Complete (master mode)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TC_A {
    ///0: Transfer is not complete
    NotComplete = 0,
    ///1: NBYTES has been transfered
    Complete = 1,
}
impl From<TC_A> for bool {
    #[inline(always)]
    fn from(variant: TC_A) -> Self {
        variant as u8 != 0
    }
}
impl TC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TC_A {
        match self.bits {
            false => TC_A::NotComplete,
            true => TC_A::Complete,
        }
    }
    ///Checks if the value of the field is `NotComplete`
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == TC_A::NotComplete
    }
    ///Checks if the value of the field is `Complete`
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == TC_A::Complete
    }
}
///Field `TCR` reader - Transfer Complete Reload
pub type TCR_R = crate::BitReader<TCR_A>;
///Transfer Complete Reload
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCR_A {
    ///0: Transfer is not complete
    NotComplete = 0,
    ///1: NBYTES has been transfered
    Complete = 1,
}
impl From<TCR_A> for bool {
    #[inline(always)]
    fn from(variant: TCR_A) -> Self {
        variant as u8 != 0
    }
}
impl TCR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TCR_A {
        match self.bits {
            false => TCR_A::NotComplete,
            true => TCR_A::Complete,
        }
    }
    ///Checks if the value of the field is `NotComplete`
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == TCR_A::NotComplete
    }
    ///Checks if the value of the field is `Complete`
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == TCR_A::Complete
    }
}
///Field `BERR` reader - Bus error
pub type BERR_R = crate::BitReader<BERR_A>;
///Bus error
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BERR_A {
    ///0: No bus error
    NoError = 0,
    ///1: Misplaced Start and Stop condition is detected
    Error = 1,
}
impl From<BERR_A> for bool {
    #[inline(always)]
    fn from(variant: BERR_A) -> Self {
        variant as u8 != 0
    }
}
impl BERR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BERR_A {
        match self.bits {
            false => BERR_A::NoError,
            true => BERR_A::Error,
        }
    }
    ///Checks if the value of the field is `NoError`
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == BERR_A::NoError
    }
    ///Checks if the value of the field is `Error`
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == BERR_A::Error
    }
}
///Field `ARLO` reader - Arbitration lost
pub type ARLO_R = crate::BitReader<ARLO_A>;
///Arbitration lost
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ARLO_A {
    ///0: No arbitration lost
    NotLost = 0,
    ///1: Arbitration lost
    Lost = 1,
}
impl From<ARLO_A> for bool {
    #[inline(always)]
    fn from(variant: ARLO_A) -> Self {
        variant as u8 != 0
    }
}
impl ARLO_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ARLO_A {
        match self.bits {
            false => ARLO_A::NotLost,
            true => ARLO_A::Lost,
        }
    }
    ///Checks if the value of the field is `NotLost`
    #[inline(always)]
    pub fn is_not_lost(&self) -> bool {
        *self == ARLO_A::NotLost
    }
    ///Checks if the value of the field is `Lost`
    #[inline(always)]
    pub fn is_lost(&self) -> bool {
        *self == ARLO_A::Lost
    }
}
///Field `OVR` reader - Overrun/Underrun (slave mode)
pub type OVR_R = crate::BitReader<OVR_A>;
///Overrun/Underrun (slave mode)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVR_A {
    ///0: No overrun/underrun error occurs
    NoOverrun = 0,
    ///1: slave mode with NOSTRETCH=1, when an overrun/underrun error occurs
    Overrun = 1,
}
impl From<OVR_A> for bool {
    #[inline(always)]
    fn from(variant: OVR_A) -> Self {
        variant as u8 != 0
    }
}
impl OVR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OVR_A {
        match self.bits {
            false => OVR_A::NoOverrun,
            true => OVR_A::Overrun,
        }
    }
    ///Checks if the value of the field is `NoOverrun`
    #[inline(always)]
    pub fn is_no_overrun(&self) -> bool {
        *self == OVR_A::NoOverrun
    }
    ///Checks if the value of the field is `Overrun`
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == OVR_A::Overrun
    }
}
///Field `PECERR` reader - PEC Error in reception
pub type PECERR_R = crate::BitReader<PECERR_A>;
///PEC Error in reception
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PECERR_A {
    ///0: Received PEC does match with PEC register
    Match = 0,
    ///1: Received PEC does not match with PEC register
    NoMatch = 1,
}
impl From<PECERR_A> for bool {
    #[inline(always)]
    fn from(variant: PECERR_A) -> Self {
        variant as u8 != 0
    }
}
impl PECERR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PECERR_A {
        match self.bits {
            false => PECERR_A::Match,
            true => PECERR_A::NoMatch,
        }
    }
    ///Checks if the value of the field is `Match`
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == PECERR_A::Match
    }
    ///Checks if the value of the field is `NoMatch`
    #[inline(always)]
    pub fn is_no_match(&self) -> bool {
        *self == PECERR_A::NoMatch
    }
}
///Field `TIMEOUT` reader - Timeout or t_low detection flag
pub type TIMEOUT_R = crate::BitReader<TIMEOUT_A>;
///Timeout or t_low detection flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIMEOUT_A {
    ///0: No timeout occured
    NoTimeout = 0,
    ///1: Timeout occured
    Timeout = 1,
}
impl From<TIMEOUT_A> for bool {
    #[inline(always)]
    fn from(variant: TIMEOUT_A) -> Self {
        variant as u8 != 0
    }
}
impl TIMEOUT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TIMEOUT_A {
        match self.bits {
            false => TIMEOUT_A::NoTimeout,
            true => TIMEOUT_A::Timeout,
        }
    }
    ///Checks if the value of the field is `NoTimeout`
    #[inline(always)]
    pub fn is_no_timeout(&self) -> bool {
        *self == TIMEOUT_A::NoTimeout
    }
    ///Checks if the value of the field is `Timeout`
    #[inline(always)]
    pub fn is_timeout(&self) -> bool {
        *self == TIMEOUT_A::Timeout
    }
}
///Field `ALERT` reader - SMBus alert
pub type ALERT_R = crate::BitReader<ALERT_A>;
///SMBus alert
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALERT_A {
    ///0: SMBA alert is not detected
    NoAlert = 0,
    ///1: SMBA alert event is detected on SMBA pin
    Alert = 1,
}
impl From<ALERT_A> for bool {
    #[inline(always)]
    fn from(variant: ALERT_A) -> Self {
        variant as u8 != 0
    }
}
impl ALERT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ALERT_A {
        match self.bits {
            false => ALERT_A::NoAlert,
            true => ALERT_A::Alert,
        }
    }
    ///Checks if the value of the field is `NoAlert`
    #[inline(always)]
    pub fn is_no_alert(&self) -> bool {
        *self == ALERT_A::NoAlert
    }
    ///Checks if the value of the field is `Alert`
    #[inline(always)]
    pub fn is_alert(&self) -> bool {
        *self == ALERT_A::Alert
    }
}
///Field `BUSY` reader - Bus busy
pub type BUSY_R = crate::BitReader<BUSY_A>;
///Bus busy
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSY_A {
    ///0: No communication is in progress on the bus
    NotBusy = 0,
    ///1: A communication is in progress on the bus
    Busy = 1,
}
impl From<BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BUSY_A {
        match self.bits {
            false => BUSY_A::NotBusy,
            true => BUSY_A::Busy,
        }
    }
    ///Checks if the value of the field is `NotBusy`
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        *self == BUSY_A::NotBusy
    }
    ///Checks if the value of the field is `Busy`
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUSY_A::Busy
    }
}
///Field `DIR` reader - Transfer direction (Slave mode)
pub type DIR_R = crate::BitReader<DIR_A>;
///Transfer direction (Slave mode)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIR_A {
    ///0: Write transfer, slave enters receiver mode
    Write = 0,
    ///1: Read transfer, slave enters transmitter mode
    Read = 1,
}
impl From<DIR_A> for bool {
    #[inline(always)]
    fn from(variant: DIR_A) -> Self {
        variant as u8 != 0
    }
}
impl DIR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DIR_A {
        match self.bits {
            false => DIR_A::Write,
            true => DIR_A::Read,
        }
    }
    ///Checks if the value of the field is `Write`
    #[inline(always)]
    pub fn is_write(&self) -> bool {
        *self == DIR_A::Write
    }
    ///Checks if the value of the field is `Read`
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == DIR_A::Read
    }
}
///Field `ADDCODE` reader - Address match code (Slave mode)
pub type ADDCODE_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bit 0 - Transmit data register empty (transmitters)
    #[inline(always)]
    pub fn txe(&self) -> TXE_R {
        TXE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Transmit interrupt status (transmitters)
    #[inline(always)]
    pub fn txis(&self) -> TXIS_R {
        TXIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Receive data register not empty (receivers)
    #[inline(always)]
    pub fn rxne(&self) -> RXNE_R {
        RXNE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Address matched (slave mode)
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Not acknowledge received flag
    #[inline(always)]
    pub fn nackf(&self) -> NACKF_R {
        NACKF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Stop detection flag
    #[inline(always)]
    pub fn stopf(&self) -> STOPF_R {
        STOPF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Transfer Complete (master mode)
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Transfer Complete Reload
    #[inline(always)]
    pub fn tcr(&self) -> TCR_R {
        TCR_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Bus error
    #[inline(always)]
    pub fn berr(&self) -> BERR_R {
        BERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Arbitration lost
    #[inline(always)]
    pub fn arlo(&self) -> ARLO_R {
        ARLO_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Overrun/Underrun (slave mode)
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - PEC Error in reception
    #[inline(always)]
    pub fn pecerr(&self) -> PECERR_R {
        PECERR_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Timeout or t_low detection flag
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - SMBus alert
    #[inline(always)]
    pub fn alert(&self) -> ALERT_R {
        ALERT_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 15 - Bus busy
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Transfer direction (Slave mode)
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:23 - Address match code (Slave mode)
    #[inline(always)]
    pub fn addcode(&self) -> ADDCODE_R {
        ADDCODE_R::new(((self.bits >> 17) & 0x7f) as u8)
    }
}
impl W {
    ///Bit 0 - Transmit data register empty (transmitters)
    #[inline(always)]
    #[must_use]
    pub fn txe(&mut self) -> TXE_W<0> {
        TXE_W::new(self)
    }
    ///Bit 1 - Transmit interrupt status (transmitters)
    #[inline(always)]
    #[must_use]
    pub fn txis(&mut self) -> TXIS_W<1> {
        TXIS_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Interrupt and Status register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
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
///`write(|w| ..)` method takes [isr::W](W) writer structure
impl crate::Writable for ISR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x03;
}
///`reset()` method sets ISR to value 0x01
impl crate::Resettable for ISR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
