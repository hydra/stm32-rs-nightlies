///Register `PECR` reader
pub struct R(crate::R<PECR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PECR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PECR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PECR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PECR` writer
pub struct W(crate::W<PECR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PECR_SPEC>;
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
impl From<crate::W<PECR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PECR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PELOCK` reader - FLASH_PECR and data EEPROM lock
pub type PELOCK_R = crate::BitReader<PELOCK_A>;
///FLASH_PECR and data EEPROM lock
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PELOCK_A {
    ///0: The FLASH_PECR register is unlocked
    Unlocked = 0,
    ///1: The FLASH_PECR register is locked and no write/erase operation can start
    Locked = 1,
}
impl From<PELOCK_A> for bool {
    #[inline(always)]
    fn from(variant: PELOCK_A) -> Self {
        variant as u8 != 0
    }
}
impl PELOCK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PELOCK_A {
        match self.bits {
            false => PELOCK_A::Unlocked,
            true => PELOCK_A::Locked,
        }
    }
    ///Checks if the value of the field is `Unlocked`
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == PELOCK_A::Unlocked
    }
    ///Checks if the value of the field is `Locked`
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == PELOCK_A::Locked
    }
}
///Field `PELOCK` writer - FLASH_PECR and data EEPROM lock
pub type PELOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PECR_SPEC, PELOCK_A, O>;
impl<'a, const O: u8> PELOCK_W<'a, O> {
    ///The FLASH_PECR register is unlocked
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(PELOCK_A::Unlocked)
    }
    ///The FLASH_PECR register is locked and no write/erase operation can start
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(PELOCK_A::Locked)
    }
}
///Field `PRGLOCK` reader - Program memory lock
pub type PRGLOCK_R = crate::BitReader<PRGLOCK_A>;
///Program memory lock
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRGLOCK_A {
    ///0: The write and erase operations in the Flash program memory are disabled
    Unlocked = 0,
    ///1: The write and erase operations in the Flash program memory are enabled
    Locked = 1,
}
impl From<PRGLOCK_A> for bool {
    #[inline(always)]
    fn from(variant: PRGLOCK_A) -> Self {
        variant as u8 != 0
    }
}
impl PRGLOCK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PRGLOCK_A {
        match self.bits {
            false => PRGLOCK_A::Unlocked,
            true => PRGLOCK_A::Locked,
        }
    }
    ///Checks if the value of the field is `Unlocked`
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == PRGLOCK_A::Unlocked
    }
    ///Checks if the value of the field is `Locked`
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == PRGLOCK_A::Locked
    }
}
///Field `PRGLOCK` writer - Program memory lock
pub type PRGLOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PECR_SPEC, PRGLOCK_A, O>;
impl<'a, const O: u8> PRGLOCK_W<'a, O> {
    ///The write and erase operations in the Flash program memory are disabled
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(PRGLOCK_A::Unlocked)
    }
    ///The write and erase operations in the Flash program memory are enabled
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(PRGLOCK_A::Locked)
    }
}
///Field `OPTLOCK` reader - Option bytes block lock
pub type OPTLOCK_R = crate::BitReader<OPTLOCK_A>;
///Option bytes block lock
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPTLOCK_A {
    ///0: The write and erase operations in the Option bytes area are disabled
    Unlocked = 0,
    ///1: The write and erase operations in the Option bytes area are enabled
    Locked = 1,
}
impl From<OPTLOCK_A> for bool {
    #[inline(always)]
    fn from(variant: OPTLOCK_A) -> Self {
        variant as u8 != 0
    }
}
impl OPTLOCK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OPTLOCK_A {
        match self.bits {
            false => OPTLOCK_A::Unlocked,
            true => OPTLOCK_A::Locked,
        }
    }
    ///Checks if the value of the field is `Unlocked`
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == OPTLOCK_A::Unlocked
    }
    ///Checks if the value of the field is `Locked`
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == OPTLOCK_A::Locked
    }
}
///Field `OPTLOCK` writer - Option bytes block lock
pub type OPTLOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PECR_SPEC, OPTLOCK_A, O>;
impl<'a, const O: u8> OPTLOCK_W<'a, O> {
    ///The write and erase operations in the Option bytes area are disabled
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(OPTLOCK_A::Unlocked)
    }
    ///The write and erase operations in the Option bytes area are enabled
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(OPTLOCK_A::Locked)
    }
}
///Field `PROG` reader - Program memory selection
pub type PROG_R = crate::BitReader<PROG_A>;
///Program memory selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PROG_A {
    ///0: The Flash program memory is not selected
    NotSelected = 0,
    ///1: The Flash program memory is selected
    Selected = 1,
}
impl From<PROG_A> for bool {
    #[inline(always)]
    fn from(variant: PROG_A) -> Self {
        variant as u8 != 0
    }
}
impl PROG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PROG_A {
        match self.bits {
            false => PROG_A::NotSelected,
            true => PROG_A::Selected,
        }
    }
    ///Checks if the value of the field is `NotSelected`
    #[inline(always)]
    pub fn is_not_selected(&self) -> bool {
        *self == PROG_A::NotSelected
    }
    ///Checks if the value of the field is `Selected`
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == PROG_A::Selected
    }
}
///Field `PROG` writer - Program memory selection
pub type PROG_W<'a, const O: u8> = crate::BitWriter<'a, u32, PECR_SPEC, PROG_A, O>;
impl<'a, const O: u8> PROG_W<'a, O> {
    ///The Flash program memory is not selected
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut W {
        self.variant(PROG_A::NotSelected)
    }
    ///The Flash program memory is selected
    #[inline(always)]
    pub fn selected(self) -> &'a mut W {
        self.variant(PROG_A::Selected)
    }
}
///Field `DATA` reader - Data EEPROM selection
pub type DATA_R = crate::BitReader<DATA_A>;
///Data EEPROM selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DATA_A {
    ///0: Data EEPROM not selected
    NotSelected = 0,
    ///1: Data memory selected
    Selected = 1,
}
impl From<DATA_A> for bool {
    #[inline(always)]
    fn from(variant: DATA_A) -> Self {
        variant as u8 != 0
    }
}
impl DATA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DATA_A {
        match self.bits {
            false => DATA_A::NotSelected,
            true => DATA_A::Selected,
        }
    }
    ///Checks if the value of the field is `NotSelected`
    #[inline(always)]
    pub fn is_not_selected(&self) -> bool {
        *self == DATA_A::NotSelected
    }
    ///Checks if the value of the field is `Selected`
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == DATA_A::Selected
    }
}
///Field `DATA` writer - Data EEPROM selection
pub type DATA_W<'a, const O: u8> = crate::BitWriter<'a, u32, PECR_SPEC, DATA_A, O>;
impl<'a, const O: u8> DATA_W<'a, O> {
    ///Data EEPROM not selected
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut W {
        self.variant(DATA_A::NotSelected)
    }
    ///Data memory selected
    #[inline(always)]
    pub fn selected(self) -> &'a mut W {
        self.variant(DATA_A::Selected)
    }
}
///Field `FIX` reader - Fixed time data write for Byte, Half Word and Word programming
pub type FIX_R = crate::BitReader<FIX_A>;
///Fixed time data write for Byte, Half Word and Word programming
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIX_A {
    ///0: An erase phase is automatically performed
    AutoErase = 0,
    ///1: The program operation is always performed with a preliminary erase
    PrelimErase = 1,
}
impl From<FIX_A> for bool {
    #[inline(always)]
    fn from(variant: FIX_A) -> Self {
        variant as u8 != 0
    }
}
impl FIX_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FIX_A {
        match self.bits {
            false => FIX_A::AutoErase,
            true => FIX_A::PrelimErase,
        }
    }
    ///Checks if the value of the field is `AutoErase`
    #[inline(always)]
    pub fn is_auto_erase(&self) -> bool {
        *self == FIX_A::AutoErase
    }
    ///Checks if the value of the field is `PrelimErase`
    #[inline(always)]
    pub fn is_prelim_erase(&self) -> bool {
        *self == FIX_A::PrelimErase
    }
}
///Field `FIX` writer - Fixed time data write for Byte, Half Word and Word programming
pub type FIX_W<'a, const O: u8> = crate::BitWriter<'a, u32, PECR_SPEC, FIX_A, O>;
impl<'a, const O: u8> FIX_W<'a, O> {
    ///An erase phase is automatically performed
    #[inline(always)]
    pub fn auto_erase(self) -> &'a mut W {
        self.variant(FIX_A::AutoErase)
    }
    ///The program operation is always performed with a preliminary erase
    #[inline(always)]
    pub fn prelim_erase(self) -> &'a mut W {
        self.variant(FIX_A::PrelimErase)
    }
}
///Field `ERASE` reader - Page or Double Word erase mode
pub type ERASE_R = crate::BitReader<ERASE_A>;
///Page or Double Word erase mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERASE_A {
    ///0: No erase operation requested
    NoErase = 0,
    ///1: Erase operation requested
    Erase = 1,
}
impl From<ERASE_A> for bool {
    #[inline(always)]
    fn from(variant: ERASE_A) -> Self {
        variant as u8 != 0
    }
}
impl ERASE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ERASE_A {
        match self.bits {
            false => ERASE_A::NoErase,
            true => ERASE_A::Erase,
        }
    }
    ///Checks if the value of the field is `NoErase`
    #[inline(always)]
    pub fn is_no_erase(&self) -> bool {
        *self == ERASE_A::NoErase
    }
    ///Checks if the value of the field is `Erase`
    #[inline(always)]
    pub fn is_erase(&self) -> bool {
        *self == ERASE_A::Erase
    }
}
///Field `ERASE` writer - Page or Double Word erase mode
pub type ERASE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PECR_SPEC, ERASE_A, O>;
impl<'a, const O: u8> ERASE_W<'a, O> {
    ///No erase operation requested
    #[inline(always)]
    pub fn no_erase(self) -> &'a mut W {
        self.variant(ERASE_A::NoErase)
    }
    ///Erase operation requested
    #[inline(always)]
    pub fn erase(self) -> &'a mut W {
        self.variant(ERASE_A::Erase)
    }
}
///Field `FPRG` reader - Half Page/Double Word programming mode
pub type FPRG_R = crate::BitReader<FPRG_A>;
///Half Page/Double Word programming mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPRG_A {
    ///0: Half Page programming disabled
    Disabled = 0,
    ///1: Half Page programming enabled
    Enabled = 1,
}
impl From<FPRG_A> for bool {
    #[inline(always)]
    fn from(variant: FPRG_A) -> Self {
        variant as u8 != 0
    }
}
impl FPRG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FPRG_A {
        match self.bits {
            false => FPRG_A::Disabled,
            true => FPRG_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FPRG_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FPRG_A::Enabled
    }
}
///Field `FPRG` writer - Half Page/Double Word programming mode
pub type FPRG_W<'a, const O: u8> = crate::BitWriter<'a, u32, PECR_SPEC, FPRG_A, O>;
impl<'a, const O: u8> FPRG_W<'a, O> {
    ///Half Page programming disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FPRG_A::Disabled)
    }
    ///Half Page programming enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FPRG_A::Enabled)
    }
}
///Field `PARALLELBANK` reader - Parallel bank mode
pub type PARALLELBANK_R = crate::BitReader<PARALLELBANK_A>;
///Parallel bank mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PARALLELBANK_A {
    ///0: Parallel bank mode disabled
    Disabled = 0,
    ///1: Parallel bank mode enabled
    Enabled = 1,
}
impl From<PARALLELBANK_A> for bool {
    #[inline(always)]
    fn from(variant: PARALLELBANK_A) -> Self {
        variant as u8 != 0
    }
}
impl PARALLELBANK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PARALLELBANK_A {
        match self.bits {
            false => PARALLELBANK_A::Disabled,
            true => PARALLELBANK_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PARALLELBANK_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PARALLELBANK_A::Enabled
    }
}
///Field `PARALLELBANK` writer - Parallel bank mode
pub type PARALLELBANK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PECR_SPEC, PARALLELBANK_A, O>;
impl<'a, const O: u8> PARALLELBANK_W<'a, O> {
    ///Parallel bank mode disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PARALLELBANK_A::Disabled)
    }
    ///Parallel bank mode enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PARALLELBANK_A::Enabled)
    }
}
///Field `EOPIE` reader - End of programming interrupt enable
pub type EOPIE_R = crate::BitReader<EOPIE_A>;
///End of programming interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOPIE_A {
    ///0: End of program interrupt disable
    Disabled = 0,
    ///1: End of program interrupt enable
    Enabled = 1,
}
impl From<EOPIE_A> for bool {
    #[inline(always)]
    fn from(variant: EOPIE_A) -> Self {
        variant as u8 != 0
    }
}
impl EOPIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EOPIE_A {
        match self.bits {
            false => EOPIE_A::Disabled,
            true => EOPIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EOPIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EOPIE_A::Enabled
    }
}
///Field `EOPIE` writer - End of programming interrupt enable
pub type EOPIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PECR_SPEC, EOPIE_A, O>;
impl<'a, const O: u8> EOPIE_W<'a, O> {
    ///End of program interrupt disable
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EOPIE_A::Disabled)
    }
    ///End of program interrupt enable
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EOPIE_A::Enabled)
    }
}
///Field `ERRIE` reader - Error interrupt enable
pub type ERRIE_R = crate::BitReader<ERRIE_A>;
///Error interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERRIE_A {
    ///0: Error interrupt disable
    Disabled = 0,
    ///1: Error interrupt enable
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
///Field `ERRIE` writer - Error interrupt enable
pub type ERRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PECR_SPEC, ERRIE_A, O>;
impl<'a, const O: u8> ERRIE_W<'a, O> {
    ///Error interrupt disable
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ERRIE_A::Disabled)
    }
    ///Error interrupt enable
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ERRIE_A::Enabled)
    }
}
///Field `OBL_LAUNCH` reader - Launch the option byte loading
pub type OBL_LAUNCH_R = crate::BitReader<OBL_LAUNCHR_A>;
///Launch the option byte loading
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OBL_LAUNCHR_A {
    ///0: Option byte loaded
    Complete = 0,
    ///1: Option byte loading to be done
    NotComplete = 1,
}
impl From<OBL_LAUNCHR_A> for bool {
    #[inline(always)]
    fn from(variant: OBL_LAUNCHR_A) -> Self {
        variant as u8 != 0
    }
}
impl OBL_LAUNCH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OBL_LAUNCHR_A {
        match self.bits {
            false => OBL_LAUNCHR_A::Complete,
            true => OBL_LAUNCHR_A::NotComplete,
        }
    }
    ///Checks if the value of the field is `Complete`
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == OBL_LAUNCHR_A::Complete
    }
    ///Checks if the value of the field is `NotComplete`
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == OBL_LAUNCHR_A::NotComplete
    }
}
///Launch the option byte loading
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OBL_LAUNCHW_AW {
    ///1: Reload option byte
    Reload = 1,
}
impl From<OBL_LAUNCHW_AW> for bool {
    #[inline(always)]
    fn from(variant: OBL_LAUNCHW_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `OBL_LAUNCH` writer - Launch the option byte loading
pub type OBL_LAUNCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, PECR_SPEC, OBL_LAUNCHW_AW, O>;
impl<'a, const O: u8> OBL_LAUNCH_W<'a, O> {
    ///Reload option byte
    #[inline(always)]
    pub fn reload(self) -> &'a mut W {
        self.variant(OBL_LAUNCHW_AW::Reload)
    }
}
impl R {
    ///Bit 0 - FLASH_PECR and data EEPROM lock
    #[inline(always)]
    pub fn pelock(&self) -> PELOCK_R {
        PELOCK_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Program memory lock
    #[inline(always)]
    pub fn prglock(&self) -> PRGLOCK_R {
        PRGLOCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Option bytes block lock
    #[inline(always)]
    pub fn optlock(&self) -> OPTLOCK_R {
        OPTLOCK_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Program memory selection
    #[inline(always)]
    pub fn prog(&self) -> PROG_R {
        PROG_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Data EEPROM selection
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - Fixed time data write for Byte, Half Word and Word programming
    #[inline(always)]
    pub fn fix(&self) -> FIX_R {
        FIX_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Page or Double Word erase mode
    #[inline(always)]
    pub fn erase(&self) -> ERASE_R {
        ERASE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Half Page/Double Word programming mode
    #[inline(always)]
    pub fn fprg(&self) -> FPRG_R {
        FPRG_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 15 - Parallel bank mode
    #[inline(always)]
    pub fn parallelbank(&self) -> PARALLELBANK_R {
        PARALLELBANK_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - End of programming interrupt enable
    #[inline(always)]
    pub fn eopie(&self) -> EOPIE_R {
        EOPIE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Error interrupt enable
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Launch the option byte loading
    #[inline(always)]
    pub fn obl_launch(&self) -> OBL_LAUNCH_R {
        OBL_LAUNCH_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - FLASH_PECR and data EEPROM lock
    #[inline(always)]
    #[must_use]
    pub fn pelock(&mut self) -> PELOCK_W<0> {
        PELOCK_W::new(self)
    }
    ///Bit 1 - Program memory lock
    #[inline(always)]
    #[must_use]
    pub fn prglock(&mut self) -> PRGLOCK_W<1> {
        PRGLOCK_W::new(self)
    }
    ///Bit 2 - Option bytes block lock
    #[inline(always)]
    #[must_use]
    pub fn optlock(&mut self) -> OPTLOCK_W<2> {
        OPTLOCK_W::new(self)
    }
    ///Bit 3 - Program memory selection
    #[inline(always)]
    #[must_use]
    pub fn prog(&mut self) -> PROG_W<3> {
        PROG_W::new(self)
    }
    ///Bit 4 - Data EEPROM selection
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<4> {
        DATA_W::new(self)
    }
    ///Bit 8 - Fixed time data write for Byte, Half Word and Word programming
    #[inline(always)]
    #[must_use]
    pub fn fix(&mut self) -> FIX_W<8> {
        FIX_W::new(self)
    }
    ///Bit 9 - Page or Double Word erase mode
    #[inline(always)]
    #[must_use]
    pub fn erase(&mut self) -> ERASE_W<9> {
        ERASE_W::new(self)
    }
    ///Bit 10 - Half Page/Double Word programming mode
    #[inline(always)]
    #[must_use]
    pub fn fprg(&mut self) -> FPRG_W<10> {
        FPRG_W::new(self)
    }
    ///Bit 15 - Parallel bank mode
    #[inline(always)]
    #[must_use]
    pub fn parallelbank(&mut self) -> PARALLELBANK_W<15> {
        PARALLELBANK_W::new(self)
    }
    ///Bit 16 - End of programming interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn eopie(&mut self) -> EOPIE_W<16> {
        EOPIE_W::new(self)
    }
    ///Bit 17 - Error interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ERRIE_W<17> {
        ERRIE_W::new(self)
    }
    ///Bit 18 - Launch the option byte loading
    #[inline(always)]
    #[must_use]
    pub fn obl_launch(&mut self) -> OBL_LAUNCH_W<18> {
        OBL_LAUNCH_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Program/erase control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pecr](index.html) module
pub struct PECR_SPEC;
impl crate::RegisterSpec for PECR_SPEC {
    type Ux = u32;
}
///`read()` method returns [pecr::R](R) reader structure
impl crate::Readable for PECR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [pecr::W](W) writer structure
impl crate::Writable for PECR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets PECR to value 0x07
impl crate::Resettable for PECR_SPEC {
    const RESET_VALUE: Self::Ux = 0x07;
}
