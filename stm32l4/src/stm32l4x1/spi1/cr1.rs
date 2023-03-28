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
pub type CPHA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, CPHA_A, O>;
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
pub type CPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, CPOL_A, O>;
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
///Field `MSTR` reader - Master selection
pub type MSTR_R = crate::BitReader<MSTR_A>;
///Master selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTR_A {
    ///0: Slave configuration
    Slave = 0,
    ///1: Master configuration
    Master = 1,
}
impl From<MSTR_A> for bool {
    #[inline(always)]
    fn from(variant: MSTR_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MSTR_A {
        match self.bits {
            false => MSTR_A::Slave,
            true => MSTR_A::Master,
        }
    }
    ///Checks if the value of the field is `Slave`
    #[inline(always)]
    pub fn is_slave(&self) -> bool {
        *self == MSTR_A::Slave
    }
    ///Checks if the value of the field is `Master`
    #[inline(always)]
    pub fn is_master(&self) -> bool {
        *self == MSTR_A::Master
    }
}
///Field `MSTR` writer - Master selection
pub type MSTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, MSTR_A, O>;
impl<'a, const O: u8> MSTR_W<'a, O> {
    ///Slave configuration
    #[inline(always)]
    pub fn slave(self) -> &'a mut W {
        self.variant(MSTR_A::Slave)
    }
    ///Master configuration
    #[inline(always)]
    pub fn master(self) -> &'a mut W {
        self.variant(MSTR_A::Master)
    }
}
///Field `BR` reader - Baud rate control
pub type BR_R = crate::FieldReader<u8, BR_A>;
///Baud rate control
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BR_A {
    ///0: f_PCLK / 2
    Div2 = 0,
    ///1: f_PCLK / 4
    Div4 = 1,
    ///2: f_PCLK / 8
    Div8 = 2,
    ///3: f_PCLK / 16
    Div16 = 3,
    ///4: f_PCLK / 32
    Div32 = 4,
    ///5: f_PCLK / 64
    Div64 = 5,
    ///6: f_PCLK / 128
    Div128 = 6,
    ///7: f_PCLK / 256
    Div256 = 7,
}
impl From<BR_A> for u8 {
    #[inline(always)]
    fn from(variant: BR_A) -> Self {
        variant as _
    }
}
impl BR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BR_A {
        match self.bits {
            0 => BR_A::Div2,
            1 => BR_A::Div4,
            2 => BR_A::Div8,
            3 => BR_A::Div16,
            4 => BR_A::Div32,
            5 => BR_A::Div64,
            6 => BR_A::Div128,
            7 => BR_A::Div256,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Div2`
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == BR_A::Div2
    }
    ///Checks if the value of the field is `Div4`
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == BR_A::Div4
    }
    ///Checks if the value of the field is `Div8`
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == BR_A::Div8
    }
    ///Checks if the value of the field is `Div16`
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == BR_A::Div16
    }
    ///Checks if the value of the field is `Div32`
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == BR_A::Div32
    }
    ///Checks if the value of the field is `Div64`
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == BR_A::Div64
    }
    ///Checks if the value of the field is `Div128`
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == BR_A::Div128
    }
    ///Checks if the value of the field is `Div256`
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == BR_A::Div256
    }
}
///Field `BR` writer - Baud rate control
pub type BR_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CR1_SPEC, u8, BR_A, 3, O>;
impl<'a, const O: u8> BR_W<'a, O> {
    ///f_PCLK / 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(BR_A::Div2)
    }
    ///f_PCLK / 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(BR_A::Div4)
    }
    ///f_PCLK / 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(BR_A::Div8)
    }
    ///f_PCLK / 16
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(BR_A::Div16)
    }
    ///f_PCLK / 32
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(BR_A::Div32)
    }
    ///f_PCLK / 64
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(BR_A::Div64)
    }
    ///f_PCLK / 128
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(BR_A::Div128)
    }
    ///f_PCLK / 256
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(BR_A::Div256)
    }
}
///Field `SPE` reader - SPI enable
pub type SPE_R = crate::BitReader<SPE_A>;
///SPI enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPE_A {
    ///0: Peripheral disabled
    Disabled = 0,
    ///1: Peripheral enabled
    Enabled = 1,
}
impl From<SPE_A> for bool {
    #[inline(always)]
    fn from(variant: SPE_A) -> Self {
        variant as u8 != 0
    }
}
impl SPE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SPE_A {
        match self.bits {
            false => SPE_A::Disabled,
            true => SPE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SPE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SPE_A::Enabled
    }
}
///Field `SPE` writer - SPI enable
pub type SPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, SPE_A, O>;
impl<'a, const O: u8> SPE_W<'a, O> {
    ///Peripheral disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SPE_A::Disabled)
    }
    ///Peripheral enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SPE_A::Enabled)
    }
}
///Field `LSBFIRST` reader - Frame format
pub type LSBFIRST_R = crate::BitReader<LSBFIRST_A>;
///Frame format
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSBFIRST_A {
    ///0: Data is transmitted/received with the MSB first
    Msbfirst = 0,
    ///1: Data is transmitted/received with the LSB first
    Lsbfirst = 1,
}
impl From<LSBFIRST_A> for bool {
    #[inline(always)]
    fn from(variant: LSBFIRST_A) -> Self {
        variant as u8 != 0
    }
}
impl LSBFIRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LSBFIRST_A {
        match self.bits {
            false => LSBFIRST_A::Msbfirst,
            true => LSBFIRST_A::Lsbfirst,
        }
    }
    ///Checks if the value of the field is `Msbfirst`
    #[inline(always)]
    pub fn is_msbfirst(&self) -> bool {
        *self == LSBFIRST_A::Msbfirst
    }
    ///Checks if the value of the field is `Lsbfirst`
    #[inline(always)]
    pub fn is_lsbfirst(&self) -> bool {
        *self == LSBFIRST_A::Lsbfirst
    }
}
///Field `LSBFIRST` writer - Frame format
pub type LSBFIRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, LSBFIRST_A, O>;
impl<'a, const O: u8> LSBFIRST_W<'a, O> {
    ///Data is transmitted/received with the MSB first
    #[inline(always)]
    pub fn msbfirst(self) -> &'a mut W {
        self.variant(LSBFIRST_A::Msbfirst)
    }
    ///Data is transmitted/received with the LSB first
    #[inline(always)]
    pub fn lsbfirst(self) -> &'a mut W {
        self.variant(LSBFIRST_A::Lsbfirst)
    }
}
///Field `SSI` reader - Internal slave select
pub type SSI_R = crate::BitReader<SSI_A>;
///Internal slave select
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSI_A {
    ///0: 0 is forced onto the NSS pin and the I/O value of the NSS pin is ignored
    SlaveSelected = 0,
    ///1: 1 is forced onto the NSS pin and the I/O value of the NSS pin is ignored
    SlaveNotSelected = 1,
}
impl From<SSI_A> for bool {
    #[inline(always)]
    fn from(variant: SSI_A) -> Self {
        variant as u8 != 0
    }
}
impl SSI_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SSI_A {
        match self.bits {
            false => SSI_A::SlaveSelected,
            true => SSI_A::SlaveNotSelected,
        }
    }
    ///Checks if the value of the field is `SlaveSelected`
    #[inline(always)]
    pub fn is_slave_selected(&self) -> bool {
        *self == SSI_A::SlaveSelected
    }
    ///Checks if the value of the field is `SlaveNotSelected`
    #[inline(always)]
    pub fn is_slave_not_selected(&self) -> bool {
        *self == SSI_A::SlaveNotSelected
    }
}
///Field `SSI` writer - Internal slave select
pub type SSI_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, SSI_A, O>;
impl<'a, const O: u8> SSI_W<'a, O> {
    ///0 is forced onto the NSS pin and the I/O value of the NSS pin is ignored
    #[inline(always)]
    pub fn slave_selected(self) -> &'a mut W {
        self.variant(SSI_A::SlaveSelected)
    }
    ///1 is forced onto the NSS pin and the I/O value of the NSS pin is ignored
    #[inline(always)]
    pub fn slave_not_selected(self) -> &'a mut W {
        self.variant(SSI_A::SlaveNotSelected)
    }
}
///Field `SSM` reader - Software slave management
pub type SSM_R = crate::BitReader<SSM_A>;
///Software slave management
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
///Field `SSM` writer - Software slave management
pub type SSM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, SSM_A, O>;
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
///Field `RXONLY` reader - Receive only
pub type RXONLY_R = crate::BitReader<RXONLY_A>;
///Receive only
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXONLY_A {
    ///0: Full duplex (Transmit and receive)
    FullDuplex = 0,
    ///1: Output disabled (Receive-only mode)
    OutputDisabled = 1,
}
impl From<RXONLY_A> for bool {
    #[inline(always)]
    fn from(variant: RXONLY_A) -> Self {
        variant as u8 != 0
    }
}
impl RXONLY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RXONLY_A {
        match self.bits {
            false => RXONLY_A::FullDuplex,
            true => RXONLY_A::OutputDisabled,
        }
    }
    ///Checks if the value of the field is `FullDuplex`
    #[inline(always)]
    pub fn is_full_duplex(&self) -> bool {
        *self == RXONLY_A::FullDuplex
    }
    ///Checks if the value of the field is `OutputDisabled`
    #[inline(always)]
    pub fn is_output_disabled(&self) -> bool {
        *self == RXONLY_A::OutputDisabled
    }
}
///Field `RXONLY` writer - Receive only
pub type RXONLY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, RXONLY_A, O>;
impl<'a, const O: u8> RXONLY_W<'a, O> {
    ///Full duplex (Transmit and receive)
    #[inline(always)]
    pub fn full_duplex(self) -> &'a mut W {
        self.variant(RXONLY_A::FullDuplex)
    }
    ///Output disabled (Receive-only mode)
    #[inline(always)]
    pub fn output_disabled(self) -> &'a mut W {
        self.variant(RXONLY_A::OutputDisabled)
    }
}
///Field `CRCL` reader - CRC length
pub type CRCL_R = crate::BitReader<bool>;
///Field `CRCL` writer - CRC length
pub type CRCL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
///Field `CRCNEXT` reader - CRC transfer next
pub type CRCNEXT_R = crate::BitReader<CRCNEXT_A>;
///CRC transfer next
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCNEXT_A {
    ///0: Next transmit value is from Tx buffer
    TxBuffer = 0,
    ///1: Next transmit value is from Tx CRC register
    Crc = 1,
}
impl From<CRCNEXT_A> for bool {
    #[inline(always)]
    fn from(variant: CRCNEXT_A) -> Self {
        variant as u8 != 0
    }
}
impl CRCNEXT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CRCNEXT_A {
        match self.bits {
            false => CRCNEXT_A::TxBuffer,
            true => CRCNEXT_A::Crc,
        }
    }
    ///Checks if the value of the field is `TxBuffer`
    #[inline(always)]
    pub fn is_tx_buffer(&self) -> bool {
        *self == CRCNEXT_A::TxBuffer
    }
    ///Checks if the value of the field is `Crc`
    #[inline(always)]
    pub fn is_crc(&self) -> bool {
        *self == CRCNEXT_A::Crc
    }
}
///Field `CRCNEXT` writer - CRC transfer next
pub type CRCNEXT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, CRCNEXT_A, O>;
impl<'a, const O: u8> CRCNEXT_W<'a, O> {
    ///Next transmit value is from Tx buffer
    #[inline(always)]
    pub fn tx_buffer(self) -> &'a mut W {
        self.variant(CRCNEXT_A::TxBuffer)
    }
    ///Next transmit value is from Tx CRC register
    #[inline(always)]
    pub fn crc(self) -> &'a mut W {
        self.variant(CRCNEXT_A::Crc)
    }
}
///Field `CRCEN` reader - Hardware CRC calculation enable
pub type CRCEN_R = crate::BitReader<CRCEN_A>;
///Hardware CRC calculation enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCEN_A {
    ///0: CRC calculation disabled
    Disabled = 0,
    ///1: CRC calculation enabled
    Enabled = 1,
}
impl From<CRCEN_A> for bool {
    #[inline(always)]
    fn from(variant: CRCEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CRCEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CRCEN_A {
        match self.bits {
            false => CRCEN_A::Disabled,
            true => CRCEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CRCEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CRCEN_A::Enabled
    }
}
///Field `CRCEN` writer - Hardware CRC calculation enable
pub type CRCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, CRCEN_A, O>;
impl<'a, const O: u8> CRCEN_W<'a, O> {
    ///CRC calculation disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CRCEN_A::Disabled)
    }
    ///CRC calculation enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CRCEN_A::Enabled)
    }
}
///Field `BIDIOE` reader - Output enable in bidirectional mode
pub type BIDIOE_R = crate::BitReader<BIDIOE_A>;
///Output enable in bidirectional mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BIDIOE_A {
    ///0: Output disabled (receive-only mode)
    OutputDisabled = 0,
    ///1: Output enabled (transmit-only mode)
    OutputEnabled = 1,
}
impl From<BIDIOE_A> for bool {
    #[inline(always)]
    fn from(variant: BIDIOE_A) -> Self {
        variant as u8 != 0
    }
}
impl BIDIOE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BIDIOE_A {
        match self.bits {
            false => BIDIOE_A::OutputDisabled,
            true => BIDIOE_A::OutputEnabled,
        }
    }
    ///Checks if the value of the field is `OutputDisabled`
    #[inline(always)]
    pub fn is_output_disabled(&self) -> bool {
        *self == BIDIOE_A::OutputDisabled
    }
    ///Checks if the value of the field is `OutputEnabled`
    #[inline(always)]
    pub fn is_output_enabled(&self) -> bool {
        *self == BIDIOE_A::OutputEnabled
    }
}
///Field `BIDIOE` writer - Output enable in bidirectional mode
pub type BIDIOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, BIDIOE_A, O>;
impl<'a, const O: u8> BIDIOE_W<'a, O> {
    ///Output disabled (receive-only mode)
    #[inline(always)]
    pub fn output_disabled(self) -> &'a mut W {
        self.variant(BIDIOE_A::OutputDisabled)
    }
    ///Output enabled (transmit-only mode)
    #[inline(always)]
    pub fn output_enabled(self) -> &'a mut W {
        self.variant(BIDIOE_A::OutputEnabled)
    }
}
///Field `BIDIMODE` reader - Bidirectional data mode enable
pub type BIDIMODE_R = crate::BitReader<BIDIMODE_A>;
///Bidirectional data mode enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BIDIMODE_A {
    ///0: 2-line unidirectional data mode selected
    Unidirectional = 0,
    ///1: 1-line bidirectional data mode selected
    Bidirectional = 1,
}
impl From<BIDIMODE_A> for bool {
    #[inline(always)]
    fn from(variant: BIDIMODE_A) -> Self {
        variant as u8 != 0
    }
}
impl BIDIMODE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BIDIMODE_A {
        match self.bits {
            false => BIDIMODE_A::Unidirectional,
            true => BIDIMODE_A::Bidirectional,
        }
    }
    ///Checks if the value of the field is `Unidirectional`
    #[inline(always)]
    pub fn is_unidirectional(&self) -> bool {
        *self == BIDIMODE_A::Unidirectional
    }
    ///Checks if the value of the field is `Bidirectional`
    #[inline(always)]
    pub fn is_bidirectional(&self) -> bool {
        *self == BIDIMODE_A::Bidirectional
    }
}
///Field `BIDIMODE` writer - Bidirectional data mode enable
pub type BIDIMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, BIDIMODE_A, O>;
impl<'a, const O: u8> BIDIMODE_W<'a, O> {
    ///2-line unidirectional data mode selected
    #[inline(always)]
    pub fn unidirectional(self) -> &'a mut W {
        self.variant(BIDIMODE_A::Unidirectional)
    }
    ///1-line bidirectional data mode selected
    #[inline(always)]
    pub fn bidirectional(self) -> &'a mut W {
        self.variant(BIDIMODE_A::Bidirectional)
    }
}
impl R {
    ///Bit 0 - Clock phase
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Clock polarity
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Master selection
    #[inline(always)]
    pub fn mstr(&self) -> MSTR_R {
        MSTR_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:5 - Baud rate control
    #[inline(always)]
    pub fn br(&self) -> BR_R {
        BR_R::new(((self.bits >> 3) & 7) as u8)
    }
    ///Bit 6 - SPI enable
    #[inline(always)]
    pub fn spe(&self) -> SPE_R {
        SPE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Frame format
    #[inline(always)]
    pub fn lsbfirst(&self) -> LSBFIRST_R {
        LSBFIRST_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Internal slave select
    #[inline(always)]
    pub fn ssi(&self) -> SSI_R {
        SSI_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Software slave management
    #[inline(always)]
    pub fn ssm(&self) -> SSM_R {
        SSM_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Receive only
    #[inline(always)]
    pub fn rxonly(&self) -> RXONLY_R {
        RXONLY_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - CRC length
    #[inline(always)]
    pub fn crcl(&self) -> CRCL_R {
        CRCL_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - CRC transfer next
    #[inline(always)]
    pub fn crcnext(&self) -> CRCNEXT_R {
        CRCNEXT_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Hardware CRC calculation enable
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Output enable in bidirectional mode
    #[inline(always)]
    pub fn bidioe(&self) -> BIDIOE_R {
        BIDIOE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Bidirectional data mode enable
    #[inline(always)]
    pub fn bidimode(&self) -> BIDIMODE_R {
        BIDIMODE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Clock phase
    #[inline(always)]
    #[must_use]
    pub fn cpha(&mut self) -> CPHA_W<0> {
        CPHA_W::new(self)
    }
    ///Bit 1 - Clock polarity
    #[inline(always)]
    #[must_use]
    pub fn cpol(&mut self) -> CPOL_W<1> {
        CPOL_W::new(self)
    }
    ///Bit 2 - Master selection
    #[inline(always)]
    #[must_use]
    pub fn mstr(&mut self) -> MSTR_W<2> {
        MSTR_W::new(self)
    }
    ///Bits 3:5 - Baud rate control
    #[inline(always)]
    #[must_use]
    pub fn br(&mut self) -> BR_W<3> {
        BR_W::new(self)
    }
    ///Bit 6 - SPI enable
    #[inline(always)]
    #[must_use]
    pub fn spe(&mut self) -> SPE_W<6> {
        SPE_W::new(self)
    }
    ///Bit 7 - Frame format
    #[inline(always)]
    #[must_use]
    pub fn lsbfirst(&mut self) -> LSBFIRST_W<7> {
        LSBFIRST_W::new(self)
    }
    ///Bit 8 - Internal slave select
    #[inline(always)]
    #[must_use]
    pub fn ssi(&mut self) -> SSI_W<8> {
        SSI_W::new(self)
    }
    ///Bit 9 - Software slave management
    #[inline(always)]
    #[must_use]
    pub fn ssm(&mut self) -> SSM_W<9> {
        SSM_W::new(self)
    }
    ///Bit 10 - Receive only
    #[inline(always)]
    #[must_use]
    pub fn rxonly(&mut self) -> RXONLY_W<10> {
        RXONLY_W::new(self)
    }
    ///Bit 11 - CRC length
    #[inline(always)]
    #[must_use]
    pub fn crcl(&mut self) -> CRCL_W<11> {
        CRCL_W::new(self)
    }
    ///Bit 12 - CRC transfer next
    #[inline(always)]
    #[must_use]
    pub fn crcnext(&mut self) -> CRCNEXT_W<12> {
        CRCNEXT_W::new(self)
    }
    ///Bit 13 - Hardware CRC calculation enable
    #[inline(always)]
    #[must_use]
    pub fn crcen(&mut self) -> CRCEN_W<13> {
        CRCEN_W::new(self)
    }
    ///Bit 14 - Output enable in bidirectional mode
    #[inline(always)]
    #[must_use]
    pub fn bidioe(&mut self) -> BIDIOE_W<14> {
        BIDIOE_W::new(self)
    }
    ///Bit 15 - Bidirectional data mode enable
    #[inline(always)]
    #[must_use]
    pub fn bidimode(&mut self) -> BIDIMODE_W<15> {
        BIDIMODE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///control register 1
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
