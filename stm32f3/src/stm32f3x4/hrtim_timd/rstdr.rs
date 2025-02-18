///Register `RSTDR` reader
pub struct R(crate::R<RSTDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSTDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSTDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSTDR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RSTDR` writer
pub struct W(crate::W<RSTDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSTDR_SPEC>;
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
impl From<crate::W<RSTDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSTDR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `UPDT` reader - Timer A Update reset
pub type UPDT_R = crate::BitReader<UPDT_A>;
///Timer A Update reset
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UPDT_A {
    ///0: Update event has no effect
    NoEffect = 0,
    ///1: Timer X counter is reset upon update event
    ResetCounter = 1,
}
impl From<UPDT_A> for bool {
    #[inline(always)]
    fn from(variant: UPDT_A) -> Self {
        variant as u8 != 0
    }
}
impl UPDT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> UPDT_A {
        match self.bits {
            false => UPDT_A::NoEffect,
            true => UPDT_A::ResetCounter,
        }
    }
    ///Checks if the value of the field is `NoEffect`
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == UPDT_A::NoEffect
    }
    ///Checks if the value of the field is `ResetCounter`
    #[inline(always)]
    pub fn is_reset_counter(&self) -> bool {
        *self == UPDT_A::ResetCounter
    }
}
///Field `UPDT` writer - Timer A Update reset
pub type UPDT_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTDR_SPEC, UPDT_A, O>;
impl<'a, const O: u8> UPDT_W<'a, O> {
    ///Update event has no effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(UPDT_A::NoEffect)
    }
    ///Timer X counter is reset upon update event
    #[inline(always)]
    pub fn reset_counter(self) -> &'a mut W {
        self.variant(UPDT_A::ResetCounter)
    }
}
///Field `CMP2` reader - Timer A compare 2 reset
pub type CMP2_R = crate::BitReader<CMP2_A>;
///Timer A compare 2 reset
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMP2_A {
    ///0: Timer X compare Z event has no effect
    NoEffect = 0,
    ///1: Timer X counter is reset upon timer X compare Z event
    ResetCounter = 1,
}
impl From<CMP2_A> for bool {
    #[inline(always)]
    fn from(variant: CMP2_A) -> Self {
        variant as u8 != 0
    }
}
impl CMP2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CMP2_A {
        match self.bits {
            false => CMP2_A::NoEffect,
            true => CMP2_A::ResetCounter,
        }
    }
    ///Checks if the value of the field is `NoEffect`
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == CMP2_A::NoEffect
    }
    ///Checks if the value of the field is `ResetCounter`
    #[inline(always)]
    pub fn is_reset_counter(&self) -> bool {
        *self == CMP2_A::ResetCounter
    }
}
///Field `CMP2` writer - Timer A compare 2 reset
pub type CMP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTDR_SPEC, CMP2_A, O>;
impl<'a, const O: u8> CMP2_W<'a, O> {
    ///Timer X compare Z event has no effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(CMP2_A::NoEffect)
    }
    ///Timer X counter is reset upon timer X compare Z event
    #[inline(always)]
    pub fn reset_counter(self) -> &'a mut W {
        self.variant(CMP2_A::ResetCounter)
    }
}
///Field `CMP4` reader - Timer A compare 4 reset
pub use CMP2_R as CMP4_R;
///Field `CMP4` writer - Timer A compare 4 reset
pub use CMP2_W as CMP4_W;
///Field `MSTPER` reader - Master timer Period
pub type MSTPER_R = crate::BitReader<MSTPER_A>;
///Master timer Period
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPER_A {
    ///0: Master timer period event has no effect
    NoEffect = 0,
    ///1: Timer X counter is reset upon master timer period event
    ResetCounter = 1,
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
            true => MSTPER_A::ResetCounter,
        }
    }
    ///Checks if the value of the field is `NoEffect`
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == MSTPER_A::NoEffect
    }
    ///Checks if the value of the field is `ResetCounter`
    #[inline(always)]
    pub fn is_reset_counter(&self) -> bool {
        *self == MSTPER_A::ResetCounter
    }
}
///Field `MSTPER` writer - Master timer Period
pub type MSTPER_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTDR_SPEC, MSTPER_A, O>;
impl<'a, const O: u8> MSTPER_W<'a, O> {
    ///Master timer period event has no effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(MSTPER_A::NoEffect)
    }
    ///Timer X counter is reset upon master timer period event
    #[inline(always)]
    pub fn reset_counter(self) -> &'a mut W {
        self.variant(MSTPER_A::ResetCounter)
    }
}
///Field `MSTCMP1` reader - Master compare 1
pub type MSTCMP1_R = crate::BitReader<MSTCMP1_A>;
///Master compare 1
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTCMP1_A {
    ///0: Master timer compare Z event has no effect
    NoEffect = 0,
    ///1: Timer X counter is reset upon master timer compare Z event
    ResetCounter = 1,
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
            true => MSTCMP1_A::ResetCounter,
        }
    }
    ///Checks if the value of the field is `NoEffect`
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == MSTCMP1_A::NoEffect
    }
    ///Checks if the value of the field is `ResetCounter`
    #[inline(always)]
    pub fn is_reset_counter(&self) -> bool {
        *self == MSTCMP1_A::ResetCounter
    }
}
///Field `MSTCMP1` writer - Master compare 1
pub type MSTCMP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTDR_SPEC, MSTCMP1_A, O>;
impl<'a, const O: u8> MSTCMP1_W<'a, O> {
    ///Master timer compare Z event has no effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(MSTCMP1_A::NoEffect)
    }
    ///Timer X counter is reset upon master timer compare Z event
    #[inline(always)]
    pub fn reset_counter(self) -> &'a mut W {
        self.variant(MSTCMP1_A::ResetCounter)
    }
}
///Field `MSTCMP2` reader - Master compare 2
pub use MSTCMP1_R as MSTCMP2_R;
///Field `MSTCMP3` reader - Master compare 3
pub use MSTCMP1_R as MSTCMP3_R;
///Field `MSTCMP4` reader - Master compare 4
pub use MSTCMP1_R as MSTCMP4_R;
///Field `MSTCMP2` writer - Master compare 2
pub use MSTCMP1_W as MSTCMP2_W;
///Field `MSTCMP3` writer - Master compare 3
pub use MSTCMP1_W as MSTCMP3_W;
///Field `MSTCMP4` writer - Master compare 4
pub use MSTCMP1_W as MSTCMP4_W;
///Field `EXTEVNT1` reader - External Event 1
pub type EXTEVNT1_R = crate::BitReader<EXTEVNT1_A>;
///External Event 1
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXTEVNT1_A {
    ///0: External event Z has no effect
    NoEffect = 0,
    ///1: Timer X counter is reset upon external event Z
    ResetCounter = 1,
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
            true => EXTEVNT1_A::ResetCounter,
        }
    }
    ///Checks if the value of the field is `NoEffect`
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == EXTEVNT1_A::NoEffect
    }
    ///Checks if the value of the field is `ResetCounter`
    #[inline(always)]
    pub fn is_reset_counter(&self) -> bool {
        *self == EXTEVNT1_A::ResetCounter
    }
}
///Field `EXTEVNT1` writer - External Event 1
pub type EXTEVNT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTDR_SPEC, EXTEVNT1_A, O>;
impl<'a, const O: u8> EXTEVNT1_W<'a, O> {
    ///External event Z has no effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(EXTEVNT1_A::NoEffect)
    }
    ///Timer X counter is reset upon external event Z
    #[inline(always)]
    pub fn reset_counter(self) -> &'a mut W {
        self.variant(EXTEVNT1_A::ResetCounter)
    }
}
///Field `EXTEVNT2` reader - External Event 2
pub use EXTEVNT1_R as EXTEVNT2_R;
///Field `EXTEVNT3` reader - External Event 3
pub use EXTEVNT1_R as EXTEVNT3_R;
///Field `EXTEVNT4` reader - External Event 4
pub use EXTEVNT1_R as EXTEVNT4_R;
///Field `EXTEVNT5` reader - External Event 5
pub use EXTEVNT1_R as EXTEVNT5_R;
///Field `EXTEVNT6` reader - External Event 6
pub use EXTEVNT1_R as EXTEVNT6_R;
///Field `EXTEVNT7` reader - External Event 7
pub use EXTEVNT1_R as EXTEVNT7_R;
///Field `EXTEVNT8` reader - External Event 8
pub use EXTEVNT1_R as EXTEVNT8_R;
///Field `EXTEVNT9` reader - External Event 9
pub use EXTEVNT1_R as EXTEVNT9_R;
///Field `EXTEVNT10` reader - External Event 10
pub use EXTEVNT1_R as EXTEVNT10_R;
///Field `EXTEVNT2` writer - External Event 2
pub use EXTEVNT1_W as EXTEVNT2_W;
///Field `EXTEVNT3` writer - External Event 3
pub use EXTEVNT1_W as EXTEVNT3_W;
///Field `EXTEVNT4` writer - External Event 4
pub use EXTEVNT1_W as EXTEVNT4_W;
///Field `EXTEVNT5` writer - External Event 5
pub use EXTEVNT1_W as EXTEVNT5_W;
///Field `EXTEVNT6` writer - External Event 6
pub use EXTEVNT1_W as EXTEVNT6_W;
///Field `EXTEVNT7` writer - External Event 7
pub use EXTEVNT1_W as EXTEVNT7_W;
///Field `EXTEVNT8` writer - External Event 8
pub use EXTEVNT1_W as EXTEVNT8_W;
///Field `EXTEVNT9` writer - External Event 9
pub use EXTEVNT1_W as EXTEVNT9_W;
///Field `EXTEVNT10` writer - External Event 10
pub use EXTEVNT1_W as EXTEVNT10_W;
///Field `TIMACMP1` reader - Timer A Compare 1
pub type TIMACMP1_R = crate::BitReader<TIMACMP1_A>;
///Timer A Compare 1
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIMACMP1_A {
    ///0: Timer Y compare Z event has no effect
    NoEffect = 0,
    ///1: Timer X counter is reset upon timer Y compare Z event
    ResetCounter = 1,
}
impl From<TIMACMP1_A> for bool {
    #[inline(always)]
    fn from(variant: TIMACMP1_A) -> Self {
        variant as u8 != 0
    }
}
impl TIMACMP1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TIMACMP1_A {
        match self.bits {
            false => TIMACMP1_A::NoEffect,
            true => TIMACMP1_A::ResetCounter,
        }
    }
    ///Checks if the value of the field is `NoEffect`
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == TIMACMP1_A::NoEffect
    }
    ///Checks if the value of the field is `ResetCounter`
    #[inline(always)]
    pub fn is_reset_counter(&self) -> bool {
        *self == TIMACMP1_A::ResetCounter
    }
}
///Field `TIMACMP1` writer - Timer A Compare 1
pub type TIMACMP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTDR_SPEC, TIMACMP1_A, O>;
impl<'a, const O: u8> TIMACMP1_W<'a, O> {
    ///Timer Y compare Z event has no effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TIMACMP1_A::NoEffect)
    }
    ///Timer X counter is reset upon timer Y compare Z event
    #[inline(always)]
    pub fn reset_counter(self) -> &'a mut W {
        self.variant(TIMACMP1_A::ResetCounter)
    }
}
///Field `TIMACMP2` reader - Timer A Compare 2
pub use TIMACMP1_R as TIMACMP2_R;
///Field `TIMACMP4` reader - Timer A Compare 4
pub use TIMACMP1_R as TIMACMP4_R;
///Field `TIMBCMP1` reader - Timer B Compare 1
pub use TIMACMP1_R as TIMBCMP1_R;
///Field `TIMBCMP2` reader - Timer B Compare 2
pub use TIMACMP1_R as TIMBCMP2_R;
///Field `TIMBCMP4` reader - Timer B Compare 4
pub use TIMACMP1_R as TIMBCMP4_R;
///Field `TIMCCMP1` reader - Timer C Compare 1
pub use TIMACMP1_R as TIMCCMP1_R;
///Field `TIMCCMP2` reader - Timer C Compare 2
pub use TIMACMP1_R as TIMCCMP2_R;
///Field `TIMCCMP4` reader - Timer C Compare 4
pub use TIMACMP1_R as TIMCCMP4_R;
///Field `TIMECMP1` reader - Timer E Compare 1
pub use TIMACMP1_R as TIMECMP1_R;
///Field `TIMECMP2` reader - Timer E Compare 2
pub use TIMACMP1_R as TIMECMP2_R;
///Field `TIMECMP4` reader - Timer E Compare 4
pub use TIMACMP1_R as TIMECMP4_R;
///Field `TIMACMP2` writer - Timer A Compare 2
pub use TIMACMP1_W as TIMACMP2_W;
///Field `TIMACMP4` writer - Timer A Compare 4
pub use TIMACMP1_W as TIMACMP4_W;
///Field `TIMBCMP1` writer - Timer B Compare 1
pub use TIMACMP1_W as TIMBCMP1_W;
///Field `TIMBCMP2` writer - Timer B Compare 2
pub use TIMACMP1_W as TIMBCMP2_W;
///Field `TIMBCMP4` writer - Timer B Compare 4
pub use TIMACMP1_W as TIMBCMP4_W;
///Field `TIMCCMP1` writer - Timer C Compare 1
pub use TIMACMP1_W as TIMCCMP1_W;
///Field `TIMCCMP2` writer - Timer C Compare 2
pub use TIMACMP1_W as TIMCCMP2_W;
///Field `TIMCCMP4` writer - Timer C Compare 4
pub use TIMACMP1_W as TIMCCMP4_W;
///Field `TIMECMP1` writer - Timer E Compare 1
pub use TIMACMP1_W as TIMECMP1_W;
///Field `TIMECMP2` writer - Timer E Compare 2
pub use TIMACMP1_W as TIMECMP2_W;
///Field `TIMECMP4` writer - Timer E Compare 4
pub use TIMACMP1_W as TIMECMP4_W;
impl R {
    ///Bit 1 - Timer A Update reset
    #[inline(always)]
    pub fn updt(&self) -> UPDT_R {
        UPDT_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Timer A compare 2 reset
    #[inline(always)]
    pub fn cmp2(&self) -> CMP2_R {
        CMP2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Timer A compare 4 reset
    #[inline(always)]
    pub fn cmp4(&self) -> CMP4_R {
        CMP4_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Master timer Period
    #[inline(always)]
    pub fn mstper(&self) -> MSTPER_R {
        MSTPER_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Master compare 1
    #[inline(always)]
    pub fn mstcmp1(&self) -> MSTCMP1_R {
        MSTCMP1_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Master compare 2
    #[inline(always)]
    pub fn mstcmp2(&self) -> MSTCMP2_R {
        MSTCMP2_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Master compare 3
    #[inline(always)]
    pub fn mstcmp3(&self) -> MSTCMP3_R {
        MSTCMP3_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Master compare 4
    #[inline(always)]
    pub fn mstcmp4(&self) -> MSTCMP4_R {
        MSTCMP4_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - External Event 1
    #[inline(always)]
    pub fn extevnt1(&self) -> EXTEVNT1_R {
        EXTEVNT1_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - External Event 2
    #[inline(always)]
    pub fn extevnt2(&self) -> EXTEVNT2_R {
        EXTEVNT2_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - External Event 3
    #[inline(always)]
    pub fn extevnt3(&self) -> EXTEVNT3_R {
        EXTEVNT3_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - External Event 4
    #[inline(always)]
    pub fn extevnt4(&self) -> EXTEVNT4_R {
        EXTEVNT4_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - External Event 5
    #[inline(always)]
    pub fn extevnt5(&self) -> EXTEVNT5_R {
        EXTEVNT5_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - External Event 6
    #[inline(always)]
    pub fn extevnt6(&self) -> EXTEVNT6_R {
        EXTEVNT6_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - External Event 7
    #[inline(always)]
    pub fn extevnt7(&self) -> EXTEVNT7_R {
        EXTEVNT7_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - External Event 8
    #[inline(always)]
    pub fn extevnt8(&self) -> EXTEVNT8_R {
        EXTEVNT8_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - External Event 9
    #[inline(always)]
    pub fn extevnt9(&self) -> EXTEVNT9_R {
        EXTEVNT9_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - External Event 10
    #[inline(always)]
    pub fn extevnt10(&self) -> EXTEVNT10_R {
        EXTEVNT10_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Timer A Compare 1
    #[inline(always)]
    pub fn timacmp1(&self) -> TIMACMP1_R {
        TIMACMP1_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Timer A Compare 2
    #[inline(always)]
    pub fn timacmp2(&self) -> TIMACMP2_R {
        TIMACMP2_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Timer A Compare 4
    #[inline(always)]
    pub fn timacmp4(&self) -> TIMACMP4_R {
        TIMACMP4_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Timer B Compare 1
    #[inline(always)]
    pub fn timbcmp1(&self) -> TIMBCMP1_R {
        TIMBCMP1_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Timer B Compare 2
    #[inline(always)]
    pub fn timbcmp2(&self) -> TIMBCMP2_R {
        TIMBCMP2_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Timer B Compare 4
    #[inline(always)]
    pub fn timbcmp4(&self) -> TIMBCMP4_R {
        TIMBCMP4_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Timer C Compare 1
    #[inline(always)]
    pub fn timccmp1(&self) -> TIMCCMP1_R {
        TIMCCMP1_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Timer C Compare 2
    #[inline(always)]
    pub fn timccmp2(&self) -> TIMCCMP2_R {
        TIMCCMP2_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Timer C Compare 4
    #[inline(always)]
    pub fn timccmp4(&self) -> TIMCCMP4_R {
        TIMCCMP4_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Timer E Compare 1
    #[inline(always)]
    pub fn timecmp1(&self) -> TIMECMP1_R {
        TIMECMP1_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Timer E Compare 2
    #[inline(always)]
    pub fn timecmp2(&self) -> TIMECMP2_R {
        TIMECMP2_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Timer E Compare 4
    #[inline(always)]
    pub fn timecmp4(&self) -> TIMECMP4_R {
        TIMECMP4_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    ///Bit 1 - Timer A Update reset
    #[inline(always)]
    #[must_use]
    pub fn updt(&mut self) -> UPDT_W<1> {
        UPDT_W::new(self)
    }
    ///Bit 2 - Timer A compare 2 reset
    #[inline(always)]
    #[must_use]
    pub fn cmp2(&mut self) -> CMP2_W<2> {
        CMP2_W::new(self)
    }
    ///Bit 3 - Timer A compare 4 reset
    #[inline(always)]
    #[must_use]
    pub fn cmp4(&mut self) -> CMP4_W<3> {
        CMP4_W::new(self)
    }
    ///Bit 4 - Master timer Period
    #[inline(always)]
    #[must_use]
    pub fn mstper(&mut self) -> MSTPER_W<4> {
        MSTPER_W::new(self)
    }
    ///Bit 5 - Master compare 1
    #[inline(always)]
    #[must_use]
    pub fn mstcmp1(&mut self) -> MSTCMP1_W<5> {
        MSTCMP1_W::new(self)
    }
    ///Bit 6 - Master compare 2
    #[inline(always)]
    #[must_use]
    pub fn mstcmp2(&mut self) -> MSTCMP2_W<6> {
        MSTCMP2_W::new(self)
    }
    ///Bit 7 - Master compare 3
    #[inline(always)]
    #[must_use]
    pub fn mstcmp3(&mut self) -> MSTCMP3_W<7> {
        MSTCMP3_W::new(self)
    }
    ///Bit 8 - Master compare 4
    #[inline(always)]
    #[must_use]
    pub fn mstcmp4(&mut self) -> MSTCMP4_W<8> {
        MSTCMP4_W::new(self)
    }
    ///Bit 9 - External Event 1
    #[inline(always)]
    #[must_use]
    pub fn extevnt1(&mut self) -> EXTEVNT1_W<9> {
        EXTEVNT1_W::new(self)
    }
    ///Bit 10 - External Event 2
    #[inline(always)]
    #[must_use]
    pub fn extevnt2(&mut self) -> EXTEVNT2_W<10> {
        EXTEVNT2_W::new(self)
    }
    ///Bit 11 - External Event 3
    #[inline(always)]
    #[must_use]
    pub fn extevnt3(&mut self) -> EXTEVNT3_W<11> {
        EXTEVNT3_W::new(self)
    }
    ///Bit 12 - External Event 4
    #[inline(always)]
    #[must_use]
    pub fn extevnt4(&mut self) -> EXTEVNT4_W<12> {
        EXTEVNT4_W::new(self)
    }
    ///Bit 13 - External Event 5
    #[inline(always)]
    #[must_use]
    pub fn extevnt5(&mut self) -> EXTEVNT5_W<13> {
        EXTEVNT5_W::new(self)
    }
    ///Bit 14 - External Event 6
    #[inline(always)]
    #[must_use]
    pub fn extevnt6(&mut self) -> EXTEVNT6_W<14> {
        EXTEVNT6_W::new(self)
    }
    ///Bit 15 - External Event 7
    #[inline(always)]
    #[must_use]
    pub fn extevnt7(&mut self) -> EXTEVNT7_W<15> {
        EXTEVNT7_W::new(self)
    }
    ///Bit 16 - External Event 8
    #[inline(always)]
    #[must_use]
    pub fn extevnt8(&mut self) -> EXTEVNT8_W<16> {
        EXTEVNT8_W::new(self)
    }
    ///Bit 17 - External Event 9
    #[inline(always)]
    #[must_use]
    pub fn extevnt9(&mut self) -> EXTEVNT9_W<17> {
        EXTEVNT9_W::new(self)
    }
    ///Bit 18 - External Event 10
    #[inline(always)]
    #[must_use]
    pub fn extevnt10(&mut self) -> EXTEVNT10_W<18> {
        EXTEVNT10_W::new(self)
    }
    ///Bit 19 - Timer A Compare 1
    #[inline(always)]
    #[must_use]
    pub fn timacmp1(&mut self) -> TIMACMP1_W<19> {
        TIMACMP1_W::new(self)
    }
    ///Bit 20 - Timer A Compare 2
    #[inline(always)]
    #[must_use]
    pub fn timacmp2(&mut self) -> TIMACMP2_W<20> {
        TIMACMP2_W::new(self)
    }
    ///Bit 21 - Timer A Compare 4
    #[inline(always)]
    #[must_use]
    pub fn timacmp4(&mut self) -> TIMACMP4_W<21> {
        TIMACMP4_W::new(self)
    }
    ///Bit 22 - Timer B Compare 1
    #[inline(always)]
    #[must_use]
    pub fn timbcmp1(&mut self) -> TIMBCMP1_W<22> {
        TIMBCMP1_W::new(self)
    }
    ///Bit 23 - Timer B Compare 2
    #[inline(always)]
    #[must_use]
    pub fn timbcmp2(&mut self) -> TIMBCMP2_W<23> {
        TIMBCMP2_W::new(self)
    }
    ///Bit 24 - Timer B Compare 4
    #[inline(always)]
    #[must_use]
    pub fn timbcmp4(&mut self) -> TIMBCMP4_W<24> {
        TIMBCMP4_W::new(self)
    }
    ///Bit 25 - Timer C Compare 1
    #[inline(always)]
    #[must_use]
    pub fn timccmp1(&mut self) -> TIMCCMP1_W<25> {
        TIMCCMP1_W::new(self)
    }
    ///Bit 26 - Timer C Compare 2
    #[inline(always)]
    #[must_use]
    pub fn timccmp2(&mut self) -> TIMCCMP2_W<26> {
        TIMCCMP2_W::new(self)
    }
    ///Bit 27 - Timer C Compare 4
    #[inline(always)]
    #[must_use]
    pub fn timccmp4(&mut self) -> TIMCCMP4_W<27> {
        TIMCCMP4_W::new(self)
    }
    ///Bit 28 - Timer E Compare 1
    #[inline(always)]
    #[must_use]
    pub fn timecmp1(&mut self) -> TIMECMP1_W<28> {
        TIMECMP1_W::new(self)
    }
    ///Bit 29 - Timer E Compare 2
    #[inline(always)]
    #[must_use]
    pub fn timecmp2(&mut self) -> TIMECMP2_W<29> {
        TIMECMP2_W::new(self)
    }
    ///Bit 30 - Timer E Compare 4
    #[inline(always)]
    #[must_use]
    pub fn timecmp4(&mut self) -> TIMECMP4_W<30> {
        TIMECMP4_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TimerA Reset Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rstdr](index.html) module
pub struct RSTDR_SPEC;
impl crate::RegisterSpec for RSTDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [rstdr::R](R) reader structure
impl crate::Readable for RSTDR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rstdr::W](W) writer structure
impl crate::Writable for RSTDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RSTDR to value 0
impl crate::Resettable for RSTDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
