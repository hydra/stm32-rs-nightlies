///Register `RSTE1R` reader
pub struct R(crate::R<RSTE1R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSTE1R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSTE1R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSTE1R_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RSTE1R` writer
pub struct W(crate::W<RSTE1R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSTE1R_SPEC>;
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
impl From<crate::W<RSTE1R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSTE1R_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SRT` reader - SRT
pub type SRT_R = crate::BitReader<SRT_A>;
///SRT
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRT_A {
    ///0: No effect
    NoEffect = 0,
    ///1: Force output to its inactive state
    SetInactive = 1,
}
impl From<SRT_A> for bool {
    #[inline(always)]
    fn from(variant: SRT_A) -> Self {
        variant as u8 != 0
    }
}
impl SRT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SRT_A {
        match self.bits {
            false => SRT_A::NoEffect,
            true => SRT_A::SetInactive,
        }
    }
    ///Checks if the value of the field is `NoEffect`
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == SRT_A::NoEffect
    }
    ///Checks if the value of the field is `SetInactive`
    #[inline(always)]
    pub fn is_set_inactive(&self) -> bool {
        *self == SRT_A::SetInactive
    }
}
///Field `SRT` writer - SRT
pub type SRT_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTE1R_SPEC, SRT_A, O>;
impl<'a, const O: u8> SRT_W<'a, O> {
    ///No effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SRT_A::NoEffect)
    }
    ///Force output to its inactive state
    #[inline(always)]
    pub fn set_inactive(self) -> &'a mut W {
        self.variant(SRT_A::SetInactive)
    }
}
///Field `RESYNC` reader - RESYNC
pub type RESYNC_R = crate::BitReader<RESYNC_A>;
///RESYNC
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESYNC_A {
    ///0: Timer reset event coming solely from software or SYNC input event has no effect
    NoEffect = 0,
    ///1: Timer reset event coming solely from software or SYNC input event forces the output to its inactive state
    SetInactive = 1,
}
impl From<RESYNC_A> for bool {
    #[inline(always)]
    fn from(variant: RESYNC_A) -> Self {
        variant as u8 != 0
    }
}
impl RESYNC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RESYNC_A {
        match self.bits {
            false => RESYNC_A::NoEffect,
            true => RESYNC_A::SetInactive,
        }
    }
    ///Checks if the value of the field is `NoEffect`
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == RESYNC_A::NoEffect
    }
    ///Checks if the value of the field is `SetInactive`
    #[inline(always)]
    pub fn is_set_inactive(&self) -> bool {
        *self == RESYNC_A::SetInactive
    }
}
///Field `RESYNC` writer - RESYNC
pub type RESYNC_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTE1R_SPEC, RESYNC_A, O>;
impl<'a, const O: u8> RESYNC_W<'a, O> {
    ///Timer reset event coming solely from software or SYNC input event has no effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(RESYNC_A::NoEffect)
    }
    ///Timer reset event coming solely from software or SYNC input event forces the output to its inactive state
    #[inline(always)]
    pub fn set_inactive(self) -> &'a mut W {
        self.variant(RESYNC_A::SetInactive)
    }
}
///Field `PER` reader - PER
pub type PER_R = crate::BitReader<PER_A>;
///PER
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PER_A {
    ///0: Timer period event has no effect
    NoEffect = 0,
    ///1: Timer period event forces the output to its inactive state
    SetInactive = 1,
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
            false => PER_A::NoEffect,
            true => PER_A::SetInactive,
        }
    }
    ///Checks if the value of the field is `NoEffect`
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == PER_A::NoEffect
    }
    ///Checks if the value of the field is `SetInactive`
    #[inline(always)]
    pub fn is_set_inactive(&self) -> bool {
        *self == PER_A::SetInactive
    }
}
///Field `PER` writer - PER
pub type PER_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTE1R_SPEC, PER_A, O>;
impl<'a, const O: u8> PER_W<'a, O> {
    ///Timer period event has no effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(PER_A::NoEffect)
    }
    ///Timer period event forces the output to its inactive state
    #[inline(always)]
    pub fn set_inactive(self) -> &'a mut W {
        self.variant(PER_A::SetInactive)
    }
}
///Field `CMP1` reader - CMP1
pub type CMP1_R = crate::BitReader<CMP1_A>;
///CMP1
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMP1_A {
    ///0: Timer compare event has no effect
    NoEffect = 0,
    ///1: Timer compare event forces the output to its inactive state
    SetInactive = 1,
}
impl From<CMP1_A> for bool {
    #[inline(always)]
    fn from(variant: CMP1_A) -> Self {
        variant as u8 != 0
    }
}
impl CMP1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CMP1_A {
        match self.bits {
            false => CMP1_A::NoEffect,
            true => CMP1_A::SetInactive,
        }
    }
    ///Checks if the value of the field is `NoEffect`
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == CMP1_A::NoEffect
    }
    ///Checks if the value of the field is `SetInactive`
    #[inline(always)]
    pub fn is_set_inactive(&self) -> bool {
        *self == CMP1_A::SetInactive
    }
}
///Field `CMP1` writer - CMP1
pub type CMP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTE1R_SPEC, CMP1_A, O>;
impl<'a, const O: u8> CMP1_W<'a, O> {
    ///Timer compare event has no effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(CMP1_A::NoEffect)
    }
    ///Timer compare event forces the output to its inactive state
    #[inline(always)]
    pub fn set_inactive(self) -> &'a mut W {
        self.variant(CMP1_A::SetInactive)
    }
}
///Field `CMP2` reader - CMP2
pub use CMP1_R as CMP2_R;
///Field `CMP3` reader - CMP3
pub use CMP1_R as CMP3_R;
///Field `CMP4` reader - CMP4
pub use CMP1_R as CMP4_R;
///Field `CMP2` writer - CMP2
pub use CMP1_W as CMP2_W;
///Field `CMP3` writer - CMP3
pub use CMP1_W as CMP3_W;
///Field `CMP4` writer - CMP4
pub use CMP1_W as CMP4_W;
///Field `MSTPER` reader - MSTPER
pub type MSTPER_R = crate::BitReader<MSTPER_A>;
///MSTPER
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPER_A {
    ///0: Master timer counter roll-over/reset has no effect
    NoEffect = 0,
    ///1: Master timer counter roll-over/reset forces the output to its inactive state
    SetInactive = 1,
}
impl From<MSTPER_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPER_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTPER_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MSTPER_A {
        match self.bits {
            false => MSTPER_A::NoEffect,
            true => MSTPER_A::SetInactive,
        }
    }
    ///Checks if the value of the field is `NoEffect`
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == MSTPER_A::NoEffect
    }
    ///Checks if the value of the field is `SetInactive`
    #[inline(always)]
    pub fn is_set_inactive(&self) -> bool {
        *self == MSTPER_A::SetInactive
    }
}
///Field `MSTPER` writer - MSTPER
pub type MSTPER_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTE1R_SPEC, MSTPER_A, O>;
impl<'a, const O: u8> MSTPER_W<'a, O> {
    ///Master timer counter roll-over/reset has no effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(MSTPER_A::NoEffect)
    }
    ///Master timer counter roll-over/reset forces the output to its inactive state
    #[inline(always)]
    pub fn set_inactive(self) -> &'a mut W {
        self.variant(MSTPER_A::SetInactive)
    }
}
///Field `MSTCMP1` reader - MSTCMP1
pub type MSTCMP1_R = crate::BitReader<MSTCMP1_A>;
///MSTCMP1
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTCMP1_A {
    ///0: Master timer compare event has no effect
    NoEffect = 0,
    ///1: Master timer compare event forces the output to its inactive state
    SetInactive = 1,
}
impl From<MSTCMP1_A> for bool {
    #[inline(always)]
    fn from(variant: MSTCMP1_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTCMP1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MSTCMP1_A {
        match self.bits {
            false => MSTCMP1_A::NoEffect,
            true => MSTCMP1_A::SetInactive,
        }
    }
    ///Checks if the value of the field is `NoEffect`
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == MSTCMP1_A::NoEffect
    }
    ///Checks if the value of the field is `SetInactive`
    #[inline(always)]
    pub fn is_set_inactive(&self) -> bool {
        *self == MSTCMP1_A::SetInactive
    }
}
///Field `MSTCMP1` writer - MSTCMP1
pub type MSTCMP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTE1R_SPEC, MSTCMP1_A, O>;
impl<'a, const O: u8> MSTCMP1_W<'a, O> {
    ///Master timer compare event has no effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(MSTCMP1_A::NoEffect)
    }
    ///Master timer compare event forces the output to its inactive state
    #[inline(always)]
    pub fn set_inactive(self) -> &'a mut W {
        self.variant(MSTCMP1_A::SetInactive)
    }
}
///Field `MSTCMP2` reader - MSTCMP2
pub use MSTCMP1_R as MSTCMP2_R;
///Field `MSTCMP3` reader - MSTCMP3
pub use MSTCMP1_R as MSTCMP3_R;
///Field `MSTCMP4` reader - MSTCMP4
pub use MSTCMP1_R as MSTCMP4_R;
///Field `MSTCMP2` writer - MSTCMP2
pub use MSTCMP1_W as MSTCMP2_W;
///Field `MSTCMP3` writer - MSTCMP3
pub use MSTCMP1_W as MSTCMP3_W;
///Field `MSTCMP4` writer - MSTCMP4
pub use MSTCMP1_W as MSTCMP4_W;
///Field `TIMEVNT1` reader - TIMEVNT1
pub type TIMEVNT1_R = crate::BitReader<TIMEVNT1_A>;
///TIMEVNT1
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIMEVNT1_A {
    ///0: Timer event has no effect
    NoEffect = 0,
    ///1: Timer event forces the output to its inactive state
    SetInactive = 1,
}
impl From<TIMEVNT1_A> for bool {
    #[inline(always)]
    fn from(variant: TIMEVNT1_A) -> Self {
        variant as u8 != 0
    }
}
impl TIMEVNT1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TIMEVNT1_A {
        match self.bits {
            false => TIMEVNT1_A::NoEffect,
            true => TIMEVNT1_A::SetInactive,
        }
    }
    ///Checks if the value of the field is `NoEffect`
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == TIMEVNT1_A::NoEffect
    }
    ///Checks if the value of the field is `SetInactive`
    #[inline(always)]
    pub fn is_set_inactive(&self) -> bool {
        *self == TIMEVNT1_A::SetInactive
    }
}
///Field `TIMEVNT1` writer - TIMEVNT1
pub type TIMEVNT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTE1R_SPEC, TIMEVNT1_A, O>;
impl<'a, const O: u8> TIMEVNT1_W<'a, O> {
    ///Timer event has no effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TIMEVNT1_A::NoEffect)
    }
    ///Timer event forces the output to its inactive state
    #[inline(always)]
    pub fn set_inactive(self) -> &'a mut W {
        self.variant(TIMEVNT1_A::SetInactive)
    }
}
///Field `TIMEVNT2` reader - TIMEVNT2
pub use TIMEVNT1_R as TIMEVNT2_R;
///Field `TIMEVNT3` reader - TIMEVNT3
pub use TIMEVNT1_R as TIMEVNT3_R;
///Field `TIMEVNT4` reader - TIMEVNT4
pub use TIMEVNT1_R as TIMEVNT4_R;
///Field `TIMEVNT5` reader - TIMEVNT5
pub use TIMEVNT1_R as TIMEVNT5_R;
///Field `TIMEVNT6` reader - TIMEVNT6
pub use TIMEVNT1_R as TIMEVNT6_R;
///Field `TIMEVNT7` reader - TIMEVNT7
pub use TIMEVNT1_R as TIMEVNT7_R;
///Field `TIMEVNT8` reader - TIMEVNT8
pub use TIMEVNT1_R as TIMEVNT8_R;
///Field `TIMEVNT9` reader - TIMEVNT9
pub use TIMEVNT1_R as TIMEVNT9_R;
///Field `TIMEVNT2` writer - TIMEVNT2
pub use TIMEVNT1_W as TIMEVNT2_W;
///Field `TIMEVNT3` writer - TIMEVNT3
pub use TIMEVNT1_W as TIMEVNT3_W;
///Field `TIMEVNT4` writer - TIMEVNT4
pub use TIMEVNT1_W as TIMEVNT4_W;
///Field `TIMEVNT5` writer - TIMEVNT5
pub use TIMEVNT1_W as TIMEVNT5_W;
///Field `TIMEVNT6` writer - TIMEVNT6
pub use TIMEVNT1_W as TIMEVNT6_W;
///Field `TIMEVNT7` writer - TIMEVNT7
pub use TIMEVNT1_W as TIMEVNT7_W;
///Field `TIMEVNT8` writer - TIMEVNT8
pub use TIMEVNT1_W as TIMEVNT8_W;
///Field `TIMEVNT9` writer - TIMEVNT9
pub use TIMEVNT1_W as TIMEVNT9_W;
///Field `EXTEVNT1` reader - EXTEVNT1
pub type EXTEVNT1_R = crate::BitReader<EXTEVNT1_A>;
///EXTEVNT1
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXTEVNT1_A {
    ///0: External event has no effect
    NoEffect = 0,
    ///1: External event forces the output to its inactive state
    SetInactive = 1,
}
impl From<EXTEVNT1_A> for bool {
    #[inline(always)]
    fn from(variant: EXTEVNT1_A) -> Self {
        variant as u8 != 0
    }
}
impl EXTEVNT1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EXTEVNT1_A {
        match self.bits {
            false => EXTEVNT1_A::NoEffect,
            true => EXTEVNT1_A::SetInactive,
        }
    }
    ///Checks if the value of the field is `NoEffect`
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == EXTEVNT1_A::NoEffect
    }
    ///Checks if the value of the field is `SetInactive`
    #[inline(always)]
    pub fn is_set_inactive(&self) -> bool {
        *self == EXTEVNT1_A::SetInactive
    }
}
///Field `EXTEVNT1` writer - EXTEVNT1
pub type EXTEVNT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTE1R_SPEC, EXTEVNT1_A, O>;
impl<'a, const O: u8> EXTEVNT1_W<'a, O> {
    ///External event has no effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(EXTEVNT1_A::NoEffect)
    }
    ///External event forces the output to its inactive state
    #[inline(always)]
    pub fn set_inactive(self) -> &'a mut W {
        self.variant(EXTEVNT1_A::SetInactive)
    }
}
///Field `EXTEVNT2` reader - EXTEVNT2
pub use EXTEVNT1_R as EXTEVNT2_R;
///Field `EXTEVNT3` reader - EXTEVNT3
pub use EXTEVNT1_R as EXTEVNT3_R;
///Field `EXTEVNT4` reader - EXTEVNT4
pub use EXTEVNT1_R as EXTEVNT4_R;
///Field `EXTEVNT5` reader - EXTEVNT5
pub use EXTEVNT1_R as EXTEVNT5_R;
///Field `EXTEVNT6` reader - EXTEVNT6
pub use EXTEVNT1_R as EXTEVNT6_R;
///Field `EXTEVNT7` reader - EXTEVNT7
pub use EXTEVNT1_R as EXTEVNT7_R;
///Field `EXTEVNT8` reader - EXTEVNT8
pub use EXTEVNT1_R as EXTEVNT8_R;
///Field `EXTEVNT9` reader - EXTEVNT9
pub use EXTEVNT1_R as EXTEVNT9_R;
///Field `EXTEVNT10` reader - EXTEVNT10
pub use EXTEVNT1_R as EXTEVNT10_R;
///Field `EXTEVNT2` writer - EXTEVNT2
pub use EXTEVNT1_W as EXTEVNT2_W;
///Field `EXTEVNT3` writer - EXTEVNT3
pub use EXTEVNT1_W as EXTEVNT3_W;
///Field `EXTEVNT4` writer - EXTEVNT4
pub use EXTEVNT1_W as EXTEVNT4_W;
///Field `EXTEVNT5` writer - EXTEVNT5
pub use EXTEVNT1_W as EXTEVNT5_W;
///Field `EXTEVNT6` writer - EXTEVNT6
pub use EXTEVNT1_W as EXTEVNT6_W;
///Field `EXTEVNT7` writer - EXTEVNT7
pub use EXTEVNT1_W as EXTEVNT7_W;
///Field `EXTEVNT8` writer - EXTEVNT8
pub use EXTEVNT1_W as EXTEVNT8_W;
///Field `EXTEVNT9` writer - EXTEVNT9
pub use EXTEVNT1_W as EXTEVNT9_W;
///Field `EXTEVNT10` writer - EXTEVNT10
pub use EXTEVNT1_W as EXTEVNT10_W;
///Field `UPDATE` reader - UPDATE
pub type UPDATE_R = crate::BitReader<UPDATE_A>;
///UPDATE
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UPDATE_A {
    ///0: Register update event has no effect
    NoEffect = 0,
    ///1: Register update event forces the output to its inactive state
    SetInactive = 1,
}
impl From<UPDATE_A> for bool {
    #[inline(always)]
    fn from(variant: UPDATE_A) -> Self {
        variant as u8 != 0
    }
}
impl UPDATE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> UPDATE_A {
        match self.bits {
            false => UPDATE_A::NoEffect,
            true => UPDATE_A::SetInactive,
        }
    }
    ///Checks if the value of the field is `NoEffect`
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == UPDATE_A::NoEffect
    }
    ///Checks if the value of the field is `SetInactive`
    #[inline(always)]
    pub fn is_set_inactive(&self) -> bool {
        *self == UPDATE_A::SetInactive
    }
}
///Field `UPDATE` writer - UPDATE
pub type UPDATE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTE1R_SPEC, UPDATE_A, O>;
impl<'a, const O: u8> UPDATE_W<'a, O> {
    ///Register update event has no effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(UPDATE_A::NoEffect)
    }
    ///Register update event forces the output to its inactive state
    #[inline(always)]
    pub fn set_inactive(self) -> &'a mut W {
        self.variant(UPDATE_A::SetInactive)
    }
}
impl R {
    ///Bit 0 - SRT
    #[inline(always)]
    pub fn srt(&self) -> SRT_R {
        SRT_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - RESYNC
    #[inline(always)]
    pub fn resync(&self) -> RESYNC_R {
        RESYNC_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - PER
    #[inline(always)]
    pub fn per(&self) -> PER_R {
        PER_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - CMP1
    #[inline(always)]
    pub fn cmp1(&self) -> CMP1_R {
        CMP1_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - CMP2
    #[inline(always)]
    pub fn cmp2(&self) -> CMP2_R {
        CMP2_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - CMP3
    #[inline(always)]
    pub fn cmp3(&self) -> CMP3_R {
        CMP3_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - CMP4
    #[inline(always)]
    pub fn cmp4(&self) -> CMP4_R {
        CMP4_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - MSTPER
    #[inline(always)]
    pub fn mstper(&self) -> MSTPER_R {
        MSTPER_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - MSTCMP1
    #[inline(always)]
    pub fn mstcmp1(&self) -> MSTCMP1_R {
        MSTCMP1_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - MSTCMP2
    #[inline(always)]
    pub fn mstcmp2(&self) -> MSTCMP2_R {
        MSTCMP2_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - MSTCMP3
    #[inline(always)]
    pub fn mstcmp3(&self) -> MSTCMP3_R {
        MSTCMP3_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - MSTCMP4
    #[inline(always)]
    pub fn mstcmp4(&self) -> MSTCMP4_R {
        MSTCMP4_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - TIMEVNT1
    #[inline(always)]
    pub fn timevnt1(&self) -> TIMEVNT1_R {
        TIMEVNT1_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - TIMEVNT2
    #[inline(always)]
    pub fn timevnt2(&self) -> TIMEVNT2_R {
        TIMEVNT2_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - TIMEVNT3
    #[inline(always)]
    pub fn timevnt3(&self) -> TIMEVNT3_R {
        TIMEVNT3_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - TIMEVNT4
    #[inline(always)]
    pub fn timevnt4(&self) -> TIMEVNT4_R {
        TIMEVNT4_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - TIMEVNT5
    #[inline(always)]
    pub fn timevnt5(&self) -> TIMEVNT5_R {
        TIMEVNT5_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - TIMEVNT6
    #[inline(always)]
    pub fn timevnt6(&self) -> TIMEVNT6_R {
        TIMEVNT6_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - TIMEVNT7
    #[inline(always)]
    pub fn timevnt7(&self) -> TIMEVNT7_R {
        TIMEVNT7_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - TIMEVNT8
    #[inline(always)]
    pub fn timevnt8(&self) -> TIMEVNT8_R {
        TIMEVNT8_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - TIMEVNT9
    #[inline(always)]
    pub fn timevnt9(&self) -> TIMEVNT9_R {
        TIMEVNT9_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - EXTEVNT1
    #[inline(always)]
    pub fn extevnt1(&self) -> EXTEVNT1_R {
        EXTEVNT1_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - EXTEVNT2
    #[inline(always)]
    pub fn extevnt2(&self) -> EXTEVNT2_R {
        EXTEVNT2_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - EXTEVNT3
    #[inline(always)]
    pub fn extevnt3(&self) -> EXTEVNT3_R {
        EXTEVNT3_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - EXTEVNT4
    #[inline(always)]
    pub fn extevnt4(&self) -> EXTEVNT4_R {
        EXTEVNT4_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - EXTEVNT5
    #[inline(always)]
    pub fn extevnt5(&self) -> EXTEVNT5_R {
        EXTEVNT5_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - EXTEVNT6
    #[inline(always)]
    pub fn extevnt6(&self) -> EXTEVNT6_R {
        EXTEVNT6_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - EXTEVNT7
    #[inline(always)]
    pub fn extevnt7(&self) -> EXTEVNT7_R {
        EXTEVNT7_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - EXTEVNT8
    #[inline(always)]
    pub fn extevnt8(&self) -> EXTEVNT8_R {
        EXTEVNT8_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - EXTEVNT9
    #[inline(always)]
    pub fn extevnt9(&self) -> EXTEVNT9_R {
        EXTEVNT9_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - EXTEVNT10
    #[inline(always)]
    pub fn extevnt10(&self) -> EXTEVNT10_R {
        EXTEVNT10_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - UPDATE
    #[inline(always)]
    pub fn update(&self) -> UPDATE_R {
        UPDATE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - SRT
    #[inline(always)]
    #[must_use]
    pub fn srt(&mut self) -> SRT_W<0> {
        SRT_W::new(self)
    }
    ///Bit 1 - RESYNC
    #[inline(always)]
    #[must_use]
    pub fn resync(&mut self) -> RESYNC_W<1> {
        RESYNC_W::new(self)
    }
    ///Bit 2 - PER
    #[inline(always)]
    #[must_use]
    pub fn per(&mut self) -> PER_W<2> {
        PER_W::new(self)
    }
    ///Bit 3 - CMP1
    #[inline(always)]
    #[must_use]
    pub fn cmp1(&mut self) -> CMP1_W<3> {
        CMP1_W::new(self)
    }
    ///Bit 4 - CMP2
    #[inline(always)]
    #[must_use]
    pub fn cmp2(&mut self) -> CMP2_W<4> {
        CMP2_W::new(self)
    }
    ///Bit 5 - CMP3
    #[inline(always)]
    #[must_use]
    pub fn cmp3(&mut self) -> CMP3_W<5> {
        CMP3_W::new(self)
    }
    ///Bit 6 - CMP4
    #[inline(always)]
    #[must_use]
    pub fn cmp4(&mut self) -> CMP4_W<6> {
        CMP4_W::new(self)
    }
    ///Bit 7 - MSTPER
    #[inline(always)]
    #[must_use]
    pub fn mstper(&mut self) -> MSTPER_W<7> {
        MSTPER_W::new(self)
    }
    ///Bit 8 - MSTCMP1
    #[inline(always)]
    #[must_use]
    pub fn mstcmp1(&mut self) -> MSTCMP1_W<8> {
        MSTCMP1_W::new(self)
    }
    ///Bit 9 - MSTCMP2
    #[inline(always)]
    #[must_use]
    pub fn mstcmp2(&mut self) -> MSTCMP2_W<9> {
        MSTCMP2_W::new(self)
    }
    ///Bit 10 - MSTCMP3
    #[inline(always)]
    #[must_use]
    pub fn mstcmp3(&mut self) -> MSTCMP3_W<10> {
        MSTCMP3_W::new(self)
    }
    ///Bit 11 - MSTCMP4
    #[inline(always)]
    #[must_use]
    pub fn mstcmp4(&mut self) -> MSTCMP4_W<11> {
        MSTCMP4_W::new(self)
    }
    ///Bit 12 - TIMEVNT1
    #[inline(always)]
    #[must_use]
    pub fn timevnt1(&mut self) -> TIMEVNT1_W<12> {
        TIMEVNT1_W::new(self)
    }
    ///Bit 13 - TIMEVNT2
    #[inline(always)]
    #[must_use]
    pub fn timevnt2(&mut self) -> TIMEVNT2_W<13> {
        TIMEVNT2_W::new(self)
    }
    ///Bit 14 - TIMEVNT3
    #[inline(always)]
    #[must_use]
    pub fn timevnt3(&mut self) -> TIMEVNT3_W<14> {
        TIMEVNT3_W::new(self)
    }
    ///Bit 15 - TIMEVNT4
    #[inline(always)]
    #[must_use]
    pub fn timevnt4(&mut self) -> TIMEVNT4_W<15> {
        TIMEVNT4_W::new(self)
    }
    ///Bit 16 - TIMEVNT5
    #[inline(always)]
    #[must_use]
    pub fn timevnt5(&mut self) -> TIMEVNT5_W<16> {
        TIMEVNT5_W::new(self)
    }
    ///Bit 17 - TIMEVNT6
    #[inline(always)]
    #[must_use]
    pub fn timevnt6(&mut self) -> TIMEVNT6_W<17> {
        TIMEVNT6_W::new(self)
    }
    ///Bit 18 - TIMEVNT7
    #[inline(always)]
    #[must_use]
    pub fn timevnt7(&mut self) -> TIMEVNT7_W<18> {
        TIMEVNT7_W::new(self)
    }
    ///Bit 19 - TIMEVNT8
    #[inline(always)]
    #[must_use]
    pub fn timevnt8(&mut self) -> TIMEVNT8_W<19> {
        TIMEVNT8_W::new(self)
    }
    ///Bit 20 - TIMEVNT9
    #[inline(always)]
    #[must_use]
    pub fn timevnt9(&mut self) -> TIMEVNT9_W<20> {
        TIMEVNT9_W::new(self)
    }
    ///Bit 21 - EXTEVNT1
    #[inline(always)]
    #[must_use]
    pub fn extevnt1(&mut self) -> EXTEVNT1_W<21> {
        EXTEVNT1_W::new(self)
    }
    ///Bit 22 - EXTEVNT2
    #[inline(always)]
    #[must_use]
    pub fn extevnt2(&mut self) -> EXTEVNT2_W<22> {
        EXTEVNT2_W::new(self)
    }
    ///Bit 23 - EXTEVNT3
    #[inline(always)]
    #[must_use]
    pub fn extevnt3(&mut self) -> EXTEVNT3_W<23> {
        EXTEVNT3_W::new(self)
    }
    ///Bit 24 - EXTEVNT4
    #[inline(always)]
    #[must_use]
    pub fn extevnt4(&mut self) -> EXTEVNT4_W<24> {
        EXTEVNT4_W::new(self)
    }
    ///Bit 25 - EXTEVNT5
    #[inline(always)]
    #[must_use]
    pub fn extevnt5(&mut self) -> EXTEVNT5_W<25> {
        EXTEVNT5_W::new(self)
    }
    ///Bit 26 - EXTEVNT6
    #[inline(always)]
    #[must_use]
    pub fn extevnt6(&mut self) -> EXTEVNT6_W<26> {
        EXTEVNT6_W::new(self)
    }
    ///Bit 27 - EXTEVNT7
    #[inline(always)]
    #[must_use]
    pub fn extevnt7(&mut self) -> EXTEVNT7_W<27> {
        EXTEVNT7_W::new(self)
    }
    ///Bit 28 - EXTEVNT8
    #[inline(always)]
    #[must_use]
    pub fn extevnt8(&mut self) -> EXTEVNT8_W<28> {
        EXTEVNT8_W::new(self)
    }
    ///Bit 29 - EXTEVNT9
    #[inline(always)]
    #[must_use]
    pub fn extevnt9(&mut self) -> EXTEVNT9_W<29> {
        EXTEVNT9_W::new(self)
    }
    ///Bit 30 - EXTEVNT10
    #[inline(always)]
    #[must_use]
    pub fn extevnt10(&mut self) -> EXTEVNT10_W<30> {
        EXTEVNT10_W::new(self)
    }
    ///Bit 31 - UPDATE
    #[inline(always)]
    #[must_use]
    pub fn update(&mut self) -> UPDATE_W<31> {
        UPDATE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Timerx Output1 Reset Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rste1r](index.html) module
pub struct RSTE1R_SPEC;
impl crate::RegisterSpec for RSTE1R_SPEC {
    type Ux = u32;
}
///`read()` method returns [rste1r::R](R) reader structure
impl crate::Readable for RSTE1R_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rste1r::W](W) writer structure
impl crate::Writable for RSTE1R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RSTE1R to value 0
impl crate::Resettable for RSTE1R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
