///Register `MPCBB1_SECCFGR1` reader
pub struct R(crate::R<MPCBB1_SECCFGR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPCBB1_SECCFGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPCBB1_SECCFGR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPCBB1_SECCFGR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MPCBB1_SECCFGR1` writer
pub struct W(crate::W<MPCBB1_SECCFGR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPCBB1_SECCFGR1_SPEC>;
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
impl From<crate::W<MPCBB1_SECCFGR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPCBB1_SECCFGR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SEC0` reader - SEC0
pub type SEC0_R = crate::BitReader<bool>;
///Field `SEC0` writer - SEC0
pub type SEC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_SECCFGR1_SPEC, bool, O>;
///Field `SEC1` reader - SEC1
pub type SEC1_R = crate::BitReader<bool>;
///Field `SEC1` writer - SEC1
pub type SEC1_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_SECCFGR1_SPEC, bool, O>;
///Field `SEC2` reader - SEC2
pub type SEC2_R = crate::BitReader<bool>;
///Field `SEC2` writer - SEC2
pub type SEC2_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_SECCFGR1_SPEC, bool, O>;
///Field `SEC3` reader - SEC3
pub type SEC3_R = crate::BitReader<bool>;
///Field `SEC3` writer - SEC3
pub type SEC3_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_SECCFGR1_SPEC, bool, O>;
///Field `SEC4` reader - SEC4
pub type SEC4_R = crate::BitReader<bool>;
///Field `SEC4` writer - SEC4
pub type SEC4_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_SECCFGR1_SPEC, bool, O>;
///Field `SEC5` reader - SEC5
pub type SEC5_R = crate::BitReader<bool>;
///Field `SEC5` writer - SEC5
pub type SEC5_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_SECCFGR1_SPEC, bool, O>;
///Field `SEC6` reader - SEC6
pub type SEC6_R = crate::BitReader<bool>;
///Field `SEC6` writer - SEC6
pub type SEC6_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_SECCFGR1_SPEC, bool, O>;
///Field `SEC7` reader - SEC7
pub type SEC7_R = crate::BitReader<bool>;
///Field `SEC7` writer - SEC7
pub type SEC7_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_SECCFGR1_SPEC, bool, O>;
///Field `SEC8` reader - SEC8
pub type SEC8_R = crate::BitReader<bool>;
///Field `SEC8` writer - SEC8
pub type SEC8_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_SECCFGR1_SPEC, bool, O>;
///Field `SEC9` reader - SEC9
pub type SEC9_R = crate::BitReader<bool>;
///Field `SEC9` writer - SEC9
pub type SEC9_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_SECCFGR1_SPEC, bool, O>;
///Field `SEC10` reader - SEC10
pub type SEC10_R = crate::BitReader<bool>;
///Field `SEC10` writer - SEC10
pub type SEC10_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_SECCFGR1_SPEC, bool, O>;
///Field `SEC11` reader - SEC11
pub type SEC11_R = crate::BitReader<bool>;
///Field `SEC11` writer - SEC11
pub type SEC11_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_SECCFGR1_SPEC, bool, O>;
///Field `SEC12` reader - SEC12
pub type SEC12_R = crate::BitReader<bool>;
///Field `SEC12` writer - SEC12
pub type SEC12_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_SECCFGR1_SPEC, bool, O>;
///Field `SEC13` reader - SEC13
pub type SEC13_R = crate::BitReader<bool>;
///Field `SEC13` writer - SEC13
pub type SEC13_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_SECCFGR1_SPEC, bool, O>;
///Field `SEC14` reader - SEC14
pub type SEC14_R = crate::BitReader<bool>;
///Field `SEC14` writer - SEC14
pub type SEC14_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_SECCFGR1_SPEC, bool, O>;
///Field `SEC15` reader - SEC15
pub type SEC15_R = crate::BitReader<bool>;
///Field `SEC15` writer - SEC15
pub type SEC15_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_SECCFGR1_SPEC, bool, O>;
///Field `SEC16` reader - SEC16
pub type SEC16_R = crate::BitReader<bool>;
///Field `SEC16` writer - SEC16
pub type SEC16_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_SECCFGR1_SPEC, bool, O>;
///Field `SEC17` reader - SEC17
pub type SEC17_R = crate::BitReader<bool>;
///Field `SEC17` writer - SEC17
pub type SEC17_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_SECCFGR1_SPEC, bool, O>;
///Field `SEC18` reader - SEC18
pub type SEC18_R = crate::BitReader<bool>;
///Field `SEC18` writer - SEC18
pub type SEC18_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_SECCFGR1_SPEC, bool, O>;
///Field `SEC19` reader - SEC19
pub type SEC19_R = crate::BitReader<bool>;
///Field `SEC19` writer - SEC19
pub type SEC19_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_SECCFGR1_SPEC, bool, O>;
///Field `SEC20` reader - SEC20
pub type SEC20_R = crate::BitReader<bool>;
///Field `SEC20` writer - SEC20
pub type SEC20_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_SECCFGR1_SPEC, bool, O>;
///Field `SEC21` reader - SEC21
pub type SEC21_R = crate::BitReader<bool>;
///Field `SEC21` writer - SEC21
pub type SEC21_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_SECCFGR1_SPEC, bool, O>;
///Field `SEC22` reader - SEC22
pub type SEC22_R = crate::BitReader<bool>;
///Field `SEC22` writer - SEC22
pub type SEC22_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_SECCFGR1_SPEC, bool, O>;
///Field `SEC23` reader - SEC23
pub type SEC23_R = crate::BitReader<bool>;
///Field `SEC23` writer - SEC23
pub type SEC23_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_SECCFGR1_SPEC, bool, O>;
///Field `SEC24` reader - SEC24
pub type SEC24_R = crate::BitReader<bool>;
///Field `SEC24` writer - SEC24
pub type SEC24_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_SECCFGR1_SPEC, bool, O>;
///Field `SEC25` reader - SEC25
pub type SEC25_R = crate::BitReader<bool>;
///Field `SEC25` writer - SEC25
pub type SEC25_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_SECCFGR1_SPEC, bool, O>;
///Field `SEC26` reader - SEC26
pub type SEC26_R = crate::BitReader<bool>;
///Field `SEC26` writer - SEC26
pub type SEC26_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_SECCFGR1_SPEC, bool, O>;
///Field `SEC27` reader - SEC27
pub type SEC27_R = crate::BitReader<bool>;
///Field `SEC27` writer - SEC27
pub type SEC27_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_SECCFGR1_SPEC, bool, O>;
///Field `SEC28` reader - SEC28
pub type SEC28_R = crate::BitReader<bool>;
///Field `SEC28` writer - SEC28
pub type SEC28_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_SECCFGR1_SPEC, bool, O>;
///Field `SEC29` reader - SEC29
pub type SEC29_R = crate::BitReader<bool>;
///Field `SEC29` writer - SEC29
pub type SEC29_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_SECCFGR1_SPEC, bool, O>;
///Field `SEC30` reader - SEC30
pub type SEC30_R = crate::BitReader<bool>;
///Field `SEC30` writer - SEC30
pub type SEC30_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_SECCFGR1_SPEC, bool, O>;
///Field `SEC31` reader - SEC31
pub type SEC31_R = crate::BitReader<bool>;
///Field `SEC31` writer - SEC31
pub type SEC31_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_SECCFGR1_SPEC, bool, O>;
impl R {
    ///Bit 0 - SEC0
    #[inline(always)]
    pub fn sec0(&self) -> SEC0_R {
        SEC0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - SEC1
    #[inline(always)]
    pub fn sec1(&self) -> SEC1_R {
        SEC1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - SEC2
    #[inline(always)]
    pub fn sec2(&self) -> SEC2_R {
        SEC2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - SEC3
    #[inline(always)]
    pub fn sec3(&self) -> SEC3_R {
        SEC3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - SEC4
    #[inline(always)]
    pub fn sec4(&self) -> SEC4_R {
        SEC4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - SEC5
    #[inline(always)]
    pub fn sec5(&self) -> SEC5_R {
        SEC5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - SEC6
    #[inline(always)]
    pub fn sec6(&self) -> SEC6_R {
        SEC6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - SEC7
    #[inline(always)]
    pub fn sec7(&self) -> SEC7_R {
        SEC7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - SEC8
    #[inline(always)]
    pub fn sec8(&self) -> SEC8_R {
        SEC8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - SEC9
    #[inline(always)]
    pub fn sec9(&self) -> SEC9_R {
        SEC9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - SEC10
    #[inline(always)]
    pub fn sec10(&self) -> SEC10_R {
        SEC10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - SEC11
    #[inline(always)]
    pub fn sec11(&self) -> SEC11_R {
        SEC11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - SEC12
    #[inline(always)]
    pub fn sec12(&self) -> SEC12_R {
        SEC12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - SEC13
    #[inline(always)]
    pub fn sec13(&self) -> SEC13_R {
        SEC13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - SEC14
    #[inline(always)]
    pub fn sec14(&self) -> SEC14_R {
        SEC14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - SEC15
    #[inline(always)]
    pub fn sec15(&self) -> SEC15_R {
        SEC15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - SEC16
    #[inline(always)]
    pub fn sec16(&self) -> SEC16_R {
        SEC16_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - SEC17
    #[inline(always)]
    pub fn sec17(&self) -> SEC17_R {
        SEC17_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - SEC18
    #[inline(always)]
    pub fn sec18(&self) -> SEC18_R {
        SEC18_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - SEC19
    #[inline(always)]
    pub fn sec19(&self) -> SEC19_R {
        SEC19_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - SEC20
    #[inline(always)]
    pub fn sec20(&self) -> SEC20_R {
        SEC20_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - SEC21
    #[inline(always)]
    pub fn sec21(&self) -> SEC21_R {
        SEC21_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - SEC22
    #[inline(always)]
    pub fn sec22(&self) -> SEC22_R {
        SEC22_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - SEC23
    #[inline(always)]
    pub fn sec23(&self) -> SEC23_R {
        SEC23_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - SEC24
    #[inline(always)]
    pub fn sec24(&self) -> SEC24_R {
        SEC24_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - SEC25
    #[inline(always)]
    pub fn sec25(&self) -> SEC25_R {
        SEC25_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - SEC26
    #[inline(always)]
    pub fn sec26(&self) -> SEC26_R {
        SEC26_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - SEC27
    #[inline(always)]
    pub fn sec27(&self) -> SEC27_R {
        SEC27_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - SEC28
    #[inline(always)]
    pub fn sec28(&self) -> SEC28_R {
        SEC28_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - SEC29
    #[inline(always)]
    pub fn sec29(&self) -> SEC29_R {
        SEC29_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - SEC30
    #[inline(always)]
    pub fn sec30(&self) -> SEC30_R {
        SEC30_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - SEC31
    #[inline(always)]
    pub fn sec31(&self) -> SEC31_R {
        SEC31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - SEC0
    #[inline(always)]
    #[must_use]
    pub fn sec0(&mut self) -> SEC0_W<0> {
        SEC0_W::new(self)
    }
    ///Bit 1 - SEC1
    #[inline(always)]
    #[must_use]
    pub fn sec1(&mut self) -> SEC1_W<1> {
        SEC1_W::new(self)
    }
    ///Bit 2 - SEC2
    #[inline(always)]
    #[must_use]
    pub fn sec2(&mut self) -> SEC2_W<2> {
        SEC2_W::new(self)
    }
    ///Bit 3 - SEC3
    #[inline(always)]
    #[must_use]
    pub fn sec3(&mut self) -> SEC3_W<3> {
        SEC3_W::new(self)
    }
    ///Bit 4 - SEC4
    #[inline(always)]
    #[must_use]
    pub fn sec4(&mut self) -> SEC4_W<4> {
        SEC4_W::new(self)
    }
    ///Bit 5 - SEC5
    #[inline(always)]
    #[must_use]
    pub fn sec5(&mut self) -> SEC5_W<5> {
        SEC5_W::new(self)
    }
    ///Bit 6 - SEC6
    #[inline(always)]
    #[must_use]
    pub fn sec6(&mut self) -> SEC6_W<6> {
        SEC6_W::new(self)
    }
    ///Bit 7 - SEC7
    #[inline(always)]
    #[must_use]
    pub fn sec7(&mut self) -> SEC7_W<7> {
        SEC7_W::new(self)
    }
    ///Bit 8 - SEC8
    #[inline(always)]
    #[must_use]
    pub fn sec8(&mut self) -> SEC8_W<8> {
        SEC8_W::new(self)
    }
    ///Bit 9 - SEC9
    #[inline(always)]
    #[must_use]
    pub fn sec9(&mut self) -> SEC9_W<9> {
        SEC9_W::new(self)
    }
    ///Bit 10 - SEC10
    #[inline(always)]
    #[must_use]
    pub fn sec10(&mut self) -> SEC10_W<10> {
        SEC10_W::new(self)
    }
    ///Bit 11 - SEC11
    #[inline(always)]
    #[must_use]
    pub fn sec11(&mut self) -> SEC11_W<11> {
        SEC11_W::new(self)
    }
    ///Bit 12 - SEC12
    #[inline(always)]
    #[must_use]
    pub fn sec12(&mut self) -> SEC12_W<12> {
        SEC12_W::new(self)
    }
    ///Bit 13 - SEC13
    #[inline(always)]
    #[must_use]
    pub fn sec13(&mut self) -> SEC13_W<13> {
        SEC13_W::new(self)
    }
    ///Bit 14 - SEC14
    #[inline(always)]
    #[must_use]
    pub fn sec14(&mut self) -> SEC14_W<14> {
        SEC14_W::new(self)
    }
    ///Bit 15 - SEC15
    #[inline(always)]
    #[must_use]
    pub fn sec15(&mut self) -> SEC15_W<15> {
        SEC15_W::new(self)
    }
    ///Bit 16 - SEC16
    #[inline(always)]
    #[must_use]
    pub fn sec16(&mut self) -> SEC16_W<16> {
        SEC16_W::new(self)
    }
    ///Bit 17 - SEC17
    #[inline(always)]
    #[must_use]
    pub fn sec17(&mut self) -> SEC17_W<17> {
        SEC17_W::new(self)
    }
    ///Bit 18 - SEC18
    #[inline(always)]
    #[must_use]
    pub fn sec18(&mut self) -> SEC18_W<18> {
        SEC18_W::new(self)
    }
    ///Bit 19 - SEC19
    #[inline(always)]
    #[must_use]
    pub fn sec19(&mut self) -> SEC19_W<19> {
        SEC19_W::new(self)
    }
    ///Bit 20 - SEC20
    #[inline(always)]
    #[must_use]
    pub fn sec20(&mut self) -> SEC20_W<20> {
        SEC20_W::new(self)
    }
    ///Bit 21 - SEC21
    #[inline(always)]
    #[must_use]
    pub fn sec21(&mut self) -> SEC21_W<21> {
        SEC21_W::new(self)
    }
    ///Bit 22 - SEC22
    #[inline(always)]
    #[must_use]
    pub fn sec22(&mut self) -> SEC22_W<22> {
        SEC22_W::new(self)
    }
    ///Bit 23 - SEC23
    #[inline(always)]
    #[must_use]
    pub fn sec23(&mut self) -> SEC23_W<23> {
        SEC23_W::new(self)
    }
    ///Bit 24 - SEC24
    #[inline(always)]
    #[must_use]
    pub fn sec24(&mut self) -> SEC24_W<24> {
        SEC24_W::new(self)
    }
    ///Bit 25 - SEC25
    #[inline(always)]
    #[must_use]
    pub fn sec25(&mut self) -> SEC25_W<25> {
        SEC25_W::new(self)
    }
    ///Bit 26 - SEC26
    #[inline(always)]
    #[must_use]
    pub fn sec26(&mut self) -> SEC26_W<26> {
        SEC26_W::new(self)
    }
    ///Bit 27 - SEC27
    #[inline(always)]
    #[must_use]
    pub fn sec27(&mut self) -> SEC27_W<27> {
        SEC27_W::new(self)
    }
    ///Bit 28 - SEC28
    #[inline(always)]
    #[must_use]
    pub fn sec28(&mut self) -> SEC28_W<28> {
        SEC28_W::new(self)
    }
    ///Bit 29 - SEC29
    #[inline(always)]
    #[must_use]
    pub fn sec29(&mut self) -> SEC29_W<29> {
        SEC29_W::new(self)
    }
    ///Bit 30 - SEC30
    #[inline(always)]
    #[must_use]
    pub fn sec30(&mut self) -> SEC30_W<30> {
        SEC30_W::new(self)
    }
    ///Bit 31 - SEC31
    #[inline(always)]
    #[must_use]
    pub fn sec31(&mut self) -> SEC31_W<31> {
        SEC31_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///MPCBBx security configuration for super-block x register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mpcbb1_seccfgr1](index.html) module
pub struct MPCBB1_SECCFGR1_SPEC;
impl crate::RegisterSpec for MPCBB1_SECCFGR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [mpcbb1_seccfgr1::R](R) reader structure
impl crate::Readable for MPCBB1_SECCFGR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [mpcbb1_seccfgr1::W](W) writer structure
impl crate::Writable for MPCBB1_SECCFGR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets MPCBB1_SECCFGR1 to value 0xffff_ffff
impl crate::Resettable for MPCBB1_SECCFGR1_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
