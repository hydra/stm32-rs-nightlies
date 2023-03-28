///Register `SETD1R` reader
pub struct R(crate::R<SETD1R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SETD1R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SETD1R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SETD1R_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SETD1R` writer
pub struct W(crate::W<SETD1R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SETD1R_SPEC>;
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
impl From<crate::W<SETD1R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SETD1R_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SST` reader - Software Set trigger
pub type SST_R = crate::BitReader<bool>;
///Field `SST` writer - Software Set trigger
pub type SST_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETD1R_SPEC, bool, O>;
///Field `RESYNC` reader - Timer A resynchronizaton
pub type RESYNC_R = crate::BitReader<bool>;
///Field `RESYNC` writer - Timer A resynchronizaton
pub type RESYNC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETD1R_SPEC, bool, O>;
///Field `PER` reader - Timer A Period
pub type PER_R = crate::BitReader<bool>;
///Field `PER` writer - Timer A Period
pub type PER_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETD1R_SPEC, bool, O>;
///Field `CMP1` reader - Timer A compare 1
pub type CMP1_R = crate::BitReader<bool>;
///Field `CMP1` writer - Timer A compare 1
pub type CMP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETD1R_SPEC, bool, O>;
///Field `CMP2` reader - Timer A compare 2
pub type CMP2_R = crate::BitReader<bool>;
///Field `CMP2` writer - Timer A compare 2
pub type CMP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETD1R_SPEC, bool, O>;
///Field `CMP3` reader - Timer A compare 3
pub type CMP3_R = crate::BitReader<bool>;
///Field `CMP3` writer - Timer A compare 3
pub type CMP3_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETD1R_SPEC, bool, O>;
///Field `CMP4` reader - Timer A compare 4
pub type CMP4_R = crate::BitReader<bool>;
///Field `CMP4` writer - Timer A compare 4
pub type CMP4_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETD1R_SPEC, bool, O>;
///Field `MSTPER` reader - Master Period
pub type MSTPER_R = crate::BitReader<bool>;
///Field `MSTPER` writer - Master Period
pub type MSTPER_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETD1R_SPEC, bool, O>;
///Field `MSTCMP1` reader - Master Compare 1
pub type MSTCMP1_R = crate::BitReader<bool>;
///Field `MSTCMP1` writer - Master Compare 1
pub type MSTCMP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETD1R_SPEC, bool, O>;
///Field `MSTCMP2` reader - Master Compare 2
pub type MSTCMP2_R = crate::BitReader<bool>;
///Field `MSTCMP2` writer - Master Compare 2
pub type MSTCMP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETD1R_SPEC, bool, O>;
///Field `MSTCMP3` reader - Master Compare 3
pub type MSTCMP3_R = crate::BitReader<bool>;
///Field `MSTCMP3` writer - Master Compare 3
pub type MSTCMP3_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETD1R_SPEC, bool, O>;
///Field `MSTCMP4` reader - Master Compare 4
pub type MSTCMP4_R = crate::BitReader<bool>;
///Field `MSTCMP4` writer - Master Compare 4
pub type MSTCMP4_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETD1R_SPEC, bool, O>;
///Field `TIMEVNT1` reader - Timer Event 1
pub type TIMEVNT1_R = crate::BitReader<bool>;
///Field `TIMEVNT1` writer - Timer Event 1
pub type TIMEVNT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETD1R_SPEC, bool, O>;
///Field `TIMEVNT2` reader - Timer Event 2
pub type TIMEVNT2_R = crate::BitReader<bool>;
///Field `TIMEVNT2` writer - Timer Event 2
pub type TIMEVNT2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETD1R_SPEC, bool, O>;
///Field `TIMEVNT3` reader - Timer Event 3
pub type TIMEVNT3_R = crate::BitReader<bool>;
///Field `TIMEVNT3` writer - Timer Event 3
pub type TIMEVNT3_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETD1R_SPEC, bool, O>;
///Field `TIMEVNT4` reader - Timer Event 4
pub type TIMEVNT4_R = crate::BitReader<bool>;
///Field `TIMEVNT4` writer - Timer Event 4
pub type TIMEVNT4_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETD1R_SPEC, bool, O>;
///Field `TIMEVNT5` reader - Timer Event 5
pub type TIMEVNT5_R = crate::BitReader<bool>;
///Field `TIMEVNT5` writer - Timer Event 5
pub type TIMEVNT5_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETD1R_SPEC, bool, O>;
///Field `TIMEVNT6` reader - Timer Event 6
pub type TIMEVNT6_R = crate::BitReader<bool>;
///Field `TIMEVNT6` writer - Timer Event 6
pub type TIMEVNT6_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETD1R_SPEC, bool, O>;
///Field `TIMEVNT7` reader - Timer Event 7
pub type TIMEVNT7_R = crate::BitReader<bool>;
///Field `TIMEVNT7` writer - Timer Event 7
pub type TIMEVNT7_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETD1R_SPEC, bool, O>;
///Field `TIMEVNT8` reader - Timer Event 8
pub type TIMEVNT8_R = crate::BitReader<bool>;
///Field `TIMEVNT8` writer - Timer Event 8
pub type TIMEVNT8_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETD1R_SPEC, bool, O>;
///Field `TIMEVNT9` reader - Timer Event 9
pub type TIMEVNT9_R = crate::BitReader<bool>;
///Field `TIMEVNT9` writer - Timer Event 9
pub type TIMEVNT9_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETD1R_SPEC, bool, O>;
///Field `EXTEVNT1` reader - External Event 1
pub type EXTEVNT1_R = crate::BitReader<bool>;
///Field `EXTEVNT1` writer - External Event 1
pub type EXTEVNT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETD1R_SPEC, bool, O>;
///Field `EXTEVNT2` reader - External Event 2
pub type EXTEVNT2_R = crate::BitReader<bool>;
///Field `EXTEVNT2` writer - External Event 2
pub type EXTEVNT2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETD1R_SPEC, bool, O>;
///Field `EXTEVNT3` reader - External Event 3
pub type EXTEVNT3_R = crate::BitReader<bool>;
///Field `EXTEVNT3` writer - External Event 3
pub type EXTEVNT3_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETD1R_SPEC, bool, O>;
///Field `EXTEVNT4` reader - External Event 4
pub type EXTEVNT4_R = crate::BitReader<bool>;
///Field `EXTEVNT4` writer - External Event 4
pub type EXTEVNT4_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETD1R_SPEC, bool, O>;
///Field `EXTEVNT5` reader - External Event 5
pub type EXTEVNT5_R = crate::BitReader<bool>;
///Field `EXTEVNT5` writer - External Event 5
pub type EXTEVNT5_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETD1R_SPEC, bool, O>;
///Field `EXTEVNT6` reader - External Event 6
pub type EXTEVNT6_R = crate::BitReader<bool>;
///Field `EXTEVNT6` writer - External Event 6
pub type EXTEVNT6_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETD1R_SPEC, bool, O>;
///Field `EXTEVNT7` reader - External Event 7
pub type EXTEVNT7_R = crate::BitReader<bool>;
///Field `EXTEVNT7` writer - External Event 7
pub type EXTEVNT7_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETD1R_SPEC, bool, O>;
///Field `EXTEVNT8` reader - External Event 8
pub type EXTEVNT8_R = crate::BitReader<bool>;
///Field `EXTEVNT8` writer - External Event 8
pub type EXTEVNT8_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETD1R_SPEC, bool, O>;
///Field `EXTEVNT9` reader - External Event 9
pub type EXTEVNT9_R = crate::BitReader<bool>;
///Field `EXTEVNT9` writer - External Event 9
pub type EXTEVNT9_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETD1R_SPEC, bool, O>;
///Field `EXTEVNT10` reader - External Event 10
pub type EXTEVNT10_R = crate::BitReader<bool>;
///Field `EXTEVNT10` writer - External Event 10
pub type EXTEVNT10_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETD1R_SPEC, bool, O>;
///Field `UPDATE` reader - Registers update (transfer preload to active)
pub type UPDATE_R = crate::BitReader<bool>;
///Field `UPDATE` writer - Registers update (transfer preload to active)
pub type UPDATE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETD1R_SPEC, bool, O>;
impl R {
    ///Bit 0 - Software Set trigger
    #[inline(always)]
    pub fn sst(&self) -> SST_R {
        SST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Timer A resynchronizaton
    #[inline(always)]
    pub fn resync(&self) -> RESYNC_R {
        RESYNC_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Timer A Period
    #[inline(always)]
    pub fn per(&self) -> PER_R {
        PER_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Timer A compare 1
    #[inline(always)]
    pub fn cmp1(&self) -> CMP1_R {
        CMP1_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Timer A compare 2
    #[inline(always)]
    pub fn cmp2(&self) -> CMP2_R {
        CMP2_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Timer A compare 3
    #[inline(always)]
    pub fn cmp3(&self) -> CMP3_R {
        CMP3_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Timer A compare 4
    #[inline(always)]
    pub fn cmp4(&self) -> CMP4_R {
        CMP4_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Master Period
    #[inline(always)]
    pub fn mstper(&self) -> MSTPER_R {
        MSTPER_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Master Compare 1
    #[inline(always)]
    pub fn mstcmp1(&self) -> MSTCMP1_R {
        MSTCMP1_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Master Compare 2
    #[inline(always)]
    pub fn mstcmp2(&self) -> MSTCMP2_R {
        MSTCMP2_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Master Compare 3
    #[inline(always)]
    pub fn mstcmp3(&self) -> MSTCMP3_R {
        MSTCMP3_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Master Compare 4
    #[inline(always)]
    pub fn mstcmp4(&self) -> MSTCMP4_R {
        MSTCMP4_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Timer Event 1
    #[inline(always)]
    pub fn timevnt1(&self) -> TIMEVNT1_R {
        TIMEVNT1_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Timer Event 2
    #[inline(always)]
    pub fn timevnt2(&self) -> TIMEVNT2_R {
        TIMEVNT2_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Timer Event 3
    #[inline(always)]
    pub fn timevnt3(&self) -> TIMEVNT3_R {
        TIMEVNT3_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Timer Event 4
    #[inline(always)]
    pub fn timevnt4(&self) -> TIMEVNT4_R {
        TIMEVNT4_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Timer Event 5
    #[inline(always)]
    pub fn timevnt5(&self) -> TIMEVNT5_R {
        TIMEVNT5_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Timer Event 6
    #[inline(always)]
    pub fn timevnt6(&self) -> TIMEVNT6_R {
        TIMEVNT6_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Timer Event 7
    #[inline(always)]
    pub fn timevnt7(&self) -> TIMEVNT7_R {
        TIMEVNT7_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Timer Event 8
    #[inline(always)]
    pub fn timevnt8(&self) -> TIMEVNT8_R {
        TIMEVNT8_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Timer Event 9
    #[inline(always)]
    pub fn timevnt9(&self) -> TIMEVNT9_R {
        TIMEVNT9_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - External Event 1
    #[inline(always)]
    pub fn extevnt1(&self) -> EXTEVNT1_R {
        EXTEVNT1_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - External Event 2
    #[inline(always)]
    pub fn extevnt2(&self) -> EXTEVNT2_R {
        EXTEVNT2_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - External Event 3
    #[inline(always)]
    pub fn extevnt3(&self) -> EXTEVNT3_R {
        EXTEVNT3_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - External Event 4
    #[inline(always)]
    pub fn extevnt4(&self) -> EXTEVNT4_R {
        EXTEVNT4_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - External Event 5
    #[inline(always)]
    pub fn extevnt5(&self) -> EXTEVNT5_R {
        EXTEVNT5_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - External Event 6
    #[inline(always)]
    pub fn extevnt6(&self) -> EXTEVNT6_R {
        EXTEVNT6_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - External Event 7
    #[inline(always)]
    pub fn extevnt7(&self) -> EXTEVNT7_R {
        EXTEVNT7_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - External Event 8
    #[inline(always)]
    pub fn extevnt8(&self) -> EXTEVNT8_R {
        EXTEVNT8_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - External Event 9
    #[inline(always)]
    pub fn extevnt9(&self) -> EXTEVNT9_R {
        EXTEVNT9_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - External Event 10
    #[inline(always)]
    pub fn extevnt10(&self) -> EXTEVNT10_R {
        EXTEVNT10_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Registers update (transfer preload to active)
    #[inline(always)]
    pub fn update(&self) -> UPDATE_R {
        UPDATE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Software Set trigger
    #[inline(always)]
    #[must_use]
    pub fn sst(&mut self) -> SST_W<0> {
        SST_W::new(self)
    }
    ///Bit 1 - Timer A resynchronizaton
    #[inline(always)]
    #[must_use]
    pub fn resync(&mut self) -> RESYNC_W<1> {
        RESYNC_W::new(self)
    }
    ///Bit 2 - Timer A Period
    #[inline(always)]
    #[must_use]
    pub fn per(&mut self) -> PER_W<2> {
        PER_W::new(self)
    }
    ///Bit 3 - Timer A compare 1
    #[inline(always)]
    #[must_use]
    pub fn cmp1(&mut self) -> CMP1_W<3> {
        CMP1_W::new(self)
    }
    ///Bit 4 - Timer A compare 2
    #[inline(always)]
    #[must_use]
    pub fn cmp2(&mut self) -> CMP2_W<4> {
        CMP2_W::new(self)
    }
    ///Bit 5 - Timer A compare 3
    #[inline(always)]
    #[must_use]
    pub fn cmp3(&mut self) -> CMP3_W<5> {
        CMP3_W::new(self)
    }
    ///Bit 6 - Timer A compare 4
    #[inline(always)]
    #[must_use]
    pub fn cmp4(&mut self) -> CMP4_W<6> {
        CMP4_W::new(self)
    }
    ///Bit 7 - Master Period
    #[inline(always)]
    #[must_use]
    pub fn mstper(&mut self) -> MSTPER_W<7> {
        MSTPER_W::new(self)
    }
    ///Bit 8 - Master Compare 1
    #[inline(always)]
    #[must_use]
    pub fn mstcmp1(&mut self) -> MSTCMP1_W<8> {
        MSTCMP1_W::new(self)
    }
    ///Bit 9 - Master Compare 2
    #[inline(always)]
    #[must_use]
    pub fn mstcmp2(&mut self) -> MSTCMP2_W<9> {
        MSTCMP2_W::new(self)
    }
    ///Bit 10 - Master Compare 3
    #[inline(always)]
    #[must_use]
    pub fn mstcmp3(&mut self) -> MSTCMP3_W<10> {
        MSTCMP3_W::new(self)
    }
    ///Bit 11 - Master Compare 4
    #[inline(always)]
    #[must_use]
    pub fn mstcmp4(&mut self) -> MSTCMP4_W<11> {
        MSTCMP4_W::new(self)
    }
    ///Bit 12 - Timer Event 1
    #[inline(always)]
    #[must_use]
    pub fn timevnt1(&mut self) -> TIMEVNT1_W<12> {
        TIMEVNT1_W::new(self)
    }
    ///Bit 13 - Timer Event 2
    #[inline(always)]
    #[must_use]
    pub fn timevnt2(&mut self) -> TIMEVNT2_W<13> {
        TIMEVNT2_W::new(self)
    }
    ///Bit 14 - Timer Event 3
    #[inline(always)]
    #[must_use]
    pub fn timevnt3(&mut self) -> TIMEVNT3_W<14> {
        TIMEVNT3_W::new(self)
    }
    ///Bit 15 - Timer Event 4
    #[inline(always)]
    #[must_use]
    pub fn timevnt4(&mut self) -> TIMEVNT4_W<15> {
        TIMEVNT4_W::new(self)
    }
    ///Bit 16 - Timer Event 5
    #[inline(always)]
    #[must_use]
    pub fn timevnt5(&mut self) -> TIMEVNT5_W<16> {
        TIMEVNT5_W::new(self)
    }
    ///Bit 17 - Timer Event 6
    #[inline(always)]
    #[must_use]
    pub fn timevnt6(&mut self) -> TIMEVNT6_W<17> {
        TIMEVNT6_W::new(self)
    }
    ///Bit 18 - Timer Event 7
    #[inline(always)]
    #[must_use]
    pub fn timevnt7(&mut self) -> TIMEVNT7_W<18> {
        TIMEVNT7_W::new(self)
    }
    ///Bit 19 - Timer Event 8
    #[inline(always)]
    #[must_use]
    pub fn timevnt8(&mut self) -> TIMEVNT8_W<19> {
        TIMEVNT8_W::new(self)
    }
    ///Bit 20 - Timer Event 9
    #[inline(always)]
    #[must_use]
    pub fn timevnt9(&mut self) -> TIMEVNT9_W<20> {
        TIMEVNT9_W::new(self)
    }
    ///Bit 21 - External Event 1
    #[inline(always)]
    #[must_use]
    pub fn extevnt1(&mut self) -> EXTEVNT1_W<21> {
        EXTEVNT1_W::new(self)
    }
    ///Bit 22 - External Event 2
    #[inline(always)]
    #[must_use]
    pub fn extevnt2(&mut self) -> EXTEVNT2_W<22> {
        EXTEVNT2_W::new(self)
    }
    ///Bit 23 - External Event 3
    #[inline(always)]
    #[must_use]
    pub fn extevnt3(&mut self) -> EXTEVNT3_W<23> {
        EXTEVNT3_W::new(self)
    }
    ///Bit 24 - External Event 4
    #[inline(always)]
    #[must_use]
    pub fn extevnt4(&mut self) -> EXTEVNT4_W<24> {
        EXTEVNT4_W::new(self)
    }
    ///Bit 25 - External Event 5
    #[inline(always)]
    #[must_use]
    pub fn extevnt5(&mut self) -> EXTEVNT5_W<25> {
        EXTEVNT5_W::new(self)
    }
    ///Bit 26 - External Event 6
    #[inline(always)]
    #[must_use]
    pub fn extevnt6(&mut self) -> EXTEVNT6_W<26> {
        EXTEVNT6_W::new(self)
    }
    ///Bit 27 - External Event 7
    #[inline(always)]
    #[must_use]
    pub fn extevnt7(&mut self) -> EXTEVNT7_W<27> {
        EXTEVNT7_W::new(self)
    }
    ///Bit 28 - External Event 8
    #[inline(always)]
    #[must_use]
    pub fn extevnt8(&mut self) -> EXTEVNT8_W<28> {
        EXTEVNT8_W::new(self)
    }
    ///Bit 29 - External Event 9
    #[inline(always)]
    #[must_use]
    pub fn extevnt9(&mut self) -> EXTEVNT9_W<29> {
        EXTEVNT9_W::new(self)
    }
    ///Bit 30 - External Event 10
    #[inline(always)]
    #[must_use]
    pub fn extevnt10(&mut self) -> EXTEVNT10_W<30> {
        EXTEVNT10_W::new(self)
    }
    ///Bit 31 - Registers update (transfer preload to active)
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
///Timerx Output1 Set Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [setd1r](index.html) module
pub struct SETD1R_SPEC;
impl crate::RegisterSpec for SETD1R_SPEC {
    type Ux = u32;
}
///`read()` method returns [setd1r::R](R) reader structure
impl crate::Readable for SETD1R_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [setd1r::W](W) writer structure
impl crate::Writable for SETD1R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets SETD1R to value 0
impl crate::Resettable for SETD1R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
