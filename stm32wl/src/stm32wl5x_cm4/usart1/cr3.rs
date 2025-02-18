///Register `CR3` reader
pub struct R(crate::R<CR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR3_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CR3` writer
pub struct W(crate::W<CR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR3_SPEC>;
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
impl From<crate::W<CR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR3_SPEC>) -> Self {
        W(writer)
    }
}
///Field `EIE` reader - Error interrupt enable
pub type EIE_R = crate::BitReader<EIE_A>;
///Error interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EIE_A {
    ///0: Interrupt is inhibited
    Disabled = 0,
    ///1: An interrupt is generated when FE=1 or ORE=1 or NF=1 in the ISR register
    Enabled = 1,
}
impl From<EIE_A> for bool {
    #[inline(always)]
    fn from(variant: EIE_A) -> Self {
        variant as u8 != 0
    }
}
impl EIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EIE_A {
        match self.bits {
            false => EIE_A::Disabled,
            true => EIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EIE_A::Enabled
    }
}
///Field `EIE` writer - Error interrupt enable
pub type EIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, EIE_A, O>;
impl<'a, const O: u8> EIE_W<'a, O> {
    ///Interrupt is inhibited
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EIE_A::Disabled)
    }
    ///An interrupt is generated when FE=1 or ORE=1 or NF=1 in the ISR register
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EIE_A::Enabled)
    }
}
///Field `IREN` reader - IrDA mode enable
pub type IREN_R = crate::BitReader<IREN_A>;
///IrDA mode enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IREN_A {
    ///0: IrDA disabled
    Disabled = 0,
    ///1: IrDA enabled
    Enabled = 1,
}
impl From<IREN_A> for bool {
    #[inline(always)]
    fn from(variant: IREN_A) -> Self {
        variant as u8 != 0
    }
}
impl IREN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> IREN_A {
        match self.bits {
            false => IREN_A::Disabled,
            true => IREN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IREN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IREN_A::Enabled
    }
}
///Field `IREN` writer - IrDA mode enable
pub type IREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, IREN_A, O>;
impl<'a, const O: u8> IREN_W<'a, O> {
    ///IrDA disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(IREN_A::Disabled)
    }
    ///IrDA enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(IREN_A::Enabled)
    }
}
///Field `IRLP` reader - IrDA low-power
pub type IRLP_R = crate::BitReader<IRLP_A>;
///IrDA low-power
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRLP_A {
    ///0: Normal mode
    Normal = 0,
    ///1: Low-power mode
    LowPower = 1,
}
impl From<IRLP_A> for bool {
    #[inline(always)]
    fn from(variant: IRLP_A) -> Self {
        variant as u8 != 0
    }
}
impl IRLP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> IRLP_A {
        match self.bits {
            false => IRLP_A::Normal,
            true => IRLP_A::LowPower,
        }
    }
    ///Checks if the value of the field is `Normal`
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == IRLP_A::Normal
    }
    ///Checks if the value of the field is `LowPower`
    #[inline(always)]
    pub fn is_low_power(&self) -> bool {
        *self == IRLP_A::LowPower
    }
}
///Field `IRLP` writer - IrDA low-power
pub type IRLP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, IRLP_A, O>;
impl<'a, const O: u8> IRLP_W<'a, O> {
    ///Normal mode
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(IRLP_A::Normal)
    }
    ///Low-power mode
    #[inline(always)]
    pub fn low_power(self) -> &'a mut W {
        self.variant(IRLP_A::LowPower)
    }
}
///Field `HDSEL` reader - Half-duplex selection
pub type HDSEL_R = crate::BitReader<HDSEL_A>;
///Half-duplex selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HDSEL_A {
    ///0: Half duplex mode is not selected
    NotSelected = 0,
    ///1: Half duplex mode is selected
    Selected = 1,
}
impl From<HDSEL_A> for bool {
    #[inline(always)]
    fn from(variant: HDSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl HDSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> HDSEL_A {
        match self.bits {
            false => HDSEL_A::NotSelected,
            true => HDSEL_A::Selected,
        }
    }
    ///Checks if the value of the field is `NotSelected`
    #[inline(always)]
    pub fn is_not_selected(&self) -> bool {
        *self == HDSEL_A::NotSelected
    }
    ///Checks if the value of the field is `Selected`
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == HDSEL_A::Selected
    }
}
///Field `HDSEL` writer - Half-duplex selection
pub type HDSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, HDSEL_A, O>;
impl<'a, const O: u8> HDSEL_W<'a, O> {
    ///Half duplex mode is not selected
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut W {
        self.variant(HDSEL_A::NotSelected)
    }
    ///Half duplex mode is selected
    #[inline(always)]
    pub fn selected(self) -> &'a mut W {
        self.variant(HDSEL_A::Selected)
    }
}
///Field `NACK` reader - Smartcard NACK enable
pub type NACK_R = crate::BitReader<NACK_A>;
///Smartcard NACK enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NACK_A {
    ///0: NACK transmission in case of parity error is disabled
    Disabled = 0,
    ///1: NACK transmission during parity error is enabled
    Enabled = 1,
}
impl From<NACK_A> for bool {
    #[inline(always)]
    fn from(variant: NACK_A) -> Self {
        variant as u8 != 0
    }
}
impl NACK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> NACK_A {
        match self.bits {
            false => NACK_A::Disabled,
            true => NACK_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == NACK_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == NACK_A::Enabled
    }
}
///Field `NACK` writer - Smartcard NACK enable
pub type NACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, NACK_A, O>;
impl<'a, const O: u8> NACK_W<'a, O> {
    ///NACK transmission in case of parity error is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(NACK_A::Disabled)
    }
    ///NACK transmission during parity error is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(NACK_A::Enabled)
    }
}
///Field `SCEN` reader - Smartcard mode enable
pub type SCEN_R = crate::BitReader<SCEN_A>;
///Smartcard mode enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCEN_A {
    ///0: Smartcard Mode disabled
    Disabled = 0,
    ///1: Smartcard Mode enabled
    Enabled = 1,
}
impl From<SCEN_A> for bool {
    #[inline(always)]
    fn from(variant: SCEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SCEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SCEN_A {
        match self.bits {
            false => SCEN_A::Disabled,
            true => SCEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SCEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SCEN_A::Enabled
    }
}
///Field `SCEN` writer - Smartcard mode enable
pub type SCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, SCEN_A, O>;
impl<'a, const O: u8> SCEN_W<'a, O> {
    ///Smartcard Mode disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SCEN_A::Disabled)
    }
    ///Smartcard Mode enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SCEN_A::Enabled)
    }
}
///Field `DMAR` reader - DMA enable receiver
pub type DMAR_R = crate::BitReader<DMAR_A>;
///DMA enable receiver
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAR_A {
    ///0: DMA mode is disabled for reception
    Disabled = 0,
    ///1: DMA mode is enabled for reception
    Enabled = 1,
}
impl From<DMAR_A> for bool {
    #[inline(always)]
    fn from(variant: DMAR_A) -> Self {
        variant as u8 != 0
    }
}
impl DMAR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DMAR_A {
        match self.bits {
            false => DMAR_A::Disabled,
            true => DMAR_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAR_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAR_A::Enabled
    }
}
///Field `DMAR` writer - DMA enable receiver
pub type DMAR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, DMAR_A, O>;
impl<'a, const O: u8> DMAR_W<'a, O> {
    ///DMA mode is disabled for reception
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMAR_A::Disabled)
    }
    ///DMA mode is enabled for reception
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMAR_A::Enabled)
    }
}
///Field `DMAT` reader - DMA enable transmitter
pub type DMAT_R = crate::BitReader<DMAT_A>;
///DMA enable transmitter
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAT_A {
    ///0: DMA mode is disabled for transmission
    Disabled = 0,
    ///1: DMA mode is enabled for transmission
    Enabled = 1,
}
impl From<DMAT_A> for bool {
    #[inline(always)]
    fn from(variant: DMAT_A) -> Self {
        variant as u8 != 0
    }
}
impl DMAT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DMAT_A {
        match self.bits {
            false => DMAT_A::Disabled,
            true => DMAT_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAT_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAT_A::Enabled
    }
}
///Field `DMAT` writer - DMA enable transmitter
pub type DMAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, DMAT_A, O>;
impl<'a, const O: u8> DMAT_W<'a, O> {
    ///DMA mode is disabled for transmission
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMAT_A::Disabled)
    }
    ///DMA mode is enabled for transmission
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMAT_A::Enabled)
    }
}
///Field `RTSE` reader - RTS enable
pub type RTSE_R = crate::BitReader<RTSE_A>;
///RTS enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTSE_A {
    ///0: RTS hardware flow control disabled
    Disabled = 0,
    ///1: RTS output enabled, data is only requested when there is space in the receive buffer
    Enabled = 1,
}
impl From<RTSE_A> for bool {
    #[inline(always)]
    fn from(variant: RTSE_A) -> Self {
        variant as u8 != 0
    }
}
impl RTSE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RTSE_A {
        match self.bits {
            false => RTSE_A::Disabled,
            true => RTSE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RTSE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RTSE_A::Enabled
    }
}
///Field `RTSE` writer - RTS enable
pub type RTSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, RTSE_A, O>;
impl<'a, const O: u8> RTSE_W<'a, O> {
    ///RTS hardware flow control disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RTSE_A::Disabled)
    }
    ///RTS output enabled, data is only requested when there is space in the receive buffer
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RTSE_A::Enabled)
    }
}
///Field `CTSE` reader - CTS enable
pub type CTSE_R = crate::BitReader<CTSE_A>;
///CTS enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSE_A {
    ///0: CTS hardware flow control disabled
    Disabled = 0,
    ///1: CTS mode enabled, data is only transmitted when the CTS input is asserted
    Enabled = 1,
}
impl From<CTSE_A> for bool {
    #[inline(always)]
    fn from(variant: CTSE_A) -> Self {
        variant as u8 != 0
    }
}
impl CTSE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CTSE_A {
        match self.bits {
            false => CTSE_A::Disabled,
            true => CTSE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CTSE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CTSE_A::Enabled
    }
}
///Field `CTSE` writer - CTS enable
pub type CTSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, CTSE_A, O>;
impl<'a, const O: u8> CTSE_W<'a, O> {
    ///CTS hardware flow control disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CTSE_A::Disabled)
    }
    ///CTS mode enabled, data is only transmitted when the CTS input is asserted
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CTSE_A::Enabled)
    }
}
///Field `CTSIE` reader - CTS interrupt enable
pub type CTSIE_R = crate::BitReader<CTSIE_A>;
///CTS interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSIE_A {
    ///0: Interrupt is inhibited
    Disabled = 0,
    ///1: An interrupt is generated whenever CTSIF=1 in the ISR register
    Enabled = 1,
}
impl From<CTSIE_A> for bool {
    #[inline(always)]
    fn from(variant: CTSIE_A) -> Self {
        variant as u8 != 0
    }
}
impl CTSIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CTSIE_A {
        match self.bits {
            false => CTSIE_A::Disabled,
            true => CTSIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CTSIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CTSIE_A::Enabled
    }
}
///Field `CTSIE` writer - CTS interrupt enable
pub type CTSIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, CTSIE_A, O>;
impl<'a, const O: u8> CTSIE_W<'a, O> {
    ///Interrupt is inhibited
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CTSIE_A::Disabled)
    }
    ///An interrupt is generated whenever CTSIF=1 in the ISR register
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CTSIE_A::Enabled)
    }
}
///Field `ONEBIT` reader - One sample bit method enable
pub type ONEBIT_R = crate::BitReader<ONEBIT_A>;
///One sample bit method enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ONEBIT_A {
    ///0: Three sample bit method
    Sample3 = 0,
    ///1: One sample bit method
    Sample1 = 1,
}
impl From<ONEBIT_A> for bool {
    #[inline(always)]
    fn from(variant: ONEBIT_A) -> Self {
        variant as u8 != 0
    }
}
impl ONEBIT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ONEBIT_A {
        match self.bits {
            false => ONEBIT_A::Sample3,
            true => ONEBIT_A::Sample1,
        }
    }
    ///Checks if the value of the field is `Sample3`
    #[inline(always)]
    pub fn is_sample3(&self) -> bool {
        *self == ONEBIT_A::Sample3
    }
    ///Checks if the value of the field is `Sample1`
    #[inline(always)]
    pub fn is_sample1(&self) -> bool {
        *self == ONEBIT_A::Sample1
    }
}
///Field `ONEBIT` writer - One sample bit method enable
pub type ONEBIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, ONEBIT_A, O>;
impl<'a, const O: u8> ONEBIT_W<'a, O> {
    ///Three sample bit method
    #[inline(always)]
    pub fn sample3(self) -> &'a mut W {
        self.variant(ONEBIT_A::Sample3)
    }
    ///One sample bit method
    #[inline(always)]
    pub fn sample1(self) -> &'a mut W {
        self.variant(ONEBIT_A::Sample1)
    }
}
///Field `OVRDIS` reader - OVRDIS: Overrun Disable
pub type OVRDIS_R = crate::BitReader<OVRDIS_A>;
///OVRDIS: Overrun Disable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRDIS_A {
    ///0: Overrun Error Flag, ORE, is set when received data is not read before receiving new data
    Enabled = 0,
    ///1: Overrun functionality is disabled. If new data is received while the RXNE flag is still set the ORE flag is not set and the new received data overwrites the previous content of the RDR register
    Disabled = 1,
}
impl From<OVRDIS_A> for bool {
    #[inline(always)]
    fn from(variant: OVRDIS_A) -> Self {
        variant as u8 != 0
    }
}
impl OVRDIS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OVRDIS_A {
        match self.bits {
            false => OVRDIS_A::Enabled,
            true => OVRDIS_A::Disabled,
        }
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OVRDIS_A::Enabled
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OVRDIS_A::Disabled
    }
}
///Field `OVRDIS` writer - OVRDIS: Overrun Disable
pub type OVRDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, OVRDIS_A, O>;
impl<'a, const O: u8> OVRDIS_W<'a, O> {
    ///Overrun Error Flag, ORE, is set when received data is not read before receiving new data
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OVRDIS_A::Enabled)
    }
    ///Overrun functionality is disabled. If new data is received while the RXNE flag is still set the ORE flag is not set and the new received data overwrites the previous content of the RDR register
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OVRDIS_A::Disabled)
    }
}
///Field `DDRE` reader - DMA Disable on Reception Error
pub type DDRE_R = crate::BitReader<DDRE_A>;
///DMA Disable on Reception Error
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DDRE_A {
    ///0: DMA is not disabled in case of reception error
    NotDisabled = 0,
    ///1: DMA is disabled following a reception error
    Disabled = 1,
}
impl From<DDRE_A> for bool {
    #[inline(always)]
    fn from(variant: DDRE_A) -> Self {
        variant as u8 != 0
    }
}
impl DDRE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DDRE_A {
        match self.bits {
            false => DDRE_A::NotDisabled,
            true => DDRE_A::Disabled,
        }
    }
    ///Checks if the value of the field is `NotDisabled`
    #[inline(always)]
    pub fn is_not_disabled(&self) -> bool {
        *self == DDRE_A::NotDisabled
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DDRE_A::Disabled
    }
}
///Field `DDRE` writer - DMA Disable on Reception Error
pub type DDRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, DDRE_A, O>;
impl<'a, const O: u8> DDRE_W<'a, O> {
    ///DMA is not disabled in case of reception error
    #[inline(always)]
    pub fn not_disabled(self) -> &'a mut W {
        self.variant(DDRE_A::NotDisabled)
    }
    ///DMA is disabled following a reception error
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DDRE_A::Disabled)
    }
}
///Field `DEM` reader - Driver enable mode
pub type DEM_R = crate::BitReader<DEM_A>;
///Driver enable mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DEM_A {
    ///0: DE function is disabled
    Disabled = 0,
    ///1: The DE signal is output on the RTS pin
    Enabled = 1,
}
impl From<DEM_A> for bool {
    #[inline(always)]
    fn from(variant: DEM_A) -> Self {
        variant as u8 != 0
    }
}
impl DEM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DEM_A {
        match self.bits {
            false => DEM_A::Disabled,
            true => DEM_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DEM_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DEM_A::Enabled
    }
}
///Field `DEM` writer - Driver enable mode
pub type DEM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, DEM_A, O>;
impl<'a, const O: u8> DEM_W<'a, O> {
    ///DE function is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DEM_A::Disabled)
    }
    ///The DE signal is output on the RTS pin
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DEM_A::Enabled)
    }
}
///Field `DEP` reader - Driver enable polarity selection
pub type DEP_R = crate::BitReader<DEP_A>;
///Driver enable polarity selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DEP_A {
    ///0: DE signal is active high
    High = 0,
    ///1: DE signal is active low
    Low = 1,
}
impl From<DEP_A> for bool {
    #[inline(always)]
    fn from(variant: DEP_A) -> Self {
        variant as u8 != 0
    }
}
impl DEP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DEP_A {
        match self.bits {
            false => DEP_A::High,
            true => DEP_A::Low,
        }
    }
    ///Checks if the value of the field is `High`
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == DEP_A::High
    }
    ///Checks if the value of the field is `Low`
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == DEP_A::Low
    }
}
///Field `DEP` writer - Driver enable polarity selection
pub type DEP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, DEP_A, O>;
impl<'a, const O: u8> DEP_W<'a, O> {
    ///DE signal is active high
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(DEP_A::High)
    }
    ///DE signal is active low
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(DEP_A::Low)
    }
}
///Field `SCARCNT` reader - Smartcard auto-retry count
pub type SCARCNT_R = crate::FieldReader<u8, u8>;
///Field `SCARCNT` writer - Smartcard auto-retry count
pub type SCARCNT_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CR3_SPEC, u8, u8, 3, O>;
///Field `WUS` reader - Wakeup from low-power mode interrupt flag selection
pub type WUS_R = crate::FieldReader<u8, WUS_A>;
///Wakeup from low-power mode interrupt flag selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WUS_A {
    ///0: WUF active on address match
    Address = 0,
    ///2: WuF active on Start bit detection
    Start = 2,
    ///3: WUF active on RXNE
    Rxne = 3,
}
impl From<WUS_A> for u8 {
    #[inline(always)]
    fn from(variant: WUS_A) -> Self {
        variant as _
    }
}
impl WUS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<WUS_A> {
        match self.bits {
            0 => Some(WUS_A::Address),
            2 => Some(WUS_A::Start),
            3 => Some(WUS_A::Rxne),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Address`
    #[inline(always)]
    pub fn is_address(&self) -> bool {
        *self == WUS_A::Address
    }
    ///Checks if the value of the field is `Start`
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == WUS_A::Start
    }
    ///Checks if the value of the field is `Rxne`
    #[inline(always)]
    pub fn is_rxne(&self) -> bool {
        *self == WUS_A::Rxne
    }
}
///Field `WUS` writer - Wakeup from low-power mode interrupt flag selection
pub type WUS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR3_SPEC, u8, WUS_A, 2, O>;
impl<'a, const O: u8> WUS_W<'a, O> {
    ///WUF active on address match
    #[inline(always)]
    pub fn address(self) -> &'a mut W {
        self.variant(WUS_A::Address)
    }
    ///WuF active on Start bit detection
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(WUS_A::Start)
    }
    ///WUF active on RXNE
    #[inline(always)]
    pub fn rxne(self) -> &'a mut W {
        self.variant(WUS_A::Rxne)
    }
}
///Field `WUFIE` reader - Wakeup from low-power mode interrupt enable
pub type WUFIE_R = crate::BitReader<WUFIE_A>;
///Wakeup from low-power mode interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUFIE_A {
    ///0: Interrupt is inhibited
    Disabled = 0,
    ///1: An USART interrupt is generated whenever WUF=1 in the ISR register
    Enabled = 1,
}
impl From<WUFIE_A> for bool {
    #[inline(always)]
    fn from(variant: WUFIE_A) -> Self {
        variant as u8 != 0
    }
}
impl WUFIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> WUFIE_A {
        match self.bits {
            false => WUFIE_A::Disabled,
            true => WUFIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WUFIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WUFIE_A::Enabled
    }
}
///Field `WUFIE` writer - Wakeup from low-power mode interrupt enable
pub type WUFIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, WUFIE_A, O>;
impl<'a, const O: u8> WUFIE_W<'a, O> {
    ///Interrupt is inhibited
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WUFIE_A::Disabled)
    }
    ///An USART interrupt is generated whenever WUF=1 in the ISR register
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WUFIE_A::Enabled)
    }
}
///Field `TXFTIE` reader - TXFIFO threshold interrupt enable
pub type TXFTIE_R = crate::BitReader<TXFTIE_A>;
///TXFIFO threshold interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXFTIE_A {
    ///0: Interrupt inhibited
    Disabled = 0,
    ///1: USART interrupt generated when Transmit FIFO reaches the threshold programmed in TXFTCFG
    Enabled = 1,
}
impl From<TXFTIE_A> for bool {
    #[inline(always)]
    fn from(variant: TXFTIE_A) -> Self {
        variant as u8 != 0
    }
}
impl TXFTIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TXFTIE_A {
        match self.bits {
            false => TXFTIE_A::Disabled,
            true => TXFTIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXFTIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXFTIE_A::Enabled
    }
}
///Field `TXFTIE` writer - TXFIFO threshold interrupt enable
pub type TXFTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, TXFTIE_A, O>;
impl<'a, const O: u8> TXFTIE_W<'a, O> {
    ///Interrupt inhibited
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TXFTIE_A::Disabled)
    }
    ///USART interrupt generated when Transmit FIFO reaches the threshold programmed in TXFTCFG
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TXFTIE_A::Enabled)
    }
}
///Field `TCBGTIE` reader - Transmission Complete before guard time, interrupt enable
pub type TCBGTIE_R = crate::BitReader<TCBGTIE_A>;
///Transmission Complete before guard time, interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCBGTIE_A {
    ///0: Interrupt inhibited
    Disabled = 0,
    ///1: USART interrupt generated whenever TCBGT=1 in the USART_ISR register
    Enabled = 1,
}
impl From<TCBGTIE_A> for bool {
    #[inline(always)]
    fn from(variant: TCBGTIE_A) -> Self {
        variant as u8 != 0
    }
}
impl TCBGTIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TCBGTIE_A {
        match self.bits {
            false => TCBGTIE_A::Disabled,
            true => TCBGTIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TCBGTIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TCBGTIE_A::Enabled
    }
}
///Field `TCBGTIE` writer - Transmission Complete before guard time, interrupt enable
pub type TCBGTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, TCBGTIE_A, O>;
impl<'a, const O: u8> TCBGTIE_W<'a, O> {
    ///Interrupt inhibited
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TCBGTIE_A::Disabled)
    }
    ///USART interrupt generated whenever TCBGT=1 in the USART_ISR register
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TCBGTIE_A::Enabled)
    }
}
///Field `RXFTCFG` reader - Receive FIFO threshold configuration
pub type RXFTCFG_R = crate::FieldReader<u8, RXFTCFG_A>;
///Receive FIFO threshold configuration
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RXFTCFG_A {
    ///0: RXFIFO reaches 1/8 of its depth
    Depth18 = 0,
    ///1: RXFIFO reaches 1/4 of its depth
    Depth14 = 1,
    ///2: RXFIFO reaches 1/2 of its depth
    Depth12 = 2,
    ///3: RXFIFO reaches 3/4 of its depth
    Depth34 = 3,
    ///4: RXFIFO reaches 7/8 of its depth
    Depth78 = 4,
    ///5: RXFIFO becomes full
    Full = 5,
}
impl From<RXFTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: RXFTCFG_A) -> Self {
        variant as _
    }
}
impl RXFTCFG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<RXFTCFG_A> {
        match self.bits {
            0 => Some(RXFTCFG_A::Depth18),
            1 => Some(RXFTCFG_A::Depth14),
            2 => Some(RXFTCFG_A::Depth12),
            3 => Some(RXFTCFG_A::Depth34),
            4 => Some(RXFTCFG_A::Depth78),
            5 => Some(RXFTCFG_A::Full),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Depth18`
    #[inline(always)]
    pub fn is_depth_1_8(&self) -> bool {
        *self == RXFTCFG_A::Depth18
    }
    ///Checks if the value of the field is `Depth14`
    #[inline(always)]
    pub fn is_depth_1_4(&self) -> bool {
        *self == RXFTCFG_A::Depth14
    }
    ///Checks if the value of the field is `Depth12`
    #[inline(always)]
    pub fn is_depth_1_2(&self) -> bool {
        *self == RXFTCFG_A::Depth12
    }
    ///Checks if the value of the field is `Depth34`
    #[inline(always)]
    pub fn is_depth_3_4(&self) -> bool {
        *self == RXFTCFG_A::Depth34
    }
    ///Checks if the value of the field is `Depth78`
    #[inline(always)]
    pub fn is_depth_7_8(&self) -> bool {
        *self == RXFTCFG_A::Depth78
    }
    ///Checks if the value of the field is `Full`
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == RXFTCFG_A::Full
    }
}
///Field `RXFTCFG` writer - Receive FIFO threshold configuration
pub type RXFTCFG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR3_SPEC, u8, RXFTCFG_A, 3, O>;
impl<'a, const O: u8> RXFTCFG_W<'a, O> {
    ///RXFIFO reaches 1/8 of its depth
    #[inline(always)]
    pub fn depth_1_8(self) -> &'a mut W {
        self.variant(RXFTCFG_A::Depth18)
    }
    ///RXFIFO reaches 1/4 of its depth
    #[inline(always)]
    pub fn depth_1_4(self) -> &'a mut W {
        self.variant(RXFTCFG_A::Depth14)
    }
    ///RXFIFO reaches 1/2 of its depth
    #[inline(always)]
    pub fn depth_1_2(self) -> &'a mut W {
        self.variant(RXFTCFG_A::Depth12)
    }
    ///RXFIFO reaches 3/4 of its depth
    #[inline(always)]
    pub fn depth_3_4(self) -> &'a mut W {
        self.variant(RXFTCFG_A::Depth34)
    }
    ///RXFIFO reaches 7/8 of its depth
    #[inline(always)]
    pub fn depth_7_8(self) -> &'a mut W {
        self.variant(RXFTCFG_A::Depth78)
    }
    ///RXFIFO becomes full
    #[inline(always)]
    pub fn full(self) -> &'a mut W {
        self.variant(RXFTCFG_A::Full)
    }
}
///Field `RXFTIE` reader - RXFIFO threshold interrupt enable
pub type RXFTIE_R = crate::BitReader<RXFTIE_A>;
///RXFIFO threshold interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXFTIE_A {
    ///0: Interrupt inhibited
    Disabled = 0,
    ///1: USART interrupt generated when Receive FIFO reaches the threshold programmed in RXFTCFG
    Enabled = 1,
}
impl From<RXFTIE_A> for bool {
    #[inline(always)]
    fn from(variant: RXFTIE_A) -> Self {
        variant as u8 != 0
    }
}
impl RXFTIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RXFTIE_A {
        match self.bits {
            false => RXFTIE_A::Disabled,
            true => RXFTIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXFTIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXFTIE_A::Enabled
    }
}
///Field `RXFTIE` writer - RXFIFO threshold interrupt enable
pub type RXFTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, RXFTIE_A, O>;
impl<'a, const O: u8> RXFTIE_W<'a, O> {
    ///Interrupt inhibited
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RXFTIE_A::Disabled)
    }
    ///USART interrupt generated when Receive FIFO reaches the threshold programmed in RXFTCFG
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RXFTIE_A::Enabled)
    }
}
///Field `TXFTCFG` reader - TXFIFO threshold configuration
pub type TXFTCFG_R = crate::FieldReader<u8, TXFTCFG_A>;
///TXFIFO threshold configuration
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TXFTCFG_A {
    ///0: TXFIFO reaches 1/8 of its depth
    Depth18 = 0,
    ///1: TXFIFO reaches 1/4 of its depth
    Depth14 = 1,
    ///2: TXFIFO reaches 1/2 of its depth
    Depth12 = 2,
    ///3: TXFIFO reaches 3/4 of its depth
    Depth34 = 3,
    ///4: TXFIFO reaches 7/8 of its depth
    Depth78 = 4,
    ///5: TXFIFO becomes empty
    Empty = 5,
}
impl From<TXFTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: TXFTCFG_A) -> Self {
        variant as _
    }
}
impl TXFTCFG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<TXFTCFG_A> {
        match self.bits {
            0 => Some(TXFTCFG_A::Depth18),
            1 => Some(TXFTCFG_A::Depth14),
            2 => Some(TXFTCFG_A::Depth12),
            3 => Some(TXFTCFG_A::Depth34),
            4 => Some(TXFTCFG_A::Depth78),
            5 => Some(TXFTCFG_A::Empty),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Depth18`
    #[inline(always)]
    pub fn is_depth_1_8(&self) -> bool {
        *self == TXFTCFG_A::Depth18
    }
    ///Checks if the value of the field is `Depth14`
    #[inline(always)]
    pub fn is_depth_1_4(&self) -> bool {
        *self == TXFTCFG_A::Depth14
    }
    ///Checks if the value of the field is `Depth12`
    #[inline(always)]
    pub fn is_depth_1_2(&self) -> bool {
        *self == TXFTCFG_A::Depth12
    }
    ///Checks if the value of the field is `Depth34`
    #[inline(always)]
    pub fn is_depth_3_4(&self) -> bool {
        *self == TXFTCFG_A::Depth34
    }
    ///Checks if the value of the field is `Depth78`
    #[inline(always)]
    pub fn is_depth_7_8(&self) -> bool {
        *self == TXFTCFG_A::Depth78
    }
    ///Checks if the value of the field is `Empty`
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == TXFTCFG_A::Empty
    }
}
///Field `TXFTCFG` writer - TXFIFO threshold configuration
pub type TXFTCFG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR3_SPEC, u8, TXFTCFG_A, 3, O>;
impl<'a, const O: u8> TXFTCFG_W<'a, O> {
    ///TXFIFO reaches 1/8 of its depth
    #[inline(always)]
    pub fn depth_1_8(self) -> &'a mut W {
        self.variant(TXFTCFG_A::Depth18)
    }
    ///TXFIFO reaches 1/4 of its depth
    #[inline(always)]
    pub fn depth_1_4(self) -> &'a mut W {
        self.variant(TXFTCFG_A::Depth14)
    }
    ///TXFIFO reaches 1/2 of its depth
    #[inline(always)]
    pub fn depth_1_2(self) -> &'a mut W {
        self.variant(TXFTCFG_A::Depth12)
    }
    ///TXFIFO reaches 3/4 of its depth
    #[inline(always)]
    pub fn depth_3_4(self) -> &'a mut W {
        self.variant(TXFTCFG_A::Depth34)
    }
    ///TXFIFO reaches 7/8 of its depth
    #[inline(always)]
    pub fn depth_7_8(self) -> &'a mut W {
        self.variant(TXFTCFG_A::Depth78)
    }
    ///TXFIFO becomes empty
    #[inline(always)]
    pub fn empty(self) -> &'a mut W {
        self.variant(TXFTCFG_A::Empty)
    }
}
impl R {
    ///Bit 0 - Error interrupt enable
    #[inline(always)]
    pub fn eie(&self) -> EIE_R {
        EIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - IrDA mode enable
    #[inline(always)]
    pub fn iren(&self) -> IREN_R {
        IREN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - IrDA low-power
    #[inline(always)]
    pub fn irlp(&self) -> IRLP_R {
        IRLP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Half-duplex selection
    #[inline(always)]
    pub fn hdsel(&self) -> HDSEL_R {
        HDSEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Smartcard NACK enable
    #[inline(always)]
    pub fn nack(&self) -> NACK_R {
        NACK_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Smartcard mode enable
    #[inline(always)]
    pub fn scen(&self) -> SCEN_R {
        SCEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - DMA enable receiver
    #[inline(always)]
    pub fn dmar(&self) -> DMAR_R {
        DMAR_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - DMA enable transmitter
    #[inline(always)]
    pub fn dmat(&self) -> DMAT_R {
        DMAT_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - RTS enable
    #[inline(always)]
    pub fn rtse(&self) -> RTSE_R {
        RTSE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - CTS enable
    #[inline(always)]
    pub fn ctse(&self) -> CTSE_R {
        CTSE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - CTS interrupt enable
    #[inline(always)]
    pub fn ctsie(&self) -> CTSIE_R {
        CTSIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - One sample bit method enable
    #[inline(always)]
    pub fn onebit(&self) -> ONEBIT_R {
        ONEBIT_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - OVRDIS: Overrun Disable
    #[inline(always)]
    pub fn ovrdis(&self) -> OVRDIS_R {
        OVRDIS_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - DMA Disable on Reception Error
    #[inline(always)]
    pub fn ddre(&self) -> DDRE_R {
        DDRE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Driver enable mode
    #[inline(always)]
    pub fn dem(&self) -> DEM_R {
        DEM_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Driver enable polarity selection
    #[inline(always)]
    pub fn dep(&self) -> DEP_R {
        DEP_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 17:19 - Smartcard auto-retry count
    #[inline(always)]
    pub fn scarcnt(&self) -> SCARCNT_R {
        SCARCNT_R::new(((self.bits >> 17) & 7) as u8)
    }
    ///Bits 20:21 - Wakeup from low-power mode interrupt flag selection
    #[inline(always)]
    pub fn wus(&self) -> WUS_R {
        WUS_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bit 22 - Wakeup from low-power mode interrupt enable
    #[inline(always)]
    pub fn wufie(&self) -> WUFIE_R {
        WUFIE_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - TXFIFO threshold interrupt enable
    #[inline(always)]
    pub fn txftie(&self) -> TXFTIE_R {
        TXFTIE_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Transmission Complete before guard time, interrupt enable
    #[inline(always)]
    pub fn tcbgtie(&self) -> TCBGTIE_R {
        TCBGTIE_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bits 25:27 - Receive FIFO threshold configuration
    #[inline(always)]
    pub fn rxftcfg(&self) -> RXFTCFG_R {
        RXFTCFG_R::new(((self.bits >> 25) & 7) as u8)
    }
    ///Bit 28 - RXFIFO threshold interrupt enable
    #[inline(always)]
    pub fn rxftie(&self) -> RXFTIE_R {
        RXFTIE_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bits 29:31 - TXFIFO threshold configuration
    #[inline(always)]
    pub fn txftcfg(&self) -> TXFTCFG_R {
        TXFTCFG_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    ///Bit 0 - Error interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn eie(&mut self) -> EIE_W<0> {
        EIE_W::new(self)
    }
    ///Bit 1 - IrDA mode enable
    #[inline(always)]
    #[must_use]
    pub fn iren(&mut self) -> IREN_W<1> {
        IREN_W::new(self)
    }
    ///Bit 2 - IrDA low-power
    #[inline(always)]
    #[must_use]
    pub fn irlp(&mut self) -> IRLP_W<2> {
        IRLP_W::new(self)
    }
    ///Bit 3 - Half-duplex selection
    #[inline(always)]
    #[must_use]
    pub fn hdsel(&mut self) -> HDSEL_W<3> {
        HDSEL_W::new(self)
    }
    ///Bit 4 - Smartcard NACK enable
    #[inline(always)]
    #[must_use]
    pub fn nack(&mut self) -> NACK_W<4> {
        NACK_W::new(self)
    }
    ///Bit 5 - Smartcard mode enable
    #[inline(always)]
    #[must_use]
    pub fn scen(&mut self) -> SCEN_W<5> {
        SCEN_W::new(self)
    }
    ///Bit 6 - DMA enable receiver
    #[inline(always)]
    #[must_use]
    pub fn dmar(&mut self) -> DMAR_W<6> {
        DMAR_W::new(self)
    }
    ///Bit 7 - DMA enable transmitter
    #[inline(always)]
    #[must_use]
    pub fn dmat(&mut self) -> DMAT_W<7> {
        DMAT_W::new(self)
    }
    ///Bit 8 - RTS enable
    #[inline(always)]
    #[must_use]
    pub fn rtse(&mut self) -> RTSE_W<8> {
        RTSE_W::new(self)
    }
    ///Bit 9 - CTS enable
    #[inline(always)]
    #[must_use]
    pub fn ctse(&mut self) -> CTSE_W<9> {
        CTSE_W::new(self)
    }
    ///Bit 10 - CTS interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn ctsie(&mut self) -> CTSIE_W<10> {
        CTSIE_W::new(self)
    }
    ///Bit 11 - One sample bit method enable
    #[inline(always)]
    #[must_use]
    pub fn onebit(&mut self) -> ONEBIT_W<11> {
        ONEBIT_W::new(self)
    }
    ///Bit 12 - OVRDIS: Overrun Disable
    #[inline(always)]
    #[must_use]
    pub fn ovrdis(&mut self) -> OVRDIS_W<12> {
        OVRDIS_W::new(self)
    }
    ///Bit 13 - DMA Disable on Reception Error
    #[inline(always)]
    #[must_use]
    pub fn ddre(&mut self) -> DDRE_W<13> {
        DDRE_W::new(self)
    }
    ///Bit 14 - Driver enable mode
    #[inline(always)]
    #[must_use]
    pub fn dem(&mut self) -> DEM_W<14> {
        DEM_W::new(self)
    }
    ///Bit 15 - Driver enable polarity selection
    #[inline(always)]
    #[must_use]
    pub fn dep(&mut self) -> DEP_W<15> {
        DEP_W::new(self)
    }
    ///Bits 17:19 - Smartcard auto-retry count
    #[inline(always)]
    #[must_use]
    pub fn scarcnt(&mut self) -> SCARCNT_W<17> {
        SCARCNT_W::new(self)
    }
    ///Bits 20:21 - Wakeup from low-power mode interrupt flag selection
    #[inline(always)]
    #[must_use]
    pub fn wus(&mut self) -> WUS_W<20> {
        WUS_W::new(self)
    }
    ///Bit 22 - Wakeup from low-power mode interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn wufie(&mut self) -> WUFIE_W<22> {
        WUFIE_W::new(self)
    }
    ///Bit 23 - TXFIFO threshold interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn txftie(&mut self) -> TXFTIE_W<23> {
        TXFTIE_W::new(self)
    }
    ///Bit 24 - Transmission Complete before guard time, interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn tcbgtie(&mut self) -> TCBGTIE_W<24> {
        TCBGTIE_W::new(self)
    }
    ///Bits 25:27 - Receive FIFO threshold configuration
    #[inline(always)]
    #[must_use]
    pub fn rxftcfg(&mut self) -> RXFTCFG_W<25> {
        RXFTCFG_W::new(self)
    }
    ///Bit 28 - RXFIFO threshold interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn rxftie(&mut self) -> RXFTIE_W<28> {
        RXFTIE_W::new(self)
    }
    ///Bits 29:31 - TXFIFO threshold configuration
    #[inline(always)]
    #[must_use]
    pub fn txftcfg(&mut self) -> TXFTCFG_W<29> {
        TXFTCFG_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///control register 3
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr3](index.html) module
pub struct CR3_SPEC;
impl crate::RegisterSpec for CR3_SPEC {
    type Ux = u32;
}
///`read()` method returns [cr3::R](R) reader structure
impl crate::Readable for CR3_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cr3::W](W) writer structure
impl crate::Writable for CR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CR3 to value 0
impl crate::Resettable for CR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
