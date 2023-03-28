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
///Field `SER` reader - Sector Erase
pub type SER_R = crate::BitReader<SER_A>;
///Sector Erase
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SER_A {
    ///1: Erase activated for selected sector
    SectorErase = 1,
}
impl From<SER_A> for bool {
    #[inline(always)]
    fn from(variant: SER_A) -> Self {
        variant as u8 != 0
    }
}
impl SER_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<SER_A> {
        match self.bits {
            true => Some(SER_A::SectorErase),
            _ => None,
        }
    }
    ///Checks if the value of the field is `SectorErase`
    #[inline(always)]
    pub fn is_sector_erase(&self) -> bool {
        *self == SER_A::SectorErase
    }
}
///Field `SER` writer - Sector Erase
pub type SER_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, SER_A, O>;
impl<'a, const O: u8> SER_W<'a, O> {
    ///Erase activated for selected sector
    #[inline(always)]
    pub fn sector_erase(self) -> &'a mut W {
        self.variant(SER_A::SectorErase)
    }
}
///Field `MER` reader - Mass Erase of sectors 0 to 11
pub type MER_R = crate::BitReader<MER_A>;
///Mass Erase of sectors 0 to 11
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
///Field `MER` writer - Mass Erase of sectors 0 to 11
pub type MER_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, MER_A, O>;
impl<'a, const O: u8> MER_W<'a, O> {
    ///Erase activated for all user sectors
    #[inline(always)]
    pub fn mass_erase(self) -> &'a mut W {
        self.variant(MER_A::MassErase)
    }
}
///Field `SNB` reader - Sector number
pub type SNB_R = crate::FieldReader<u8, u8>;
///Field `SNB` writer - Sector number
pub type SNB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 4, O>;
///Field `PSIZE` reader - Program size
pub type PSIZE_R = crate::FieldReader<u8, PSIZE_A>;
///Program size
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PSIZE_A {
    ///0: Program x8
    Psize8 = 0,
    ///1: Program x16
    Psize16 = 1,
    ///2: Program x32
    Psize32 = 2,
    ///3: Program x64
    Psize64 = 3,
}
impl From<PSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: PSIZE_A) -> Self {
        variant as _
    }
}
impl PSIZE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PSIZE_A {
        match self.bits {
            0 => PSIZE_A::Psize8,
            1 => PSIZE_A::Psize16,
            2 => PSIZE_A::Psize32,
            3 => PSIZE_A::Psize64,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Psize8`
    #[inline(always)]
    pub fn is_psize8(&self) -> bool {
        *self == PSIZE_A::Psize8
    }
    ///Checks if the value of the field is `Psize16`
    #[inline(always)]
    pub fn is_psize16(&self) -> bool {
        *self == PSIZE_A::Psize16
    }
    ///Checks if the value of the field is `Psize32`
    #[inline(always)]
    pub fn is_psize32(&self) -> bool {
        *self == PSIZE_A::Psize32
    }
    ///Checks if the value of the field is `Psize64`
    #[inline(always)]
    pub fn is_psize64(&self) -> bool {
        *self == PSIZE_A::Psize64
    }
}
///Field `PSIZE` writer - Program size
pub type PSIZE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CR_SPEC, u8, PSIZE_A, 2, O>;
impl<'a, const O: u8> PSIZE_W<'a, O> {
    ///Program x8
    #[inline(always)]
    pub fn psize8(self) -> &'a mut W {
        self.variant(PSIZE_A::Psize8)
    }
    ///Program x16
    #[inline(always)]
    pub fn psize16(self) -> &'a mut W {
        self.variant(PSIZE_A::Psize16)
    }
    ///Program x32
    #[inline(always)]
    pub fn psize32(self) -> &'a mut W {
        self.variant(PSIZE_A::Psize32)
    }
    ///Program x64
    #[inline(always)]
    pub fn psize64(self) -> &'a mut W {
        self.variant(PSIZE_A::Psize64)
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
///Field `LOCK` reader - Lock
pub type LOCK_R = crate::BitReader<LOCK_A>;
///Lock
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOCK_A {
    ///0: FLASH_CR register is unlocked
    Unlocked = 0,
    ///1: FLASH_CR register is locked
    Locked = 1,
}
impl From<LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: LOCK_A) -> Self {
        variant as u8 != 0
    }
}
impl LOCK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LOCK_A {
        match self.bits {
            false => LOCK_A::Unlocked,
            true => LOCK_A::Locked,
        }
    }
    ///Checks if the value of the field is `Unlocked`
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == LOCK_A::Unlocked
    }
    ///Checks if the value of the field is `Locked`
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == LOCK_A::Locked
    }
}
///Field `LOCK` writer - Lock
pub type LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, LOCK_A, O>;
impl<'a, const O: u8> LOCK_W<'a, O> {
    ///FLASH_CR register is unlocked
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(LOCK_A::Unlocked)
    }
    ///FLASH_CR register is locked
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(LOCK_A::Locked)
    }
}
impl R {
    ///Bit 0 - Programming
    #[inline(always)]
    pub fn pg(&self) -> PG_R {
        PG_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Sector Erase
    #[inline(always)]
    pub fn ser(&self) -> SER_R {
        SER_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Mass Erase of sectors 0 to 11
    #[inline(always)]
    pub fn mer(&self) -> MER_R {
        MER_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:6 - Sector number
    #[inline(always)]
    pub fn snb(&self) -> SNB_R {
        SNB_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
    ///Bits 8:9 - Program size
    #[inline(always)]
    pub fn psize(&self) -> PSIZE_R {
        PSIZE_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 16 - Start
    #[inline(always)]
    pub fn strt(&self) -> STRT_R {
        STRT_R::new(((self.bits >> 16) & 1) != 0)
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
    ///Bit 31 - Lock
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
    ///Bit 1 - Sector Erase
    #[inline(always)]
    #[must_use]
    pub fn ser(&mut self) -> SER_W<1> {
        SER_W::new(self)
    }
    ///Bit 2 - Mass Erase of sectors 0 to 11
    #[inline(always)]
    #[must_use]
    pub fn mer(&mut self) -> MER_W<2> {
        MER_W::new(self)
    }
    ///Bits 3:6 - Sector number
    #[inline(always)]
    #[must_use]
    pub fn snb(&mut self) -> SNB_W<3> {
        SNB_W::new(self)
    }
    ///Bits 8:9 - Program size
    #[inline(always)]
    #[must_use]
    pub fn psize(&mut self) -> PSIZE_W<8> {
        PSIZE_W::new(self)
    }
    ///Bit 16 - Start
    #[inline(always)]
    #[must_use]
    pub fn strt(&mut self) -> STRT_W<16> {
        STRT_W::new(self)
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
    ///Bit 31 - Lock
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
///Control register
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
///`reset()` method sets CR to value 0x8000_0000
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000_0000;
}
