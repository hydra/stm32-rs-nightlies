///Register `NDAT2` reader
pub struct R(crate::R<NDAT2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NDAT2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NDAT2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NDAT2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `NDAT2` writer
pub struct W(crate::W<NDAT2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NDAT2_SPEC>;
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
impl From<crate::W<NDAT2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NDAT2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ND32` reader - New data
pub type ND32_R = crate::BitReader<bool>;
///Field `ND32` writer - New data
pub type ND32_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT2_SPEC, bool, O>;
///Field `ND33` reader - New data
pub type ND33_R = crate::BitReader<bool>;
///Field `ND33` writer - New data
pub type ND33_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT2_SPEC, bool, O>;
///Field `ND34` reader - New data
pub type ND34_R = crate::BitReader<bool>;
///Field `ND34` writer - New data
pub type ND34_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT2_SPEC, bool, O>;
///Field `ND35` reader - New data
pub type ND35_R = crate::BitReader<bool>;
///Field `ND35` writer - New data
pub type ND35_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT2_SPEC, bool, O>;
///Field `ND36` reader - New data
pub type ND36_R = crate::BitReader<bool>;
///Field `ND36` writer - New data
pub type ND36_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT2_SPEC, bool, O>;
///Field `ND37` reader - New data
pub type ND37_R = crate::BitReader<bool>;
///Field `ND37` writer - New data
pub type ND37_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT2_SPEC, bool, O>;
///Field `ND38` reader - New data
pub type ND38_R = crate::BitReader<bool>;
///Field `ND38` writer - New data
pub type ND38_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT2_SPEC, bool, O>;
///Field `ND39` reader - New data
pub type ND39_R = crate::BitReader<bool>;
///Field `ND39` writer - New data
pub type ND39_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT2_SPEC, bool, O>;
///Field `ND40` reader - New data
pub type ND40_R = crate::BitReader<bool>;
///Field `ND40` writer - New data
pub type ND40_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT2_SPEC, bool, O>;
///Field `ND41` reader - New data
pub type ND41_R = crate::BitReader<bool>;
///Field `ND41` writer - New data
pub type ND41_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT2_SPEC, bool, O>;
///Field `ND42` reader - New data
pub type ND42_R = crate::BitReader<bool>;
///Field `ND42` writer - New data
pub type ND42_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT2_SPEC, bool, O>;
///Field `ND43` reader - New data
pub type ND43_R = crate::BitReader<bool>;
///Field `ND43` writer - New data
pub type ND43_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT2_SPEC, bool, O>;
///Field `ND44` reader - New data
pub type ND44_R = crate::BitReader<bool>;
///Field `ND44` writer - New data
pub type ND44_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT2_SPEC, bool, O>;
///Field `ND45` reader - New data
pub type ND45_R = crate::BitReader<bool>;
///Field `ND45` writer - New data
pub type ND45_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT2_SPEC, bool, O>;
///Field `ND46` reader - New data
pub type ND46_R = crate::BitReader<bool>;
///Field `ND46` writer - New data
pub type ND46_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT2_SPEC, bool, O>;
///Field `ND47` reader - New data
pub type ND47_R = crate::BitReader<bool>;
///Field `ND47` writer - New data
pub type ND47_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT2_SPEC, bool, O>;
///Field `ND48` reader - New data
pub type ND48_R = crate::BitReader<bool>;
///Field `ND48` writer - New data
pub type ND48_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT2_SPEC, bool, O>;
///Field `ND49` reader - New data
pub type ND49_R = crate::BitReader<bool>;
///Field `ND49` writer - New data
pub type ND49_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT2_SPEC, bool, O>;
///Field `ND50` reader - New data
pub type ND50_R = crate::BitReader<bool>;
///Field `ND50` writer - New data
pub type ND50_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT2_SPEC, bool, O>;
///Field `ND51` reader - New data
pub type ND51_R = crate::BitReader<bool>;
///Field `ND51` writer - New data
pub type ND51_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT2_SPEC, bool, O>;
///Field `ND52` reader - New data
pub type ND52_R = crate::BitReader<bool>;
///Field `ND52` writer - New data
pub type ND52_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT2_SPEC, bool, O>;
///Field `ND53` reader - New data
pub type ND53_R = crate::BitReader<bool>;
///Field `ND53` writer - New data
pub type ND53_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT2_SPEC, bool, O>;
///Field `ND54` reader - New data
pub type ND54_R = crate::BitReader<bool>;
///Field `ND54` writer - New data
pub type ND54_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT2_SPEC, bool, O>;
///Field `ND55` reader - New data
pub type ND55_R = crate::BitReader<bool>;
///Field `ND55` writer - New data
pub type ND55_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT2_SPEC, bool, O>;
///Field `ND56` reader - New data
pub type ND56_R = crate::BitReader<bool>;
///Field `ND56` writer - New data
pub type ND56_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT2_SPEC, bool, O>;
///Field `ND57` reader - New data
pub type ND57_R = crate::BitReader<bool>;
///Field `ND57` writer - New data
pub type ND57_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT2_SPEC, bool, O>;
///Field `ND58` reader - New data
pub type ND58_R = crate::BitReader<bool>;
///Field `ND58` writer - New data
pub type ND58_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT2_SPEC, bool, O>;
///Field `ND59` reader - New data
pub type ND59_R = crate::BitReader<bool>;
///Field `ND59` writer - New data
pub type ND59_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT2_SPEC, bool, O>;
///Field `ND60` reader - New data
pub type ND60_R = crate::BitReader<bool>;
///Field `ND60` writer - New data
pub type ND60_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT2_SPEC, bool, O>;
///Field `ND61` reader - New data
pub type ND61_R = crate::BitReader<bool>;
///Field `ND61` writer - New data
pub type ND61_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT2_SPEC, bool, O>;
///Field `ND62` reader - New data
pub type ND62_R = crate::BitReader<bool>;
///Field `ND62` writer - New data
pub type ND62_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT2_SPEC, bool, O>;
///Field `ND63` reader - New data
pub type ND63_R = crate::BitReader<bool>;
///Field `ND63` writer - New data
pub type ND63_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT2_SPEC, bool, O>;
impl R {
    ///Bit 0 - New data
    #[inline(always)]
    pub fn nd32(&self) -> ND32_R {
        ND32_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - New data
    #[inline(always)]
    pub fn nd33(&self) -> ND33_R {
        ND33_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - New data
    #[inline(always)]
    pub fn nd34(&self) -> ND34_R {
        ND34_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - New data
    #[inline(always)]
    pub fn nd35(&self) -> ND35_R {
        ND35_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - New data
    #[inline(always)]
    pub fn nd36(&self) -> ND36_R {
        ND36_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - New data
    #[inline(always)]
    pub fn nd37(&self) -> ND37_R {
        ND37_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - New data
    #[inline(always)]
    pub fn nd38(&self) -> ND38_R {
        ND38_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - New data
    #[inline(always)]
    pub fn nd39(&self) -> ND39_R {
        ND39_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - New data
    #[inline(always)]
    pub fn nd40(&self) -> ND40_R {
        ND40_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - New data
    #[inline(always)]
    pub fn nd41(&self) -> ND41_R {
        ND41_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - New data
    #[inline(always)]
    pub fn nd42(&self) -> ND42_R {
        ND42_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - New data
    #[inline(always)]
    pub fn nd43(&self) -> ND43_R {
        ND43_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - New data
    #[inline(always)]
    pub fn nd44(&self) -> ND44_R {
        ND44_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - New data
    #[inline(always)]
    pub fn nd45(&self) -> ND45_R {
        ND45_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - New data
    #[inline(always)]
    pub fn nd46(&self) -> ND46_R {
        ND46_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - New data
    #[inline(always)]
    pub fn nd47(&self) -> ND47_R {
        ND47_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - New data
    #[inline(always)]
    pub fn nd48(&self) -> ND48_R {
        ND48_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - New data
    #[inline(always)]
    pub fn nd49(&self) -> ND49_R {
        ND49_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - New data
    #[inline(always)]
    pub fn nd50(&self) -> ND50_R {
        ND50_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - New data
    #[inline(always)]
    pub fn nd51(&self) -> ND51_R {
        ND51_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - New data
    #[inline(always)]
    pub fn nd52(&self) -> ND52_R {
        ND52_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - New data
    #[inline(always)]
    pub fn nd53(&self) -> ND53_R {
        ND53_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - New data
    #[inline(always)]
    pub fn nd54(&self) -> ND54_R {
        ND54_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - New data
    #[inline(always)]
    pub fn nd55(&self) -> ND55_R {
        ND55_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - New data
    #[inline(always)]
    pub fn nd56(&self) -> ND56_R {
        ND56_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - New data
    #[inline(always)]
    pub fn nd57(&self) -> ND57_R {
        ND57_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - New data
    #[inline(always)]
    pub fn nd58(&self) -> ND58_R {
        ND58_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - New data
    #[inline(always)]
    pub fn nd59(&self) -> ND59_R {
        ND59_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - New data
    #[inline(always)]
    pub fn nd60(&self) -> ND60_R {
        ND60_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - New data
    #[inline(always)]
    pub fn nd61(&self) -> ND61_R {
        ND61_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - New data
    #[inline(always)]
    pub fn nd62(&self) -> ND62_R {
        ND62_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - New data
    #[inline(always)]
    pub fn nd63(&self) -> ND63_R {
        ND63_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - New data
    #[inline(always)]
    #[must_use]
    pub fn nd32(&mut self) -> ND32_W<0> {
        ND32_W::new(self)
    }
    ///Bit 1 - New data
    #[inline(always)]
    #[must_use]
    pub fn nd33(&mut self) -> ND33_W<1> {
        ND33_W::new(self)
    }
    ///Bit 2 - New data
    #[inline(always)]
    #[must_use]
    pub fn nd34(&mut self) -> ND34_W<2> {
        ND34_W::new(self)
    }
    ///Bit 3 - New data
    #[inline(always)]
    #[must_use]
    pub fn nd35(&mut self) -> ND35_W<3> {
        ND35_W::new(self)
    }
    ///Bit 4 - New data
    #[inline(always)]
    #[must_use]
    pub fn nd36(&mut self) -> ND36_W<4> {
        ND36_W::new(self)
    }
    ///Bit 5 - New data
    #[inline(always)]
    #[must_use]
    pub fn nd37(&mut self) -> ND37_W<5> {
        ND37_W::new(self)
    }
    ///Bit 6 - New data
    #[inline(always)]
    #[must_use]
    pub fn nd38(&mut self) -> ND38_W<6> {
        ND38_W::new(self)
    }
    ///Bit 7 - New data
    #[inline(always)]
    #[must_use]
    pub fn nd39(&mut self) -> ND39_W<7> {
        ND39_W::new(self)
    }
    ///Bit 8 - New data
    #[inline(always)]
    #[must_use]
    pub fn nd40(&mut self) -> ND40_W<8> {
        ND40_W::new(self)
    }
    ///Bit 9 - New data
    #[inline(always)]
    #[must_use]
    pub fn nd41(&mut self) -> ND41_W<9> {
        ND41_W::new(self)
    }
    ///Bit 10 - New data
    #[inline(always)]
    #[must_use]
    pub fn nd42(&mut self) -> ND42_W<10> {
        ND42_W::new(self)
    }
    ///Bit 11 - New data
    #[inline(always)]
    #[must_use]
    pub fn nd43(&mut self) -> ND43_W<11> {
        ND43_W::new(self)
    }
    ///Bit 12 - New data
    #[inline(always)]
    #[must_use]
    pub fn nd44(&mut self) -> ND44_W<12> {
        ND44_W::new(self)
    }
    ///Bit 13 - New data
    #[inline(always)]
    #[must_use]
    pub fn nd45(&mut self) -> ND45_W<13> {
        ND45_W::new(self)
    }
    ///Bit 14 - New data
    #[inline(always)]
    #[must_use]
    pub fn nd46(&mut self) -> ND46_W<14> {
        ND46_W::new(self)
    }
    ///Bit 15 - New data
    #[inline(always)]
    #[must_use]
    pub fn nd47(&mut self) -> ND47_W<15> {
        ND47_W::new(self)
    }
    ///Bit 16 - New data
    #[inline(always)]
    #[must_use]
    pub fn nd48(&mut self) -> ND48_W<16> {
        ND48_W::new(self)
    }
    ///Bit 17 - New data
    #[inline(always)]
    #[must_use]
    pub fn nd49(&mut self) -> ND49_W<17> {
        ND49_W::new(self)
    }
    ///Bit 18 - New data
    #[inline(always)]
    #[must_use]
    pub fn nd50(&mut self) -> ND50_W<18> {
        ND50_W::new(self)
    }
    ///Bit 19 - New data
    #[inline(always)]
    #[must_use]
    pub fn nd51(&mut self) -> ND51_W<19> {
        ND51_W::new(self)
    }
    ///Bit 20 - New data
    #[inline(always)]
    #[must_use]
    pub fn nd52(&mut self) -> ND52_W<20> {
        ND52_W::new(self)
    }
    ///Bit 21 - New data
    #[inline(always)]
    #[must_use]
    pub fn nd53(&mut self) -> ND53_W<21> {
        ND53_W::new(self)
    }
    ///Bit 22 - New data
    #[inline(always)]
    #[must_use]
    pub fn nd54(&mut self) -> ND54_W<22> {
        ND54_W::new(self)
    }
    ///Bit 23 - New data
    #[inline(always)]
    #[must_use]
    pub fn nd55(&mut self) -> ND55_W<23> {
        ND55_W::new(self)
    }
    ///Bit 24 - New data
    #[inline(always)]
    #[must_use]
    pub fn nd56(&mut self) -> ND56_W<24> {
        ND56_W::new(self)
    }
    ///Bit 25 - New data
    #[inline(always)]
    #[must_use]
    pub fn nd57(&mut self) -> ND57_W<25> {
        ND57_W::new(self)
    }
    ///Bit 26 - New data
    #[inline(always)]
    #[must_use]
    pub fn nd58(&mut self) -> ND58_W<26> {
        ND58_W::new(self)
    }
    ///Bit 27 - New data
    #[inline(always)]
    #[must_use]
    pub fn nd59(&mut self) -> ND59_W<27> {
        ND59_W::new(self)
    }
    ///Bit 28 - New data
    #[inline(always)]
    #[must_use]
    pub fn nd60(&mut self) -> ND60_W<28> {
        ND60_W::new(self)
    }
    ///Bit 29 - New data
    #[inline(always)]
    #[must_use]
    pub fn nd61(&mut self) -> ND61_W<29> {
        ND61_W::new(self)
    }
    ///Bit 30 - New data
    #[inline(always)]
    #[must_use]
    pub fn nd62(&mut self) -> ND62_W<30> {
        ND62_W::new(self)
    }
    ///Bit 31 - New data
    #[inline(always)]
    #[must_use]
    pub fn nd63(&mut self) -> ND63_W<31> {
        ND63_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FDCAN New Data 2 Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ndat2](index.html) module
pub struct NDAT2_SPEC;
impl crate::RegisterSpec for NDAT2_SPEC {
    type Ux = u32;
}
///`read()` method returns [ndat2::R](R) reader structure
impl crate::Readable for NDAT2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ndat2::W](W) writer structure
impl crate::Writable for NDAT2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets NDAT2 to value 0
impl crate::Resettable for NDAT2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
