///Register `BMTRG` reader
pub struct R(crate::R<BMTRG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BMTRG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BMTRG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BMTRG_SPEC>) -> Self {
        R(reader)
    }
}
///Register `BMTRG` writer
pub struct W(crate::W<BMTRG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BMTRG_SPEC>;
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
impl From<crate::W<BMTRG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BMTRG_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SW` reader - SW
pub type SW_R = crate::BitReader<bool>;
///Field `SW` writer - SW
pub type SW_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMTRG_SPEC, bool, O>;
///Field `MSTRST` reader - MSTRST
pub type MSTRST_R = crate::BitReader<bool>;
///Field `MSTRST` writer - MSTRST
pub type MSTRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMTRG_SPEC, bool, O>;
///Field `MSTREP` reader - MSTREP
pub type MSTREP_R = crate::BitReader<bool>;
///Field `MSTREP` writer - MSTREP
pub type MSTREP_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMTRG_SPEC, bool, O>;
///Field `MSTCMP1` reader - MSTCMP1
pub type MSTCMP1_R = crate::BitReader<bool>;
///Field `MSTCMP1` writer - MSTCMP1
pub type MSTCMP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMTRG_SPEC, bool, O>;
///Field `MSTCMP2` reader - MSTCMP2
pub type MSTCMP2_R = crate::BitReader<bool>;
///Field `MSTCMP2` writer - MSTCMP2
pub type MSTCMP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMTRG_SPEC, bool, O>;
///Field `MSTCMP3` reader - MSTCMP3
pub type MSTCMP3_R = crate::BitReader<bool>;
///Field `MSTCMP3` writer - MSTCMP3
pub type MSTCMP3_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMTRG_SPEC, bool, O>;
///Field `MSTCMP4` reader - MSTCMP4
pub type MSTCMP4_R = crate::BitReader<bool>;
///Field `MSTCMP4` writer - MSTCMP4
pub type MSTCMP4_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMTRG_SPEC, bool, O>;
///Field `TARST` reader - TARST
pub type TARST_R = crate::BitReader<bool>;
///Field `TARST` writer - TARST
pub type TARST_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMTRG_SPEC, bool, O>;
///Field `TAREP` reader - TAREP
pub type TAREP_R = crate::BitReader<bool>;
///Field `TAREP` writer - TAREP
pub type TAREP_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMTRG_SPEC, bool, O>;
///Field `TACMP1` reader - TACMP1
pub type TACMP1_R = crate::BitReader<bool>;
///Field `TACMP1` writer - TACMP1
pub type TACMP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMTRG_SPEC, bool, O>;
///Field `TACMP2` reader - TACMP2
pub type TACMP2_R = crate::BitReader<bool>;
///Field `TACMP2` writer - TACMP2
pub type TACMP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMTRG_SPEC, bool, O>;
///Field `TBRST` reader - TBRST
pub type TBRST_R = crate::BitReader<bool>;
///Field `TBRST` writer - TBRST
pub type TBRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMTRG_SPEC, bool, O>;
///Field `TBREP` reader - TBREP
pub type TBREP_R = crate::BitReader<bool>;
///Field `TBREP` writer - TBREP
pub type TBREP_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMTRG_SPEC, bool, O>;
///Field `TBCMP1` reader - TBCMP1
pub type TBCMP1_R = crate::BitReader<bool>;
///Field `TBCMP1` writer - TBCMP1
pub type TBCMP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMTRG_SPEC, bool, O>;
///Field `TBCMP2` reader - TBCMP2
pub type TBCMP2_R = crate::BitReader<bool>;
///Field `TBCMP2` writer - TBCMP2
pub type TBCMP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMTRG_SPEC, bool, O>;
///Field `TCRST` reader - TCRST
pub type TCRST_R = crate::BitReader<bool>;
///Field `TCRST` writer - TCRST
pub type TCRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMTRG_SPEC, bool, O>;
///Field `TCREP` reader - TCREP
pub type TCREP_R = crate::BitReader<bool>;
///Field `TCREP` writer - TCREP
pub type TCREP_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMTRG_SPEC, bool, O>;
///Field `TCCMP1` reader - TCCMP1
pub type TCCMP1_R = crate::BitReader<bool>;
///Field `TCCMP1` writer - TCCMP1
pub type TCCMP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMTRG_SPEC, bool, O>;
///Field `TCCMP2` reader - TCCMP2
pub type TCCMP2_R = crate::BitReader<bool>;
///Field `TCCMP2` writer - TCCMP2
pub type TCCMP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMTRG_SPEC, bool, O>;
///Field `TDRST` reader - TDRST
pub type TDRST_R = crate::BitReader<bool>;
///Field `TDRST` writer - TDRST
pub type TDRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMTRG_SPEC, bool, O>;
///Field `TDREP` reader - TDREP
pub type TDREP_R = crate::BitReader<bool>;
///Field `TDREP` writer - TDREP
pub type TDREP_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMTRG_SPEC, bool, O>;
///Field `TDCMP1` reader - TDCMP1
pub type TDCMP1_R = crate::BitReader<bool>;
///Field `TDCMP1` writer - TDCMP1
pub type TDCMP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMTRG_SPEC, bool, O>;
///Field `TDCMP2` reader - TDCMP2
pub type TDCMP2_R = crate::BitReader<bool>;
///Field `TDCMP2` writer - TDCMP2
pub type TDCMP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMTRG_SPEC, bool, O>;
///Field `TERST` reader - TERST
pub type TERST_R = crate::BitReader<bool>;
///Field `TERST` writer - TERST
pub type TERST_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMTRG_SPEC, bool, O>;
///Field `TEREP` reader - TEREP
pub type TEREP_R = crate::BitReader<bool>;
///Field `TEREP` writer - TEREP
pub type TEREP_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMTRG_SPEC, bool, O>;
///Field `TECMP1` reader - TECMP1
pub type TECMP1_R = crate::BitReader<bool>;
///Field `TECMP1` writer - TECMP1
pub type TECMP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMTRG_SPEC, bool, O>;
///Field `TECMP2` reader - TECMP2
pub type TECMP2_R = crate::BitReader<bool>;
///Field `TECMP2` writer - TECMP2
pub type TECMP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMTRG_SPEC, bool, O>;
///Field `OCHPEV` reader - OCHPEV
pub type OCHPEV_R = crate::BitReader<bool>;
///Field `OCHPEV` writer - OCHPEV
pub type OCHPEV_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMTRG_SPEC, bool, O>;
impl R {
    ///Bit 0 - SW
    #[inline(always)]
    pub fn sw(&self) -> SW_R {
        SW_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - MSTRST
    #[inline(always)]
    pub fn mstrst(&self) -> MSTRST_R {
        MSTRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - MSTREP
    #[inline(always)]
    pub fn mstrep(&self) -> MSTREP_R {
        MSTREP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - MSTCMP1
    #[inline(always)]
    pub fn mstcmp1(&self) -> MSTCMP1_R {
        MSTCMP1_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - MSTCMP2
    #[inline(always)]
    pub fn mstcmp2(&self) -> MSTCMP2_R {
        MSTCMP2_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - MSTCMP3
    #[inline(always)]
    pub fn mstcmp3(&self) -> MSTCMP3_R {
        MSTCMP3_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - MSTCMP4
    #[inline(always)]
    pub fn mstcmp4(&self) -> MSTCMP4_R {
        MSTCMP4_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - TARST
    #[inline(always)]
    pub fn tarst(&self) -> TARST_R {
        TARST_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - TAREP
    #[inline(always)]
    pub fn tarep(&self) -> TAREP_R {
        TAREP_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - TACMP1
    #[inline(always)]
    pub fn tacmp1(&self) -> TACMP1_R {
        TACMP1_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - TACMP2
    #[inline(always)]
    pub fn tacmp2(&self) -> TACMP2_R {
        TACMP2_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - TBRST
    #[inline(always)]
    pub fn tbrst(&self) -> TBRST_R {
        TBRST_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - TBREP
    #[inline(always)]
    pub fn tbrep(&self) -> TBREP_R {
        TBREP_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - TBCMP1
    #[inline(always)]
    pub fn tbcmp1(&self) -> TBCMP1_R {
        TBCMP1_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - TBCMP2
    #[inline(always)]
    pub fn tbcmp2(&self) -> TBCMP2_R {
        TBCMP2_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - TCRST
    #[inline(always)]
    pub fn tcrst(&self) -> TCRST_R {
        TCRST_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - TCREP
    #[inline(always)]
    pub fn tcrep(&self) -> TCREP_R {
        TCREP_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - TCCMP1
    #[inline(always)]
    pub fn tccmp1(&self) -> TCCMP1_R {
        TCCMP1_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - TCCMP2
    #[inline(always)]
    pub fn tccmp2(&self) -> TCCMP2_R {
        TCCMP2_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - TDRST
    #[inline(always)]
    pub fn tdrst(&self) -> TDRST_R {
        TDRST_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - TDREP
    #[inline(always)]
    pub fn tdrep(&self) -> TDREP_R {
        TDREP_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - TDCMP1
    #[inline(always)]
    pub fn tdcmp1(&self) -> TDCMP1_R {
        TDCMP1_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - TDCMP2
    #[inline(always)]
    pub fn tdcmp2(&self) -> TDCMP2_R {
        TDCMP2_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - TERST
    #[inline(always)]
    pub fn terst(&self) -> TERST_R {
        TERST_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - TEREP
    #[inline(always)]
    pub fn terep(&self) -> TEREP_R {
        TEREP_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - TECMP1
    #[inline(always)]
    pub fn tecmp1(&self) -> TECMP1_R {
        TECMP1_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - TECMP2
    #[inline(always)]
    pub fn tecmp2(&self) -> TECMP2_R {
        TECMP2_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 31 - OCHPEV
    #[inline(always)]
    pub fn ochpev(&self) -> OCHPEV_R {
        OCHPEV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - SW
    #[inline(always)]
    #[must_use]
    pub fn sw(&mut self) -> SW_W<0> {
        SW_W::new(self)
    }
    ///Bit 1 - MSTRST
    #[inline(always)]
    #[must_use]
    pub fn mstrst(&mut self) -> MSTRST_W<1> {
        MSTRST_W::new(self)
    }
    ///Bit 2 - MSTREP
    #[inline(always)]
    #[must_use]
    pub fn mstrep(&mut self) -> MSTREP_W<2> {
        MSTREP_W::new(self)
    }
    ///Bit 3 - MSTCMP1
    #[inline(always)]
    #[must_use]
    pub fn mstcmp1(&mut self) -> MSTCMP1_W<3> {
        MSTCMP1_W::new(self)
    }
    ///Bit 4 - MSTCMP2
    #[inline(always)]
    #[must_use]
    pub fn mstcmp2(&mut self) -> MSTCMP2_W<4> {
        MSTCMP2_W::new(self)
    }
    ///Bit 5 - MSTCMP3
    #[inline(always)]
    #[must_use]
    pub fn mstcmp3(&mut self) -> MSTCMP3_W<5> {
        MSTCMP3_W::new(self)
    }
    ///Bit 6 - MSTCMP4
    #[inline(always)]
    #[must_use]
    pub fn mstcmp4(&mut self) -> MSTCMP4_W<6> {
        MSTCMP4_W::new(self)
    }
    ///Bit 7 - TARST
    #[inline(always)]
    #[must_use]
    pub fn tarst(&mut self) -> TARST_W<7> {
        TARST_W::new(self)
    }
    ///Bit 8 - TAREP
    #[inline(always)]
    #[must_use]
    pub fn tarep(&mut self) -> TAREP_W<8> {
        TAREP_W::new(self)
    }
    ///Bit 9 - TACMP1
    #[inline(always)]
    #[must_use]
    pub fn tacmp1(&mut self) -> TACMP1_W<9> {
        TACMP1_W::new(self)
    }
    ///Bit 10 - TACMP2
    #[inline(always)]
    #[must_use]
    pub fn tacmp2(&mut self) -> TACMP2_W<10> {
        TACMP2_W::new(self)
    }
    ///Bit 11 - TBRST
    #[inline(always)]
    #[must_use]
    pub fn tbrst(&mut self) -> TBRST_W<11> {
        TBRST_W::new(self)
    }
    ///Bit 12 - TBREP
    #[inline(always)]
    #[must_use]
    pub fn tbrep(&mut self) -> TBREP_W<12> {
        TBREP_W::new(self)
    }
    ///Bit 13 - TBCMP1
    #[inline(always)]
    #[must_use]
    pub fn tbcmp1(&mut self) -> TBCMP1_W<13> {
        TBCMP1_W::new(self)
    }
    ///Bit 14 - TBCMP2
    #[inline(always)]
    #[must_use]
    pub fn tbcmp2(&mut self) -> TBCMP2_W<14> {
        TBCMP2_W::new(self)
    }
    ///Bit 15 - TCRST
    #[inline(always)]
    #[must_use]
    pub fn tcrst(&mut self) -> TCRST_W<15> {
        TCRST_W::new(self)
    }
    ///Bit 16 - TCREP
    #[inline(always)]
    #[must_use]
    pub fn tcrep(&mut self) -> TCREP_W<16> {
        TCREP_W::new(self)
    }
    ///Bit 17 - TCCMP1
    #[inline(always)]
    #[must_use]
    pub fn tccmp1(&mut self) -> TCCMP1_W<17> {
        TCCMP1_W::new(self)
    }
    ///Bit 18 - TCCMP2
    #[inline(always)]
    #[must_use]
    pub fn tccmp2(&mut self) -> TCCMP2_W<18> {
        TCCMP2_W::new(self)
    }
    ///Bit 19 - TDRST
    #[inline(always)]
    #[must_use]
    pub fn tdrst(&mut self) -> TDRST_W<19> {
        TDRST_W::new(self)
    }
    ///Bit 20 - TDREP
    #[inline(always)]
    #[must_use]
    pub fn tdrep(&mut self) -> TDREP_W<20> {
        TDREP_W::new(self)
    }
    ///Bit 21 - TDCMP1
    #[inline(always)]
    #[must_use]
    pub fn tdcmp1(&mut self) -> TDCMP1_W<21> {
        TDCMP1_W::new(self)
    }
    ///Bit 22 - TDCMP2
    #[inline(always)]
    #[must_use]
    pub fn tdcmp2(&mut self) -> TDCMP2_W<22> {
        TDCMP2_W::new(self)
    }
    ///Bit 23 - TERST
    #[inline(always)]
    #[must_use]
    pub fn terst(&mut self) -> TERST_W<23> {
        TERST_W::new(self)
    }
    ///Bit 24 - TEREP
    #[inline(always)]
    #[must_use]
    pub fn terep(&mut self) -> TEREP_W<24> {
        TEREP_W::new(self)
    }
    ///Bit 25 - TECMP1
    #[inline(always)]
    #[must_use]
    pub fn tecmp1(&mut self) -> TECMP1_W<25> {
        TECMP1_W::new(self)
    }
    ///Bit 26 - TECMP2
    #[inline(always)]
    #[must_use]
    pub fn tecmp2(&mut self) -> TECMP2_W<26> {
        TECMP2_W::new(self)
    }
    ///Bit 31 - OCHPEV
    #[inline(always)]
    #[must_use]
    pub fn ochpev(&mut self) -> OCHPEV_W<31> {
        OCHPEV_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///BMTRG
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [bmtrg](index.html) module
pub struct BMTRG_SPEC;
impl crate::RegisterSpec for BMTRG_SPEC {
    type Ux = u32;
}
///`read()` method returns [bmtrg::R](R) reader structure
impl crate::Readable for BMTRG_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [bmtrg::W](W) writer structure
impl crate::Writable for BMTRG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets BMTRG to value 0
impl crate::Resettable for BMTRG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
