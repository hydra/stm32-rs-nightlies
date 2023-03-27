///Register `SETC2R` reader
pub struct R(crate::R<SETC2R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SETC2R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SETC2R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SETC2R_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SETC2R` writer
pub struct W(crate::W<SETC2R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SETC2R_SPEC>;
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
impl From<crate::W<SETC2R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SETC2R_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SST` reader - SST
pub type SST_R = crate::BitReader<bool>;
///Field `SST` writer - SST
pub type SST_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETC2R_SPEC, bool, O>;
///Field `RESYNC` reader - RESYNC
pub type RESYNC_R = crate::BitReader<bool>;
///Field `RESYNC` writer - RESYNC
pub type RESYNC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETC2R_SPEC, bool, O>;
///Field `PER` reader - PER
pub type PER_R = crate::BitReader<bool>;
///Field `PER` writer - PER
pub type PER_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETC2R_SPEC, bool, O>;
///Field `CMP1` reader - CMP1
pub type CMP1_R = crate::BitReader<bool>;
///Field `CMP1` writer - CMP1
pub type CMP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETC2R_SPEC, bool, O>;
///Field `CMP2` reader - CMP2
pub type CMP2_R = crate::BitReader<bool>;
///Field `CMP2` writer - CMP2
pub type CMP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETC2R_SPEC, bool, O>;
///Field `CMP3` reader - CMP3
pub type CMP3_R = crate::BitReader<bool>;
///Field `CMP3` writer - CMP3
pub type CMP3_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETC2R_SPEC, bool, O>;
///Field `CMP4` reader - CMP4
pub type CMP4_R = crate::BitReader<bool>;
///Field `CMP4` writer - CMP4
pub type CMP4_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETC2R_SPEC, bool, O>;
///Field `MSTPER` reader - MSTPER
pub type MSTPER_R = crate::BitReader<bool>;
///Field `MSTPER` writer - MSTPER
pub type MSTPER_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETC2R_SPEC, bool, O>;
///Field `MSTCMP1` reader - MSTCMP1
pub type MSTCMP1_R = crate::BitReader<bool>;
///Field `MSTCMP1` writer - MSTCMP1
pub type MSTCMP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETC2R_SPEC, bool, O>;
///Field `MSTCMP2` reader - MSTCMP2
pub type MSTCMP2_R = crate::BitReader<bool>;
///Field `MSTCMP2` writer - MSTCMP2
pub type MSTCMP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETC2R_SPEC, bool, O>;
///Field `MSTCMP3` reader - MSTCMP3
pub type MSTCMP3_R = crate::BitReader<bool>;
///Field `MSTCMP3` writer - MSTCMP3
pub type MSTCMP3_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETC2R_SPEC, bool, O>;
///Field `MSTCMP4` reader - MSTCMP4
pub type MSTCMP4_R = crate::BitReader<bool>;
///Field `MSTCMP4` writer - MSTCMP4
pub type MSTCMP4_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETC2R_SPEC, bool, O>;
///Field `TIMEVNT1` reader - TIMEVNT1
pub type TIMEVNT1_R = crate::BitReader<bool>;
///Field `TIMEVNT1` writer - TIMEVNT1
pub type TIMEVNT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETC2R_SPEC, bool, O>;
///Field `TIMEVNT2` reader - TIMEVNT2
pub type TIMEVNT2_R = crate::BitReader<bool>;
///Field `TIMEVNT2` writer - TIMEVNT2
pub type TIMEVNT2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETC2R_SPEC, bool, O>;
///Field `TIMEVNT3` reader - TIMEVNT3
pub type TIMEVNT3_R = crate::BitReader<bool>;
///Field `TIMEVNT3` writer - TIMEVNT3
pub type TIMEVNT3_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETC2R_SPEC, bool, O>;
///Field `TIMEVNT4` reader - TIMEVNT4
pub type TIMEVNT4_R = crate::BitReader<bool>;
///Field `TIMEVNT4` writer - TIMEVNT4
pub type TIMEVNT4_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETC2R_SPEC, bool, O>;
///Field `TIMEVNT5` reader - TIMEVNT5
pub type TIMEVNT5_R = crate::BitReader<bool>;
///Field `TIMEVNT5` writer - TIMEVNT5
pub type TIMEVNT5_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETC2R_SPEC, bool, O>;
///Field `TIMEVNT6` reader - TIMEVNT6
pub type TIMEVNT6_R = crate::BitReader<bool>;
///Field `TIMEVNT6` writer - TIMEVNT6
pub type TIMEVNT6_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETC2R_SPEC, bool, O>;
///Field `TIMEVNT7` reader - TIMEVNT7
pub type TIMEVNT7_R = crate::BitReader<bool>;
///Field `TIMEVNT7` writer - TIMEVNT7
pub type TIMEVNT7_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETC2R_SPEC, bool, O>;
///Field `TIMEVNT8` reader - TIMEVNT8
pub type TIMEVNT8_R = crate::BitReader<bool>;
///Field `TIMEVNT8` writer - TIMEVNT8
pub type TIMEVNT8_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETC2R_SPEC, bool, O>;
///Field `TIMEVNT9` reader - TIMEVNT9
pub type TIMEVNT9_R = crate::BitReader<bool>;
///Field `TIMEVNT9` writer - TIMEVNT9
pub type TIMEVNT9_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETC2R_SPEC, bool, O>;
///Field `EXTEVNT1` reader - EXTEVNT1
pub type EXTEVNT1_R = crate::BitReader<bool>;
///Field `EXTEVNT1` writer - EXTEVNT1
pub type EXTEVNT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETC2R_SPEC, bool, O>;
///Field `EXTEVNT2` reader - EXTEVNT2
pub type EXTEVNT2_R = crate::BitReader<bool>;
///Field `EXTEVNT2` writer - EXTEVNT2
pub type EXTEVNT2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETC2R_SPEC, bool, O>;
///Field `EXTEVNT3` reader - EXTEVNT3
pub type EXTEVNT3_R = crate::BitReader<bool>;
///Field `EXTEVNT3` writer - EXTEVNT3
pub type EXTEVNT3_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETC2R_SPEC, bool, O>;
///Field `EXTEVNT4` reader - EXTEVNT4
pub type EXTEVNT4_R = crate::BitReader<bool>;
///Field `EXTEVNT4` writer - EXTEVNT4
pub type EXTEVNT4_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETC2R_SPEC, bool, O>;
///Field `EXTEVNT5` reader - EXTEVNT5
pub type EXTEVNT5_R = crate::BitReader<bool>;
///Field `EXTEVNT5` writer - EXTEVNT5
pub type EXTEVNT5_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETC2R_SPEC, bool, O>;
///Field `EXTEVNT6` reader - EXTEVNT6
pub type EXTEVNT6_R = crate::BitReader<bool>;
///Field `EXTEVNT6` writer - EXTEVNT6
pub type EXTEVNT6_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETC2R_SPEC, bool, O>;
///Field `EXTEVNT7` reader - EXTEVNT7
pub type EXTEVNT7_R = crate::BitReader<bool>;
///Field `EXTEVNT7` writer - EXTEVNT7
pub type EXTEVNT7_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETC2R_SPEC, bool, O>;
///Field `EXTEVNT8` reader - EXTEVNT8
pub type EXTEVNT8_R = crate::BitReader<bool>;
///Field `EXTEVNT8` writer - EXTEVNT8
pub type EXTEVNT8_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETC2R_SPEC, bool, O>;
///Field `EXTEVNT9` reader - EXTEVNT9
pub type EXTEVNT9_R = crate::BitReader<bool>;
///Field `EXTEVNT9` writer - EXTEVNT9
pub type EXTEVNT9_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETC2R_SPEC, bool, O>;
///Field `EXTEVNT10` reader - EXTEVNT10
pub type EXTEVNT10_R = crate::BitReader<bool>;
///Field `EXTEVNT10` writer - EXTEVNT10
pub type EXTEVNT10_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETC2R_SPEC, bool, O>;
///Field `UPDATE` reader - UPDATE
pub type UPDATE_R = crate::BitReader<bool>;
///Field `UPDATE` writer - UPDATE
pub type UPDATE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETC2R_SPEC, bool, O>;
impl R {
    ///Bit 0 - SST
    #[inline(always)]
    pub fn sst(&self) -> SST_R {
        SST_R::new((self.bits & 1) != 0)
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
    ///Bit 0 - SST
    #[inline(always)]
    #[must_use]
    pub fn sst(&mut self) -> SST_W<0> {
        SST_W::new(self)
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
///Timerx Output2 Set Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [setc2r](index.html) module
pub struct SETC2R_SPEC;
impl crate::RegisterSpec for SETC2R_SPEC {
    type Ux = u32;
}
///`read()` method returns [setc2r::R](R) reader structure
impl crate::Readable for SETC2R_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [setc2r::W](W) writer structure
impl crate::Writable for SETC2R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets SETC2R to value 0
impl crate::Resettable for SETC2R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
