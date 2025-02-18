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
    ///1: Flash programming activated
    Program = 1,
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
    pub fn variant(&self) -> Option<PG_A> {
        match self.bits {
            true => Some(PG_A::Program),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Program`
    #[inline(always)]
    pub fn is_program(&self) -> bool {
        *self == PG_A::Program
    }
}
///Field `PG` writer - Programming
pub type PG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, PG_A, O>;
impl<'a, const O: u8> PG_W<'a, O> {
    ///Flash programming activated
    #[inline(always)]
    pub fn program(self) -> &'a mut W {
        self.variant(PG_A::Program)
    }
}
///Field `PER` reader - Page erase
pub type PER_R = crate::BitReader<PER_A>;
///Page erase
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PER_A {
    ///1: Erase activated for selected page
    PageErase = 1,
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
    pub fn variant(&self) -> Option<PER_A> {
        match self.bits {
            true => Some(PER_A::PageErase),
            _ => None,
        }
    }
    ///Checks if the value of the field is `PageErase`
    #[inline(always)]
    pub fn is_page_erase(&self) -> bool {
        *self == PER_A::PageErase
    }
}
///Field `PER` writer - Page erase
pub type PER_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, PER_A, O>;
impl<'a, const O: u8> PER_W<'a, O> {
    ///Erase activated for selected page
    #[inline(always)]
    pub fn page_erase(self) -> &'a mut W {
        self.variant(PER_A::PageErase)
    }
}
///Field `MER` reader - Mass erase
pub type MER_R = crate::BitReader<MER_A>;
///Mass erase
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MER_A {
    ///1: Erase activated for all user sectors
    MassErase = 1,
}
impl From<MER_A> for bool {
    #[inline(always)]
    fn from(variant: MER_A) -> Self {
        variant as u8 != 0
    }
}
impl MER_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<MER_A> {
        match self.bits {
            true => Some(MER_A::MassErase),
            _ => None,
        }
    }
    ///Checks if the value of the field is `MassErase`
    #[inline(always)]
    pub fn is_mass_erase(&self) -> bool {
        *self == MER_A::MassErase
    }
}
///Field `MER` writer - Mass erase
pub type MER_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, MER_A, O>;
impl<'a, const O: u8> MER_W<'a, O> {
    ///Erase activated for all user sectors
    #[inline(always)]
    pub fn mass_erase(self) -> &'a mut W {
        self.variant(MER_A::MassErase)
    }
}
///Field `OPTPG` reader - Option byte programming
pub type OPTPG_R = crate::BitReader<OPTPG_A>;
///Option byte programming
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPTPG_A {
    ///1: Program option byte activated
    OptionByteProgramming = 1,
}
impl From<OPTPG_A> for bool {
    #[inline(always)]
    fn from(variant: OPTPG_A) -> Self {
        variant as u8 != 0
    }
}
impl OPTPG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<OPTPG_A> {
        match self.bits {
            true => Some(OPTPG_A::OptionByteProgramming),
            _ => None,
        }
    }
    ///Checks if the value of the field is `OptionByteProgramming`
    #[inline(always)]
    pub fn is_option_byte_programming(&self) -> bool {
        *self == OPTPG_A::OptionByteProgramming
    }
}
///Field `OPTPG` writer - Option byte programming
pub type OPTPG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, OPTPG_A, O>;
impl<'a, const O: u8> OPTPG_W<'a, O> {
    ///Program option byte activated
    #[inline(always)]
    pub fn option_byte_programming(self) -> &'a mut W {
        self.variant(OPTPG_A::OptionByteProgramming)
    }
}
///Field `OPTER` reader - Option byte erase
pub type OPTER_R = crate::BitReader<OPTER_A>;
///Option byte erase
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPTER_A {
    ///1: Erase option byte activated
    OptionByteErase = 1,
}
impl From<OPTER_A> for bool {
    #[inline(always)]
    fn from(variant: OPTER_A) -> Self {
        variant as u8 != 0
    }
}
impl OPTER_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<OPTER_A> {
        match self.bits {
            true => Some(OPTER_A::OptionByteErase),
            _ => None,
        }
    }
    ///Checks if the value of the field is `OptionByteErase`
    #[inline(always)]
    pub fn is_option_byte_erase(&self) -> bool {
        *self == OPTER_A::OptionByteErase
    }
}
///Field `OPTER` writer - Option byte erase
pub type OPTER_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, OPTER_A, O>;
impl<'a, const O: u8> OPTER_W<'a, O> {
    ///Erase option byte activated
    #[inline(always)]
    pub fn option_byte_erase(self) -> &'a mut W {
        self.variant(OPTER_A::OptionByteErase)
    }
}
///Field `STRT` reader - Start
pub type STRT_R = crate::BitReader<STRT_A>;
///Start
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STRT_A {
    ///1: Trigger an erase operation
    Start = 1,
}
impl From<STRT_A> for bool {
    #[inline(always)]
    fn from(variant: STRT_A) -> Self {
        variant as u8 != 0
    }
}
impl STRT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<STRT_A> {
        match self.bits {
            true => Some(STRT_A::Start),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Start`
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == STRT_A::Start
    }
}
///Field `STRT` writer - Start
pub type STRT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, STRT_A, O>;
impl<'a, const O: u8> STRT_W<'a, O> {
    ///Trigger an erase operation
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(STRT_A::Start)
    }
}
///Field `LOCK` reader - Lock
pub type LOCK_R = crate::BitReader<LOCKR_A>;
///Lock
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
///Lock
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOCKW_AW {
    ///1: Lock the FLASH_CR register
    Lock = 1,
}
impl From<LOCKW_AW> for bool {
    #[inline(always)]
    fn from(variant: LOCKW_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `LOCK` writer - Lock
pub type LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, LOCKW_AW, O>;
impl<'a, const O: u8> LOCK_W<'a, O> {
    ///Lock the FLASH_CR register
    #[inline(always)]
    pub fn lock(self) -> &'a mut W {
        self.variant(LOCKW_AW::Lock)
    }
}
///Field `OPTWRE` reader - Option bytes write enable
pub type OPTWRE_R = crate::BitReader<OPTWRE_A>;
///Option bytes write enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPTWRE_A {
    ///0: Option byte write enabled
    Disabled = 0,
    ///1: Option byte write disabled
    Enabled = 1,
}
impl From<OPTWRE_A> for bool {
    #[inline(always)]
    fn from(variant: OPTWRE_A) -> Self {
        variant as u8 != 0
    }
}
impl OPTWRE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OPTWRE_A {
        match self.bits {
            false => OPTWRE_A::Disabled,
            true => OPTWRE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OPTWRE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OPTWRE_A::Enabled
    }
}
///Field `OPTWRE` writer - Option bytes write enable
pub type OPTWRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, OPTWRE_A, O>;
impl<'a, const O: u8> OPTWRE_W<'a, O> {
    ///Option byte write enabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OPTWRE_A::Disabled)
    }
    ///Option byte write disabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OPTWRE_A::Enabled)
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
///Field `OBL_LAUNCH` reader - Force option byte loading
pub type OBL_LAUNCH_R = crate::BitReader<OBL_LAUNCH_A>;
///Force option byte loading
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OBL_LAUNCH_A {
    ///0: Force option byte loading inactive
    Inactive = 0,
    ///1: Force option byte loading active
    Active = 1,
}
impl From<OBL_LAUNCH_A> for bool {
    #[inline(always)]
    fn from(variant: OBL_LAUNCH_A) -> Self {
        variant as u8 != 0
    }
}
impl OBL_LAUNCH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OBL_LAUNCH_A {
        match self.bits {
            false => OBL_LAUNCH_A::Inactive,
            true => OBL_LAUNCH_A::Active,
        }
    }
    ///Checks if the value of the field is `Inactive`
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == OBL_LAUNCH_A::Inactive
    }
    ///Checks if the value of the field is `Active`
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == OBL_LAUNCH_A::Active
    }
}
///Field `OBL_LAUNCH` writer - Force option byte loading
pub type OBL_LAUNCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, OBL_LAUNCH_A, O>;
impl<'a, const O: u8> OBL_LAUNCH_W<'a, O> {
    ///Force option byte loading inactive
    #[inline(always)]
    pub fn inactive(self) -> &'a mut W {
        self.variant(OBL_LAUNCH_A::Inactive)
    }
    ///Force option byte loading active
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(OBL_LAUNCH_A::Active)
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
    ///Bit 2 - Mass erase
    #[inline(always)]
    pub fn mer(&self) -> MER_R {
        MER_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - Option byte programming
    #[inline(always)]
    pub fn optpg(&self) -> OPTPG_R {
        OPTPG_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Option byte erase
    #[inline(always)]
    pub fn opter(&self) -> OPTER_R {
        OPTER_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Start
    #[inline(always)]
    pub fn strt(&self) -> STRT_R {
        STRT_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Lock
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 9 - Option bytes write enable
    #[inline(always)]
    pub fn optwre(&self) -> OPTWRE_R {
        OPTWRE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Error interrupt enable
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 12 - End of operation interrupt enable
    #[inline(always)]
    pub fn eopie(&self) -> EOPIE_R {
        EOPIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Force option byte loading
    #[inline(always)]
    pub fn obl_launch(&self) -> OBL_LAUNCH_R {
        OBL_LAUNCH_R::new(((self.bits >> 13) & 1) != 0)
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
    ///Bit 2 - Mass erase
    #[inline(always)]
    #[must_use]
    pub fn mer(&mut self) -> MER_W<2> {
        MER_W::new(self)
    }
    ///Bit 4 - Option byte programming
    #[inline(always)]
    #[must_use]
    pub fn optpg(&mut self) -> OPTPG_W<4> {
        OPTPG_W::new(self)
    }
    ///Bit 5 - Option byte erase
    #[inline(always)]
    #[must_use]
    pub fn opter(&mut self) -> OPTER_W<5> {
        OPTER_W::new(self)
    }
    ///Bit 6 - Start
    #[inline(always)]
    #[must_use]
    pub fn strt(&mut self) -> STRT_W<6> {
        STRT_W::new(self)
    }
    ///Bit 7 - Lock
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<7> {
        LOCK_W::new(self)
    }
    ///Bit 9 - Option bytes write enable
    #[inline(always)]
    #[must_use]
    pub fn optwre(&mut self) -> OPTWRE_W<9> {
        OPTWRE_W::new(self)
    }
    ///Bit 10 - Error interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ERRIE_W<10> {
        ERRIE_W::new(self)
    }
    ///Bit 12 - End of operation interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn eopie(&mut self) -> EOPIE_W<12> {
        EOPIE_W::new(self)
    }
    ///Bit 13 - Force option byte loading
    #[inline(always)]
    #[must_use]
    pub fn obl_launch(&mut self) -> OBL_LAUNCH_W<13> {
        OBL_LAUNCH_W::new(self)
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
///`reset()` method sets CR to value 0x80
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0x80;
}
