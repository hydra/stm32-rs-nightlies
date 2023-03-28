///Register `RSTCR` reader
pub struct R(crate::R<RSTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSTCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RSTCR` writer
pub struct W(crate::W<RSTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSTCR_SPEC>;
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
impl From<crate::W<RSTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSTCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TIMFCMP1` reader - Timer A Update reset
pub type TIMFCMP1_R = crate::BitReader<bool>;
///Field `TIMFCMP1` writer - Timer A Update reset
pub type TIMFCMP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTCR_SPEC, bool, O>;
///Field `UPDT` reader - Timer A Update reset
pub type UPDT_R = crate::BitReader<bool>;
///Field `UPDT` writer - Timer A Update reset
pub type UPDT_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTCR_SPEC, bool, O>;
///Field `CMP2` reader - Timer A compare 2 reset
pub type CMP2_R = crate::BitReader<bool>;
///Field `CMP2` writer - Timer A compare 2 reset
pub type CMP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTCR_SPEC, bool, O>;
///Field `CMP4` reader - Timer A compare 4 reset
pub type CMP4_R = crate::BitReader<bool>;
///Field `CMP4` writer - Timer A compare 4 reset
pub type CMP4_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTCR_SPEC, bool, O>;
///Field `MSTPER` reader - Master timer Period
pub type MSTPER_R = crate::BitReader<bool>;
///Field `MSTPER` writer - Master timer Period
pub type MSTPER_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTCR_SPEC, bool, O>;
///Field `MSTCMP1` reader - Master compare 1
pub type MSTCMP1_R = crate::BitReader<bool>;
///Field `MSTCMP1` writer - Master compare 1
pub type MSTCMP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTCR_SPEC, bool, O>;
///Field `MSTCMP2` reader - Master compare 2
pub type MSTCMP2_R = crate::BitReader<bool>;
///Field `MSTCMP2` writer - Master compare 2
pub type MSTCMP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTCR_SPEC, bool, O>;
///Field `MSTCMP3` reader - Master compare 3
pub type MSTCMP3_R = crate::BitReader<bool>;
///Field `MSTCMP3` writer - Master compare 3
pub type MSTCMP3_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTCR_SPEC, bool, O>;
///Field `MSTCMP4` reader - Master compare 4
pub type MSTCMP4_R = crate::BitReader<bool>;
///Field `MSTCMP4` writer - Master compare 4
pub type MSTCMP4_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTCR_SPEC, bool, O>;
///Field `EXTEVNT1` reader - External Event 1
pub type EXTEVNT1_R = crate::BitReader<bool>;
///Field `EXTEVNT1` writer - External Event 1
pub type EXTEVNT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTCR_SPEC, bool, O>;
///Field `EXTEVNT2` reader - External Event 2
pub type EXTEVNT2_R = crate::BitReader<bool>;
///Field `EXTEVNT2` writer - External Event 2
pub type EXTEVNT2_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTCR_SPEC, bool, O>;
///Field `EXTEVNT3` reader - External Event 3
pub type EXTEVNT3_R = crate::BitReader<bool>;
///Field `EXTEVNT3` writer - External Event 3
pub type EXTEVNT3_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTCR_SPEC, bool, O>;
///Field `EXTEVNT4` reader - External Event 4
pub type EXTEVNT4_R = crate::BitReader<bool>;
///Field `EXTEVNT4` writer - External Event 4
pub type EXTEVNT4_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTCR_SPEC, bool, O>;
///Field `EXTEVNT5` reader - External Event 5
pub type EXTEVNT5_R = crate::BitReader<bool>;
///Field `EXTEVNT5` writer - External Event 5
pub type EXTEVNT5_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTCR_SPEC, bool, O>;
///Field `EXTEVNT6` reader - External Event 6
pub type EXTEVNT6_R = crate::BitReader<bool>;
///Field `EXTEVNT6` writer - External Event 6
pub type EXTEVNT6_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTCR_SPEC, bool, O>;
///Field `EXTEVNT7` reader - External Event 7
pub type EXTEVNT7_R = crate::BitReader<bool>;
///Field `EXTEVNT7` writer - External Event 7
pub type EXTEVNT7_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTCR_SPEC, bool, O>;
///Field `EXTEVNT8` reader - External Event 8
pub type EXTEVNT8_R = crate::BitReader<bool>;
///Field `EXTEVNT8` writer - External Event 8
pub type EXTEVNT8_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTCR_SPEC, bool, O>;
///Field `EXTEVNT9` reader - External Event 9
pub type EXTEVNT9_R = crate::BitReader<bool>;
///Field `EXTEVNT9` writer - External Event 9
pub type EXTEVNT9_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTCR_SPEC, bool, O>;
///Field `EXTEVNT10` reader - External Event 10
pub type EXTEVNT10_R = crate::BitReader<bool>;
///Field `EXTEVNT10` writer - External Event 10
pub type EXTEVNT10_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTCR_SPEC, bool, O>;
///Field `TIMACMP1` reader - Timer A Compare 1
pub type TIMACMP1_R = crate::BitReader<bool>;
///Field `TIMACMP1` writer - Timer A Compare 1
pub type TIMACMP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTCR_SPEC, bool, O>;
///Field `TIMACMP2` reader - Timer A Compare 2
pub type TIMACMP2_R = crate::BitReader<bool>;
///Field `TIMACMP2` writer - Timer A Compare 2
pub type TIMACMP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTCR_SPEC, bool, O>;
///Field `TIMACMP4` reader - Timer A Compare 4
pub type TIMACMP4_R = crate::BitReader<bool>;
///Field `TIMACMP4` writer - Timer A Compare 4
pub type TIMACMP4_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTCR_SPEC, bool, O>;
///Field `TIMBCMP1` reader - Timer B Compare 1
pub type TIMBCMP1_R = crate::BitReader<bool>;
///Field `TIMBCMP1` writer - Timer B Compare 1
pub type TIMBCMP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTCR_SPEC, bool, O>;
///Field `TIMBCMP2` reader - Timer B Compare 2
pub type TIMBCMP2_R = crate::BitReader<bool>;
///Field `TIMBCMP2` writer - Timer B Compare 2
pub type TIMBCMP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTCR_SPEC, bool, O>;
///Field `TIMBCMP4` reader - Timer B Compare 4
pub type TIMBCMP4_R = crate::BitReader<bool>;
///Field `TIMBCMP4` writer - Timer B Compare 4
pub type TIMBCMP4_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTCR_SPEC, bool, O>;
///Field `TIMDCMP1` reader - Timer D Compare 1
pub type TIMDCMP1_R = crate::BitReader<bool>;
///Field `TIMDCMP1` writer - Timer D Compare 1
pub type TIMDCMP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTCR_SPEC, bool, O>;
///Field `TIMDCMP2` reader - Timer D Compare 2
pub type TIMDCMP2_R = crate::BitReader<bool>;
///Field `TIMDCMP2` writer - Timer D Compare 2
pub type TIMDCMP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTCR_SPEC, bool, O>;
///Field `TIMDCMP4` reader - Timer D Compare 4
pub type TIMDCMP4_R = crate::BitReader<bool>;
///Field `TIMDCMP4` writer - Timer D Compare 4
pub type TIMDCMP4_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTCR_SPEC, bool, O>;
///Field `TIMECMP1` reader - Timer E Compare 1
pub type TIMECMP1_R = crate::BitReader<bool>;
///Field `TIMECMP1` writer - Timer E Compare 1
pub type TIMECMP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTCR_SPEC, bool, O>;
///Field `TIMECMP2` reader - Timer E Compare 2
pub type TIMECMP2_R = crate::BitReader<bool>;
///Field `TIMECMP2` writer - Timer E Compare 2
pub type TIMECMP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTCR_SPEC, bool, O>;
///Field `TIMECMP4` reader - Timer E Compare 4
pub type TIMECMP4_R = crate::BitReader<bool>;
///Field `TIMECMP4` writer - Timer E Compare 4
pub type TIMECMP4_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTCR_SPEC, bool, O>;
///Field `TIMFCPM2` reader - Timer F Compare 2
pub type TIMFCPM2_R = crate::BitReader<bool>;
///Field `TIMFCPM2` writer - Timer F Compare 2
pub type TIMFCPM2_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTCR_SPEC, bool, O>;
impl R {
    ///Bit 0 - Timer A Update reset
    #[inline(always)]
    pub fn timfcmp1(&self) -> TIMFCMP1_R {
        TIMFCMP1_R::new((self.bits & 1) != 0)
    }
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
    ///Bit 25 - Timer D Compare 1
    #[inline(always)]
    pub fn timdcmp1(&self) -> TIMDCMP1_R {
        TIMDCMP1_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Timer D Compare 2
    #[inline(always)]
    pub fn timdcmp2(&self) -> TIMDCMP2_R {
        TIMDCMP2_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Timer D Compare 4
    #[inline(always)]
    pub fn timdcmp4(&self) -> TIMDCMP4_R {
        TIMDCMP4_R::new(((self.bits >> 27) & 1) != 0)
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
    ///Bit 31 - Timer F Compare 2
    #[inline(always)]
    pub fn timfcpm2(&self) -> TIMFCPM2_R {
        TIMFCPM2_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Timer A Update reset
    #[inline(always)]
    #[must_use]
    pub fn timfcmp1(&mut self) -> TIMFCMP1_W<0> {
        TIMFCMP1_W::new(self)
    }
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
    ///Bit 25 - Timer D Compare 1
    #[inline(always)]
    #[must_use]
    pub fn timdcmp1(&mut self) -> TIMDCMP1_W<25> {
        TIMDCMP1_W::new(self)
    }
    ///Bit 26 - Timer D Compare 2
    #[inline(always)]
    #[must_use]
    pub fn timdcmp2(&mut self) -> TIMDCMP2_W<26> {
        TIMDCMP2_W::new(self)
    }
    ///Bit 27 - Timer D Compare 4
    #[inline(always)]
    #[must_use]
    pub fn timdcmp4(&mut self) -> TIMDCMP4_W<27> {
        TIMDCMP4_W::new(self)
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
    ///Bit 31 - Timer F Compare 2
    #[inline(always)]
    #[must_use]
    pub fn timfcpm2(&mut self) -> TIMFCPM2_W<31> {
        TIMFCPM2_W::new(self)
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
///For information about available fields see [rstcr](index.html) module
pub struct RSTCR_SPEC;
impl crate::RegisterSpec for RSTCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [rstcr::R](R) reader structure
impl crate::Readable for RSTCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rstcr::W](W) writer structure
impl crate::Writable for RSTCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RSTCR to value 0
impl crate::Resettable for RSTCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
