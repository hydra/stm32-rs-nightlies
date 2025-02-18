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
///Field `UE` reader - USART enable
pub type UE_R = crate::BitReader<UE_A>;
///USART enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UE_A {
    ///0: UART is disabled
    Disabled = 0,
    ///1: UART is enabled
    Enabled = 1,
}
impl From<UE_A> for bool {
    #[inline(always)]
    fn from(variant: UE_A) -> Self {
        variant as u8 != 0
    }
}
impl UE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> UE_A {
        match self.bits {
            false => UE_A::Disabled,
            true => UE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UE_A::Enabled
    }
}
///Field `UE` writer - USART enable
pub type UE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, UE_A, O>;
impl<'a, const O: u8> UE_W<'a, O> {
    ///UART is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(UE_A::Disabled)
    }
    ///UART is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(UE_A::Enabled)
    }
}
///Field `UESM` reader - USART enable in Stop mode
pub type UESM_R = crate::BitReader<UESM_A>;
///USART enable in Stop mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UESM_A {
    ///0: USART not able to wake up the MCU from Stop mode
    Disabled = 0,
    ///1: USART able to wake up the MCU from Stop mode
    Enabled = 1,
}
impl From<UESM_A> for bool {
    #[inline(always)]
    fn from(variant: UESM_A) -> Self {
        variant as u8 != 0
    }
}
impl UESM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> UESM_A {
        match self.bits {
            false => UESM_A::Disabled,
            true => UESM_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UESM_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UESM_A::Enabled
    }
}
///Field `UESM` writer - USART enable in Stop mode
pub type UESM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, UESM_A, O>;
impl<'a, const O: u8> UESM_W<'a, O> {
    ///USART not able to wake up the MCU from Stop mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(UESM_A::Disabled)
    }
    ///USART able to wake up the MCU from Stop mode
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(UESM_A::Enabled)
    }
}
///Field `RE` reader - Receiver enable
pub type RE_R = crate::BitReader<RE_A>;
///Receiver enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RE_A {
    ///0: Receiver is disabled
    Disabled = 0,
    ///1: Receiver is enabled
    Enabled = 1,
}
impl From<RE_A> for bool {
    #[inline(always)]
    fn from(variant: RE_A) -> Self {
        variant as u8 != 0
    }
}
impl RE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RE_A {
        match self.bits {
            false => RE_A::Disabled,
            true => RE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RE_A::Enabled
    }
}
///Field `RE` writer - Receiver enable
pub type RE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, RE_A, O>;
impl<'a, const O: u8> RE_W<'a, O> {
    ///Receiver is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RE_A::Disabled)
    }
    ///Receiver is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RE_A::Enabled)
    }
}
///Field `TE` reader - Transmitter enable
pub type TE_R = crate::BitReader<TE_A>;
///Transmitter enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TE_A {
    ///0: Transmitter is disabled
    Disabled = 0,
    ///1: Transmitter is enabled
    Enabled = 1,
}
impl From<TE_A> for bool {
    #[inline(always)]
    fn from(variant: TE_A) -> Self {
        variant as u8 != 0
    }
}
impl TE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TE_A {
        match self.bits {
            false => TE_A::Disabled,
            true => TE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TE_A::Enabled
    }
}
///Field `TE` writer - Transmitter enable
pub type TE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, TE_A, O>;
impl<'a, const O: u8> TE_W<'a, O> {
    ///Transmitter is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TE_A::Disabled)
    }
    ///Transmitter is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TE_A::Enabled)
    }
}
///Field `IDLEIE` reader - IDLE interrupt enable
pub type IDLEIE_R = crate::BitReader<IDLEIE_A>;
///IDLE interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDLEIE_A {
    ///0: Interrupt is disabled
    Disabled = 0,
    ///1: Interrupt is generated whenever IDLE=1 in the ISR register
    Enabled = 1,
}
impl From<IDLEIE_A> for bool {
    #[inline(always)]
    fn from(variant: IDLEIE_A) -> Self {
        variant as u8 != 0
    }
}
impl IDLEIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> IDLEIE_A {
        match self.bits {
            false => IDLEIE_A::Disabled,
            true => IDLEIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IDLEIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IDLEIE_A::Enabled
    }
}
///Field `IDLEIE` writer - IDLE interrupt enable
pub type IDLEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, IDLEIE_A, O>;
impl<'a, const O: u8> IDLEIE_W<'a, O> {
    ///Interrupt is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(IDLEIE_A::Disabled)
    }
    ///Interrupt is generated whenever IDLE=1 in the ISR register
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(IDLEIE_A::Enabled)
    }
}
///Field `RXNEIE` reader - RXNE interrupt enable
pub type RXNEIE_R = crate::BitReader<RXNEIE_A>;
///RXNE interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXNEIE_A {
    ///0: Interrupt is disabled
    Disabled = 0,
    ///1: Interrupt is generated whenever ORE=1 or RXNE=1 in the ISR register
    Enabled = 1,
}
impl From<RXNEIE_A> for bool {
    #[inline(always)]
    fn from(variant: RXNEIE_A) -> Self {
        variant as u8 != 0
    }
}
impl RXNEIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RXNEIE_A {
        match self.bits {
            false => RXNEIE_A::Disabled,
            true => RXNEIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXNEIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXNEIE_A::Enabled
    }
}
///Field `RXNEIE` writer - RXNE interrupt enable
pub type RXNEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, RXNEIE_A, O>;
impl<'a, const O: u8> RXNEIE_W<'a, O> {
    ///Interrupt is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RXNEIE_A::Disabled)
    }
    ///Interrupt is generated whenever ORE=1 or RXNE=1 in the ISR register
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RXNEIE_A::Enabled)
    }
}
///Field `TCIE` reader - Transmission complete interrupt enable
pub type TCIE_R = crate::BitReader<TCIE_A>;
///Transmission complete interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCIE_A {
    ///0: Interrupt is disabled
    Disabled = 0,
    ///1: Interrupt is generated whenever TC=1 in the ISR register
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
///Field `TCIE` writer - Transmission complete interrupt enable
pub type TCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, TCIE_A, O>;
impl<'a, const O: u8> TCIE_W<'a, O> {
    ///Interrupt is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TCIE_A::Disabled)
    }
    ///Interrupt is generated whenever TC=1 in the ISR register
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TCIE_A::Enabled)
    }
}
///Field `TXEIE` reader - interrupt enable
pub type TXEIE_R = crate::BitReader<TXEIE_A>;
///interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXEIE_A {
    ///0: Interrupt is disabled
    Disabled = 0,
    ///1: Interrupt is generated whenever TXE=1 in the ISR register
    Enabled = 1,
}
impl From<TXEIE_A> for bool {
    #[inline(always)]
    fn from(variant: TXEIE_A) -> Self {
        variant as u8 != 0
    }
}
impl TXEIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TXEIE_A {
        match self.bits {
            false => TXEIE_A::Disabled,
            true => TXEIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXEIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXEIE_A::Enabled
    }
}
///Field `TXEIE` writer - interrupt enable
pub type TXEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, TXEIE_A, O>;
impl<'a, const O: u8> TXEIE_W<'a, O> {
    ///Interrupt is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TXEIE_A::Disabled)
    }
    ///Interrupt is generated whenever TXE=1 in the ISR register
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TXEIE_A::Enabled)
    }
}
///Field `PEIE` reader - PE interrupt enable
pub type PEIE_R = crate::BitReader<PEIE_A>;
///PE interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PEIE_A {
    ///0: Interrupt is disabled
    Disabled = 0,
    ///1: Interrupt is generated whenever PE=1 in the ISR register
    Enabled = 1,
}
impl From<PEIE_A> for bool {
    #[inline(always)]
    fn from(variant: PEIE_A) -> Self {
        variant as u8 != 0
    }
}
impl PEIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PEIE_A {
        match self.bits {
            false => PEIE_A::Disabled,
            true => PEIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PEIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PEIE_A::Enabled
    }
}
///Field `PEIE` writer - PE interrupt enable
pub type PEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, PEIE_A, O>;
impl<'a, const O: u8> PEIE_W<'a, O> {
    ///Interrupt is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PEIE_A::Disabled)
    }
    ///Interrupt is generated whenever PE=1 in the ISR register
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PEIE_A::Enabled)
    }
}
///Field `PS` reader - Parity selection
pub type PS_R = crate::BitReader<PS_A>;
///Parity selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PS_A {
    ///0: Even parity
    Even = 0,
    ///1: Odd parity
    Odd = 1,
}
impl From<PS_A> for bool {
    #[inline(always)]
    fn from(variant: PS_A) -> Self {
        variant as u8 != 0
    }
}
impl PS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PS_A {
        match self.bits {
            false => PS_A::Even,
            true => PS_A::Odd,
        }
    }
    ///Checks if the value of the field is `Even`
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == PS_A::Even
    }
    ///Checks if the value of the field is `Odd`
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == PS_A::Odd
    }
}
///Field `PS` writer - Parity selection
pub type PS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, PS_A, O>;
impl<'a, const O: u8> PS_W<'a, O> {
    ///Even parity
    #[inline(always)]
    pub fn even(self) -> &'a mut W {
        self.variant(PS_A::Even)
    }
    ///Odd parity
    #[inline(always)]
    pub fn odd(self) -> &'a mut W {
        self.variant(PS_A::Odd)
    }
}
///Field `PCE` reader - Parity control enable
pub type PCE_R = crate::BitReader<PCE_A>;
///Parity control enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PCE_A {
    ///0: Parity control disabled
    Disabled = 0,
    ///1: Parity control enabled
    Enabled = 1,
}
impl From<PCE_A> for bool {
    #[inline(always)]
    fn from(variant: PCE_A) -> Self {
        variant as u8 != 0
    }
}
impl PCE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PCE_A {
        match self.bits {
            false => PCE_A::Disabled,
            true => PCE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PCE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PCE_A::Enabled
    }
}
///Field `PCE` writer - Parity control enable
pub type PCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, PCE_A, O>;
impl<'a, const O: u8> PCE_W<'a, O> {
    ///Parity control disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PCE_A::Disabled)
    }
    ///Parity control enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PCE_A::Enabled)
    }
}
///Field `WAKE` reader - Receiver wakeup method
pub type WAKE_R = crate::BitReader<WAKE_A>;
///Receiver wakeup method
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAKE_A {
    ///0: Idle line
    Idle = 0,
    ///1: Address mask
    Address = 1,
}
impl From<WAKE_A> for bool {
    #[inline(always)]
    fn from(variant: WAKE_A) -> Self {
        variant as u8 != 0
    }
}
impl WAKE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> WAKE_A {
        match self.bits {
            false => WAKE_A::Idle,
            true => WAKE_A::Address,
        }
    }
    ///Checks if the value of the field is `Idle`
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == WAKE_A::Idle
    }
    ///Checks if the value of the field is `Address`
    #[inline(always)]
    pub fn is_address(&self) -> bool {
        *self == WAKE_A::Address
    }
}
///Field `WAKE` writer - Receiver wakeup method
pub type WAKE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, WAKE_A, O>;
impl<'a, const O: u8> WAKE_W<'a, O> {
    ///Idle line
    #[inline(always)]
    pub fn idle(self) -> &'a mut W {
        self.variant(WAKE_A::Idle)
    }
    ///Address mask
    #[inline(always)]
    pub fn address(self) -> &'a mut W {
        self.variant(WAKE_A::Address)
    }
}
///Field `M0` reader - Word length
pub type M0_R = crate::BitReader<M0_A>;
///Word length
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum M0_A {
    ///0: 1 start bit, 8 data bits, n stop bits
    Bit8 = 0,
    ///1: 1 start bit, 9 data bits, n stop bits
    Bit9 = 1,
}
impl From<M0_A> for bool {
    #[inline(always)]
    fn from(variant: M0_A) -> Self {
        variant as u8 != 0
    }
}
impl M0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> M0_A {
        match self.bits {
            false => M0_A::Bit8,
            true => M0_A::Bit9,
        }
    }
    ///Checks if the value of the field is `Bit8`
    #[inline(always)]
    pub fn is_bit8(&self) -> bool {
        *self == M0_A::Bit8
    }
    ///Checks if the value of the field is `Bit9`
    #[inline(always)]
    pub fn is_bit9(&self) -> bool {
        *self == M0_A::Bit9
    }
}
///Field `M0` writer - Word length
pub type M0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, M0_A, O>;
impl<'a, const O: u8> M0_W<'a, O> {
    ///1 start bit, 8 data bits, n stop bits
    #[inline(always)]
    pub fn bit8(self) -> &'a mut W {
        self.variant(M0_A::Bit8)
    }
    ///1 start bit, 9 data bits, n stop bits
    #[inline(always)]
    pub fn bit9(self) -> &'a mut W {
        self.variant(M0_A::Bit9)
    }
}
///Field `MME` reader - Mute mode enable
pub type MME_R = crate::BitReader<MME_A>;
///Mute mode enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MME_A {
    ///0: Receiver in active mode permanently
    Disabled = 0,
    ///1: Receiver can switch between mute mode and active mode
    Enabled = 1,
}
impl From<MME_A> for bool {
    #[inline(always)]
    fn from(variant: MME_A) -> Self {
        variant as u8 != 0
    }
}
impl MME_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MME_A {
        match self.bits {
            false => MME_A::Disabled,
            true => MME_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MME_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MME_A::Enabled
    }
}
///Field `MME` writer - Mute mode enable
pub type MME_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, MME_A, O>;
impl<'a, const O: u8> MME_W<'a, O> {
    ///Receiver in active mode permanently
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MME_A::Disabled)
    }
    ///Receiver can switch between mute mode and active mode
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MME_A::Enabled)
    }
}
///Field `CMIE` reader - Character match interrupt enable
pub type CMIE_R = crate::BitReader<CMIE_A>;
///Character match interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMIE_A {
    ///0: Interrupt is disabled
    Disabled = 0,
    ///1: Interrupt is generated when the CMF bit is set in the ISR register
    Enabled = 1,
}
impl From<CMIE_A> for bool {
    #[inline(always)]
    fn from(variant: CMIE_A) -> Self {
        variant as u8 != 0
    }
}
impl CMIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CMIE_A {
        match self.bits {
            false => CMIE_A::Disabled,
            true => CMIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CMIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CMIE_A::Enabled
    }
}
///Field `CMIE` writer - Character match interrupt enable
pub type CMIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, CMIE_A, O>;
impl<'a, const O: u8> CMIE_W<'a, O> {
    ///Interrupt is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CMIE_A::Disabled)
    }
    ///Interrupt is generated when the CMF bit is set in the ISR register
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CMIE_A::Enabled)
    }
}
///Field `OVER8` reader - Oversampling mode
pub type OVER8_R = crate::BitReader<OVER8_A>;
///Oversampling mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVER8_A {
    ///0: Oversampling by 16
    Oversampling16 = 0,
    ///1: Oversampling by 8
    Oversampling8 = 1,
}
impl From<OVER8_A> for bool {
    #[inline(always)]
    fn from(variant: OVER8_A) -> Self {
        variant as u8 != 0
    }
}
impl OVER8_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OVER8_A {
        match self.bits {
            false => OVER8_A::Oversampling16,
            true => OVER8_A::Oversampling8,
        }
    }
    ///Checks if the value of the field is `Oversampling16`
    #[inline(always)]
    pub fn is_oversampling16(&self) -> bool {
        *self == OVER8_A::Oversampling16
    }
    ///Checks if the value of the field is `Oversampling8`
    #[inline(always)]
    pub fn is_oversampling8(&self) -> bool {
        *self == OVER8_A::Oversampling8
    }
}
///Field `OVER8` writer - Oversampling mode
pub type OVER8_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, OVER8_A, O>;
impl<'a, const O: u8> OVER8_W<'a, O> {
    ///Oversampling by 16
    #[inline(always)]
    pub fn oversampling16(self) -> &'a mut W {
        self.variant(OVER8_A::Oversampling16)
    }
    ///Oversampling by 8
    #[inline(always)]
    pub fn oversampling8(self) -> &'a mut W {
        self.variant(OVER8_A::Oversampling8)
    }
}
///Field `DEDT` reader - DEDT
pub type DEDT_R = crate::FieldReader<u8, u8>;
///Field `DEDT` writer - DEDT
pub type DEDT_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CR1_SPEC, u8, u8, 5, O>;
///Field `DEAT` reader - DEAT
pub type DEAT_R = crate::FieldReader<u8, u8>;
///Field `DEAT` writer - DEAT
pub type DEAT_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CR1_SPEC, u8, u8, 5, O>;
///Field `RTOIE` reader - Receiver timeout interrupt enable
pub type RTOIE_R = crate::BitReader<RTOIE_A>;
///Receiver timeout interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTOIE_A {
    ///0: Interrupt is inhibited
    Disabled = 0,
    ///1: An USART interrupt is generated when the RTOF bit is set in the ISR register
    Enabled = 1,
}
impl From<RTOIE_A> for bool {
    #[inline(always)]
    fn from(variant: RTOIE_A) -> Self {
        variant as u8 != 0
    }
}
impl RTOIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RTOIE_A {
        match self.bits {
            false => RTOIE_A::Disabled,
            true => RTOIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RTOIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RTOIE_A::Enabled
    }
}
///Field `RTOIE` writer - Receiver timeout interrupt enable
pub type RTOIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, RTOIE_A, O>;
impl<'a, const O: u8> RTOIE_W<'a, O> {
    ///Interrupt is inhibited
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RTOIE_A::Disabled)
    }
    ///An USART interrupt is generated when the RTOF bit is set in the ISR register
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RTOIE_A::Enabled)
    }
}
///Field `EOBIE` reader - End of Block interrupt enable
pub type EOBIE_R = crate::BitReader<EOBIE_A>;
///End of Block interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOBIE_A {
    ///0: Interrupt is inhibited
    Disabled = 0,
    ///1: A USART interrupt is generated when the EOBF flag is set in the ISR register
    Enabled = 1,
}
impl From<EOBIE_A> for bool {
    #[inline(always)]
    fn from(variant: EOBIE_A) -> Self {
        variant as u8 != 0
    }
}
impl EOBIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EOBIE_A {
        match self.bits {
            false => EOBIE_A::Disabled,
            true => EOBIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EOBIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EOBIE_A::Enabled
    }
}
///Field `EOBIE` writer - End of Block interrupt enable
pub type EOBIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, EOBIE_A, O>;
impl<'a, const O: u8> EOBIE_W<'a, O> {
    ///Interrupt is inhibited
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EOBIE_A::Disabled)
    }
    ///A USART interrupt is generated when the EOBF flag is set in the ISR register
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EOBIE_A::Enabled)
    }
}
///Field `M1` reader - Word length
pub type M1_R = crate::BitReader<M1_A>;
///Word length
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum M1_A {
    ///0: Use M0 to set the data bits
    M0 = 0,
    ///1: 1 start bit, 7 data bits, n stop bits
    Bit7 = 1,
}
impl From<M1_A> for bool {
    #[inline(always)]
    fn from(variant: M1_A) -> Self {
        variant as u8 != 0
    }
}
impl M1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> M1_A {
        match self.bits {
            false => M1_A::M0,
            true => M1_A::Bit7,
        }
    }
    ///Checks if the value of the field is `M0`
    #[inline(always)]
    pub fn is_m0(&self) -> bool {
        *self == M1_A::M0
    }
    ///Checks if the value of the field is `Bit7`
    #[inline(always)]
    pub fn is_bit7(&self) -> bool {
        *self == M1_A::Bit7
    }
}
///Field `M1` writer - Word length
pub type M1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, M1_A, O>;
impl<'a, const O: u8> M1_W<'a, O> {
    ///Use M0 to set the data bits
    #[inline(always)]
    pub fn m0(self) -> &'a mut W {
        self.variant(M1_A::M0)
    }
    ///1 start bit, 7 data bits, n stop bits
    #[inline(always)]
    pub fn bit7(self) -> &'a mut W {
        self.variant(M1_A::Bit7)
    }
}
///Field `FIFOEN` reader - FIFO mode enable
pub type FIFOEN_R = crate::BitReader<FIFOEN_A>;
///FIFO mode enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFOEN_A {
    ///0: FIFO mode is disabled
    Disabled = 0,
    ///1: FIFO mode is enabled
    Enabled = 1,
}
impl From<FIFOEN_A> for bool {
    #[inline(always)]
    fn from(variant: FIFOEN_A) -> Self {
        variant as u8 != 0
    }
}
impl FIFOEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FIFOEN_A {
        match self.bits {
            false => FIFOEN_A::Disabled,
            true => FIFOEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FIFOEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FIFOEN_A::Enabled
    }
}
///Field `FIFOEN` writer - FIFO mode enable
pub type FIFOEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, FIFOEN_A, O>;
impl<'a, const O: u8> FIFOEN_W<'a, O> {
    ///FIFO mode is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FIFOEN_A::Disabled)
    }
    ///FIFO mode is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FIFOEN_A::Enabled)
    }
}
///Field `TXFEIE` reader - TXFIFO empty interrupt enable
pub type TXFEIE_R = crate::BitReader<TXFEIE_A>;
///TXFIFO empty interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXFEIE_A {
    ///0: Interrupt inhibited
    Disabled = 0,
    ///1: USART interrupt generated when TXFE = 1 in the USART_ISR register
    Enabled = 1,
}
impl From<TXFEIE_A> for bool {
    #[inline(always)]
    fn from(variant: TXFEIE_A) -> Self {
        variant as u8 != 0
    }
}
impl TXFEIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TXFEIE_A {
        match self.bits {
            false => TXFEIE_A::Disabled,
            true => TXFEIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXFEIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXFEIE_A::Enabled
    }
}
///Field `TXFEIE` writer - TXFIFO empty interrupt enable
pub type TXFEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, TXFEIE_A, O>;
impl<'a, const O: u8> TXFEIE_W<'a, O> {
    ///Interrupt inhibited
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TXFEIE_A::Disabled)
    }
    ///USART interrupt generated when TXFE = 1 in the USART_ISR register
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TXFEIE_A::Enabled)
    }
}
///Field `RXFFIE` reader - RXFIFO Full interrupt enable
pub type RXFFIE_R = crate::BitReader<RXFFIE_A>;
///RXFIFO Full interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXFFIE_A {
    ///0: Interrupt inhibited
    Disabled = 0,
    ///1: USART interrupt generated when RXFF = 1 in the USART_ISR register
    Enabled = 1,
}
impl From<RXFFIE_A> for bool {
    #[inline(always)]
    fn from(variant: RXFFIE_A) -> Self {
        variant as u8 != 0
    }
}
impl RXFFIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RXFFIE_A {
        match self.bits {
            false => RXFFIE_A::Disabled,
            true => RXFFIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXFFIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXFFIE_A::Enabled
    }
}
///Field `RXFFIE` writer - RXFIFO Full interrupt enable
pub type RXFFIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, RXFFIE_A, O>;
impl<'a, const O: u8> RXFFIE_W<'a, O> {
    ///Interrupt inhibited
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RXFFIE_A::Disabled)
    }
    ///USART interrupt generated when RXFF = 1 in the USART_ISR register
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RXFFIE_A::Enabled)
    }
}
impl R {
    ///Bit 0 - USART enable
    #[inline(always)]
    pub fn ue(&self) -> UE_R {
        UE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - USART enable in Stop mode
    #[inline(always)]
    pub fn uesm(&self) -> UESM_R {
        UESM_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Receiver enable
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Transmitter enable
    #[inline(always)]
    pub fn te(&self) -> TE_R {
        TE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - IDLE interrupt enable
    #[inline(always)]
    pub fn idleie(&self) -> IDLEIE_R {
        IDLEIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - RXNE interrupt enable
    #[inline(always)]
    pub fn rxneie(&self) -> RXNEIE_R {
        RXNEIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Transmission complete interrupt enable
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - interrupt enable
    #[inline(always)]
    pub fn txeie(&self) -> TXEIE_R {
        TXEIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - PE interrupt enable
    #[inline(always)]
    pub fn peie(&self) -> PEIE_R {
        PEIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Parity selection
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Parity control enable
    #[inline(always)]
    pub fn pce(&self) -> PCE_R {
        PCE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Receiver wakeup method
    #[inline(always)]
    pub fn wake(&self) -> WAKE_R {
        WAKE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Word length
    #[inline(always)]
    pub fn m0(&self) -> M0_R {
        M0_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Mute mode enable
    #[inline(always)]
    pub fn mme(&self) -> MME_R {
        MME_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Character match interrupt enable
    #[inline(always)]
    pub fn cmie(&self) -> CMIE_R {
        CMIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Oversampling mode
    #[inline(always)]
    pub fn over8(&self) -> OVER8_R {
        OVER8_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:20 - DEDT
    #[inline(always)]
    pub fn dedt(&self) -> DEDT_R {
        DEDT_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bits 21:25 - DEAT
    #[inline(always)]
    pub fn deat(&self) -> DEAT_R {
        DEAT_R::new(((self.bits >> 21) & 0x1f) as u8)
    }
    ///Bit 26 - Receiver timeout interrupt enable
    #[inline(always)]
    pub fn rtoie(&self) -> RTOIE_R {
        RTOIE_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - End of Block interrupt enable
    #[inline(always)]
    pub fn eobie(&self) -> EOBIE_R {
        EOBIE_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Word length
    #[inline(always)]
    pub fn m1(&self) -> M1_R {
        M1_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - FIFO mode enable
    #[inline(always)]
    pub fn fifoen(&self) -> FIFOEN_R {
        FIFOEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - TXFIFO empty interrupt enable
    #[inline(always)]
    pub fn txfeie(&self) -> TXFEIE_R {
        TXFEIE_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - RXFIFO Full interrupt enable
    #[inline(always)]
    pub fn rxffie(&self) -> RXFFIE_R {
        RXFFIE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - USART enable
    #[inline(always)]
    #[must_use]
    pub fn ue(&mut self) -> UE_W<0> {
        UE_W::new(self)
    }
    ///Bit 1 - USART enable in Stop mode
    #[inline(always)]
    #[must_use]
    pub fn uesm(&mut self) -> UESM_W<1> {
        UESM_W::new(self)
    }
    ///Bit 2 - Receiver enable
    #[inline(always)]
    #[must_use]
    pub fn re(&mut self) -> RE_W<2> {
        RE_W::new(self)
    }
    ///Bit 3 - Transmitter enable
    #[inline(always)]
    #[must_use]
    pub fn te(&mut self) -> TE_W<3> {
        TE_W::new(self)
    }
    ///Bit 4 - IDLE interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn idleie(&mut self) -> IDLEIE_W<4> {
        IDLEIE_W::new(self)
    }
    ///Bit 5 - RXNE interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn rxneie(&mut self) -> RXNEIE_W<5> {
        RXNEIE_W::new(self)
    }
    ///Bit 6 - Transmission complete interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn tcie(&mut self) -> TCIE_W<6> {
        TCIE_W::new(self)
    }
    ///Bit 7 - interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn txeie(&mut self) -> TXEIE_W<7> {
        TXEIE_W::new(self)
    }
    ///Bit 8 - PE interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn peie(&mut self) -> PEIE_W<8> {
        PEIE_W::new(self)
    }
    ///Bit 9 - Parity selection
    #[inline(always)]
    #[must_use]
    pub fn ps(&mut self) -> PS_W<9> {
        PS_W::new(self)
    }
    ///Bit 10 - Parity control enable
    #[inline(always)]
    #[must_use]
    pub fn pce(&mut self) -> PCE_W<10> {
        PCE_W::new(self)
    }
    ///Bit 11 - Receiver wakeup method
    #[inline(always)]
    #[must_use]
    pub fn wake(&mut self) -> WAKE_W<11> {
        WAKE_W::new(self)
    }
    ///Bit 12 - Word length
    #[inline(always)]
    #[must_use]
    pub fn m0(&mut self) -> M0_W<12> {
        M0_W::new(self)
    }
    ///Bit 13 - Mute mode enable
    #[inline(always)]
    #[must_use]
    pub fn mme(&mut self) -> MME_W<13> {
        MME_W::new(self)
    }
    ///Bit 14 - Character match interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn cmie(&mut self) -> CMIE_W<14> {
        CMIE_W::new(self)
    }
    ///Bit 15 - Oversampling mode
    #[inline(always)]
    #[must_use]
    pub fn over8(&mut self) -> OVER8_W<15> {
        OVER8_W::new(self)
    }
    ///Bits 16:20 - DEDT
    #[inline(always)]
    #[must_use]
    pub fn dedt(&mut self) -> DEDT_W<16> {
        DEDT_W::new(self)
    }
    ///Bits 21:25 - DEAT
    #[inline(always)]
    #[must_use]
    pub fn deat(&mut self) -> DEAT_W<21> {
        DEAT_W::new(self)
    }
    ///Bit 26 - Receiver timeout interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn rtoie(&mut self) -> RTOIE_W<26> {
        RTOIE_W::new(self)
    }
    ///Bit 27 - End of Block interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn eobie(&mut self) -> EOBIE_W<27> {
        EOBIE_W::new(self)
    }
    ///Bit 28 - Word length
    #[inline(always)]
    #[must_use]
    pub fn m1(&mut self) -> M1_W<28> {
        M1_W::new(self)
    }
    ///Bit 29 - FIFO mode enable
    #[inline(always)]
    #[must_use]
    pub fn fifoen(&mut self) -> FIFOEN_W<29> {
        FIFOEN_W::new(self)
    }
    ///Bit 30 - TXFIFO empty interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn txfeie(&mut self) -> TXFEIE_W<30> {
        TXFEIE_W::new(self)
    }
    ///Bit 31 - RXFIFO Full interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn rxffie(&mut self) -> RXFFIE_W<31> {
        RXFFIE_W::new(self)
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
