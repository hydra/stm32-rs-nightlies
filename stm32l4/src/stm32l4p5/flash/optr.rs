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
pub type RDP_R = crate::FieldReader<u8, u8>;
///Field `RDP` writer - Read protection level
pub type RDP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPTR_SPEC, u8, u8, 8, O>;
///Field `BOR_LEV` reader - BOR reset Level
pub type BOR_LEV_R = crate::FieldReader<u8, u8>;
///Field `BOR_LEV` writer - BOR reset Level
pub type BOR_LEV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPTR_SPEC, u8, u8, 3, O>;
///Field `nRST_STOP` reader - nRST_STOP
pub type N_RST_STOP_R = crate::BitReader<bool>;
///Field `nRST_STOP` writer - nRST_STOP
pub type N_RST_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTR_SPEC, bool, O>;
///Field `nRST_STDBY` reader - nRST_STDBY
pub type N_RST_STDBY_R = crate::BitReader<bool>;
///Field `nRST_STDBY` writer - nRST_STDBY
pub type N_RST_STDBY_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTR_SPEC, bool, O>;
///Field `IDWG_SW` reader - Independent watchdog selection
pub type IDWG_SW_R = crate::BitReader<IDWG_SW_A>;
///Independent watchdog selection
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDWG_SW_A {
    ///0: Hardware independent watchdog
    Hardware = 0,
    ///1: Software independent watchdog
    Software = 1,
}
impl From<IDWG_SW_A> for bool {
    #[inline(always)]
    fn from(variant: IDWG_SW_A) -> Self {
        variant as u8 != 0
    }
}
impl IDWG_SW_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> IDWG_SW_A {
        match self.bits {
            false => IDWG_SW_A::Hardware,
            true => IDWG_SW_A::Software,
        }
    }
    ///Checks if the value of the field is `Hardware`
    #[inline(always)]
    pub fn is_hardware(&self) -> bool {
        *self == IDWG_SW_A::Hardware
    }
    ///Checks if the value of the field is `Software`
    #[inline(always)]
    pub fn is_software(&self) -> bool {
        *self == IDWG_SW_A::Software
    }
}
///Field `IDWG_SW` writer - Independent watchdog selection
pub type IDWG_SW_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTR_SPEC, IDWG_SW_A, O>;
impl<'a, const O: u8> IDWG_SW_W<'a, O> {
    ///Hardware independent watchdog
    #[inline(always)]
    pub fn hardware(self) -> &'a mut W {
        self.variant(IDWG_SW_A::Hardware)
    }
    ///Software independent watchdog
    #[inline(always)]
    pub fn software(self) -> &'a mut W {
        self.variant(IDWG_SW_A::Software)
    }
}
///Field `IWDG_STOP` reader - Independent watchdog counter freeze in Stop mode
pub type IWDG_STOP_R = crate::BitReader<IWDG_STOP_A>;
///Independent watchdog counter freeze in Stop mode
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IWDG_STOP_A {
    ///0: Independent watchdog counter is frozen in Stop mode
    Frozen = 0,
    ///1: Independent watchdog counter is running in Stop mode
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
    ///Independent watchdog counter is frozen in Stop mode
    #[inline(always)]
    pub fn frozen(self) -> &'a mut W {
        self.variant(IWDG_STOP_A::Frozen)
    }
    ///Independent watchdog counter is running in Stop mode
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
    ///0: Independent watchdog counter is frozen in Standby mode
    Frozen = 0,
    ///1: Independent watchdog counter is running in Standby mode
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
    ///Independent watchdog counter is frozen in Standby mode
    #[inline(always)]
    pub fn frozen(self) -> &'a mut W {
        self.variant(IWDG_STDBY_A::Frozen)
    }
    ///Independent watchdog counter is running in Standby mode
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
///Field `BFB2` reader - Dual-bank boot
pub type BFB2_R = crate::BitReader<BFB2_A>;
///Dual-bank boot
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BFB2_A {
    ///0: Dual-bank boot disabled
    Disabled = 0,
    ///1: Dual-bank boot enabled
    Enabled = 1,
}
impl From<BFB2_A> for bool {
    #[inline(always)]
    fn from(variant: BFB2_A) -> Self {
        variant as u8 != 0
    }
}
impl BFB2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BFB2_A {
        match self.bits {
            false => BFB2_A::Disabled,
            true => BFB2_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BFB2_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BFB2_A::Enabled
    }
}
///Field `BFB2` writer - Dual-bank boot
pub type BFB2_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTR_SPEC, BFB2_A, O>;
impl<'a, const O: u8> BFB2_W<'a, O> {
    ///Dual-bank boot disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BFB2_A::Disabled)
    }
    ///Dual-bank boot enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BFB2_A::Enabled)
    }
}
///Field `DB1M` reader -
pub type DB1M_R = crate::BitReader<DB1M_A>;
///
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DB1M_A {
    ///0: Single Flash contiguous address in Bank 1
    SingleBank = 0,
    ///1: Dual-bank Flash with contiguous addresses
    DualBank = 1,
}
impl From<DB1M_A> for bool {
    #[inline(always)]
    fn from(variant: DB1M_A) -> Self {
        variant as u8 != 0
    }
}
impl DB1M_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DB1M_A {
        match self.bits {
            false => DB1M_A::SingleBank,
            true => DB1M_A::DualBank,
        }
    }
    ///Checks if the value of the field is `SingleBank`
    #[inline(always)]
    pub fn is_single_bank(&self) -> bool {
        *self == DB1M_A::SingleBank
    }
    ///Checks if the value of the field is `DualBank`
    #[inline(always)]
    pub fn is_dual_bank(&self) -> bool {
        *self == DB1M_A::DualBank
    }
}
///Field `DB1M` writer -
pub type DB1M_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTR_SPEC, DB1M_A, O>;
impl<'a, const O: u8> DB1M_W<'a, O> {
    ///Single Flash contiguous address in Bank 1
    #[inline(always)]
    pub fn single_bank(self) -> &'a mut W {
        self.variant(DB1M_A::SingleBank)
    }
    ///Dual-bank Flash with contiguous addresses
    #[inline(always)]
    pub fn dual_bank(self) -> &'a mut W {
        self.variant(DB1M_A::DualBank)
    }
}
///Field `DBANK` reader -
pub type DBANK_R = crate::BitReader<DBANK_A>;
///
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBANK_A {
    ///0: Single-bank mode with 128 bits data read width
    SingleBankMode = 0,
    ///1: Dual-bank mode with 64 bits data
    DualBankMode = 1,
}
impl From<DBANK_A> for bool {
    #[inline(always)]
    fn from(variant: DBANK_A) -> Self {
        variant as u8 != 0
    }
}
impl DBANK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DBANK_A {
        match self.bits {
            false => DBANK_A::SingleBankMode,
            true => DBANK_A::DualBankMode,
        }
    }
    ///Checks if the value of the field is `SingleBankMode`
    #[inline(always)]
    pub fn is_single_bank_mode(&self) -> bool {
        *self == DBANK_A::SingleBankMode
    }
    ///Checks if the value of the field is `DualBankMode`
    #[inline(always)]
    pub fn is_dual_bank_mode(&self) -> bool {
        *self == DBANK_A::DualBankMode
    }
}
///Field `DBANK` writer -
pub type DBANK_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTR_SPEC, DBANK_A, O>;
impl<'a, const O: u8> DBANK_W<'a, O> {
    ///Single-bank mode with 128 bits data read width
    #[inline(always)]
    pub fn single_bank_mode(self) -> &'a mut W {
        self.variant(DBANK_A::SingleBankMode)
    }
    ///Dual-bank mode with 64 bits data
    #[inline(always)]
    pub fn dual_bank_mode(self) -> &'a mut W {
        self.variant(DBANK_A::DualBankMode)
    }
}
///Field `nBOOT1` reader - Boot configuration
pub type N_BOOT1_R = crate::BitReader<bool>;
///Field `nBOOT1` writer - Boot configuration
pub type N_BOOT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTR_SPEC, bool, O>;
///Field `SRAM2_PE` reader - SRAM2 parity check enable
pub type SRAM2_PE_R = crate::BitReader<SRAM2_PE_A>;
///SRAM2 parity check enable
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM2_PE_A {
    ///0: SRAM2 parity check enabled
    Enabled = 0,
    ///1: SRAM2 parity check disabled
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
    ///SRAM2 parity check enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SRAM2_PE_A::Enabled)
    }
    ///SRAM2 parity check disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SRAM2_PE_A::Disabled)
    }
}
///Field `SRAM2_RST` reader - SRAM2 Erase when system reset
pub type SRAM2_RST_R = crate::BitReader<SRAM2_RST_A>;
///SRAM2 Erase when system reset
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM2_RST_A {
    ///0: SRAM2 erased when a system reset occurs
    Enabled = 0,
    ///1: SRAM2 is not erased when a system reset occurs
    Disabled = 1,
}
impl From<SRAM2_RST_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM2_RST_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM2_RST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SRAM2_RST_A {
        match self.bits {
            false => SRAM2_RST_A::Enabled,
            true => SRAM2_RST_A::Disabled,
        }
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SRAM2_RST_A::Enabled
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SRAM2_RST_A::Disabled
    }
}
///Field `SRAM2_RST` writer - SRAM2 Erase when system reset
pub type SRAM2_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTR_SPEC, SRAM2_RST_A, O>;
impl<'a, const O: u8> SRAM2_RST_W<'a, O> {
    ///SRAM2 erased when a system reset occurs
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SRAM2_RST_A::Enabled)
    }
    ///SRAM2 is not erased when a system reset occurs
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SRAM2_RST_A::Disabled)
    }
}
///Field `nSWBOOT0` reader - nSWBOOT0 option bit
pub type N_SWBOOT0_R = crate::BitReader<N_SWBOOT0_A>;
///nSWBOOT0 option bit
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum N_SWBOOT0_A {
    ///0: BOOT0 taken from the option bit nBOOT0
    OptionBit = 0,
    ///1: BOOT0 taken from PH3/BOOT0 pin
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
            false => N_SWBOOT0_A::OptionBit,
            true => N_SWBOOT0_A::Pin,
        }
    }
    ///Checks if the value of the field is `OptionBit`
    #[inline(always)]
    pub fn is_option_bit(&self) -> bool {
        *self == N_SWBOOT0_A::OptionBit
    }
    ///Checks if the value of the field is `Pin`
    #[inline(always)]
    pub fn is_pin(&self) -> bool {
        *self == N_SWBOOT0_A::Pin
    }
}
///Field `nSWBOOT0` writer - nSWBOOT0 option bit
pub type N_SWBOOT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTR_SPEC, N_SWBOOT0_A, O>;
impl<'a, const O: u8> N_SWBOOT0_W<'a, O> {
    ///BOOT0 taken from the option bit nBOOT0
    #[inline(always)]
    pub fn option_bit(self) -> &'a mut W {
        self.variant(N_SWBOOT0_A::OptionBit)
    }
    ///BOOT0 taken from PH3/BOOT0 pin
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
    ///0: nBOOT0 = 0
    Disabled = 0,
    ///1: nBOOT0 = 1
    Enabled = 1,
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
            false => N_BOOT0_A::Disabled,
            true => N_BOOT0_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == N_BOOT0_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == N_BOOT0_A::Enabled
    }
}
///Field `nBOOT0` writer - nBOOT0 option bit
pub type N_BOOT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTR_SPEC, N_BOOT0_A, O>;
impl<'a, const O: u8> N_BOOT0_W<'a, O> {
    ///nBOOT0 = 0
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(N_BOOT0_A::Disabled)
    }
    ///nBOOT0 = 1
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(N_BOOT0_A::Enabled)
    }
}
impl R {
    ///Bits 0:7 - Read protection level
    #[inline(always)]
    pub fn rdp(&self) -> RDP_R {
        RDP_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:10 - BOR reset Level
    #[inline(always)]
    pub fn bor_lev(&self) -> BOR_LEV_R {
        BOR_LEV_R::new(((self.bits >> 8) & 7) as u8)
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
    ///Bit 16 - Independent watchdog selection
    #[inline(always)]
    pub fn idwg_sw(&self) -> IDWG_SW_R {
        IDWG_SW_R::new(((self.bits >> 16) & 1) != 0)
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
    ///Bit 20 - Dual-bank boot
    #[inline(always)]
    pub fn bfb2(&self) -> BFB2_R {
        BFB2_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21
    #[inline(always)]
    pub fn db1m(&self) -> DB1M_R {
        DB1M_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22
    #[inline(always)]
    pub fn dbank(&self) -> DBANK_R {
        DBANK_R::new(((self.bits >> 22) & 1) != 0)
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
    pub fn sram2_rst(&self) -> SRAM2_RST_R {
        SRAM2_RST_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - nSWBOOT0 option bit
    #[inline(always)]
    pub fn n_swboot0(&self) -> N_SWBOOT0_R {
        N_SWBOOT0_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - nBOOT0 option bit
    #[inline(always)]
    pub fn n_boot0(&self) -> N_BOOT0_R {
        N_BOOT0_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    ///Bits 0:7 - Read protection level
    #[inline(always)]
    #[must_use]
    pub fn rdp(&mut self) -> RDP_W<0> {
        RDP_W::new(self)
    }
    ///Bits 8:10 - BOR reset Level
    #[inline(always)]
    #[must_use]
    pub fn bor_lev(&mut self) -> BOR_LEV_W<8> {
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
    ///Bit 16 - Independent watchdog selection
    #[inline(always)]
    #[must_use]
    pub fn idwg_sw(&mut self) -> IDWG_SW_W<16> {
        IDWG_SW_W::new(self)
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
    ///Bit 20 - Dual-bank boot
    #[inline(always)]
    #[must_use]
    pub fn bfb2(&mut self) -> BFB2_W<20> {
        BFB2_W::new(self)
    }
    ///Bit 21
    #[inline(always)]
    #[must_use]
    pub fn db1m(&mut self) -> DB1M_W<21> {
        DB1M_W::new(self)
    }
    ///Bit 22
    #[inline(always)]
    #[must_use]
    pub fn dbank(&mut self) -> DBANK_W<22> {
        DBANK_W::new(self)
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
    pub fn sram2_rst(&mut self) -> SRAM2_RST_W<25> {
        SRAM2_RST_W::new(self)
    }
    ///Bit 26 - nSWBOOT0 option bit
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
///`reset()` method sets OPTR to value 0xffef_f8aa
impl crate::Resettable for OPTR_SPEC {
    const RESET_VALUE: Self::Ux = 0xffef_f8aa;
}
