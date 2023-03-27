///Register `CR1` reader
pub struct R(crate::R<CR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CR1` writer
pub struct W(crate::W<CR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR1_SPEC>;
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
impl From<crate::W<CR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PE` reader - Peripheral enable
pub type PE_R = crate::BitReader<PE_A>;
///Peripheral enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PE_A {
    ///0: Peripheral disabled
    Disabled = 0,
    ///1: Peripheral enabled
    Enabled = 1,
}
impl From<PE_A> for bool {
    #[inline(always)]
    fn from(variant: PE_A) -> Self {
        variant as u8 != 0
    }
}
impl PE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PE_A {
        match self.bits {
            false => PE_A::Disabled,
            true => PE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PE_A::Enabled
    }
}
///Field `PE` writer - Peripheral enable
pub type PE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, PE_A, O>;
impl<'a, const O: u8> PE_W<'a, O> {
    ///Peripheral disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PE_A::Disabled)
    }
    ///Peripheral enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PE_A::Enabled)
    }
}
///Field `SMBUS` reader - SMBus mode
pub type SMBUS_R = crate::BitReader<SMBUS_A>;
///SMBus mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMBUS_A {
    ///0: I2C Mode
    I2c = 0,
    ///1: SMBus
    Smbus = 1,
}
impl From<SMBUS_A> for bool {
    #[inline(always)]
    fn from(variant: SMBUS_A) -> Self {
        variant as u8 != 0
    }
}
impl SMBUS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SMBUS_A {
        match self.bits {
            false => SMBUS_A::I2c,
            true => SMBUS_A::Smbus,
        }
    }
    ///Checks if the value of the field is `I2c`
    #[inline(always)]
    pub fn is_i2c(&self) -> bool {
        *self == SMBUS_A::I2c
    }
    ///Checks if the value of the field is `Smbus`
    #[inline(always)]
    pub fn is_smbus(&self) -> bool {
        *self == SMBUS_A::Smbus
    }
}
///Field `SMBUS` writer - SMBus mode
pub type SMBUS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, SMBUS_A, O>;
impl<'a, const O: u8> SMBUS_W<'a, O> {
    ///I2C Mode
    #[inline(always)]
    pub fn i2c(self) -> &'a mut W {
        self.variant(SMBUS_A::I2c)
    }
    ///SMBus
    #[inline(always)]
    pub fn smbus(self) -> &'a mut W {
        self.variant(SMBUS_A::Smbus)
    }
}
///Field `SMBTYPE` reader - SMBus type
pub type SMBTYPE_R = crate::BitReader<SMBTYPE_A>;
///SMBus type
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMBTYPE_A {
    ///0: SMBus Device
    Device = 0,
    ///1: SMBus Host
    Host = 1,
}
impl From<SMBTYPE_A> for bool {
    #[inline(always)]
    fn from(variant: SMBTYPE_A) -> Self {
        variant as u8 != 0
    }
}
impl SMBTYPE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SMBTYPE_A {
        match self.bits {
            false => SMBTYPE_A::Device,
            true => SMBTYPE_A::Host,
        }
    }
    ///Checks if the value of the field is `Device`
    #[inline(always)]
    pub fn is_device(&self) -> bool {
        *self == SMBTYPE_A::Device
    }
    ///Checks if the value of the field is `Host`
    #[inline(always)]
    pub fn is_host(&self) -> bool {
        *self == SMBTYPE_A::Host
    }
}
///Field `SMBTYPE` writer - SMBus type
pub type SMBTYPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, SMBTYPE_A, O>;
impl<'a, const O: u8> SMBTYPE_W<'a, O> {
    ///SMBus Device
    #[inline(always)]
    pub fn device(self) -> &'a mut W {
        self.variant(SMBTYPE_A::Device)
    }
    ///SMBus Host
    #[inline(always)]
    pub fn host(self) -> &'a mut W {
        self.variant(SMBTYPE_A::Host)
    }
}
///Field `ENARP` reader - ARP enable
pub type ENARP_R = crate::BitReader<ENARP_A>;
///ARP enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENARP_A {
    ///0: ARP disabled
    Disabled = 0,
    ///1: ARP enabled
    Enabled = 1,
}
impl From<ENARP_A> for bool {
    #[inline(always)]
    fn from(variant: ENARP_A) -> Self {
        variant as u8 != 0
    }
}
impl ENARP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ENARP_A {
        match self.bits {
            false => ENARP_A::Disabled,
            true => ENARP_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENARP_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENARP_A::Enabled
    }
}
///Field `ENARP` writer - ARP enable
pub type ENARP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, ENARP_A, O>;
impl<'a, const O: u8> ENARP_W<'a, O> {
    ///ARP disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENARP_A::Disabled)
    }
    ///ARP enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENARP_A::Enabled)
    }
}
///Field `ENPEC` reader - PEC enable
pub type ENPEC_R = crate::BitReader<ENPEC_A>;
///PEC enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENPEC_A {
    ///0: PEC calculation disabled
    Disabled = 0,
    ///1: PEC calculation enabled
    Enabled = 1,
}
impl From<ENPEC_A> for bool {
    #[inline(always)]
    fn from(variant: ENPEC_A) -> Self {
        variant as u8 != 0
    }
}
impl ENPEC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ENPEC_A {
        match self.bits {
            false => ENPEC_A::Disabled,
            true => ENPEC_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENPEC_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENPEC_A::Enabled
    }
}
///Field `ENPEC` writer - PEC enable
pub type ENPEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, ENPEC_A, O>;
impl<'a, const O: u8> ENPEC_W<'a, O> {
    ///PEC calculation disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENPEC_A::Disabled)
    }
    ///PEC calculation enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENPEC_A::Enabled)
    }
}
///Field `ENGC` reader - General call enable
pub type ENGC_R = crate::BitReader<ENGC_A>;
///General call enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENGC_A {
    ///0: General call disabled
    Disabled = 0,
    ///1: General call enabled
    Enabled = 1,
}
impl From<ENGC_A> for bool {
    #[inline(always)]
    fn from(variant: ENGC_A) -> Self {
        variant as u8 != 0
    }
}
impl ENGC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ENGC_A {
        match self.bits {
            false => ENGC_A::Disabled,
            true => ENGC_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENGC_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENGC_A::Enabled
    }
}
///Field `ENGC` writer - General call enable
pub type ENGC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, ENGC_A, O>;
impl<'a, const O: u8> ENGC_W<'a, O> {
    ///General call disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENGC_A::Disabled)
    }
    ///General call enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENGC_A::Enabled)
    }
}
///Field `NOSTRETCH` reader - Clock stretching disable (Slave mode)
pub type NOSTRETCH_R = crate::BitReader<NOSTRETCH_A>;
///Clock stretching disable (Slave mode)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NOSTRETCH_A {
    ///0: Clock stretching enabled
    Enabled = 0,
    ///1: Clock stretching disabled
    Disabled = 1,
}
impl From<NOSTRETCH_A> for bool {
    #[inline(always)]
    fn from(variant: NOSTRETCH_A) -> Self {
        variant as u8 != 0
    }
}
impl NOSTRETCH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> NOSTRETCH_A {
        match self.bits {
            false => NOSTRETCH_A::Enabled,
            true => NOSTRETCH_A::Disabled,
        }
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == NOSTRETCH_A::Enabled
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == NOSTRETCH_A::Disabled
    }
}
///Field `NOSTRETCH` writer - Clock stretching disable (Slave mode)
pub type NOSTRETCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, NOSTRETCH_A, O>;
impl<'a, const O: u8> NOSTRETCH_W<'a, O> {
    ///Clock stretching enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(NOSTRETCH_A::Enabled)
    }
    ///Clock stretching disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(NOSTRETCH_A::Disabled)
    }
}
///Field `START` reader - Start generation
pub type START_R = crate::BitReader<START_A>;
///Start generation
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum START_A {
    ///0: No Start generation
    NoStart = 0,
    ///1: In master mode: repeated start generation, in slave mode: start generation when bus is free
    Start = 1,
}
impl From<START_A> for bool {
    #[inline(always)]
    fn from(variant: START_A) -> Self {
        variant as u8 != 0
    }
}
impl START_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> START_A {
        match self.bits {
            false => START_A::NoStart,
            true => START_A::Start,
        }
    }
    ///Checks if the value of the field is `NoStart`
    #[inline(always)]
    pub fn is_no_start(&self) -> bool {
        *self == START_A::NoStart
    }
    ///Checks if the value of the field is `Start`
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == START_A::Start
    }
}
///Field `START` writer - Start generation
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, START_A, O>;
impl<'a, const O: u8> START_W<'a, O> {
    ///No Start generation
    #[inline(always)]
    pub fn no_start(self) -> &'a mut W {
        self.variant(START_A::NoStart)
    }
    ///In master mode: repeated start generation, in slave mode: start generation when bus is free
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(START_A::Start)
    }
}
///Field `STOP` reader - Stop generation
pub type STOP_R = crate::BitReader<STOP_A>;
///Stop generation
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOP_A {
    ///0: No Stop generation
    NoStop = 0,
    ///1: In master mode: stop generation after current byte/start, in slave mode: release SCL and SDA after current byte
    Stop = 1,
}
impl From<STOP_A> for bool {
    #[inline(always)]
    fn from(variant: STOP_A) -> Self {
        variant as u8 != 0
    }
}
impl STOP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> STOP_A {
        match self.bits {
            false => STOP_A::NoStop,
            true => STOP_A::Stop,
        }
    }
    ///Checks if the value of the field is `NoStop`
    #[inline(always)]
    pub fn is_no_stop(&self) -> bool {
        *self == STOP_A::NoStop
    }
    ///Checks if the value of the field is `Stop`
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == STOP_A::Stop
    }
}
///Field `STOP` writer - Stop generation
pub type STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, STOP_A, O>;
impl<'a, const O: u8> STOP_W<'a, O> {
    ///No Stop generation
    #[inline(always)]
    pub fn no_stop(self) -> &'a mut W {
        self.variant(STOP_A::NoStop)
    }
    ///In master mode: stop generation after current byte/start, in slave mode: release SCL and SDA after current byte
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(STOP_A::Stop)
    }
}
///Field `ACK` reader - Acknowledge enable
pub type ACK_R = crate::BitReader<ACK_A>;
///Acknowledge enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACK_A {
    ///0: No acknowledge returned
    Nak = 0,
    ///1: Acknowledge returned after a byte is received
    Ack = 1,
}
impl From<ACK_A> for bool {
    #[inline(always)]
    fn from(variant: ACK_A) -> Self {
        variant as u8 != 0
    }
}
impl ACK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ACK_A {
        match self.bits {
            false => ACK_A::Nak,
            true => ACK_A::Ack,
        }
    }
    ///Checks if the value of the field is `Nak`
    #[inline(always)]
    pub fn is_nak(&self) -> bool {
        *self == ACK_A::Nak
    }
    ///Checks if the value of the field is `Ack`
    #[inline(always)]
    pub fn is_ack(&self) -> bool {
        *self == ACK_A::Ack
    }
}
///Field `ACK` writer - Acknowledge enable
pub type ACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, ACK_A, O>;
impl<'a, const O: u8> ACK_W<'a, O> {
    ///No acknowledge returned
    #[inline(always)]
    pub fn nak(self) -> &'a mut W {
        self.variant(ACK_A::Nak)
    }
    ///Acknowledge returned after a byte is received
    #[inline(always)]
    pub fn ack(self) -> &'a mut W {
        self.variant(ACK_A::Ack)
    }
}
///Field `POS` reader - Acknowledge/PEC Position (for data reception)
pub type POS_R = crate::BitReader<POS_A>;
///Acknowledge/PEC Position (for data reception)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POS_A {
    ///0: ACK bit controls the (N)ACK of the current byte being received
    Current = 0,
    ///1: ACK bit controls the (N)ACK of the next byte to be received
    Next = 1,
}
impl From<POS_A> for bool {
    #[inline(always)]
    fn from(variant: POS_A) -> Self {
        variant as u8 != 0
    }
}
impl POS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> POS_A {
        match self.bits {
            false => POS_A::Current,
            true => POS_A::Next,
        }
    }
    ///Checks if the value of the field is `Current`
    #[inline(always)]
    pub fn is_current(&self) -> bool {
        *self == POS_A::Current
    }
    ///Checks if the value of the field is `Next`
    #[inline(always)]
    pub fn is_next(&self) -> bool {
        *self == POS_A::Next
    }
}
///Field `POS` writer - Acknowledge/PEC Position (for data reception)
pub type POS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, POS_A, O>;
impl<'a, const O: u8> POS_W<'a, O> {
    ///ACK bit controls the (N)ACK of the current byte being received
    #[inline(always)]
    pub fn current(self) -> &'a mut W {
        self.variant(POS_A::Current)
    }
    ///ACK bit controls the (N)ACK of the next byte to be received
    #[inline(always)]
    pub fn next(self) -> &'a mut W {
        self.variant(POS_A::Next)
    }
}
///Field `PEC` reader - Packet error checking
pub type PEC_R = crate::BitReader<PEC_A>;
///Packet error checking
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PEC_A {
    ///0: No PEC transfer
    Disabled = 0,
    ///1: PEC transfer
    Enabled = 1,
}
impl From<PEC_A> for bool {
    #[inline(always)]
    fn from(variant: PEC_A) -> Self {
        variant as u8 != 0
    }
}
impl PEC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PEC_A {
        match self.bits {
            false => PEC_A::Disabled,
            true => PEC_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PEC_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PEC_A::Enabled
    }
}
///Field `PEC` writer - Packet error checking
pub type PEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, PEC_A, O>;
impl<'a, const O: u8> PEC_W<'a, O> {
    ///No PEC transfer
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PEC_A::Disabled)
    }
    ///PEC transfer
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PEC_A::Enabled)
    }
}
///Field `ALERT` reader - SMBus alert
pub type ALERT_R = crate::BitReader<ALERT_A>;
///SMBus alert
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALERT_A {
    ///0: SMBA pin released high
    Release = 0,
    ///1: SMBA pin driven low
    Drive = 1,
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
            false => ALERT_A::Release,
            true => ALERT_A::Drive,
        }
    }
    ///Checks if the value of the field is `Release`
    #[inline(always)]
    pub fn is_release(&self) -> bool {
        *self == ALERT_A::Release
    }
    ///Checks if the value of the field is `Drive`
    #[inline(always)]
    pub fn is_drive(&self) -> bool {
        *self == ALERT_A::Drive
    }
}
///Field `ALERT` writer - SMBus alert
pub type ALERT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, ALERT_A, O>;
impl<'a, const O: u8> ALERT_W<'a, O> {
    ///SMBA pin released high
    #[inline(always)]
    pub fn release(self) -> &'a mut W {
        self.variant(ALERT_A::Release)
    }
    ///SMBA pin driven low
    #[inline(always)]
    pub fn drive(self) -> &'a mut W {
        self.variant(ALERT_A::Drive)
    }
}
///Field `SWRST` reader - Software reset
pub type SWRST_R = crate::BitReader<SWRST_A>;
///Software reset
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWRST_A {
    ///0: I2C peripheral not under reset
    NotReset = 0,
    ///1: I2C peripheral under reset
    Reset = 1,
}
impl From<SWRST_A> for bool {
    #[inline(always)]
    fn from(variant: SWRST_A) -> Self {
        variant as u8 != 0
    }
}
impl SWRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SWRST_A {
        match self.bits {
            false => SWRST_A::NotReset,
            true => SWRST_A::Reset,
        }
    }
    ///Checks if the value of the field is `NotReset`
    #[inline(always)]
    pub fn is_not_reset(&self) -> bool {
        *self == SWRST_A::NotReset
    }
    ///Checks if the value of the field is `Reset`
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SWRST_A::Reset
    }
}
///Field `SWRST` writer - Software reset
pub type SWRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, SWRST_A, O>;
impl<'a, const O: u8> SWRST_W<'a, O> {
    ///I2C peripheral not under reset
    #[inline(always)]
    pub fn not_reset(self) -> &'a mut W {
        self.variant(SWRST_A::NotReset)
    }
    ///I2C peripheral under reset
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(SWRST_A::Reset)
    }
}
impl R {
    ///Bit 0 - Peripheral enable
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - SMBus mode
    #[inline(always)]
    pub fn smbus(&self) -> SMBUS_R {
        SMBUS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - SMBus type
    #[inline(always)]
    pub fn smbtype(&self) -> SMBTYPE_R {
        SMBTYPE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - ARP enable
    #[inline(always)]
    pub fn enarp(&self) -> ENARP_R {
        ENARP_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - PEC enable
    #[inline(always)]
    pub fn enpec(&self) -> ENPEC_R {
        ENPEC_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - General call enable
    #[inline(always)]
    pub fn engc(&self) -> ENGC_R {
        ENGC_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Clock stretching disable (Slave mode)
    #[inline(always)]
    pub fn nostretch(&self) -> NOSTRETCH_R {
        NOSTRETCH_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Start generation
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Stop generation
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Acknowledge enable
    #[inline(always)]
    pub fn ack(&self) -> ACK_R {
        ACK_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Acknowledge/PEC Position (for data reception)
    #[inline(always)]
    pub fn pos(&self) -> POS_R {
        POS_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Packet error checking
    #[inline(always)]
    pub fn pec(&self) -> PEC_R {
        PEC_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - SMBus alert
    #[inline(always)]
    pub fn alert(&self) -> ALERT_R {
        ALERT_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 15 - Software reset
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Peripheral enable
    #[inline(always)]
    #[must_use]
    pub fn pe(&mut self) -> PE_W<0> {
        PE_W::new(self)
    }
    ///Bit 1 - SMBus mode
    #[inline(always)]
    #[must_use]
    pub fn smbus(&mut self) -> SMBUS_W<1> {
        SMBUS_W::new(self)
    }
    ///Bit 3 - SMBus type
    #[inline(always)]
    #[must_use]
    pub fn smbtype(&mut self) -> SMBTYPE_W<3> {
        SMBTYPE_W::new(self)
    }
    ///Bit 4 - ARP enable
    #[inline(always)]
    #[must_use]
    pub fn enarp(&mut self) -> ENARP_W<4> {
        ENARP_W::new(self)
    }
    ///Bit 5 - PEC enable
    #[inline(always)]
    #[must_use]
    pub fn enpec(&mut self) -> ENPEC_W<5> {
        ENPEC_W::new(self)
    }
    ///Bit 6 - General call enable
    #[inline(always)]
    #[must_use]
    pub fn engc(&mut self) -> ENGC_W<6> {
        ENGC_W::new(self)
    }
    ///Bit 7 - Clock stretching disable (Slave mode)
    #[inline(always)]
    #[must_use]
    pub fn nostretch(&mut self) -> NOSTRETCH_W<7> {
        NOSTRETCH_W::new(self)
    }
    ///Bit 8 - Start generation
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<8> {
        START_W::new(self)
    }
    ///Bit 9 - Stop generation
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> STOP_W<9> {
        STOP_W::new(self)
    }
    ///Bit 10 - Acknowledge enable
    #[inline(always)]
    #[must_use]
    pub fn ack(&mut self) -> ACK_W<10> {
        ACK_W::new(self)
    }
    ///Bit 11 - Acknowledge/PEC Position (for data reception)
    #[inline(always)]
    #[must_use]
    pub fn pos(&mut self) -> POS_W<11> {
        POS_W::new(self)
    }
    ///Bit 12 - Packet error checking
    #[inline(always)]
    #[must_use]
    pub fn pec(&mut self) -> PEC_W<12> {
        PEC_W::new(self)
    }
    ///Bit 13 - SMBus alert
    #[inline(always)]
    #[must_use]
    pub fn alert(&mut self) -> ALERT_W<13> {
        ALERT_W::new(self)
    }
    ///Bit 15 - Software reset
    #[inline(always)]
    #[must_use]
    pub fn swrst(&mut self) -> SWRST_W<15> {
        SWRST_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Control register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr1](index.html) module
pub struct CR1_SPEC;
impl crate::RegisterSpec for CR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [cr1::R](R) reader structure
impl crate::Readable for CR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cr1::W](W) writer structure
impl crate::Writable for CR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CR1 to value 0
impl crate::Resettable for CR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
