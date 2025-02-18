///Register `CR2` reader
pub struct R(crate::R<CR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CR2` writer
pub struct W(crate::W<CR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR2_SPEC>;
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
impl From<crate::W<CR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ADDM7` reader - 7-bit Address Detection/4-bit Address Detection
pub type ADDM7_R = crate::BitReader<ADDM7_A>;
///7-bit Address Detection/4-bit Address Detection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDM7_A {
    ///0: 4-bit address detection
    Bit4 = 0,
    ///1: 7-bit address detection
    Bit7 = 1,
}
impl From<ADDM7_A> for bool {
    #[inline(always)]
    fn from(variant: ADDM7_A) -> Self {
        variant as u8 != 0
    }
}
impl ADDM7_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ADDM7_A {
        match self.bits {
            false => ADDM7_A::Bit4,
            true => ADDM7_A::Bit7,
        }
    }
    ///Checks if the value of the field is `Bit4`
    #[inline(always)]
    pub fn is_bit4(&self) -> bool {
        *self == ADDM7_A::Bit4
    }
    ///Checks if the value of the field is `Bit7`
    #[inline(always)]
    pub fn is_bit7(&self) -> bool {
        *self == ADDM7_A::Bit7
    }
}
///Field `ADDM7` writer - 7-bit Address Detection/4-bit Address Detection
pub type ADDM7_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, ADDM7_A, O>;
impl<'a, const O: u8> ADDM7_W<'a, O> {
    ///4-bit address detection
    #[inline(always)]
    pub fn bit4(self) -> &'a mut W {
        self.variant(ADDM7_A::Bit4)
    }
    ///7-bit address detection
    #[inline(always)]
    pub fn bit7(self) -> &'a mut W {
        self.variant(ADDM7_A::Bit7)
    }
}
///Field `LBDL` reader - LIN break detection length
pub type LBDL_R = crate::BitReader<LBDL_A>;
///LIN break detection length
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LBDL_A {
    ///0: 10-bit break detection
    Bit10 = 0,
    ///1: 11-bit break detection
    Bit11 = 1,
}
impl From<LBDL_A> for bool {
    #[inline(always)]
    fn from(variant: LBDL_A) -> Self {
        variant as u8 != 0
    }
}
impl LBDL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LBDL_A {
        match self.bits {
            false => LBDL_A::Bit10,
            true => LBDL_A::Bit11,
        }
    }
    ///Checks if the value of the field is `Bit10`
    #[inline(always)]
    pub fn is_bit10(&self) -> bool {
        *self == LBDL_A::Bit10
    }
    ///Checks if the value of the field is `Bit11`
    #[inline(always)]
    pub fn is_bit11(&self) -> bool {
        *self == LBDL_A::Bit11
    }
}
///Field `LBDL` writer - LIN break detection length
pub type LBDL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, LBDL_A, O>;
impl<'a, const O: u8> LBDL_W<'a, O> {
    ///10-bit break detection
    #[inline(always)]
    pub fn bit10(self) -> &'a mut W {
        self.variant(LBDL_A::Bit10)
    }
    ///11-bit break detection
    #[inline(always)]
    pub fn bit11(self) -> &'a mut W {
        self.variant(LBDL_A::Bit11)
    }
}
///Field `LBDIE` reader - LIN break detection interrupt enable
pub type LBDIE_R = crate::BitReader<LBDIE_A>;
///LIN break detection interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LBDIE_A {
    ///0: Interrupt is inhibited
    Disabled = 0,
    ///1: An interrupt is generated whenever LBDF=1 in the ISR register
    Enabled = 1,
}
impl From<LBDIE_A> for bool {
    #[inline(always)]
    fn from(variant: LBDIE_A) -> Self {
        variant as u8 != 0
    }
}
impl LBDIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LBDIE_A {
        match self.bits {
            false => LBDIE_A::Disabled,
            true => LBDIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LBDIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LBDIE_A::Enabled
    }
}
///Field `LBDIE` writer - LIN break detection interrupt enable
pub type LBDIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, LBDIE_A, O>;
impl<'a, const O: u8> LBDIE_W<'a, O> {
    ///Interrupt is inhibited
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LBDIE_A::Disabled)
    }
    ///An interrupt is generated whenever LBDF=1 in the ISR register
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LBDIE_A::Enabled)
    }
}
///Field `LBCL` reader - Last bit clock pulse
pub type LBCL_R = crate::BitReader<LBCL_A>;
///Last bit clock pulse
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LBCL_A {
    ///0: The clock pulse of the last data bit is not output to the CK pin
    NotOutput = 0,
    ///1: The clock pulse of the last data bit is output to the CK pin
    Output = 1,
}
impl From<LBCL_A> for bool {
    #[inline(always)]
    fn from(variant: LBCL_A) -> Self {
        variant as u8 != 0
    }
}
impl LBCL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LBCL_A {
        match self.bits {
            false => LBCL_A::NotOutput,
            true => LBCL_A::Output,
        }
    }
    ///Checks if the value of the field is `NotOutput`
    #[inline(always)]
    pub fn is_not_output(&self) -> bool {
        *self == LBCL_A::NotOutput
    }
    ///Checks if the value of the field is `Output`
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == LBCL_A::Output
    }
}
///Field `LBCL` writer - Last bit clock pulse
pub type LBCL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, LBCL_A, O>;
impl<'a, const O: u8> LBCL_W<'a, O> {
    ///The clock pulse of the last data bit is not output to the CK pin
    #[inline(always)]
    pub fn not_output(self) -> &'a mut W {
        self.variant(LBCL_A::NotOutput)
    }
    ///The clock pulse of the last data bit is output to the CK pin
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(LBCL_A::Output)
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
    First = 0,
    ///1: The second clock transition is the first data capture edge
    Second = 1,
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
            false => CPHA_A::First,
            true => CPHA_A::Second,
        }
    }
    ///Checks if the value of the field is `First`
    #[inline(always)]
    pub fn is_first(&self) -> bool {
        *self == CPHA_A::First
    }
    ///Checks if the value of the field is `Second`
    #[inline(always)]
    pub fn is_second(&self) -> bool {
        *self == CPHA_A::Second
    }
}
///Field `CPHA` writer - Clock phase
pub type CPHA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, CPHA_A, O>;
impl<'a, const O: u8> CPHA_W<'a, O> {
    ///The first clock transition is the first data capture edge
    #[inline(always)]
    pub fn first(self) -> &'a mut W {
        self.variant(CPHA_A::First)
    }
    ///The second clock transition is the first data capture edge
    #[inline(always)]
    pub fn second(self) -> &'a mut W {
        self.variant(CPHA_A::Second)
    }
}
///Field `CPOL` reader - Clock polarity
pub type CPOL_R = crate::BitReader<CPOL_A>;
///Clock polarity
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPOL_A {
    ///0: Steady low value on CK pin outside transmission window
    Low = 0,
    ///1: Steady high value on CK pin outside transmission window
    High = 1,
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
            false => CPOL_A::Low,
            true => CPOL_A::High,
        }
    }
    ///Checks if the value of the field is `Low`
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CPOL_A::Low
    }
    ///Checks if the value of the field is `High`
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CPOL_A::High
    }
}
///Field `CPOL` writer - Clock polarity
pub type CPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, CPOL_A, O>;
impl<'a, const O: u8> CPOL_W<'a, O> {
    ///Steady low value on CK pin outside transmission window
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(CPOL_A::Low)
    }
    ///Steady high value on CK pin outside transmission window
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(CPOL_A::High)
    }
}
///Field `CLKEN` reader - Clock enable
pub type CLKEN_R = crate::BitReader<CLKEN_A>;
///Clock enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLKEN_A {
    ///0: CK pin disabled
    Disabled = 0,
    ///1: CK pin enabled
    Enabled = 1,
}
impl From<CLKEN_A> for bool {
    #[inline(always)]
    fn from(variant: CLKEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CLKEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CLKEN_A {
        match self.bits {
            false => CLKEN_A::Disabled,
            true => CLKEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CLKEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CLKEN_A::Enabled
    }
}
///Field `CLKEN` writer - Clock enable
pub type CLKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, CLKEN_A, O>;
impl<'a, const O: u8> CLKEN_W<'a, O> {
    ///CK pin disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CLKEN_A::Disabled)
    }
    ///CK pin enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CLKEN_A::Enabled)
    }
}
///Field `STOP` reader - STOP bits
pub type STOP_R = crate::FieldReader<u8, STOP_A>;
///STOP bits
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STOP_A {
    ///0: 1 stop bit
    Stop1 = 0,
    ///1: 0.5 stop bit
    Stop0p5 = 1,
    ///2: 2 stop bit
    Stop2 = 2,
    ///3: 1.5 stop bit
    Stop1p5 = 3,
}
impl From<STOP_A> for u8 {
    #[inline(always)]
    fn from(variant: STOP_A) -> Self {
        variant as _
    }
}
impl STOP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> STOP_A {
        match self.bits {
            0 => STOP_A::Stop1,
            1 => STOP_A::Stop0p5,
            2 => STOP_A::Stop2,
            3 => STOP_A::Stop1p5,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Stop1`
    #[inline(always)]
    pub fn is_stop1(&self) -> bool {
        *self == STOP_A::Stop1
    }
    ///Checks if the value of the field is `Stop0p5`
    #[inline(always)]
    pub fn is_stop0p5(&self) -> bool {
        *self == STOP_A::Stop0p5
    }
    ///Checks if the value of the field is `Stop2`
    #[inline(always)]
    pub fn is_stop2(&self) -> bool {
        *self == STOP_A::Stop2
    }
    ///Checks if the value of the field is `Stop1p5`
    #[inline(always)]
    pub fn is_stop1p5(&self) -> bool {
        *self == STOP_A::Stop1p5
    }
}
///Field `STOP` writer - STOP bits
pub type STOP_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CR2_SPEC, u8, STOP_A, 2, O>;
impl<'a, const O: u8> STOP_W<'a, O> {
    ///1 stop bit
    #[inline(always)]
    pub fn stop1(self) -> &'a mut W {
        self.variant(STOP_A::Stop1)
    }
    ///0.5 stop bit
    #[inline(always)]
    pub fn stop0p5(self) -> &'a mut W {
        self.variant(STOP_A::Stop0p5)
    }
    ///2 stop bit
    #[inline(always)]
    pub fn stop2(self) -> &'a mut W {
        self.variant(STOP_A::Stop2)
    }
    ///1.5 stop bit
    #[inline(always)]
    pub fn stop1p5(self) -> &'a mut W {
        self.variant(STOP_A::Stop1p5)
    }
}
///Field `LINEN` reader - LIN mode enable
pub type LINEN_R = crate::BitReader<LINEN_A>;
///LIN mode enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LINEN_A {
    ///0: LIN mode disabled
    Disabled = 0,
    ///1: LIN mode enabled
    Enabled = 1,
}
impl From<LINEN_A> for bool {
    #[inline(always)]
    fn from(variant: LINEN_A) -> Self {
        variant as u8 != 0
    }
}
impl LINEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LINEN_A {
        match self.bits {
            false => LINEN_A::Disabled,
            true => LINEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LINEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LINEN_A::Enabled
    }
}
///Field `LINEN` writer - LIN mode enable
pub type LINEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, LINEN_A, O>;
impl<'a, const O: u8> LINEN_W<'a, O> {
    ///LIN mode disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LINEN_A::Disabled)
    }
    ///LIN mode enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LINEN_A::Enabled)
    }
}
///Field `SWAP` reader - Swap TX/RX pins
pub type SWAP_R = crate::BitReader<SWAP_A>;
///Swap TX/RX pins
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWAP_A {
    ///0: TX/RX pins are used as defined in standard pinout
    Standard = 0,
    ///1: The TX and RX pins functions are swapped
    Swapped = 1,
}
impl From<SWAP_A> for bool {
    #[inline(always)]
    fn from(variant: SWAP_A) -> Self {
        variant as u8 != 0
    }
}
impl SWAP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SWAP_A {
        match self.bits {
            false => SWAP_A::Standard,
            true => SWAP_A::Swapped,
        }
    }
    ///Checks if the value of the field is `Standard`
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == SWAP_A::Standard
    }
    ///Checks if the value of the field is `Swapped`
    #[inline(always)]
    pub fn is_swapped(&self) -> bool {
        *self == SWAP_A::Swapped
    }
}
///Field `SWAP` writer - Swap TX/RX pins
pub type SWAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, SWAP_A, O>;
impl<'a, const O: u8> SWAP_W<'a, O> {
    ///TX/RX pins are used as defined in standard pinout
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(SWAP_A::Standard)
    }
    ///The TX and RX pins functions are swapped
    #[inline(always)]
    pub fn swapped(self) -> &'a mut W {
        self.variant(SWAP_A::Swapped)
    }
}
///Field `RXINV` reader - RX pin active level inversion
pub type RXINV_R = crate::BitReader<RXINV_A>;
///RX pin active level inversion
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXINV_A {
    ///0: RX pin signal works using the standard logic levels
    Standard = 0,
    ///1: RX pin signal values are inverted
    Inverted = 1,
}
impl From<RXINV_A> for bool {
    #[inline(always)]
    fn from(variant: RXINV_A) -> Self {
        variant as u8 != 0
    }
}
impl RXINV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RXINV_A {
        match self.bits {
            false => RXINV_A::Standard,
            true => RXINV_A::Inverted,
        }
    }
    ///Checks if the value of the field is `Standard`
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == RXINV_A::Standard
    }
    ///Checks if the value of the field is `Inverted`
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == RXINV_A::Inverted
    }
}
///Field `RXINV` writer - RX pin active level inversion
pub type RXINV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, RXINV_A, O>;
impl<'a, const O: u8> RXINV_W<'a, O> {
    ///RX pin signal works using the standard logic levels
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(RXINV_A::Standard)
    }
    ///RX pin signal values are inverted
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(RXINV_A::Inverted)
    }
}
///Field `TXINV` reader - TX pin active level inversion
pub type TXINV_R = crate::BitReader<TXINV_A>;
///TX pin active level inversion
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXINV_A {
    ///0: TX pin signal works using the standard logic levels
    Standard = 0,
    ///1: TX pin signal values are inverted
    Inverted = 1,
}
impl From<TXINV_A> for bool {
    #[inline(always)]
    fn from(variant: TXINV_A) -> Self {
        variant as u8 != 0
    }
}
impl TXINV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TXINV_A {
        match self.bits {
            false => TXINV_A::Standard,
            true => TXINV_A::Inverted,
        }
    }
    ///Checks if the value of the field is `Standard`
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == TXINV_A::Standard
    }
    ///Checks if the value of the field is `Inverted`
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == TXINV_A::Inverted
    }
}
///Field `TXINV` writer - TX pin active level inversion
pub type TXINV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, TXINV_A, O>;
impl<'a, const O: u8> TXINV_W<'a, O> {
    ///TX pin signal works using the standard logic levels
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(TXINV_A::Standard)
    }
    ///TX pin signal values are inverted
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(TXINV_A::Inverted)
    }
}
///Field `DATAINV` reader - Binary data inversion
pub type DATAINV_R = crate::BitReader<DATAINV_A>;
///Binary data inversion
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DATAINV_A {
    ///0: Logical data from the data register are send/received in positive/direct logic
    Positive = 0,
    ///1: Logical data from the data register are send/received in negative/inverse logic
    Negative = 1,
}
impl From<DATAINV_A> for bool {
    #[inline(always)]
    fn from(variant: DATAINV_A) -> Self {
        variant as u8 != 0
    }
}
impl DATAINV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DATAINV_A {
        match self.bits {
            false => DATAINV_A::Positive,
            true => DATAINV_A::Negative,
        }
    }
    ///Checks if the value of the field is `Positive`
    #[inline(always)]
    pub fn is_positive(&self) -> bool {
        *self == DATAINV_A::Positive
    }
    ///Checks if the value of the field is `Negative`
    #[inline(always)]
    pub fn is_negative(&self) -> bool {
        *self == DATAINV_A::Negative
    }
}
///Field `DATAINV` writer - Binary data inversion
pub type DATAINV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, DATAINV_A, O>;
impl<'a, const O: u8> DATAINV_W<'a, O> {
    ///Logical data from the data register are send/received in positive/direct logic
    #[inline(always)]
    pub fn positive(self) -> &'a mut W {
        self.variant(DATAINV_A::Positive)
    }
    ///Logical data from the data register are send/received in negative/inverse logic
    #[inline(always)]
    pub fn negative(self) -> &'a mut W {
        self.variant(DATAINV_A::Negative)
    }
}
///Field `MSBFIRST` reader - Most significant bit first
pub type MSBFIRST_R = crate::BitReader<MSBFIRST_A>;
///Most significant bit first
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSBFIRST_A {
    ///0: data is transmitted/received with data bit 0 first, following the start bit
    Lsb = 0,
    ///1: data is transmitted/received with MSB (bit 7/8/9) first, following the start bit
    Msb = 1,
}
impl From<MSBFIRST_A> for bool {
    #[inline(always)]
    fn from(variant: MSBFIRST_A) -> Self {
        variant as u8 != 0
    }
}
impl MSBFIRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MSBFIRST_A {
        match self.bits {
            false => MSBFIRST_A::Lsb,
            true => MSBFIRST_A::Msb,
        }
    }
    ///Checks if the value of the field is `Lsb`
    #[inline(always)]
    pub fn is_lsb(&self) -> bool {
        *self == MSBFIRST_A::Lsb
    }
    ///Checks if the value of the field is `Msb`
    #[inline(always)]
    pub fn is_msb(&self) -> bool {
        *self == MSBFIRST_A::Msb
    }
}
///Field `MSBFIRST` writer - Most significant bit first
pub type MSBFIRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, MSBFIRST_A, O>;
impl<'a, const O: u8> MSBFIRST_W<'a, O> {
    ///data is transmitted/received with data bit 0 first, following the start bit
    #[inline(always)]
    pub fn lsb(self) -> &'a mut W {
        self.variant(MSBFIRST_A::Lsb)
    }
    ///data is transmitted/received with MSB (bit 7/8/9) first, following the start bit
    #[inline(always)]
    pub fn msb(self) -> &'a mut W {
        self.variant(MSBFIRST_A::Msb)
    }
}
///Field `ABREN` reader - Auto baud rate enable
pub type ABREN_R = crate::BitReader<ABREN_A>;
///Auto baud rate enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ABREN_A {
    ///0: Auto baud rate detection is disabled
    Disabled = 0,
    ///1: Auto baud rate detection is enabled
    Enabled = 1,
}
impl From<ABREN_A> for bool {
    #[inline(always)]
    fn from(variant: ABREN_A) -> Self {
        variant as u8 != 0
    }
}
impl ABREN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ABREN_A {
        match self.bits {
            false => ABREN_A::Disabled,
            true => ABREN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ABREN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ABREN_A::Enabled
    }
}
///Field `ABREN` writer - Auto baud rate enable
pub type ABREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, ABREN_A, O>;
impl<'a, const O: u8> ABREN_W<'a, O> {
    ///Auto baud rate detection is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ABREN_A::Disabled)
    }
    ///Auto baud rate detection is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ABREN_A::Enabled)
    }
}
///Field `ABRMOD` reader - Auto baud rate mode
pub type ABRMOD_R = crate::FieldReader<u8, ABRMOD_A>;
///Auto baud rate mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ABRMOD_A {
    ///0: Measurement of the start bit is used to detect the baud rate
    Start = 0,
    ///1: Falling edge to falling edge measurement
    Edge = 1,
    ///2: 0x7F frame detection
    Frame7f = 2,
    ///3: 0x55 frame detection
    Frame55 = 3,
}
impl From<ABRMOD_A> for u8 {
    #[inline(always)]
    fn from(variant: ABRMOD_A) -> Self {
        variant as _
    }
}
impl ABRMOD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ABRMOD_A {
        match self.bits {
            0 => ABRMOD_A::Start,
            1 => ABRMOD_A::Edge,
            2 => ABRMOD_A::Frame7f,
            3 => ABRMOD_A::Frame55,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Start`
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == ABRMOD_A::Start
    }
    ///Checks if the value of the field is `Edge`
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == ABRMOD_A::Edge
    }
    ///Checks if the value of the field is `Frame7f`
    #[inline(always)]
    pub fn is_frame7f(&self) -> bool {
        *self == ABRMOD_A::Frame7f
    }
    ///Checks if the value of the field is `Frame55`
    #[inline(always)]
    pub fn is_frame55(&self) -> bool {
        *self == ABRMOD_A::Frame55
    }
}
///Field `ABRMOD` writer - Auto baud rate mode
pub type ABRMOD_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CR2_SPEC, u8, ABRMOD_A, 2, O>;
impl<'a, const O: u8> ABRMOD_W<'a, O> {
    ///Measurement of the start bit is used to detect the baud rate
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(ABRMOD_A::Start)
    }
    ///Falling edge to falling edge measurement
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(ABRMOD_A::Edge)
    }
    ///0x7F frame detection
    #[inline(always)]
    pub fn frame7f(self) -> &'a mut W {
        self.variant(ABRMOD_A::Frame7f)
    }
    ///0x55 frame detection
    #[inline(always)]
    pub fn frame55(self) -> &'a mut W {
        self.variant(ABRMOD_A::Frame55)
    }
}
///Field `RTOEN` reader - Receiver timeout enable
pub type RTOEN_R = crate::BitReader<RTOEN_A>;
///Receiver timeout enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTOEN_A {
    ///0: Receiver timeout feature disabled
    Disabled = 0,
    ///1: Receiver timeout feature enabled
    Enabled = 1,
}
impl From<RTOEN_A> for bool {
    #[inline(always)]
    fn from(variant: RTOEN_A) -> Self {
        variant as u8 != 0
    }
}
impl RTOEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RTOEN_A {
        match self.bits {
            false => RTOEN_A::Disabled,
            true => RTOEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RTOEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RTOEN_A::Enabled
    }
}
///Field `RTOEN` writer - Receiver timeout enable
pub type RTOEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, RTOEN_A, O>;
impl<'a, const O: u8> RTOEN_W<'a, O> {
    ///Receiver timeout feature disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RTOEN_A::Disabled)
    }
    ///Receiver timeout feature enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RTOEN_A::Enabled)
    }
}
///Field `ADD` reader - Address of the USART node
pub type ADD_R = crate::FieldReader<u8, u8>;
///Field `ADD` writer - Address of the USART node
pub type ADD_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CR2_SPEC, u8, u8, 8, O>;
impl R {
    ///Bit 4 - 7-bit Address Detection/4-bit Address Detection
    #[inline(always)]
    pub fn addm7(&self) -> ADDM7_R {
        ADDM7_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - LIN break detection length
    #[inline(always)]
    pub fn lbdl(&self) -> LBDL_R {
        LBDL_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - LIN break detection interrupt enable
    #[inline(always)]
    pub fn lbdie(&self) -> LBDIE_R {
        LBDIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - Last bit clock pulse
    #[inline(always)]
    pub fn lbcl(&self) -> LBCL_R {
        LBCL_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Clock phase
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Clock polarity
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Clock enable
    #[inline(always)]
    pub fn clken(&self) -> CLKEN_R {
        CLKEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:13 - STOP bits
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bit 14 - LIN mode enable
    #[inline(always)]
    pub fn linen(&self) -> LINEN_R {
        LINEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Swap TX/RX pins
    #[inline(always)]
    pub fn swap(&self) -> SWAP_R {
        SWAP_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - RX pin active level inversion
    #[inline(always)]
    pub fn rxinv(&self) -> RXINV_R {
        RXINV_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - TX pin active level inversion
    #[inline(always)]
    pub fn txinv(&self) -> TXINV_R {
        TXINV_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Binary data inversion
    #[inline(always)]
    pub fn datainv(&self) -> DATAINV_R {
        DATAINV_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Most significant bit first
    #[inline(always)]
    pub fn msbfirst(&self) -> MSBFIRST_R {
        MSBFIRST_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Auto baud rate enable
    #[inline(always)]
    pub fn abren(&self) -> ABREN_R {
        ABREN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bits 21:22 - Auto baud rate mode
    #[inline(always)]
    pub fn abrmod(&self) -> ABRMOD_R {
        ABRMOD_R::new(((self.bits >> 21) & 3) as u8)
    }
    ///Bit 23 - Receiver timeout enable
    #[inline(always)]
    pub fn rtoen(&self) -> RTOEN_R {
        RTOEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bits 24:31 - Address of the USART node
    #[inline(always)]
    pub fn add(&self) -> ADD_R {
        ADD_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    ///Bit 4 - 7-bit Address Detection/4-bit Address Detection
    #[inline(always)]
    #[must_use]
    pub fn addm7(&mut self) -> ADDM7_W<4> {
        ADDM7_W::new(self)
    }
    ///Bit 5 - LIN break detection length
    #[inline(always)]
    #[must_use]
    pub fn lbdl(&mut self) -> LBDL_W<5> {
        LBDL_W::new(self)
    }
    ///Bit 6 - LIN break detection interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn lbdie(&mut self) -> LBDIE_W<6> {
        LBDIE_W::new(self)
    }
    ///Bit 8 - Last bit clock pulse
    #[inline(always)]
    #[must_use]
    pub fn lbcl(&mut self) -> LBCL_W<8> {
        LBCL_W::new(self)
    }
    ///Bit 9 - Clock phase
    #[inline(always)]
    #[must_use]
    pub fn cpha(&mut self) -> CPHA_W<9> {
        CPHA_W::new(self)
    }
    ///Bit 10 - Clock polarity
    #[inline(always)]
    #[must_use]
    pub fn cpol(&mut self) -> CPOL_W<10> {
        CPOL_W::new(self)
    }
    ///Bit 11 - Clock enable
    #[inline(always)]
    #[must_use]
    pub fn clken(&mut self) -> CLKEN_W<11> {
        CLKEN_W::new(self)
    }
    ///Bits 12:13 - STOP bits
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> STOP_W<12> {
        STOP_W::new(self)
    }
    ///Bit 14 - LIN mode enable
    #[inline(always)]
    #[must_use]
    pub fn linen(&mut self) -> LINEN_W<14> {
        LINEN_W::new(self)
    }
    ///Bit 15 - Swap TX/RX pins
    #[inline(always)]
    #[must_use]
    pub fn swap(&mut self) -> SWAP_W<15> {
        SWAP_W::new(self)
    }
    ///Bit 16 - RX pin active level inversion
    #[inline(always)]
    #[must_use]
    pub fn rxinv(&mut self) -> RXINV_W<16> {
        RXINV_W::new(self)
    }
    ///Bit 17 - TX pin active level inversion
    #[inline(always)]
    #[must_use]
    pub fn txinv(&mut self) -> TXINV_W<17> {
        TXINV_W::new(self)
    }
    ///Bit 18 - Binary data inversion
    #[inline(always)]
    #[must_use]
    pub fn datainv(&mut self) -> DATAINV_W<18> {
        DATAINV_W::new(self)
    }
    ///Bit 19 - Most significant bit first
    #[inline(always)]
    #[must_use]
    pub fn msbfirst(&mut self) -> MSBFIRST_W<19> {
        MSBFIRST_W::new(self)
    }
    ///Bit 20 - Auto baud rate enable
    #[inline(always)]
    #[must_use]
    pub fn abren(&mut self) -> ABREN_W<20> {
        ABREN_W::new(self)
    }
    ///Bits 21:22 - Auto baud rate mode
    #[inline(always)]
    #[must_use]
    pub fn abrmod(&mut self) -> ABRMOD_W<21> {
        ABRMOD_W::new(self)
    }
    ///Bit 23 - Receiver timeout enable
    #[inline(always)]
    #[must_use]
    pub fn rtoen(&mut self) -> RTOEN_W<23> {
        RTOEN_W::new(self)
    }
    ///Bits 24:31 - Address of the USART node
    #[inline(always)]
    #[must_use]
    pub fn add(&mut self) -> ADD_W<24> {
        ADD_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Control register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr2](index.html) module
pub struct CR2_SPEC;
impl crate::RegisterSpec for CR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [cr2::R](R) reader structure
impl crate::Readable for CR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cr2::W](W) writer structure
impl crate::Writable for CR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CR2 to value 0
impl crate::Resettable for CR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
