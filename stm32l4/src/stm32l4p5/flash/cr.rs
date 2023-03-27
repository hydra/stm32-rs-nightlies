///Register `CR` reader
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CR` writer
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PG` reader - Programming
pub type PG_R = crate::BitReader<PG_A>;
///Programming
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PG_A {
    ///0: Flash programming disabled
    Disabled = 0,
    ///1: Flash programming enabled
    Enabled = 1,
}
impl From<PG_A> for bool {
    #[inline(always)]
    fn from(variant: PG_A) -> Self {
        variant as u8 != 0
    }
}
impl PG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PG_A {
        match self.bits {
            false => PG_A::Disabled,
            true => PG_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PG_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PG_A::Enabled
    }
}
///Field `PG` writer - Programming
pub type PG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, PG_A, O>;
impl<'a, const O: u8> PG_W<'a, O> {
    ///Flash programming disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PG_A::Disabled)
    }
    ///Flash programming enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PG_A::Enabled)
    }
}
///Field `PER` reader - Page erase
pub type PER_R = crate::BitReader<PER_A>;
///Page erase
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PER_A {
    ///0: Page erase disabled
    Disabled = 0,
    ///1: Page erase enabled
    Enabled = 1,
}
impl From<PER_A> for bool {
    #[inline(always)]
    fn from(variant: PER_A) -> Self {
        variant as u8 != 0
    }
}
impl PER_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PER_A {
        match self.bits {
            false => PER_A::Disabled,
            true => PER_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PER_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PER_A::Enabled
    }
}
///Field `PER` writer - Page erase
pub type PER_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, PER_A, O>;
impl<'a, const O: u8> PER_W<'a, O> {
    ///Page erase disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PER_A::Disabled)
    }
    ///Page erase enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PER_A::Enabled)
    }
}
///Field `MER1` reader - Bank 1 Mass erase
pub type MER1_R = crate::BitReader<MER1W_A>;
///Bank 1 Mass erase
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MER1W_A {
    ///1: This bit triggers the bank 1 mass erase (all bank 1 user pages) when set
    MassErase = 1,
}
impl From<MER1W_A> for bool {
    #[inline(always)]
    fn from(variant: MER1W_A) -> Self {
        variant as u8 != 0
    }
}
impl MER1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<MER1W_A> {
        match self.bits {
            true => Some(MER1W_A::MassErase),
            _ => None,
        }
    }
    ///Checks if the value of the field is `MassErase`
    #[inline(always)]
    pub fn is_mass_erase(&self) -> bool {
        *self == MER1W_A::MassErase
    }
}
///Field `MER1` writer - Bank 1 Mass erase
pub type MER1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, MER1W_A, O>;
impl<'a, const O: u8> MER1_W<'a, O> {
    ///This bit triggers the bank 1 mass erase (all bank 1 user pages) when set
    #[inline(always)]
    pub fn mass_erase(self) -> &'a mut W {
        self.variant(MER1W_A::MassErase)
    }
}
///Field `PNB` reader - Page number
pub type PNB_R = crate::FieldReader<u8, u8>;
///Field `PNB` writer - Page number
pub type PNB_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CR_SPEC, u8, u8, 8, O>;
///Field `BKER` reader - Bank erase
pub type BKER_R = crate::BitReader<BKER_A>;
///Bank erase
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKER_A {
    ///0: Bank 1 is selected for page erase
    Bank1 = 0,
    ///1: Bank 2 is selected for page erase
    Bank2 = 1,
}
impl From<BKER_A> for bool {
    #[inline(always)]
    fn from(variant: BKER_A) -> Self {
        variant as u8 != 0
    }
}
impl BKER_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BKER_A {
        match self.bits {
            false => BKER_A::Bank1,
            true => BKER_A::Bank2,
        }
    }
    ///Checks if the value of the field is `Bank1`
    #[inline(always)]
    pub fn is_bank1(&self) -> bool {
        *self == BKER_A::Bank1
    }
    ///Checks if the value of the field is `Bank2`
    #[inline(always)]
    pub fn is_bank2(&self) -> bool {
        *self == BKER_A::Bank2
    }
}
///Field `BKER` writer - Bank erase
pub type BKER_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, BKER_A, O>;
impl<'a, const O: u8> BKER_W<'a, O> {
    ///Bank 1 is selected for page erase
    #[inline(always)]
    pub fn bank1(self) -> &'a mut W {
        self.variant(BKER_A::Bank1)
    }
    ///Bank 2 is selected for page erase
    #[inline(always)]
    pub fn bank2(self) -> &'a mut W {
        self.variant(BKER_A::Bank2)
    }
}
///Field `MER2` reader - Bank 2 Mass erase
pub type MER2_R = crate::BitReader<MER2W_A>;
///Bank 2 Mass erase
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MER2W_A {
    ///1: This bit triggers the bank 2 mass erase (all bank 2 user pages) when set
    MassErase = 1,
}
impl From<MER2W_A> for bool {
    #[inline(always)]
    fn from(variant: MER2W_A) -> Self {
        variant as u8 != 0
    }
}
impl MER2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<MER2W_A> {
        match self.bits {
            true => Some(MER2W_A::MassErase),
            _ => None,
        }
    }
    ///Checks if the value of the field is `MassErase`
    #[inline(always)]
    pub fn is_mass_erase(&self) -> bool {
        *self == MER2W_A::MassErase
    }
}
///Field `MER2` writer - Bank 2 Mass erase
pub type MER2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, MER2W_A, O>;
impl<'a, const O: u8> MER2_W<'a, O> {
    ///This bit triggers the bank 2 mass erase (all bank 2 user pages) when set
    #[inline(always)]
    pub fn mass_erase(self) -> &'a mut W {
        self.variant(MER2W_A::MassErase)
    }
}
///Field `START` reader - Start
pub type START_R = crate::BitReader<STARTR_A>;
///Start
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STARTR_A {
    ///0: Cleared when BSY bit is cleared in SR
    Complete = 0,
    ///1: Erase operation requested
    Requested = 1,
}
impl From<STARTR_A> for bool {
    #[inline(always)]
    fn from(variant: STARTR_A) -> Self {
        variant as u8 != 0
    }
}
impl START_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> STARTR_A {
        match self.bits {
            false => STARTR_A::Complete,
            true => STARTR_A::Requested,
        }
    }
    ///Checks if the value of the field is `Complete`
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == STARTR_A::Complete
    }
    ///Checks if the value of the field is `Requested`
    #[inline(always)]
    pub fn is_requested(&self) -> bool {
        *self == STARTR_A::Requested
    }
}
///Start
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STARTW_AW {
    ///1: Trigger an erase operation
    Start = 1,
}
impl From<STARTW_AW> for bool {
    #[inline(always)]
    fn from(variant: STARTW_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `START` writer - Start
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, STARTW_AW, O>;
impl<'a, const O: u8> START_W<'a, O> {
    ///Trigger an erase operation
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(STARTW_AW::Start)
    }
}
///Field `OPTSTRT` reader - Options modification start
pub type OPTSTRT_R = crate::BitReader<OPTSTRTR_A>;
///Options modification start
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPTSTRTR_A {
    ///0: Cleared when BSY bit is cleared in SR
    Complete = 0,
    ///1: Options modification requested
    Requested = 1,
}
impl From<OPTSTRTR_A> for bool {
    #[inline(always)]
    fn from(variant: OPTSTRTR_A) -> Self {
        variant as u8 != 0
    }
}
impl OPTSTRT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OPTSTRTR_A {
        match self.bits {
            false => OPTSTRTR_A::Complete,
            true => OPTSTRTR_A::Requested,
        }
    }
    ///Checks if the value of the field is `Complete`
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == OPTSTRTR_A::Complete
    }
    ///Checks if the value of the field is `Requested`
    #[inline(always)]
    pub fn is_requested(&self) -> bool {
        *self == OPTSTRTR_A::Requested
    }
}
///Options modification start
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPTSTRTW_AW {
    ///1: This bit triggers an options operation when set
    Set = 1,
}
impl From<OPTSTRTW_AW> for bool {
    #[inline(always)]
    fn from(variant: OPTSTRTW_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `OPTSTRT` writer - Options modification start
pub type OPTSTRT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, OPTSTRTW_AW, O>;
impl<'a, const O: u8> OPTSTRT_W<'a, O> {
    ///This bit triggers an options operation when set
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(OPTSTRTW_AW::Set)
    }
}
///Field `FSTPG` reader - Fast programming
pub type FSTPG_R = crate::BitReader<FSTPG_A>;
///Fast programming
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FSTPG_A {
    ///0: Fast programming disabled
    Disabled = 0,
    ///1: Fast programming enabled
    Enabled = 1,
}
impl From<FSTPG_A> for bool {
    #[inline(always)]
    fn from(variant: FSTPG_A) -> Self {
        variant as u8 != 0
    }
}
impl FSTPG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FSTPG_A {
        match self.bits {
            false => FSTPG_A::Disabled,
            true => FSTPG_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FSTPG_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FSTPG_A::Enabled
    }
}
///Field `FSTPG` writer - Fast programming
pub type FSTPG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, FSTPG_A, O>;
impl<'a, const O: u8> FSTPG_W<'a, O> {
    ///Fast programming disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FSTPG_A::Disabled)
    }
    ///Fast programming enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FSTPG_A::Enabled)
    }
}
///Field `EOPIE` reader - End of operation interrupt enable
pub type EOPIE_R = crate::BitReader<EOPIE_A>;
///End of operation interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOPIE_A {
    ///0: End of operation interrupt disabled
    Disabled = 0,
    ///1: End of operation interrupt enabled
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
///Field `EOPIE` writer - End of operation interrupt enable
pub type EOPIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, EOPIE_A, O>;
impl<'a, const O: u8> EOPIE_W<'a, O> {
    ///End of operation interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EOPIE_A::Disabled)
    }
    ///End of operation interrupt enabled
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
    ///0: Error interrupt generation disabled
    Disabled = 0,
    ///1: Error interrupt generation enabled
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
pub type ERRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, ERRIE_A, O>;
impl<'a, const O: u8> ERRIE_W<'a, O> {
    ///Error interrupt generation disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ERRIE_A::Disabled)
    }
    ///Error interrupt generation enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ERRIE_A::Enabled)
    }
}
///Field `RDERRIE` reader - PCROP read error interrupt enable
pub type RDERRIE_R = crate::BitReader<RDERRIE_A>;
///PCROP read error interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDERRIE_A {
    ///0: PCROP read error interrupt disabled
    Disabled = 0,
    ///1: PCROP read error interrupt enabled
    Enabled = 1,
}
impl From<RDERRIE_A> for bool {
    #[inline(always)]
    fn from(variant: RDERRIE_A) -> Self {
        variant as u8 != 0
    }
}
impl RDERRIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RDERRIE_A {
        match self.bits {
            false => RDERRIE_A::Disabled,
            true => RDERRIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RDERRIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RDERRIE_A::Enabled
    }
}
///Field `RDERRIE` writer - PCROP read error interrupt enable
pub type RDERRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, RDERRIE_A, O>;
impl<'a, const O: u8> RDERRIE_W<'a, O> {
    ///PCROP read error interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RDERRIE_A::Disabled)
    }
    ///PCROP read error interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RDERRIE_A::Enabled)
    }
}
///Field `OBL_LAUNCH` reader - Force the option byte loading
pub type OBL_LAUNCH_R = crate::BitReader<OBL_LAUNCHR_A>;
///Force the option byte loading
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OBL_LAUNCHR_A {
    ///0: Option byte loading complete
    Complete = 0,
    ///1: Option byte loading requested
    Requested = 1,
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
            true => OBL_LAUNCHR_A::Requested,
        }
    }
    ///Checks if the value of the field is `Complete`
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == OBL_LAUNCHR_A::Complete
    }
    ///Checks if the value of the field is `Requested`
    #[inline(always)]
    pub fn is_requested(&self) -> bool {
        *self == OBL_LAUNCHR_A::Requested
    }
}
///Force the option byte loading
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OBL_LAUNCHW_AW {
    ///1: Force option byte reloading
    Set = 1,
}
impl From<OBL_LAUNCHW_AW> for bool {
    #[inline(always)]
    fn from(variant: OBL_LAUNCHW_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `OBL_LAUNCH` writer - Force the option byte loading
pub type OBL_LAUNCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, OBL_LAUNCHW_AW, O>;
impl<'a, const O: u8> OBL_LAUNCH_W<'a, O> {
    ///Force option byte reloading
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(OBL_LAUNCHW_AW::Set)
    }
}
///Field `OPTLOCK` reader - Options Lock
pub type OPTLOCK_R = crate::BitReader<OPTLOCKR_A>;
///Options Lock
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPTLOCKR_A {
    ///0: Option page is unlocked
    Unlocked = 0,
    ///1: All bits concerning user option in FLASH_CR register and so option page are locked
    Locked = 1,
}
impl From<OPTLOCKR_A> for bool {
    #[inline(always)]
    fn from(variant: OPTLOCKR_A) -> Self {
        variant as u8 != 0
    }
}
impl OPTLOCK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OPTLOCKR_A {
        match self.bits {
            false => OPTLOCKR_A::Unlocked,
            true => OPTLOCKR_A::Locked,
        }
    }
    ///Checks if the value of the field is `Unlocked`
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == OPTLOCKR_A::Unlocked
    }
    ///Checks if the value of the field is `Locked`
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == OPTLOCKR_A::Locked
    }
}
///Options Lock
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPTLOCKW_AW {
    ///1: This bit is set only. When set, all bits concerning user option in FLASH_CR register and so option page are locked
    Set = 1,
}
impl From<OPTLOCKW_AW> for bool {
    #[inline(always)]
    fn from(variant: OPTLOCKW_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `OPTLOCK` writer - Options Lock
pub type OPTLOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, OPTLOCKW_AW, O>;
impl<'a, const O: u8> OPTLOCK_W<'a, O> {
    ///This bit is set only. When set, all bits concerning user option in FLASH_CR register and so option page are locked
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(OPTLOCKW_AW::Set)
    }
}
///Field `LOCK` reader - FLASH_CR Lock
pub type LOCK_R = crate::BitReader<LOCKR_A>;
///FLASH_CR Lock
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOCKR_A {
    ///0: FLASH_CR register is unlocked
    Unlocked = 0,
    ///1: FLASH_CR register is locked
    Locked = 1,
}
impl From<LOCKR_A> for bool {
    #[inline(always)]
    fn from(variant: LOCKR_A) -> Self {
        variant as u8 != 0
    }
}
impl LOCK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LOCKR_A {
        match self.bits {
            false => LOCKR_A::Unlocked,
            true => LOCKR_A::Locked,
        }
    }
    ///Checks if the value of the field is `Unlocked`
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == LOCKR_A::Unlocked
    }
    ///Checks if the value of the field is `Locked`
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == LOCKR_A::Locked
    }
}
///FLASH_CR Lock
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOCKW_AW {
    ///1: This bit is set only. When set, the FLASH_CR register is locked
    Set = 1,
}
impl From<LOCKW_AW> for bool {
    #[inline(always)]
    fn from(variant: LOCKW_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `LOCK` writer - FLASH_CR Lock
pub type LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, LOCKW_AW, O>;
impl<'a, const O: u8> LOCK_W<'a, O> {
    ///This bit is set only. When set, the FLASH_CR register is locked
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(LOCKW_AW::Set)
    }
}
impl R {
    ///Bit 0 - Programming
    #[inline(always)]
    pub fn pg(&self) -> PG_R {
        PG_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Page erase
    #[inline(always)]
    pub fn per(&self) -> PER_R {
        PER_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Bank 1 Mass erase
    #[inline(always)]
    pub fn mer1(&self) -> MER1_R {
        MER1_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:10 - Page number
    #[inline(always)]
    pub fn pnb(&self) -> PNB_R {
        PNB_R::new(((self.bits >> 3) & 0xff) as u8)
    }
    ///Bit 11 - Bank erase
    #[inline(always)]
    pub fn bker(&self) -> BKER_R {
        BKER_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 15 - Bank 2 Mass erase
    #[inline(always)]
    pub fn mer2(&self) -> MER2_R {
        MER2_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Start
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Options modification start
    #[inline(always)]
    pub fn optstrt(&self) -> OPTSTRT_R {
        OPTSTRT_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Fast programming
    #[inline(always)]
    pub fn fstpg(&self) -> FSTPG_R {
        FSTPG_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 24 - End of operation interrupt enable
    #[inline(always)]
    pub fn eopie(&self) -> EOPIE_R {
        EOPIE_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Error interrupt enable
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - PCROP read error interrupt enable
    #[inline(always)]
    pub fn rderrie(&self) -> RDERRIE_R {
        RDERRIE_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Force the option byte loading
    #[inline(always)]
    pub fn obl_launch(&self) -> OBL_LAUNCH_R {
        OBL_LAUNCH_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 30 - Options Lock
    #[inline(always)]
    pub fn optlock(&self) -> OPTLOCK_R {
        OPTLOCK_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - FLASH_CR Lock
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Programming
    #[inline(always)]
    #[must_use]
    pub fn pg(&mut self) -> PG_W<0> {
        PG_W::new(self)
    }
    ///Bit 1 - Page erase
    #[inline(always)]
    #[must_use]
    pub fn per(&mut self) -> PER_W<1> {
        PER_W::new(self)
    }
    ///Bit 2 - Bank 1 Mass erase
    #[inline(always)]
    #[must_use]
    pub fn mer1(&mut self) -> MER1_W<2> {
        MER1_W::new(self)
    }
    ///Bits 3:10 - Page number
    #[inline(always)]
    #[must_use]
    pub fn pnb(&mut self) -> PNB_W<3> {
        PNB_W::new(self)
    }
    ///Bit 11 - Bank erase
    #[inline(always)]
    #[must_use]
    pub fn bker(&mut self) -> BKER_W<11> {
        BKER_W::new(self)
    }
    ///Bit 15 - Bank 2 Mass erase
    #[inline(always)]
    #[must_use]
    pub fn mer2(&mut self) -> MER2_W<15> {
        MER2_W::new(self)
    }
    ///Bit 16 - Start
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<16> {
        START_W::new(self)
    }
    ///Bit 17 - Options modification start
    #[inline(always)]
    #[must_use]
    pub fn optstrt(&mut self) -> OPTSTRT_W<17> {
        OPTSTRT_W::new(self)
    }
    ///Bit 18 - Fast programming
    #[inline(always)]
    #[must_use]
    pub fn fstpg(&mut self) -> FSTPG_W<18> {
        FSTPG_W::new(self)
    }
    ///Bit 24 - End of operation interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn eopie(&mut self) -> EOPIE_W<24> {
        EOPIE_W::new(self)
    }
    ///Bit 25 - Error interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ERRIE_W<25> {
        ERRIE_W::new(self)
    }
    ///Bit 26 - PCROP read error interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn rderrie(&mut self) -> RDERRIE_W<26> {
        RDERRIE_W::new(self)
    }
    ///Bit 27 - Force the option byte loading
    #[inline(always)]
    #[must_use]
    pub fn obl_launch(&mut self) -> OBL_LAUNCH_W<27> {
        OBL_LAUNCH_W::new(self)
    }
    ///Bit 30 - Options Lock
    #[inline(always)]
    #[must_use]
    pub fn optlock(&mut self) -> OPTLOCK_W<30> {
        OPTLOCK_W::new(self)
    }
    ///Bit 31 - FLASH_CR Lock
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<31> {
        LOCK_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Flash control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr](index.html) module
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
///`read()` method returns [cr::R](R) reader structure
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cr::W](W) writer structure
impl crate::Writable for CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CR to value 0xc000_0000
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0xc000_0000;
}
