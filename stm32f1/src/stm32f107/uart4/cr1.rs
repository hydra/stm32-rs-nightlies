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
///Field `SBK` reader - Send break
pub type SBK_R = crate::BitReader<SBK_A>;
///Send break
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBK_A {
    ///0: No break character is transmitted
    NoBreak = 0,
    ///1: Break character transmitted
    Break = 1,
}
impl From<SBK_A> for bool {
    #[inline(always)]
    fn from(variant: SBK_A) -> Self {
        variant as u8 != 0
    }
}
impl SBK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SBK_A {
        match self.bits {
            false => SBK_A::NoBreak,
            true => SBK_A::Break,
        }
    }
    ///Checks if the value of the field is `NoBreak`
    #[inline(always)]
    pub fn is_no_break(&self) -> bool {
        *self == SBK_A::NoBreak
    }
    ///Checks if the value of the field is `Break`
    #[inline(always)]
    pub fn is_break(&self) -> bool {
        *self == SBK_A::Break
    }
}
///Field `SBK` writer - Send break
pub type SBK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, SBK_A, O>;
impl<'a, const O: u8> SBK_W<'a, O> {
    ///No break character is transmitted
    #[inline(always)]
    pub fn no_break(self) -> &'a mut W {
        self.variant(SBK_A::NoBreak)
    }
    ///Break character transmitted
    #[inline(always)]
    pub fn break_(self) -> &'a mut W {
        self.variant(SBK_A::Break)
    }
}
///Field `RWU` reader - Receiver wakeup
pub type RWU_R = crate::BitReader<RWU_A>;
///Receiver wakeup
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RWU_A {
    ///0: Receiver in active mode
    Active = 0,
    ///1: Receiver in mute mode
    Mute = 1,
}
impl From<RWU_A> for bool {
    #[inline(always)]
    fn from(variant: RWU_A) -> Self {
        variant as u8 != 0
    }
}
impl RWU_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RWU_A {
        match self.bits {
            false => RWU_A::Active,
            true => RWU_A::Mute,
        }
    }
    ///Checks if the value of the field is `Active`
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == RWU_A::Active
    }
    ///Checks if the value of the field is `Mute`
    #[inline(always)]
    pub fn is_mute(&self) -> bool {
        *self == RWU_A::Mute
    }
}
///Field `RWU` writer - Receiver wakeup
pub type RWU_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, RWU_A, O>;
impl<'a, const O: u8> RWU_W<'a, O> {
    ///Receiver in active mode
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(RWU_A::Active)
    }
    ///Receiver in mute mode
    #[inline(always)]
    pub fn mute(self) -> &'a mut W {
        self.variant(RWU_A::Mute)
    }
}
///Field `RE` reader - Receiver enable
pub type RE_R = crate::BitReader<RE_A>;
///Receiver enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RE_A {
    ///0: Receiver disabled
    Disabled = 0,
    ///1: Receiver enabled
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
    ///Receiver disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RE_A::Disabled)
    }
    ///Receiver enabled
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
    ///0: Transmitter disabled
    Disabled = 0,
    ///1: Transmitter enabled
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
    ///Transmitter disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TE_A::Disabled)
    }
    ///Transmitter enabled
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
    ///0: IDLE interrupt disabled
    Disabled = 0,
    ///1: IDLE interrupt enabled
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
    ///IDLE interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(IDLEIE_A::Disabled)
    }
    ///IDLE interrupt enabled
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
    ///0: RXNE interrupt disabled
    Disabled = 0,
    ///1: RXNE interrupt enabled
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
    ///RXNE interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RXNEIE_A::Disabled)
    }
    ///RXNE interrupt enabled
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
    ///0: TC interrupt disabled
    Disabled = 0,
    ///1: TC interrupt enabled
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
    ///TC interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TCIE_A::Disabled)
    }
    ///TC interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TCIE_A::Enabled)
    }
}
///Field `TXEIE` reader - TXE interrupt enable
pub type TXEIE_R = crate::BitReader<TXEIE_A>;
///TXE interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXEIE_A {
    ///0: TXE interrupt disabled
    Disabled = 0,
    ///1: TXE interrupt enabled
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
///Field `TXEIE` writer - TXE interrupt enable
pub type TXEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, TXEIE_A, O>;
impl<'a, const O: u8> TXEIE_W<'a, O> {
    ///TXE interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TXEIE_A::Disabled)
    }
    ///TXE interrupt enabled
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
    ///0: PE interrupt disabled
    Disabled = 0,
    ///1: PE interrupt enabled
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
    ///PE interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PEIE_A::Disabled)
    }
    ///PE interrupt enabled
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
///Field `WAKE` reader - Wakeup method
pub type WAKE_R = crate::BitReader<WAKE_A>;
///Wakeup method
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAKE_A {
    ///0: USART wakeup on idle line
    IdleLine = 0,
    ///1: USART wakeup on address mark
    AddressMark = 1,
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
            false => WAKE_A::IdleLine,
            true => WAKE_A::AddressMark,
        }
    }
    ///Checks if the value of the field is `IdleLine`
    #[inline(always)]
    pub fn is_idle_line(&self) -> bool {
        *self == WAKE_A::IdleLine
    }
    ///Checks if the value of the field is `AddressMark`
    #[inline(always)]
    pub fn is_address_mark(&self) -> bool {
        *self == WAKE_A::AddressMark
    }
}
///Field `WAKE` writer - Wakeup method
pub type WAKE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, WAKE_A, O>;
impl<'a, const O: u8> WAKE_W<'a, O> {
    ///USART wakeup on idle line
    #[inline(always)]
    pub fn idle_line(self) -> &'a mut W {
        self.variant(WAKE_A::IdleLine)
    }
    ///USART wakeup on address mark
    #[inline(always)]
    pub fn address_mark(self) -> &'a mut W {
        self.variant(WAKE_A::AddressMark)
    }
}
///Field `M` reader - Word length
pub type M_R = crate::BitReader<M_A>;
///Word length
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum M_A {
    ///0: 8 data bits
    M8 = 0,
    ///1: 9 data bits
    M9 = 1,
}
impl From<M_A> for bool {
    #[inline(always)]
    fn from(variant: M_A) -> Self {
        variant as u8 != 0
    }
}
impl M_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> M_A {
        match self.bits {
            false => M_A::M8,
            true => M_A::M9,
        }
    }
    ///Checks if the value of the field is `M8`
    #[inline(always)]
    pub fn is_m8(&self) -> bool {
        *self == M_A::M8
    }
    ///Checks if the value of the field is `M9`
    #[inline(always)]
    pub fn is_m9(&self) -> bool {
        *self == M_A::M9
    }
}
///Field `M` writer - Word length
pub type M_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, M_A, O>;
impl<'a, const O: u8> M_W<'a, O> {
    ///8 data bits
    #[inline(always)]
    pub fn m8(self) -> &'a mut W {
        self.variant(M_A::M8)
    }
    ///9 data bits
    #[inline(always)]
    pub fn m9(self) -> &'a mut W {
        self.variant(M_A::M9)
    }
}
///Field `UE` reader - USART enable
pub type UE_R = crate::BitReader<UE_A>;
///USART enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UE_A {
    ///0: USART prescaler and outputs disabled
    Disabled = 0,
    ///1: USART enabled
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
    ///USART prescaler and outputs disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(UE_A::Disabled)
    }
    ///USART enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(UE_A::Enabled)
    }
}
impl R {
    ///Bit 0 - Send break
    #[inline(always)]
    pub fn sbk(&self) -> SBK_R {
        SBK_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Receiver wakeup
    #[inline(always)]
    pub fn rwu(&self) -> RWU_R {
        RWU_R::new(((self.bits >> 1) & 1) != 0)
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
    ///Bit 7 - TXE interrupt enable
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
    ///Bit 11 - Wakeup method
    #[inline(always)]
    pub fn wake(&self) -> WAKE_R {
        WAKE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Word length
    #[inline(always)]
    pub fn m(&self) -> M_R {
        M_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - USART enable
    #[inline(always)]
    pub fn ue(&self) -> UE_R {
        UE_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Send break
    #[inline(always)]
    #[must_use]
    pub fn sbk(&mut self) -> SBK_W<0> {
        SBK_W::new(self)
    }
    ///Bit 1 - Receiver wakeup
    #[inline(always)]
    #[must_use]
    pub fn rwu(&mut self) -> RWU_W<1> {
        RWU_W::new(self)
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
    ///Bit 7 - TXE interrupt enable
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
    ///Bit 11 - Wakeup method
    #[inline(always)]
    #[must_use]
    pub fn wake(&mut self) -> WAKE_W<11> {
        WAKE_W::new(self)
    }
    ///Bit 12 - Word length
    #[inline(always)]
    #[must_use]
    pub fn m(&mut self) -> M_W<12> {
        M_W::new(self)
    }
    ///Bit 13 - USART enable
    #[inline(always)]
    #[must_use]
    pub fn ue(&mut self) -> UE_W<13> {
        UE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///UART4 CR1
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
