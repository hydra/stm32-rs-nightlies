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
///Field `TXIE` reader - TX Interrupt enable
pub type TXIE_R = crate::BitReader<TXIE_A>;
///TX Interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXIE_A {
    ///0: Transmit (TXIS) interrupt disabled
    Disabled = 0,
    ///1: Transmit (TXIS) interrupt enabled
    Enabled = 1,
}
impl From<TXIE_A> for bool {
    #[inline(always)]
    fn from(variant: TXIE_A) -> Self {
        variant as u8 != 0
    }
}
impl TXIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TXIE_A {
        match self.bits {
            false => TXIE_A::Disabled,
            true => TXIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXIE_A::Enabled
    }
}
///Field `TXIE` writer - TX Interrupt enable
pub type TXIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, TXIE_A, O>;
impl<'a, const O: u8> TXIE_W<'a, O> {
    ///Transmit (TXIS) interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TXIE_A::Disabled)
    }
    ///Transmit (TXIS) interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TXIE_A::Enabled)
    }
}
///Field `RXIE` reader - RX Interrupt enable
pub type RXIE_R = crate::BitReader<RXIE_A>;
///RX Interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXIE_A {
    ///0: Receive (RXNE) interrupt disabled
    Disabled = 0,
    ///1: Receive (RXNE) interrupt enabled
    Enabled = 1,
}
impl From<RXIE_A> for bool {
    #[inline(always)]
    fn from(variant: RXIE_A) -> Self {
        variant as u8 != 0
    }
}
impl RXIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RXIE_A {
        match self.bits {
            false => RXIE_A::Disabled,
            true => RXIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXIE_A::Enabled
    }
}
///Field `RXIE` writer - RX Interrupt enable
pub type RXIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, RXIE_A, O>;
impl<'a, const O: u8> RXIE_W<'a, O> {
    ///Receive (RXNE) interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RXIE_A::Disabled)
    }
    ///Receive (RXNE) interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RXIE_A::Enabled)
    }
}
///Field `ADDRIE` reader - Address match interrupt enable (slave only)
pub type ADDRIE_R = crate::BitReader<ADDRIE_A>;
///Address match interrupt enable (slave only)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDRIE_A {
    ///0: Address match (ADDR) interrupts disabled
    Disabled = 0,
    ///1: Address match (ADDR) interrupts enabled
    Enabled = 1,
}
impl From<ADDRIE_A> for bool {
    #[inline(always)]
    fn from(variant: ADDRIE_A) -> Self {
        variant as u8 != 0
    }
}
impl ADDRIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ADDRIE_A {
        match self.bits {
            false => ADDRIE_A::Disabled,
            true => ADDRIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADDRIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADDRIE_A::Enabled
    }
}
///Field `ADDRIE` writer - Address match interrupt enable (slave only)
pub type ADDRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, ADDRIE_A, O>;
impl<'a, const O: u8> ADDRIE_W<'a, O> {
    ///Address match (ADDR) interrupts disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADDRIE_A::Disabled)
    }
    ///Address match (ADDR) interrupts enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADDRIE_A::Enabled)
    }
}
///Field `NACKIE` reader - Not acknowledge received interrupt enable
pub type NACKIE_R = crate::BitReader<NACKIE_A>;
///Not acknowledge received interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NACKIE_A {
    ///0: Not acknowledge (NACKF) received interrupts disabled
    Disabled = 0,
    ///1: Not acknowledge (NACKF) received interrupts enabled
    Enabled = 1,
}
impl From<NACKIE_A> for bool {
    #[inline(always)]
    fn from(variant: NACKIE_A) -> Self {
        variant as u8 != 0
    }
}
impl NACKIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> NACKIE_A {
        match self.bits {
            false => NACKIE_A::Disabled,
            true => NACKIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == NACKIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == NACKIE_A::Enabled
    }
}
///Field `NACKIE` writer - Not acknowledge received interrupt enable
pub type NACKIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, NACKIE_A, O>;
impl<'a, const O: u8> NACKIE_W<'a, O> {
    ///Not acknowledge (NACKF) received interrupts disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(NACKIE_A::Disabled)
    }
    ///Not acknowledge (NACKF) received interrupts enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(NACKIE_A::Enabled)
    }
}
///Field `STOPIE` reader - STOP detection Interrupt enable
pub type STOPIE_R = crate::BitReader<STOPIE_A>;
///STOP detection Interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOPIE_A {
    ///0: Stop detection (STOPF) interrupt disabled
    Disabled = 0,
    ///1: Stop detection (STOPF) interrupt enabled
    Enabled = 1,
}
impl From<STOPIE_A> for bool {
    #[inline(always)]
    fn from(variant: STOPIE_A) -> Self {
        variant as u8 != 0
    }
}
impl STOPIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> STOPIE_A {
        match self.bits {
            false => STOPIE_A::Disabled,
            true => STOPIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == STOPIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == STOPIE_A::Enabled
    }
}
///Field `STOPIE` writer - STOP detection Interrupt enable
pub type STOPIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, STOPIE_A, O>;
impl<'a, const O: u8> STOPIE_W<'a, O> {
    ///Stop detection (STOPF) interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(STOPIE_A::Disabled)
    }
    ///Stop detection (STOPF) interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(STOPIE_A::Enabled)
    }
}
///Field `TCIE` reader - Transfer Complete interrupt enable
pub type TCIE_R = crate::BitReader<TCIE_A>;
///Transfer Complete interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCIE_A {
    ///0: Transfer Complete interrupt disabled
    Disabled = 0,
    ///1: Transfer Complete interrupt enabled
    Enabled = 1,
}
impl From<TCIE_A> for bool {
    #[inline(always)]
    fn from(variant: TCIE_A) -> Self {
        variant as u8 != 0
    }
}
impl TCIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TCIE_A {
        match self.bits {
            false => TCIE_A::Disabled,
            true => TCIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TCIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TCIE_A::Enabled
    }
}
///Field `TCIE` writer - Transfer Complete interrupt enable
pub type TCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, TCIE_A, O>;
impl<'a, const O: u8> TCIE_W<'a, O> {
    ///Transfer Complete interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TCIE_A::Disabled)
    }
    ///Transfer Complete interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TCIE_A::Enabled)
    }
}
///Field `ERRIE` reader - Error interrupts enable
pub type ERRIE_R = crate::BitReader<ERRIE_A>;
///Error interrupts enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERRIE_A {
    ///0: Error detection interrupts disabled
    Disabled = 0,
    ///1: Error detection interrupts enabled
    Enabled = 1,
}
impl From<ERRIE_A> for bool {
    #[inline(always)]
    fn from(variant: ERRIE_A) -> Self {
        variant as u8 != 0
    }
}
impl ERRIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ERRIE_A {
        match self.bits {
            false => ERRIE_A::Disabled,
            true => ERRIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ERRIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ERRIE_A::Enabled
    }
}
///Field `ERRIE` writer - Error interrupts enable
pub type ERRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, ERRIE_A, O>;
impl<'a, const O: u8> ERRIE_W<'a, O> {
    ///Error detection interrupts disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ERRIE_A::Disabled)
    }
    ///Error detection interrupts enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ERRIE_A::Enabled)
    }
}
///Field `DNF` reader - Digital noise filter
pub type DNF_R = crate::FieldReader<u8, DNF_A>;
///Digital noise filter
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DNF_A {
    ///0: Digital filter disabled
    NoFilter = 0,
    ///1: Digital filter enabled and filtering capability up to 1 tI2CCLK
    Filter1 = 1,
    ///2: Digital filter enabled and filtering capability up to 2 tI2CCLK
    Filter2 = 2,
    ///3: Digital filter enabled and filtering capability up to 3 tI2CCLK
    Filter3 = 3,
    ///4: Digital filter enabled and filtering capability up to 4 tI2CCLK
    Filter4 = 4,
    ///5: Digital filter enabled and filtering capability up to 5 tI2CCLK
    Filter5 = 5,
    ///6: Digital filter enabled and filtering capability up to 6 tI2CCLK
    Filter6 = 6,
    ///7: Digital filter enabled and filtering capability up to 7 tI2CCLK
    Filter7 = 7,
    ///8: Digital filter enabled and filtering capability up to 8 tI2CCLK
    Filter8 = 8,
    ///9: Digital filter enabled and filtering capability up to 9 tI2CCLK
    Filter9 = 9,
    ///10: Digital filter enabled and filtering capability up to 10 tI2CCLK
    Filter10 = 10,
    ///11: Digital filter enabled and filtering capability up to 11 tI2CCLK
    Filter11 = 11,
    ///12: Digital filter enabled and filtering capability up to 12 tI2CCLK
    Filter12 = 12,
    ///13: Digital filter enabled and filtering capability up to 13 tI2CCLK
    Filter13 = 13,
    ///14: Digital filter enabled and filtering capability up to 14 tI2CCLK
    Filter14 = 14,
    ///15: Digital filter enabled and filtering capability up to 15 tI2CCLK
    Filter15 = 15,
}
impl From<DNF_A> for u8 {
    #[inline(always)]
    fn from(variant: DNF_A) -> Self {
        variant as _
    }
}
impl DNF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DNF_A {
        match self.bits {
            0 => DNF_A::NoFilter,
            1 => DNF_A::Filter1,
            2 => DNF_A::Filter2,
            3 => DNF_A::Filter3,
            4 => DNF_A::Filter4,
            5 => DNF_A::Filter5,
            6 => DNF_A::Filter6,
            7 => DNF_A::Filter7,
            8 => DNF_A::Filter8,
            9 => DNF_A::Filter9,
            10 => DNF_A::Filter10,
            11 => DNF_A::Filter11,
            12 => DNF_A::Filter12,
            13 => DNF_A::Filter13,
            14 => DNF_A::Filter14,
            15 => DNF_A::Filter15,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `NoFilter`
    #[inline(always)]
    pub fn is_no_filter(&self) -> bool {
        *self == DNF_A::NoFilter
    }
    ///Checks if the value of the field is `Filter1`
    #[inline(always)]
    pub fn is_filter1(&self) -> bool {
        *self == DNF_A::Filter1
    }
    ///Checks if the value of the field is `Filter2`
    #[inline(always)]
    pub fn is_filter2(&self) -> bool {
        *self == DNF_A::Filter2
    }
    ///Checks if the value of the field is `Filter3`
    #[inline(always)]
    pub fn is_filter3(&self) -> bool {
        *self == DNF_A::Filter3
    }
    ///Checks if the value of the field is `Filter4`
    #[inline(always)]
    pub fn is_filter4(&self) -> bool {
        *self == DNF_A::Filter4
    }
    ///Checks if the value of the field is `Filter5`
    #[inline(always)]
    pub fn is_filter5(&self) -> bool {
        *self == DNF_A::Filter5
    }
    ///Checks if the value of the field is `Filter6`
    #[inline(always)]
    pub fn is_filter6(&self) -> bool {
        *self == DNF_A::Filter6
    }
    ///Checks if the value of the field is `Filter7`
    #[inline(always)]
    pub fn is_filter7(&self) -> bool {
        *self == DNF_A::Filter7
    }
    ///Checks if the value of the field is `Filter8`
    #[inline(always)]
    pub fn is_filter8(&self) -> bool {
        *self == DNF_A::Filter8
    }
    ///Checks if the value of the field is `Filter9`
    #[inline(always)]
    pub fn is_filter9(&self) -> bool {
        *self == DNF_A::Filter9
    }
    ///Checks if the value of the field is `Filter10`
    #[inline(always)]
    pub fn is_filter10(&self) -> bool {
        *self == DNF_A::Filter10
    }
    ///Checks if the value of the field is `Filter11`
    #[inline(always)]
    pub fn is_filter11(&self) -> bool {
        *self == DNF_A::Filter11
    }
    ///Checks if the value of the field is `Filter12`
    #[inline(always)]
    pub fn is_filter12(&self) -> bool {
        *self == DNF_A::Filter12
    }
    ///Checks if the value of the field is `Filter13`
    #[inline(always)]
    pub fn is_filter13(&self) -> bool {
        *self == DNF_A::Filter13
    }
    ///Checks if the value of the field is `Filter14`
    #[inline(always)]
    pub fn is_filter14(&self) -> bool {
        *self == DNF_A::Filter14
    }
    ///Checks if the value of the field is `Filter15`
    #[inline(always)]
    pub fn is_filter15(&self) -> bool {
        *self == DNF_A::Filter15
    }
}
///Field `DNF` writer - Digital noise filter
pub type DNF_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CR1_SPEC, u8, DNF_A, 4, O>;
impl<'a, const O: u8> DNF_W<'a, O> {
    ///Digital filter disabled
    #[inline(always)]
    pub fn no_filter(self) -> &'a mut W {
        self.variant(DNF_A::NoFilter)
    }
    ///Digital filter enabled and filtering capability up to 1 tI2CCLK
    #[inline(always)]
    pub fn filter1(self) -> &'a mut W {
        self.variant(DNF_A::Filter1)
    }
    ///Digital filter enabled and filtering capability up to 2 tI2CCLK
    #[inline(always)]
    pub fn filter2(self) -> &'a mut W {
        self.variant(DNF_A::Filter2)
    }
    ///Digital filter enabled and filtering capability up to 3 tI2CCLK
    #[inline(always)]
    pub fn filter3(self) -> &'a mut W {
        self.variant(DNF_A::Filter3)
    }
    ///Digital filter enabled and filtering capability up to 4 tI2CCLK
    #[inline(always)]
    pub fn filter4(self) -> &'a mut W {
        self.variant(DNF_A::Filter4)
    }
    ///Digital filter enabled and filtering capability up to 5 tI2CCLK
    #[inline(always)]
    pub fn filter5(self) -> &'a mut W {
        self.variant(DNF_A::Filter5)
    }
    ///Digital filter enabled and filtering capability up to 6 tI2CCLK
    #[inline(always)]
    pub fn filter6(self) -> &'a mut W {
        self.variant(DNF_A::Filter6)
    }
    ///Digital filter enabled and filtering capability up to 7 tI2CCLK
    #[inline(always)]
    pub fn filter7(self) -> &'a mut W {
        self.variant(DNF_A::Filter7)
    }
    ///Digital filter enabled and filtering capability up to 8 tI2CCLK
    #[inline(always)]
    pub fn filter8(self) -> &'a mut W {
        self.variant(DNF_A::Filter8)
    }
    ///Digital filter enabled and filtering capability up to 9 tI2CCLK
    #[inline(always)]
    pub fn filter9(self) -> &'a mut W {
        self.variant(DNF_A::Filter9)
    }
    ///Digital filter enabled and filtering capability up to 10 tI2CCLK
    #[inline(always)]
    pub fn filter10(self) -> &'a mut W {
        self.variant(DNF_A::Filter10)
    }
    ///Digital filter enabled and filtering capability up to 11 tI2CCLK
    #[inline(always)]
    pub fn filter11(self) -> &'a mut W {
        self.variant(DNF_A::Filter11)
    }
    ///Digital filter enabled and filtering capability up to 12 tI2CCLK
    #[inline(always)]
    pub fn filter12(self) -> &'a mut W {
        self.variant(DNF_A::Filter12)
    }
    ///Digital filter enabled and filtering capability up to 13 tI2CCLK
    #[inline(always)]
    pub fn filter13(self) -> &'a mut W {
        self.variant(DNF_A::Filter13)
    }
    ///Digital filter enabled and filtering capability up to 14 tI2CCLK
    #[inline(always)]
    pub fn filter14(self) -> &'a mut W {
        self.variant(DNF_A::Filter14)
    }
    ///Digital filter enabled and filtering capability up to 15 tI2CCLK
    #[inline(always)]
    pub fn filter15(self) -> &'a mut W {
        self.variant(DNF_A::Filter15)
    }
}
///Field `ANFOFF` reader - Analog noise filter OFF
pub type ANFOFF_R = crate::BitReader<ANFOFF_A>;
///Analog noise filter OFF
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANFOFF_A {
    ///0: Analog noise filter enabled
    Enabled = 0,
    ///1: Analog noise filter disabled
    Disabled = 1,
}
impl From<ANFOFF_A> for bool {
    #[inline(always)]
    fn from(variant: ANFOFF_A) -> Self {
        variant as u8 != 0
    }
}
impl ANFOFF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ANFOFF_A {
        match self.bits {
            false => ANFOFF_A::Enabled,
            true => ANFOFF_A::Disabled,
        }
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ANFOFF_A::Enabled
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ANFOFF_A::Disabled
    }
}
///Field `ANFOFF` writer - Analog noise filter OFF
pub type ANFOFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, ANFOFF_A, O>;
impl<'a, const O: u8> ANFOFF_W<'a, O> {
    ///Analog noise filter enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ANFOFF_A::Enabled)
    }
    ///Analog noise filter disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ANFOFF_A::Disabled)
    }
}
///Field `TXDMAEN` reader - DMA transmission requests enable
pub type TXDMAEN_R = crate::BitReader<TXDMAEN_A>;
///DMA transmission requests enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXDMAEN_A {
    ///0: DMA mode disabled for transmission
    Disabled = 0,
    ///1: DMA mode enabled for transmission
    Enabled = 1,
}
impl From<TXDMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: TXDMAEN_A) -> Self {
        variant as u8 != 0
    }
}
impl TXDMAEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TXDMAEN_A {
        match self.bits {
            false => TXDMAEN_A::Disabled,
            true => TXDMAEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXDMAEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXDMAEN_A::Enabled
    }
}
///Field `TXDMAEN` writer - DMA transmission requests enable
pub type TXDMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, TXDMAEN_A, O>;
impl<'a, const O: u8> TXDMAEN_W<'a, O> {
    ///DMA mode disabled for transmission
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TXDMAEN_A::Disabled)
    }
    ///DMA mode enabled for transmission
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TXDMAEN_A::Enabled)
    }
}
///Field `RXDMAEN` reader - DMA reception requests enable
pub type RXDMAEN_R = crate::BitReader<RXDMAEN_A>;
///DMA reception requests enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXDMAEN_A {
    ///0: DMA mode disabled for reception
    Disabled = 0,
    ///1: DMA mode enabled for reception
    Enabled = 1,
}
impl From<RXDMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: RXDMAEN_A) -> Self {
        variant as u8 != 0
    }
}
impl RXDMAEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RXDMAEN_A {
        match self.bits {
            false => RXDMAEN_A::Disabled,
            true => RXDMAEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXDMAEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXDMAEN_A::Enabled
    }
}
///Field `RXDMAEN` writer - DMA reception requests enable
pub type RXDMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, RXDMAEN_A, O>;
impl<'a, const O: u8> RXDMAEN_W<'a, O> {
    ///DMA mode disabled for reception
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RXDMAEN_A::Disabled)
    }
    ///DMA mode enabled for reception
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RXDMAEN_A::Enabled)
    }
}
///Field `SBC` reader - Slave byte control
pub type SBC_R = crate::BitReader<SBC_A>;
///Slave byte control
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBC_A {
    ///0: Slave byte control disabled
    Disabled = 0,
    ///1: Slave byte control enabled
    Enabled = 1,
}
impl From<SBC_A> for bool {
    #[inline(always)]
    fn from(variant: SBC_A) -> Self {
        variant as u8 != 0
    }
}
impl SBC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SBC_A {
        match self.bits {
            false => SBC_A::Disabled,
            true => SBC_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SBC_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SBC_A::Enabled
    }
}
///Field `SBC` writer - Slave byte control
pub type SBC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, SBC_A, O>;
impl<'a, const O: u8> SBC_W<'a, O> {
    ///Slave byte control disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SBC_A::Disabled)
    }
    ///Slave byte control enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SBC_A::Enabled)
    }
}
///Field `NOSTRETCH` reader - Clock stretching disable
pub type NOSTRETCH_R = crate::BitReader<NOSTRETCH_A>;
///Clock stretching disable
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
///Field `NOSTRETCH` writer - Clock stretching disable
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
///Field `WUPEN` reader - Wakeup from STOP enable
pub type WUPEN_R = crate::BitReader<WUPEN_A>;
///Wakeup from STOP enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUPEN_A {
    ///0: Wakeup from Stop mode disabled
    Disabled = 0,
    ///1: Wakeup from Stop mode enabled
    Enabled = 1,
}
impl From<WUPEN_A> for bool {
    #[inline(always)]
    fn from(variant: WUPEN_A) -> Self {
        variant as u8 != 0
    }
}
impl WUPEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> WUPEN_A {
        match self.bits {
            false => WUPEN_A::Disabled,
            true => WUPEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WUPEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WUPEN_A::Enabled
    }
}
///Field `WUPEN` writer - Wakeup from STOP enable
pub type WUPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, WUPEN_A, O>;
impl<'a, const O: u8> WUPEN_W<'a, O> {
    ///Wakeup from Stop mode disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WUPEN_A::Disabled)
    }
    ///Wakeup from Stop mode enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WUPEN_A::Enabled)
    }
}
///Field `GCEN` reader - General call enable
pub type GCEN_R = crate::BitReader<GCEN_A>;
///General call enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GCEN_A {
    ///0: General call disabled. Address 0b00000000 is NACKed
    Disabled = 0,
    ///1: General call enabled. Address 0b00000000 is ACKed
    Enabled = 1,
}
impl From<GCEN_A> for bool {
    #[inline(always)]
    fn from(variant: GCEN_A) -> Self {
        variant as u8 != 0
    }
}
impl GCEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> GCEN_A {
        match self.bits {
            false => GCEN_A::Disabled,
            true => GCEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GCEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GCEN_A::Enabled
    }
}
///Field `GCEN` writer - General call enable
pub type GCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, GCEN_A, O>;
impl<'a, const O: u8> GCEN_W<'a, O> {
    ///General call disabled. Address 0b00000000 is NACKed
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GCEN_A::Disabled)
    }
    ///General call enabled. Address 0b00000000 is ACKed
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GCEN_A::Enabled)
    }
}
///Field `SMBHEN` reader - SMBus Host address enable
pub type SMBHEN_R = crate::BitReader<SMBHEN_A>;
///SMBus Host address enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMBHEN_A {
    ///0: Host address disabled. Address 0b0001000x is NACKed
    Disabled = 0,
    ///1: Host address enabled. Address 0b0001000x is ACKed
    Enabled = 1,
}
impl From<SMBHEN_A> for bool {
    #[inline(always)]
    fn from(variant: SMBHEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SMBHEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SMBHEN_A {
        match self.bits {
            false => SMBHEN_A::Disabled,
            true => SMBHEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SMBHEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SMBHEN_A::Enabled
    }
}
///Field `SMBHEN` writer - SMBus Host address enable
pub type SMBHEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, SMBHEN_A, O>;
impl<'a, const O: u8> SMBHEN_W<'a, O> {
    ///Host address disabled. Address 0b0001000x is NACKed
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SMBHEN_A::Disabled)
    }
    ///Host address enabled. Address 0b0001000x is ACKed
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SMBHEN_A::Enabled)
    }
}
///Field `SMBDEN` reader - SMBus Device Default address enable
pub type SMBDEN_R = crate::BitReader<SMBDEN_A>;
///SMBus Device Default address enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMBDEN_A {
    ///0: Device default address disabled. Address 0b1100001x is NACKed
    Disabled = 0,
    ///1: Device default address enabled. Address 0b1100001x is ACKed
    Enabled = 1,
}
impl From<SMBDEN_A> for bool {
    #[inline(always)]
    fn from(variant: SMBDEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SMBDEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SMBDEN_A {
        match self.bits {
            false => SMBDEN_A::Disabled,
            true => SMBDEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SMBDEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SMBDEN_A::Enabled
    }
}
///Field `SMBDEN` writer - SMBus Device Default address enable
pub type SMBDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, SMBDEN_A, O>;
impl<'a, const O: u8> SMBDEN_W<'a, O> {
    ///Device default address disabled. Address 0b1100001x is NACKed
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SMBDEN_A::Disabled)
    }
    ///Device default address enabled. Address 0b1100001x is ACKed
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SMBDEN_A::Enabled)
    }
}
///Field `ALERTEN` reader - SMBUS alert enable
pub type ALERTEN_R = crate::BitReader<ALERTEN_A>;
///SMBUS alert enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALERTEN_A {
    ///0: In device mode (SMBHEN=Disabled) Releases SMBA pin high and Alert Response Address Header disabled (0001100x) followed by NACK. In host mode (SMBHEN=Enabled) SMBus Alert pin (SMBA) not supported
    Disabled = 0,
    ///1: In device mode (SMBHEN=Disabled) Drives SMBA pin low and Alert Response Address Header enabled (0001100x) followed by ACK.In host mode (SMBHEN=Enabled) SMBus Alert pin (SMBA) supported
    Enabled = 1,
}
impl From<ALERTEN_A> for bool {
    #[inline(always)]
    fn from(variant: ALERTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl ALERTEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ALERTEN_A {
        match self.bits {
            false => ALERTEN_A::Disabled,
            true => ALERTEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ALERTEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ALERTEN_A::Enabled
    }
}
///Field `ALERTEN` writer - SMBUS alert enable
pub type ALERTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, ALERTEN_A, O>;
impl<'a, const O: u8> ALERTEN_W<'a, O> {
    ///In device mode (SMBHEN=Disabled) Releases SMBA pin high and Alert Response Address Header disabled (0001100x) followed by NACK. In host mode (SMBHEN=Enabled) SMBus Alert pin (SMBA) not supported
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ALERTEN_A::Disabled)
    }
    ///In device mode (SMBHEN=Disabled) Drives SMBA pin low and Alert Response Address Header enabled (0001100x) followed by ACK.In host mode (SMBHEN=Enabled) SMBus Alert pin (SMBA) supported
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ALERTEN_A::Enabled)
    }
}
///Field `PECEN` reader - PEC enable
pub type PECEN_R = crate::BitReader<PECEN_A>;
///PEC enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PECEN_A {
    ///0: PEC calculation disabled
    Disabled = 0,
    ///1: PEC calculation enabled
    Enabled = 1,
}
impl From<PECEN_A> for bool {
    #[inline(always)]
    fn from(variant: PECEN_A) -> Self {
        variant as u8 != 0
    }
}
impl PECEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PECEN_A {
        match self.bits {
            false => PECEN_A::Disabled,
            true => PECEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PECEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PECEN_A::Enabled
    }
}
///Field `PECEN` writer - PEC enable
pub type PECEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, PECEN_A, O>;
impl<'a, const O: u8> PECEN_W<'a, O> {
    ///PEC calculation disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PECEN_A::Disabled)
    }
    ///PEC calculation enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PECEN_A::Enabled)
    }
}
impl R {
    ///Bit 0 - Peripheral enable
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TX Interrupt enable
    #[inline(always)]
    pub fn txie(&self) -> TXIE_R {
        TXIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - RX Interrupt enable
    #[inline(always)]
    pub fn rxie(&self) -> RXIE_R {
        RXIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Address match interrupt enable (slave only)
    #[inline(always)]
    pub fn addrie(&self) -> ADDRIE_R {
        ADDRIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Not acknowledge received interrupt enable
    #[inline(always)]
    pub fn nackie(&self) -> NACKIE_R {
        NACKIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - STOP detection Interrupt enable
    #[inline(always)]
    pub fn stopie(&self) -> STOPIE_R {
        STOPIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Transfer Complete interrupt enable
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Error interrupts enable
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:11 - Digital noise filter
    #[inline(always)]
    pub fn dnf(&self) -> DNF_R {
        DNF_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 12 - Analog noise filter OFF
    #[inline(always)]
    pub fn anfoff(&self) -> ANFOFF_R {
        ANFOFF_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - DMA transmission requests enable
    #[inline(always)]
    pub fn txdmaen(&self) -> TXDMAEN_R {
        TXDMAEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - DMA reception requests enable
    #[inline(always)]
    pub fn rxdmaen(&self) -> RXDMAEN_R {
        RXDMAEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Slave byte control
    #[inline(always)]
    pub fn sbc(&self) -> SBC_R {
        SBC_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Clock stretching disable
    #[inline(always)]
    pub fn nostretch(&self) -> NOSTRETCH_R {
        NOSTRETCH_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Wakeup from STOP enable
    #[inline(always)]
    pub fn wupen(&self) -> WUPEN_R {
        WUPEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - General call enable
    #[inline(always)]
    pub fn gcen(&self) -> GCEN_R {
        GCEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - SMBus Host address enable
    #[inline(always)]
    pub fn smbhen(&self) -> SMBHEN_R {
        SMBHEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - SMBus Device Default address enable
    #[inline(always)]
    pub fn smbden(&self) -> SMBDEN_R {
        SMBDEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - SMBUS alert enable
    #[inline(always)]
    pub fn alerten(&self) -> ALERTEN_R {
        ALERTEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - PEC enable
    #[inline(always)]
    pub fn pecen(&self) -> PECEN_R {
        PECEN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Peripheral enable
    #[inline(always)]
    #[must_use]
    pub fn pe(&mut self) -> PE_W<0> {
        PE_W::new(self)
    }
    ///Bit 1 - TX Interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn txie(&mut self) -> TXIE_W<1> {
        TXIE_W::new(self)
    }
    ///Bit 2 - RX Interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn rxie(&mut self) -> RXIE_W<2> {
        RXIE_W::new(self)
    }
    ///Bit 3 - Address match interrupt enable (slave only)
    #[inline(always)]
    #[must_use]
    pub fn addrie(&mut self) -> ADDRIE_W<3> {
        ADDRIE_W::new(self)
    }
    ///Bit 4 - Not acknowledge received interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn nackie(&mut self) -> NACKIE_W<4> {
        NACKIE_W::new(self)
    }
    ///Bit 5 - STOP detection Interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn stopie(&mut self) -> STOPIE_W<5> {
        STOPIE_W::new(self)
    }
    ///Bit 6 - Transfer Complete interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn tcie(&mut self) -> TCIE_W<6> {
        TCIE_W::new(self)
    }
    ///Bit 7 - Error interrupts enable
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ERRIE_W<7> {
        ERRIE_W::new(self)
    }
    ///Bits 8:11 - Digital noise filter
    #[inline(always)]
    #[must_use]
    pub fn dnf(&mut self) -> DNF_W<8> {
        DNF_W::new(self)
    }
    ///Bit 12 - Analog noise filter OFF
    #[inline(always)]
    #[must_use]
    pub fn anfoff(&mut self) -> ANFOFF_W<12> {
        ANFOFF_W::new(self)
    }
    ///Bit 14 - DMA transmission requests enable
    #[inline(always)]
    #[must_use]
    pub fn txdmaen(&mut self) -> TXDMAEN_W<14> {
        TXDMAEN_W::new(self)
    }
    ///Bit 15 - DMA reception requests enable
    #[inline(always)]
    #[must_use]
    pub fn rxdmaen(&mut self) -> RXDMAEN_W<15> {
        RXDMAEN_W::new(self)
    }
    ///Bit 16 - Slave byte control
    #[inline(always)]
    #[must_use]
    pub fn sbc(&mut self) -> SBC_W<16> {
        SBC_W::new(self)
    }
    ///Bit 17 - Clock stretching disable
    #[inline(always)]
    #[must_use]
    pub fn nostretch(&mut self) -> NOSTRETCH_W<17> {
        NOSTRETCH_W::new(self)
    }
    ///Bit 18 - Wakeup from STOP enable
    #[inline(always)]
    #[must_use]
    pub fn wupen(&mut self) -> WUPEN_W<18> {
        WUPEN_W::new(self)
    }
    ///Bit 19 - General call enable
    #[inline(always)]
    #[must_use]
    pub fn gcen(&mut self) -> GCEN_W<19> {
        GCEN_W::new(self)
    }
    ///Bit 20 - SMBus Host address enable
    #[inline(always)]
    #[must_use]
    pub fn smbhen(&mut self) -> SMBHEN_W<20> {
        SMBHEN_W::new(self)
    }
    ///Bit 21 - SMBus Device Default address enable
    #[inline(always)]
    #[must_use]
    pub fn smbden(&mut self) -> SMBDEN_W<21> {
        SMBDEN_W::new(self)
    }
    ///Bit 22 - SMBUS alert enable
    #[inline(always)]
    #[must_use]
    pub fn alerten(&mut self) -> ALERTEN_W<22> {
        ALERTEN_W::new(self)
    }
    ///Bit 23 - PEC enable
    #[inline(always)]
    #[must_use]
    pub fn pecen(&mut self) -> PECEN_W<23> {
        PECEN_W::new(self)
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
