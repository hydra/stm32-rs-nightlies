///Register `CFG2` reader
pub struct R(crate::R<CFG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CFG2` writer
pub struct W(crate::W<CFG2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG2_SPEC>;
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
impl From<crate::W<CFG2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MSSI` reader - Master SS Idleness
pub type MSSI_R = crate::FieldReader<u8, u8>;
///Field `MSSI` writer - Master SS Idleness
pub type MSSI_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CFG2_SPEC, u8, u8, 4, O>;
///Field `MIDI` reader - Master Inter-Data Idleness
pub type MIDI_R = crate::FieldReader<u8, u8>;
///Field `MIDI` writer - Master Inter-Data Idleness
pub type MIDI_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CFG2_SPEC, u8, u8, 4, O>;
///Field `IOSWP` reader - Swap functionality of MISO and MOSI pins
pub type IOSWP_R = crate::BitReader<IOSWP_A>;
///Swap functionality of MISO and MOSI pins
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IOSWP_A {
    ///0: MISO and MOSI not swapped
    Disabled = 0,
    ///1: MISO and MOSI swapped
    Enabled = 1,
}
impl From<IOSWP_A> for bool {
    #[inline(always)]
    fn from(variant: IOSWP_A) -> Self {
        variant as u8 != 0
    }
}
impl IOSWP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> IOSWP_A {
        match self.bits {
            false => IOSWP_A::Disabled,
            true => IOSWP_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IOSWP_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IOSWP_A::Enabled
    }
}
///Field `IOSWP` writer - Swap functionality of MISO and MOSI pins
pub type IOSWP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG2_SPEC, IOSWP_A, O>;
impl<'a, const O: u8> IOSWP_W<'a, O> {
    ///MISO and MOSI not swapped
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(IOSWP_A::Disabled)
    }
    ///MISO and MOSI swapped
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(IOSWP_A::Enabled)
    }
}
///Field `COMM` reader - SPI Communication Mode
pub type COMM_R = crate::FieldReader<u8, COMM_A>;
///SPI Communication Mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COMM_A {
    ///0: Full duplex
    FullDuplex = 0,
    ///1: Simplex transmitter only
    Transmitter = 1,
    ///2: Simplex receiver only
    Receiver = 2,
    ///3: Half duplex
    HalfDuplex = 3,
}
impl From<COMM_A> for u8 {
    #[inline(always)]
    fn from(variant: COMM_A) -> Self {
        variant as _
    }
}
impl COMM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> COMM_A {
        match self.bits {
            0 => COMM_A::FullDuplex,
            1 => COMM_A::Transmitter,
            2 => COMM_A::Receiver,
            3 => COMM_A::HalfDuplex,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `FullDuplex`
    #[inline(always)]
    pub fn is_full_duplex(&self) -> bool {
        *self == COMM_A::FullDuplex
    }
    ///Checks if the value of the field is `Transmitter`
    #[inline(always)]
    pub fn is_transmitter(&self) -> bool {
        *self == COMM_A::Transmitter
    }
    ///Checks if the value of the field is `Receiver`
    #[inline(always)]
    pub fn is_receiver(&self) -> bool {
        *self == COMM_A::Receiver
    }
    ///Checks if the value of the field is `HalfDuplex`
    #[inline(always)]
    pub fn is_half_duplex(&self) -> bool {
        *self == COMM_A::HalfDuplex
    }
}
///Field `COMM` writer - SPI Communication Mode
pub type COMM_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CFG2_SPEC, u8, COMM_A, 2, O>;
impl<'a, const O: u8> COMM_W<'a, O> {
    ///Full duplex
    #[inline(always)]
    pub fn full_duplex(self) -> &'a mut W {
        self.variant(COMM_A::FullDuplex)
    }
    ///Simplex transmitter only
    #[inline(always)]
    pub fn transmitter(self) -> &'a mut W {
        self.variant(COMM_A::Transmitter)
    }
    ///Simplex receiver only
    #[inline(always)]
    pub fn receiver(self) -> &'a mut W {
        self.variant(COMM_A::Receiver)
    }
    ///Half duplex
    #[inline(always)]
    pub fn half_duplex(self) -> &'a mut W {
        self.variant(COMM_A::HalfDuplex)
    }
}
///Field `SP` reader - Serial Protocol
pub type SP_R = crate::FieldReader<u8, SP_A>;
///Serial Protocol
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SP_A {
    ///0: Motorola SPI protocol
    Motorola = 0,
    ///1: TI SPI protocol
    Ti = 1,
}
impl From<SP_A> for u8 {
    #[inline(always)]
    fn from(variant: SP_A) -> Self {
        variant as _
    }
}
impl SP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<SP_A> {
        match self.bits {
            0 => Some(SP_A::Motorola),
            1 => Some(SP_A::Ti),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Motorola`
    #[inline(always)]
    pub fn is_motorola(&self) -> bool {
        *self == SP_A::Motorola
    }
    ///Checks if the value of the field is `Ti`
    #[inline(always)]
    pub fn is_ti(&self) -> bool {
        *self == SP_A::Ti
    }
}
///Field `SP` writer - Serial Protocol
pub type SP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG2_SPEC, u8, SP_A, 3, O>;
impl<'a, const O: u8> SP_W<'a, O> {
    ///Motorola SPI protocol
    #[inline(always)]
    pub fn motorola(self) -> &'a mut W {
        self.variant(SP_A::Motorola)
    }
    ///TI SPI protocol
    #[inline(always)]
    pub fn ti(self) -> &'a mut W {
        self.variant(SP_A::Ti)
    }
}
///Field `MASTER` reader - SPI Master
pub type MASTER_R = crate::BitReader<MASTER_A>;
///SPI Master
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MASTER_A {
    ///0: Slave configuration
    Slave = 0,
    ///1: Master configuration
    Master = 1,
}
impl From<MASTER_A> for bool {
    #[inline(always)]
    fn from(variant: MASTER_A) -> Self {
        variant as u8 != 0
    }
}
impl MASTER_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MASTER_A {
        match self.bits {
            false => MASTER_A::Slave,
            true => MASTER_A::Master,
        }
    }
    ///Checks if the value of the field is `Slave`
    #[inline(always)]
    pub fn is_slave(&self) -> bool {
        *self == MASTER_A::Slave
    }
    ///Checks if the value of the field is `Master`
    #[inline(always)]
    pub fn is_master(&self) -> bool {
        *self == MASTER_A::Master
    }
}
///Field `MASTER` writer - SPI Master
pub type MASTER_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG2_SPEC, MASTER_A, O>;
impl<'a, const O: u8> MASTER_W<'a, O> {
    ///Slave configuration
    #[inline(always)]
    pub fn slave(self) -> &'a mut W {
        self.variant(MASTER_A::Slave)
    }
    ///Master configuration
    #[inline(always)]
    pub fn master(self) -> &'a mut W {
        self.variant(MASTER_A::Master)
    }
}
///Field `LSBFRST` reader - Data frame format
pub type LSBFRST_R = crate::BitReader<LSBFRST_A>;
///Data frame format
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSBFRST_A {
    ///0: Data is transmitted/received with the MSB first
    Msbfirst = 0,
    ///1: Data is transmitted/received with the LSB first
    Lsbfirst = 1,
}
impl From<LSBFRST_A> for bool {
    #[inline(always)]
    fn from(variant: LSBFRST_A) -> Self {
        variant as u8 != 0
    }
}
impl LSBFRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LSBFRST_A {
        match self.bits {
            false => LSBFRST_A::Msbfirst,
            true => LSBFRST_A::Lsbfirst,
        }
    }
    ///Checks if the value of the field is `Msbfirst`
    #[inline(always)]
    pub fn is_msbfirst(&self) -> bool {
        *self == LSBFRST_A::Msbfirst
    }
    ///Checks if the value of the field is `Lsbfirst`
    #[inline(always)]
    pub fn is_lsbfirst(&self) -> bool {
        *self == LSBFRST_A::Lsbfirst
    }
}
///Field `LSBFRST` writer - Data frame format
pub type LSBFRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG2_SPEC, LSBFRST_A, O>;
impl<'a, const O: u8> LSBFRST_W<'a, O> {
    ///Data is transmitted/received with the MSB first
    #[inline(always)]
    pub fn msbfirst(self) -> &'a mut W {
        self.variant(LSBFRST_A::Msbfirst)
    }
    ///Data is transmitted/received with the LSB first
    #[inline(always)]
    pub fn lsbfirst(self) -> &'a mut W {
        self.variant(LSBFRST_A::Lsbfirst)
    }
}
///Field `CPHA` reader - Clock phase
pub type CPHA_R = crate::BitReader<CPHA_A>;
///Clock phase
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPHA_A {
    ///0: The first clock transition is the first data capture edge
    FirstEdge = 0,
    ///1: The second clock transition is the first data capture edge
    SecondEdge = 1,
}
impl From<CPHA_A> for bool {
    #[inline(always)]
    fn from(variant: CPHA_A) -> Self {
        variant as u8 != 0
    }
}
impl CPHA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CPHA_A {
        match self.bits {
            false => CPHA_A::FirstEdge,
            true => CPHA_A::SecondEdge,
        }
    }
    ///Checks if the value of the field is `FirstEdge`
    #[inline(always)]
    pub fn is_first_edge(&self) -> bool {
        *self == CPHA_A::FirstEdge
    }
    ///Checks if the value of the field is `SecondEdge`
    #[inline(always)]
    pub fn is_second_edge(&self) -> bool {
        *self == CPHA_A::SecondEdge
    }
}
///Field `CPHA` writer - Clock phase
pub type CPHA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG2_SPEC, CPHA_A, O>;
impl<'a, const O: u8> CPHA_W<'a, O> {
    ///The first clock transition is the first data capture edge
    #[inline(always)]
    pub fn first_edge(self) -> &'a mut W {
        self.variant(CPHA_A::FirstEdge)
    }
    ///The second clock transition is the first data capture edge
    #[inline(always)]
    pub fn second_edge(self) -> &'a mut W {
        self.variant(CPHA_A::SecondEdge)
    }
}
///Field `CPOL` reader - Clock polarity
pub type CPOL_R = crate::BitReader<CPOL_A>;
///Clock polarity
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPOL_A {
    ///0: CK to 0 when idle
    IdleLow = 0,
    ///1: CK to 1 when idle
    IdleHigh = 1,
}
impl From<CPOL_A> for bool {
    #[inline(always)]
    fn from(variant: CPOL_A) -> Self {
        variant as u8 != 0
    }
}
impl CPOL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CPOL_A {
        match self.bits {
            false => CPOL_A::IdleLow,
            true => CPOL_A::IdleHigh,
        }
    }
    ///Checks if the value of the field is `IdleLow`
    #[inline(always)]
    pub fn is_idle_low(&self) -> bool {
        *self == CPOL_A::IdleLow
    }
    ///Checks if the value of the field is `IdleHigh`
    #[inline(always)]
    pub fn is_idle_high(&self) -> bool {
        *self == CPOL_A::IdleHigh
    }
}
///Field `CPOL` writer - Clock polarity
pub type CPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG2_SPEC, CPOL_A, O>;
impl<'a, const O: u8> CPOL_W<'a, O> {
    ///CK to 0 when idle
    #[inline(always)]
    pub fn idle_low(self) -> &'a mut W {
        self.variant(CPOL_A::IdleLow)
    }
    ///CK to 1 when idle
    #[inline(always)]
    pub fn idle_high(self) -> &'a mut W {
        self.variant(CPOL_A::IdleHigh)
    }
}
///Field `SSM` reader - Software management of SS signal input
pub type SSM_R = crate::BitReader<SSM_A>;
///Software management of SS signal input
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSM_A {
    ///0: Software slave management disabled
    Disabled = 0,
    ///1: Software slave management enabled
    Enabled = 1,
}
impl From<SSM_A> for bool {
    #[inline(always)]
    fn from(variant: SSM_A) -> Self {
        variant as u8 != 0
    }
}
impl SSM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SSM_A {
        match self.bits {
            false => SSM_A::Disabled,
            true => SSM_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SSM_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SSM_A::Enabled
    }
}
///Field `SSM` writer - Software management of SS signal input
pub type SSM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG2_SPEC, SSM_A, O>;
impl<'a, const O: u8> SSM_W<'a, O> {
    ///Software slave management disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SSM_A::Disabled)
    }
    ///Software slave management enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SSM_A::Enabled)
    }
}
///Field `SSIOP` reader - SS input/output polarity
pub type SSIOP_R = crate::BitReader<SSIOP_A>;
///SS input/output polarity
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSIOP_A {
    ///0: Low level is active for SS signal
    ActiveLow = 0,
    ///1: High level is active for SS signal
    ActiveHigh = 1,
}
impl From<SSIOP_A> for bool {
    #[inline(always)]
    fn from(variant: SSIOP_A) -> Self {
        variant as u8 != 0
    }
}
impl SSIOP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SSIOP_A {
        match self.bits {
            false => SSIOP_A::ActiveLow,
            true => SSIOP_A::ActiveHigh,
        }
    }
    ///Checks if the value of the field is `ActiveLow`
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == SSIOP_A::ActiveLow
    }
    ///Checks if the value of the field is `ActiveHigh`
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == SSIOP_A::ActiveHigh
    }
}
///Field `SSIOP` writer - SS input/output polarity
pub type SSIOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG2_SPEC, SSIOP_A, O>;
impl<'a, const O: u8> SSIOP_W<'a, O> {
    ///Low level is active for SS signal
    #[inline(always)]
    pub fn active_low(self) -> &'a mut W {
        self.variant(SSIOP_A::ActiveLow)
    }
    ///High level is active for SS signal
    #[inline(always)]
    pub fn active_high(self) -> &'a mut W {
        self.variant(SSIOP_A::ActiveHigh)
    }
}
///Field `SSOE` reader - SS output enable
pub type SSOE_R = crate::BitReader<SSOE_A>;
///SS output enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSOE_A {
    ///0: SS output is disabled in master mode
    Disabled = 0,
    ///1: SS output is enabled in master mode
    Enabled = 1,
}
impl From<SSOE_A> for bool {
    #[inline(always)]
    fn from(variant: SSOE_A) -> Self {
        variant as u8 != 0
    }
}
impl SSOE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SSOE_A {
        match self.bits {
            false => SSOE_A::Disabled,
            true => SSOE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SSOE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SSOE_A::Enabled
    }
}
///Field `SSOE` writer - SS output enable
pub type SSOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG2_SPEC, SSOE_A, O>;
impl<'a, const O: u8> SSOE_W<'a, O> {
    ///SS output is disabled in master mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SSOE_A::Disabled)
    }
    ///SS output is enabled in master mode
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SSOE_A::Enabled)
    }
}
///Field `SSOM` reader - SS output management in master mode
pub type SSOM_R = crate::BitReader<SSOM_A>;
///SS output management in master mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSOM_A {
    ///0: SS is asserted until data transfer complete
    Asserted = 0,
    ///1: Data frames interleaved with SS not asserted during MIDI
    NotAsserted = 1,
}
impl From<SSOM_A> for bool {
    #[inline(always)]
    fn from(variant: SSOM_A) -> Self {
        variant as u8 != 0
    }
}
impl SSOM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SSOM_A {
        match self.bits {
            false => SSOM_A::Asserted,
            true => SSOM_A::NotAsserted,
        }
    }
    ///Checks if the value of the field is `Asserted`
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == SSOM_A::Asserted
    }
    ///Checks if the value of the field is `NotAsserted`
    #[inline(always)]
    pub fn is_not_asserted(&self) -> bool {
        *self == SSOM_A::NotAsserted
    }
}
///Field `SSOM` writer - SS output management in master mode
pub type SSOM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG2_SPEC, SSOM_A, O>;
impl<'a, const O: u8> SSOM_W<'a, O> {
    ///SS is asserted until data transfer complete
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(SSOM_A::Asserted)
    }
    ///Data frames interleaved with SS not asserted during MIDI
    #[inline(always)]
    pub fn not_asserted(self) -> &'a mut W {
        self.variant(SSOM_A::NotAsserted)
    }
}
///Field `AFCNTR` reader - Alternate function GPIOs control
pub type AFCNTR_R = crate::BitReader<AFCNTR_A>;
///Alternate function GPIOs control
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AFCNTR_A {
    ///0: Peripheral takes no control of GPIOs while disabled
    NotControlled = 0,
    ///1: Peripheral controls GPIOs while disabled
    Controlled = 1,
}
impl From<AFCNTR_A> for bool {
    #[inline(always)]
    fn from(variant: AFCNTR_A) -> Self {
        variant as u8 != 0
    }
}
impl AFCNTR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> AFCNTR_A {
        match self.bits {
            false => AFCNTR_A::NotControlled,
            true => AFCNTR_A::Controlled,
        }
    }
    ///Checks if the value of the field is `NotControlled`
    #[inline(always)]
    pub fn is_not_controlled(&self) -> bool {
        *self == AFCNTR_A::NotControlled
    }
    ///Checks if the value of the field is `Controlled`
    #[inline(always)]
    pub fn is_controlled(&self) -> bool {
        *self == AFCNTR_A::Controlled
    }
}
///Field `AFCNTR` writer - Alternate function GPIOs control
pub type AFCNTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG2_SPEC, AFCNTR_A, O>;
impl<'a, const O: u8> AFCNTR_W<'a, O> {
    ///Peripheral takes no control of GPIOs while disabled
    #[inline(always)]
    pub fn not_controlled(self) -> &'a mut W {
        self.variant(AFCNTR_A::NotControlled)
    }
    ///Peripheral controls GPIOs while disabled
    #[inline(always)]
    pub fn controlled(self) -> &'a mut W {
        self.variant(AFCNTR_A::Controlled)
    }
}
impl R {
    ///Bits 0:3 - Master SS Idleness
    #[inline(always)]
    pub fn mssi(&self) -> MSSI_R {
        MSSI_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Master Inter-Data Idleness
    #[inline(always)]
    pub fn midi(&self) -> MIDI_R {
        MIDI_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bit 15 - Swap functionality of MISO and MOSI pins
    #[inline(always)]
    pub fn ioswp(&self) -> IOSWP_R {
        IOSWP_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 17:18 - SPI Communication Mode
    #[inline(always)]
    pub fn comm(&self) -> COMM_R {
        COMM_R::new(((self.bits >> 17) & 3) as u8)
    }
    ///Bits 19:21 - Serial Protocol
    #[inline(always)]
    pub fn sp(&self) -> SP_R {
        SP_R::new(((self.bits >> 19) & 7) as u8)
    }
    ///Bit 22 - SPI Master
    #[inline(always)]
    pub fn master(&self) -> MASTER_R {
        MASTER_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Data frame format
    #[inline(always)]
    pub fn lsbfrst(&self) -> LSBFRST_R {
        LSBFRST_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Clock phase
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Clock polarity
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Software management of SS signal input
    #[inline(always)]
    pub fn ssm(&self) -> SSM_R {
        SSM_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 28 - SS input/output polarity
    #[inline(always)]
    pub fn ssiop(&self) -> SSIOP_R {
        SSIOP_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - SS output enable
    #[inline(always)]
    pub fn ssoe(&self) -> SSOE_R {
        SSOE_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - SS output management in master mode
    #[inline(always)]
    pub fn ssom(&self) -> SSOM_R {
        SSOM_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Alternate function GPIOs control
    #[inline(always)]
    pub fn afcntr(&self) -> AFCNTR_R {
        AFCNTR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 0:3 - Master SS Idleness
    #[inline(always)]
    #[must_use]
    pub fn mssi(&mut self) -> MSSI_W<0> {
        MSSI_W::new(self)
    }
    ///Bits 4:7 - Master Inter-Data Idleness
    #[inline(always)]
    #[must_use]
    pub fn midi(&mut self) -> MIDI_W<4> {
        MIDI_W::new(self)
    }
    ///Bit 15 - Swap functionality of MISO and MOSI pins
    #[inline(always)]
    #[must_use]
    pub fn ioswp(&mut self) -> IOSWP_W<15> {
        IOSWP_W::new(self)
    }
    ///Bits 17:18 - SPI Communication Mode
    #[inline(always)]
    #[must_use]
    pub fn comm(&mut self) -> COMM_W<17> {
        COMM_W::new(self)
    }
    ///Bits 19:21 - Serial Protocol
    #[inline(always)]
    #[must_use]
    pub fn sp(&mut self) -> SP_W<19> {
        SP_W::new(self)
    }
    ///Bit 22 - SPI Master
    #[inline(always)]
    #[must_use]
    pub fn master(&mut self) -> MASTER_W<22> {
        MASTER_W::new(self)
    }
    ///Bit 23 - Data frame format
    #[inline(always)]
    #[must_use]
    pub fn lsbfrst(&mut self) -> LSBFRST_W<23> {
        LSBFRST_W::new(self)
    }
    ///Bit 24 - Clock phase
    #[inline(always)]
    #[must_use]
    pub fn cpha(&mut self) -> CPHA_W<24> {
        CPHA_W::new(self)
    }
    ///Bit 25 - Clock polarity
    #[inline(always)]
    #[must_use]
    pub fn cpol(&mut self) -> CPOL_W<25> {
        CPOL_W::new(self)
    }
    ///Bit 26 - Software management of SS signal input
    #[inline(always)]
    #[must_use]
    pub fn ssm(&mut self) -> SSM_W<26> {
        SSM_W::new(self)
    }
    ///Bit 28 - SS input/output polarity
    #[inline(always)]
    #[must_use]
    pub fn ssiop(&mut self) -> SSIOP_W<28> {
        SSIOP_W::new(self)
    }
    ///Bit 29 - SS output enable
    #[inline(always)]
    #[must_use]
    pub fn ssoe(&mut self) -> SSOE_W<29> {
        SSOE_W::new(self)
    }
    ///Bit 30 - SS output management in master mode
    #[inline(always)]
    #[must_use]
    pub fn ssom(&mut self) -> SSOM_W<30> {
        SSOM_W::new(self)
    }
    ///Bit 31 - Alternate function GPIOs control
    #[inline(always)]
    #[must_use]
    pub fn afcntr(&mut self) -> AFCNTR_W<31> {
        AFCNTR_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///configuration register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cfg2](index.html) module
pub struct CFG2_SPEC;
impl crate::RegisterSpec for CFG2_SPEC {
    type Ux = u32;
}
///`read()` method returns [cfg2::R](R) reader structure
impl crate::Readable for CFG2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cfg2::W](W) writer structure
impl crate::Writable for CFG2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CFG2 to value 0
impl crate::Resettable for CFG2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
