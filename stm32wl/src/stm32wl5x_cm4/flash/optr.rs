///Register `OPTR` reader
pub struct R(crate::R<OPTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPTR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `OPTR` writer
pub struct W(crate::W<OPTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPTR_SPEC>;
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
impl From<crate::W<OPTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPTR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `RDP` reader - Read protection level
pub type RDP_R = crate::FieldReader<u8, RDP_A>;
///Read protection level
///
///Value on reset: 170
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RDP_A {
    ///136: Level 1, memories readout protection active (writes 0x88)
    Level1 = 136,
    ///170: Level 0, readout protection not active
    Level0 = 170,
    ///204: Level 2, chip readout protection active
    Level2 = 204,
}
impl From<RDP_A> for u8 {
    #[inline(always)]
    fn from(variant: RDP_A) -> Self {
        variant as _
    }
}
impl RDP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<RDP_A> {
        match self.bits {
            136 => Some(RDP_A::Level1),
            170 => Some(RDP_A::Level0),
            204 => Some(RDP_A::Level2),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Level1`
    #[inline(always)]
    pub fn is_level1(&self) -> bool {
        *self == RDP_A::Level1
    }
    ///Checks if the value of the field is `Level0`
    #[inline(always)]
    pub fn is_level0(&self) -> bool {
        *self == RDP_A::Level0
    }
    ///Checks if the value of the field is `Level2`
    #[inline(always)]
    pub fn is_level2(&self) -> bool {
        *self == RDP_A::Level2
    }
}
///Field `RDP` writer - Read protection level
pub type RDP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPTR_SPEC, u8, RDP_A, 8, O>;
impl<'a, const O: u8> RDP_W<'a, O> {
    ///Level 1, memories readout protection active (writes 0x88)
    #[inline(always)]
    pub fn level1(self) -> &'a mut W {
        self.variant(RDP_A::Level1)
    }
    ///Level 0, readout protection not active
    #[inline(always)]
    pub fn level0(self) -> &'a mut W {
        self.variant(RDP_A::Level0)
    }
    ///Level 2, chip readout protection active
    #[inline(always)]
    pub fn level2(self) -> &'a mut W {
        self.variant(RDP_A::Level2)
    }
}
///Field `ESE` reader - System security enabled flag
pub type ESE_R = crate::BitReader<ESE_A>;
///System security enabled flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ESE_A {
    ///0: Security disabled
    Disabled = 0,
    ///1: Security enabled
    Enabled = 1,
}
impl From<ESE_A> for bool {
    #[inline(always)]
    fn from(variant: ESE_A) -> Self {
        variant as u8 != 0
    }
}
impl ESE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ESE_A {
        match self.bits {
            false => ESE_A::Disabled,
            true => ESE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ESE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ESE_A::Enabled
    }
}
///Field `ESE` writer - System security enabled flag
pub type ESE_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTR_SPEC, ESE_A, O>;
impl<'a, const O: u8> ESE_W<'a, O> {
    ///Security disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ESE_A::Disabled)
    }
    ///Security enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ESE_A::Enabled)
    }
}
///Field `BOR_LEV` reader - BOR reset Level
pub type BOR_LEV_R = crate::FieldReader<u8, BOR_LEV_A>;
///BOR reset Level
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BOR_LEV_A {
    ///0: BOR level 0. Reset level threshold is around 1.7 V
    Level0 = 0,
    ///1: BOR level 1. Reset level threshold is around 2.0 V
    Level1 = 1,
    ///2: BOR level 2. Reset level threshold is around 2.2 V
    Level2 = 2,
    ///3: BOR level 3. Reset level threshold is around 2.5 V
    Level3 = 3,
    ///4: BOR level 4. Reset level threshold is around 2.8 V
    Level4 = 4,
}
impl From<BOR_LEV_A> for u8 {
    #[inline(always)]
    fn from(variant: BOR_LEV_A) -> Self {
        variant as _
    }
}
impl BOR_LEV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<BOR_LEV_A> {
        match self.bits {
            0 => Some(BOR_LEV_A::Level0),
            1 => Some(BOR_LEV_A::Level1),
            2 => Some(BOR_LEV_A::Level2),
            3 => Some(BOR_LEV_A::Level3),
            4 => Some(BOR_LEV_A::Level4),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Level0`
    #[inline(always)]
    pub fn is_level0(&self) -> bool {
        *self == BOR_LEV_A::Level0
    }
    ///Checks if the value of the field is `Level1`
    #[inline(always)]
    pub fn is_level1(&self) -> bool {
        *self == BOR_LEV_A::Level1
    }
    ///Checks if the value of the field is `Level2`
    #[inline(always)]
    pub fn is_level2(&self) -> bool {
        *self == BOR_LEV_A::Level2
    }
    ///Checks if the value of the field is `Level3`
    #[inline(always)]
    pub fn is_level3(&self) -> bool {
        *self == BOR_LEV_A::Level3
    }
    ///Checks if the value of the field is `Level4`
    #[inline(always)]
    pub fn is_level4(&self) -> bool {
        *self == BOR_LEV_A::Level4
    }
}
///Field `BOR_LEV` writer - BOR reset Level
pub type BOR_LEV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPTR_SPEC, u8, BOR_LEV_A, 3, O>;
impl<'a, const O: u8> BOR_LEV_W<'a, O> {
    ///BOR level 0. Reset level threshold is around 1.7 V
    #[inline(always)]
    pub fn level0(self) -> &'a mut W {
        self.variant(BOR_LEV_A::Level0)
    }
    ///BOR level 1. Reset level threshold is around 2.0 V
    #[inline(always)]
    pub fn level1(self) -> &'a mut W {
        self.variant(BOR_LEV_A::Level1)
    }
    ///BOR level 2. Reset level threshold is around 2.2 V
    #[inline(always)]
    pub fn level2(self) -> &'a mut W {
        self.variant(BOR_LEV_A::Level2)
    }
    ///BOR level 3. Reset level threshold is around 2.5 V
    #[inline(always)]
    pub fn level3(self) -> &'a mut W {
        self.variant(BOR_LEV_A::Level3)
    }
    ///BOR level 4. Reset level threshold is around 2.8 V
    #[inline(always)]
    pub fn level4(self) -> &'a mut W {
        self.variant(BOR_LEV_A::Level4)
    }
}
///Field `nRST_STOP` reader - nRST_STOP
pub type N_RST_STOP_R = crate::BitReader<N_RST_STOP_A>;
///nRST_STOP
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum N_RST_STOP_A {
    ///0: Reset generated when entering the Standby mode
    Enabled = 0,
    ///1: No reset generated when entering the Standby mode
    Disabled = 1,
}
impl From<N_RST_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: N_RST_STOP_A) -> Self {
        variant as u8 != 0
    }
}
impl N_RST_STOP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> N_RST_STOP_A {
        match self.bits {
            false => N_RST_STOP_A::Enabled,
            true => N_RST_STOP_A::Disabled,
        }
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == N_RST_STOP_A::Enabled
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == N_RST_STOP_A::Disabled
    }
}
///Field `nRST_STOP` writer - nRST_STOP
pub type N_RST_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTR_SPEC, N_RST_STOP_A, O>;
impl<'a, const O: u8> N_RST_STOP_W<'a, O> {
    ///Reset generated when entering the Standby mode
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(N_RST_STOP_A::Enabled)
    }
    ///No reset generated when entering the Standby mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(N_RST_STOP_A::Disabled)
    }
}
///Field `nRST_STDBY` reader - nRST_STDBY
pub type N_RST_STDBY_R = crate::BitReader<N_RST_STDBY_A>;
///nRST_STDBY
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum N_RST_STDBY_A {
    ///0: Reset generated when entering the Standby mode
    Enabled = 0,
    ///1: No reset generated when entering the Standby mode
    Disabled = 1,
}
impl From<N_RST_STDBY_A> for bool {
    #[inline(always)]
    fn from(variant: N_RST_STDBY_A) -> Self {
        variant as u8 != 0
    }
}
impl N_RST_STDBY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> N_RST_STDBY_A {
        match self.bits {
            false => N_RST_STDBY_A::Enabled,
            true => N_RST_STDBY_A::Disabled,
        }
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == N_RST_STDBY_A::Enabled
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == N_RST_STDBY_A::Disabled
    }
}
///Field `nRST_STDBY` writer - nRST_STDBY
pub type N_RST_STDBY_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTR_SPEC, N_RST_STDBY_A, O>;
impl<'a, const O: u8> N_RST_STDBY_W<'a, O> {
    ///Reset generated when entering the Standby mode
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(N_RST_STDBY_A::Enabled)
    }
    ///No reset generated when entering the Standby mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(N_RST_STDBY_A::Disabled)
    }
}
///Field `nRST_SHDW` reader - nRSTSHDW
pub type N_RST_SHDW_R = crate::BitReader<N_RST_SHDW_A>;
///nRSTSHDW
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum N_RST_SHDW_A {
    ///0: Reset generated when entering the Shutdown mode
    Enabled = 0,
    ///1: No reset generated when entering the Shutdown mode
    Disabled = 1,
}
impl From<N_RST_SHDW_A> for bool {
    #[inline(always)]
    fn from(variant: N_RST_SHDW_A) -> Self {
        variant as u8 != 0
    }
}
impl N_RST_SHDW_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> N_RST_SHDW_A {
        match self.bits {
            false => N_RST_SHDW_A::Enabled,
            true => N_RST_SHDW_A::Disabled,
        }
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == N_RST_SHDW_A::Enabled
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == N_RST_SHDW_A::Disabled
    }
}
///Field `nRST_SHDW` writer - nRSTSHDW
pub type N_RST_SHDW_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTR_SPEC, N_RST_SHDW_A, O>;
impl<'a, const O: u8> N_RST_SHDW_W<'a, O> {
    ///Reset generated when entering the Shutdown mode
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(N_RST_SHDW_A::Enabled)
    }
    ///No reset generated when entering the Shutdown mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(N_RST_SHDW_A::Disabled)
    }
}
///Field `IWDG_SW` reader - Independent watchdog selection
pub type IWDG_SW_R = crate::BitReader<IWDG_SW_A>;
///Independent watchdog selection
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IWDG_SW_A {
    ///0: Hardware independent watchdog
    Hardware = 0,
    ///1: Software independent watchdog
    Software = 1,
}
impl From<IWDG_SW_A> for bool {
    #[inline(always)]
    fn from(variant: IWDG_SW_A) -> Self {
        variant as u8 != 0
    }
}
impl IWDG_SW_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> IWDG_SW_A {
        match self.bits {
            false => IWDG_SW_A::Hardware,
            true => IWDG_SW_A::Software,
        }
    }
    ///Checks if the value of the field is `Hardware`
    #[inline(always)]
    pub fn is_hardware(&self) -> bool {
        *self == IWDG_SW_A::Hardware
    }
    ///Checks if the value of the field is `Software`
    #[inline(always)]
    pub fn is_software(&self) -> bool {
        *self == IWDG_SW_A::Software
    }
}
///Field `IWDG_SW` writer - Independent watchdog selection
pub type IWDG_SW_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTR_SPEC, IWDG_SW_A, O>;
impl<'a, const O: u8> IWDG_SW_W<'a, O> {
    ///Hardware independent watchdog
    #[inline(always)]
    pub fn hardware(self) -> &'a mut W {
        self.variant(IWDG_SW_A::Hardware)
    }
    ///Software independent watchdog
    #[inline(always)]
    pub fn software(self) -> &'a mut W {
        self.variant(IWDG_SW_A::Software)
    }
}
///Field `IWDG_STOP` reader - Independent watchdog counter freeze in Stop mode
pub type IWDG_STOP_R = crate::BitReader<IWDG_STOP_A>;
///Independent watchdog counter freeze in Stop mode
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IWDG_STOP_A {
    ///0: Independent watchdog counter frozen in Stop mode
    Frozen = 0,
    ///1: Independent watchdog counter running in Stop mode
    Running = 1,
}
impl From<IWDG_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: IWDG_STOP_A) -> Self {
        variant as u8 != 0
    }
}
impl IWDG_STOP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> IWDG_STOP_A {
        match self.bits {
            false => IWDG_STOP_A::Frozen,
            true => IWDG_STOP_A::Running,
        }
    }
    ///Checks if the value of the field is `Frozen`
    #[inline(always)]
    pub fn is_frozen(&self) -> bool {
        *self == IWDG_STOP_A::Frozen
    }
    ///Checks if the value of the field is `Running`
    #[inline(always)]
    pub fn is_running(&self) -> bool {
        *self == IWDG_STOP_A::Running
    }
}
///Field `IWDG_STOP` writer - Independent watchdog counter freeze in Stop mode
pub type IWDG_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTR_SPEC, IWDG_STOP_A, O>;
impl<'a, const O: u8> IWDG_STOP_W<'a, O> {
    ///Independent watchdog counter frozen in Stop mode
    #[inline(always)]
    pub fn frozen(self) -> &'a mut W {
        self.variant(IWDG_STOP_A::Frozen)
    }
    ///Independent watchdog counter running in Stop mode
    #[inline(always)]
    pub fn running(self) -> &'a mut W {
        self.variant(IWDG_STOP_A::Running)
    }
}
///Field `IWDG_STDBY` reader - Independent watchdog counter freeze in Standby mode
pub type IWDG_STDBY_R = crate::BitReader<IWDG_STDBY_A>;
///Independent watchdog counter freeze in Standby mode
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IWDG_STDBY_A {
    ///0: Independent watchdog counter frozen in Standby mode
    Frozen = 0,
    ///1: Independent watchdog counter running in Standby mode
    Running = 1,
}
impl From<IWDG_STDBY_A> for bool {
    #[inline(always)]
    fn from(variant: IWDG_STDBY_A) -> Self {
        variant as u8 != 0
    }
}
impl IWDG_STDBY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> IWDG_STDBY_A {
        match self.bits {
            false => IWDG_STDBY_A::Frozen,
            true => IWDG_STDBY_A::Running,
        }
    }
    ///Checks if the value of the field is `Frozen`
    #[inline(always)]
    pub fn is_frozen(&self) -> bool {
        *self == IWDG_STDBY_A::Frozen
    }
    ///Checks if the value of the field is `Running`
    #[inline(always)]
    pub fn is_running(&self) -> bool {
        *self == IWDG_STDBY_A::Running
    }
}
///Field `IWDG_STDBY` writer - Independent watchdog counter freeze in Standby mode
pub type IWDG_STDBY_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTR_SPEC, IWDG_STDBY_A, O>;
impl<'a, const O: u8> IWDG_STDBY_W<'a, O> {
    ///Independent watchdog counter frozen in Standby mode
    #[inline(always)]
    pub fn frozen(self) -> &'a mut W {
        self.variant(IWDG_STDBY_A::Frozen)
    }
    ///Independent watchdog counter running in Standby mode
    #[inline(always)]
    pub fn running(self) -> &'a mut W {
        self.variant(IWDG_STDBY_A::Running)
    }
}
///Field `WWDG_SW` reader - Window watchdog selection
pub type WWDG_SW_R = crate::BitReader<WWDG_SW_A>;
///Window watchdog selection
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WWDG_SW_A {
    ///0: Hardware window watchdog
    Hardware = 0,
    ///1: Software window watchdog
    Software = 1,
}
impl From<WWDG_SW_A> for bool {
    #[inline(always)]
    fn from(variant: WWDG_SW_A) -> Self {
        variant as u8 != 0
    }
}
impl WWDG_SW_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> WWDG_SW_A {
        match self.bits {
            false => WWDG_SW_A::Hardware,
            true => WWDG_SW_A::Software,
        }
    }
    ///Checks if the value of the field is `Hardware`
    #[inline(always)]
    pub fn is_hardware(&self) -> bool {
        *self == WWDG_SW_A::Hardware
    }
    ///Checks if the value of the field is `Software`
    #[inline(always)]
    pub fn is_software(&self) -> bool {
        *self == WWDG_SW_A::Software
    }
}
///Field `WWDG_SW` writer - Window watchdog selection
pub type WWDG_SW_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTR_SPEC, WWDG_SW_A, O>;
impl<'a, const O: u8> WWDG_SW_W<'a, O> {
    ///Hardware window watchdog
    #[inline(always)]
    pub fn hardware(self) -> &'a mut W {
        self.variant(WWDG_SW_A::Hardware)
    }
    ///Software window watchdog
    #[inline(always)]
    pub fn software(self) -> &'a mut W {
        self.variant(WWDG_SW_A::Software)
    }
}
///Field `nBOOT1` reader - Boot configuration
pub type N_BOOT1_R = crate::BitReader<N_BOOT1_A>;
///Boot configuration
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum N_BOOT1_A {
    ///0: When nSWBOOT0 is cleared, select boot mode together with nBOOT0
    Clear = 0,
    ///1: When nSWBOOT0 is cleared, select boot mode together with nBOOT0
    Set = 1,
}
impl From<N_BOOT1_A> for bool {
    #[inline(always)]
    fn from(variant: N_BOOT1_A) -> Self {
        variant as u8 != 0
    }
}
impl N_BOOT1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> N_BOOT1_A {
        match self.bits {
            false => N_BOOT1_A::Clear,
            true => N_BOOT1_A::Set,
        }
    }
    ///Checks if the value of the field is `Clear`
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == N_BOOT1_A::Clear
    }
    ///Checks if the value of the field is `Set`
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == N_BOOT1_A::Set
    }
}
///Field `nBOOT1` writer - Boot configuration
pub type N_BOOT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTR_SPEC, N_BOOT1_A, O>;
impl<'a, const O: u8> N_BOOT1_W<'a, O> {
    ///When nSWBOOT0 is cleared, select boot mode together with nBOOT0
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(N_BOOT1_A::Clear)
    }
    ///When nSWBOOT0 is cleared, select boot mode together with nBOOT0
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(N_BOOT1_A::Set)
    }
}
///Field `SRAM2_PE` reader - SRAM2 parity check enable
pub type SRAM2_PE_R = crate::BitReader<SRAM2_PE_A>;
///SRAM2 parity check enable
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM2_PE_A {
    ///0: SRAM2 Parity check enabled
    Enabled = 0,
    ///1: SRAM2 Parity check disabled
    Disabled = 1,
}
impl From<SRAM2_PE_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM2_PE_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM2_PE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SRAM2_PE_A {
        match self.bits {
            false => SRAM2_PE_A::Enabled,
            true => SRAM2_PE_A::Disabled,
        }
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SRAM2_PE_A::Enabled
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SRAM2_PE_A::Disabled
    }
}
///Field `SRAM2_PE` writer - SRAM2 parity check enable
pub type SRAM2_PE_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTR_SPEC, SRAM2_PE_A, O>;
impl<'a, const O: u8> SRAM2_PE_W<'a, O> {
    ///SRAM2 Parity check enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SRAM2_PE_A::Enabled)
    }
    ///SRAM2 Parity check disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SRAM2_PE_A::Disabled)
    }
}
///Field `SRAM_RST` reader - SRAM2 Erase when system reset
pub type SRAM_RST_R = crate::BitReader<SRAM_RST_A>;
///SRAM2 Erase when system reset
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_RST_A {
    ///0: SRAM1 and SRAM2 erased when a system reset occurs
    Reset = 0,
    ///1: SRAM1 and SRAM2 not erased when a system reset occurs
    NotReset = 1,
}
impl From<SRAM_RST_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM_RST_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM_RST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SRAM_RST_A {
        match self.bits {
            false => SRAM_RST_A::Reset,
            true => SRAM_RST_A::NotReset,
        }
    }
    ///Checks if the value of the field is `Reset`
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SRAM_RST_A::Reset
    }
    ///Checks if the value of the field is `NotReset`
    #[inline(always)]
    pub fn is_not_reset(&self) -> bool {
        *self == SRAM_RST_A::NotReset
    }
}
///Field `SRAM_RST` writer - SRAM2 Erase when system reset
pub type SRAM_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTR_SPEC, SRAM_RST_A, O>;
impl<'a, const O: u8> SRAM_RST_W<'a, O> {
    ///SRAM1 and SRAM2 erased when a system reset occurs
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(SRAM_RST_A::Reset)
    }
    ///SRAM1 and SRAM2 not erased when a system reset occurs
    #[inline(always)]
    pub fn not_reset(self) -> &'a mut W {
        self.variant(SRAM_RST_A::NotReset)
    }
}
///Field `nSWBOOT0` reader - Software BOOT0 selection
pub type N_SWBOOT0_R = crate::BitReader<N_SWBOOT0_A>;
///Software BOOT0 selection
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum N_SWBOOT0_A {
    ///0: BOOT0 taken from nBOOT0 in this register
    Bit = 0,
    ///1: BOOT0 taken from GPIO PH3/BOOT0
    Pin = 1,
}
impl From<N_SWBOOT0_A> for bool {
    #[inline(always)]
    fn from(variant: N_SWBOOT0_A) -> Self {
        variant as u8 != 0
    }
}
impl N_SWBOOT0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> N_SWBOOT0_A {
        match self.bits {
            false => N_SWBOOT0_A::Bit,
            true => N_SWBOOT0_A::Pin,
        }
    }
    ///Checks if the value of the field is `Bit`
    #[inline(always)]
    pub fn is_bit_(&self) -> bool {
        *self == N_SWBOOT0_A::Bit
    }
    ///Checks if the value of the field is `Pin`
    #[inline(always)]
    pub fn is_pin(&self) -> bool {
        *self == N_SWBOOT0_A::Pin
    }
}
///Field `nSWBOOT0` writer - Software BOOT0 selection
pub type N_SWBOOT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTR_SPEC, N_SWBOOT0_A, O>;
impl<'a, const O: u8> N_SWBOOT0_W<'a, O> {
    ///BOOT0 taken from nBOOT0 in this register
    #[inline(always)]
    pub fn bit_(self) -> &'a mut W {
        self.variant(N_SWBOOT0_A::Bit)
    }
    ///BOOT0 taken from GPIO PH3/BOOT0
    #[inline(always)]
    pub fn pin(self) -> &'a mut W {
        self.variant(N_SWBOOT0_A::Pin)
    }
}
///Field `nBOOT0` reader - nBOOT0 option bit
pub type N_BOOT0_R = crate::BitReader<N_BOOT0_A>;
///nBOOT0 option bit
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum N_BOOT0_A {
    ///0: When nSWBOOT0 is cleared, select boot mode together with nBOOT1
    Clear = 0,
    ///1: When nSWBOOT0 is cleared, select boot mode together with nBOOT1
    Set = 1,
}
impl From<N_BOOT0_A> for bool {
    #[inline(always)]
    fn from(variant: N_BOOT0_A) -> Self {
        variant as u8 != 0
    }
}
impl N_BOOT0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> N_BOOT0_A {
        match self.bits {
            false => N_BOOT0_A::Clear,
            true => N_BOOT0_A::Set,
        }
    }
    ///Checks if the value of the field is `Clear`
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == N_BOOT0_A::Clear
    }
    ///Checks if the value of the field is `Set`
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == N_BOOT0_A::Set
    }
}
///Field `nBOOT0` writer - nBOOT0 option bit
pub type N_BOOT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTR_SPEC, N_BOOT0_A, O>;
impl<'a, const O: u8> N_BOOT0_W<'a, O> {
    ///When nSWBOOT0 is cleared, select boot mode together with nBOOT1
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(N_BOOT0_A::Clear)
    }
    ///When nSWBOOT0 is cleared, select boot mode together with nBOOT1
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(N_BOOT0_A::Set)
    }
}
///Field `BOOT_LOCK` reader - CPU1 CM4 Unique Boot entry enable option bit
pub type BOOT_LOCK_R = crate::BitReader<BOOT_LOCK_A>;
///CPU1 CM4 Unique Boot entry enable option bit
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BOOT_LOCK_A {
    ///0: Boot lock is disabled
    Disabled = 0,
    ///1: Boot lock is enabled
    Enabled = 1,
}
impl From<BOOT_LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: BOOT_LOCK_A) -> Self {
        variant as u8 != 0
    }
}
impl BOOT_LOCK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BOOT_LOCK_A {
        match self.bits {
            false => BOOT_LOCK_A::Disabled,
            true => BOOT_LOCK_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BOOT_LOCK_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BOOT_LOCK_A::Enabled
    }
}
///Field `BOOT_LOCK` writer - CPU1 CM4 Unique Boot entry enable option bit
pub type BOOT_LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTR_SPEC, BOOT_LOCK_A, O>;
impl<'a, const O: u8> BOOT_LOCK_W<'a, O> {
    ///Boot lock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BOOT_LOCK_A::Disabled)
    }
    ///Boot lock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BOOT_LOCK_A::Enabled)
    }
}
///Field `C2BOOT_LOCK` reader - CPU2 CM0+ Unique Boot entry enable option bit
pub type C2BOOT_LOCK_R = crate::BitReader<bool>;
///Field `C2BOOT_LOCK` writer - CPU2 CM0+ Unique Boot entry enable option bit
pub type C2BOOT_LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTR_SPEC, bool, O>;
impl R {
    ///Bits 0:7 - Read protection level
    #[inline(always)]
    pub fn rdp(&self) -> RDP_R {
        RDP_R::new((self.bits & 0xff) as u8)
    }
    ///Bit 8 - System security enabled flag
    #[inline(always)]
    pub fn ese(&self) -> ESE_R {
        ESE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 9:11 - BOR reset Level
    #[inline(always)]
    pub fn bor_lev(&self) -> BOR_LEV_R {
        BOR_LEV_R::new(((self.bits >> 9) & 7) as u8)
    }
    ///Bit 12 - nRST_STOP
    #[inline(always)]
    pub fn n_rst_stop(&self) -> N_RST_STOP_R {
        N_RST_STOP_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - nRST_STDBY
    #[inline(always)]
    pub fn n_rst_stdby(&self) -> N_RST_STDBY_R {
        N_RST_STDBY_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - nRSTSHDW
    #[inline(always)]
    pub fn n_rst_shdw(&self) -> N_RST_SHDW_R {
        N_RST_SHDW_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - Independent watchdog selection
    #[inline(always)]
    pub fn iwdg_sw(&self) -> IWDG_SW_R {
        IWDG_SW_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Independent watchdog counter freeze in Stop mode
    #[inline(always)]
    pub fn iwdg_stop(&self) -> IWDG_STOP_R {
        IWDG_STOP_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Independent watchdog counter freeze in Standby mode
    #[inline(always)]
    pub fn iwdg_stdby(&self) -> IWDG_STDBY_R {
        IWDG_STDBY_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Window watchdog selection
    #[inline(always)]
    pub fn wwdg_sw(&self) -> WWDG_SW_R {
        WWDG_SW_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 23 - Boot configuration
    #[inline(always)]
    pub fn n_boot1(&self) -> N_BOOT1_R {
        N_BOOT1_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - SRAM2 parity check enable
    #[inline(always)]
    pub fn sram2_pe(&self) -> SRAM2_PE_R {
        SRAM2_PE_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - SRAM2 Erase when system reset
    #[inline(always)]
    pub fn sram_rst(&self) -> SRAM_RST_R {
        SRAM_RST_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Software BOOT0 selection
    #[inline(always)]
    pub fn n_swboot0(&self) -> N_SWBOOT0_R {
        N_SWBOOT0_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - nBOOT0 option bit
    #[inline(always)]
    pub fn n_boot0(&self) -> N_BOOT0_R {
        N_BOOT0_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 30 - CPU1 CM4 Unique Boot entry enable option bit
    #[inline(always)]
    pub fn boot_lock(&self) -> BOOT_LOCK_R {
        BOOT_LOCK_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - CPU2 CM0+ Unique Boot entry enable option bit
    #[inline(always)]
    pub fn c2boot_lock(&self) -> C2BOOT_LOCK_R {
        C2BOOT_LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 0:7 - Read protection level
    #[inline(always)]
    #[must_use]
    pub fn rdp(&mut self) -> RDP_W<0> {
        RDP_W::new(self)
    }
    ///Bit 8 - System security enabled flag
    #[inline(always)]
    #[must_use]
    pub fn ese(&mut self) -> ESE_W<8> {
        ESE_W::new(self)
    }
    ///Bits 9:11 - BOR reset Level
    #[inline(always)]
    #[must_use]
    pub fn bor_lev(&mut self) -> BOR_LEV_W<9> {
        BOR_LEV_W::new(self)
    }
    ///Bit 12 - nRST_STOP
    #[inline(always)]
    #[must_use]
    pub fn n_rst_stop(&mut self) -> N_RST_STOP_W<12> {
        N_RST_STOP_W::new(self)
    }
    ///Bit 13 - nRST_STDBY
    #[inline(always)]
    #[must_use]
    pub fn n_rst_stdby(&mut self) -> N_RST_STDBY_W<13> {
        N_RST_STDBY_W::new(self)
    }
    ///Bit 14 - nRSTSHDW
    #[inline(always)]
    #[must_use]
    pub fn n_rst_shdw(&mut self) -> N_RST_SHDW_W<14> {
        N_RST_SHDW_W::new(self)
    }
    ///Bit 16 - Independent watchdog selection
    #[inline(always)]
    #[must_use]
    pub fn iwdg_sw(&mut self) -> IWDG_SW_W<16> {
        IWDG_SW_W::new(self)
    }
    ///Bit 17 - Independent watchdog counter freeze in Stop mode
    #[inline(always)]
    #[must_use]
    pub fn iwdg_stop(&mut self) -> IWDG_STOP_W<17> {
        IWDG_STOP_W::new(self)
    }
    ///Bit 18 - Independent watchdog counter freeze in Standby mode
    #[inline(always)]
    #[must_use]
    pub fn iwdg_stdby(&mut self) -> IWDG_STDBY_W<18> {
        IWDG_STDBY_W::new(self)
    }
    ///Bit 19 - Window watchdog selection
    #[inline(always)]
    #[must_use]
    pub fn wwdg_sw(&mut self) -> WWDG_SW_W<19> {
        WWDG_SW_W::new(self)
    }
    ///Bit 23 - Boot configuration
    #[inline(always)]
    #[must_use]
    pub fn n_boot1(&mut self) -> N_BOOT1_W<23> {
        N_BOOT1_W::new(self)
    }
    ///Bit 24 - SRAM2 parity check enable
    #[inline(always)]
    #[must_use]
    pub fn sram2_pe(&mut self) -> SRAM2_PE_W<24> {
        SRAM2_PE_W::new(self)
    }
    ///Bit 25 - SRAM2 Erase when system reset
    #[inline(always)]
    #[must_use]
    pub fn sram_rst(&mut self) -> SRAM_RST_W<25> {
        SRAM_RST_W::new(self)
    }
    ///Bit 26 - Software BOOT0 selection
    #[inline(always)]
    #[must_use]
    pub fn n_swboot0(&mut self) -> N_SWBOOT0_W<26> {
        N_SWBOOT0_W::new(self)
    }
    ///Bit 27 - nBOOT0 option bit
    #[inline(always)]
    #[must_use]
    pub fn n_boot0(&mut self) -> N_BOOT0_W<27> {
        N_BOOT0_W::new(self)
    }
    ///Bit 30 - CPU1 CM4 Unique Boot entry enable option bit
    #[inline(always)]
    #[must_use]
    pub fn boot_lock(&mut self) -> BOOT_LOCK_W<30> {
        BOOT_LOCK_W::new(self)
    }
    ///Bit 31 - CPU2 CM0+ Unique Boot entry enable option bit
    #[inline(always)]
    #[must_use]
    pub fn c2boot_lock(&mut self) -> C2BOOT_LOCK_W<31> {
        C2BOOT_LOCK_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Flash option register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [optr](index.html) module
pub struct OPTR_SPEC;
impl crate::RegisterSpec for OPTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [optr::R](R) reader structure
impl crate::Readable for OPTR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [optr::W](W) writer structure
impl crate::Writable for OPTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets OPTR to value 0x3fff_f0aa
impl crate::Resettable for OPTR_SPEC {
    const RESET_VALUE: Self::Ux = 0x3fff_f0aa;
}
