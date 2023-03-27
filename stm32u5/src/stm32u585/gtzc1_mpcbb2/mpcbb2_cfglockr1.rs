///Register `MPCBB2_CFGLOCKR1` reader
pub struct R(crate::R<MPCBB2_CFGLOCKR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPCBB2_CFGLOCKR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPCBB2_CFGLOCKR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPCBB2_CFGLOCKR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MPCBB2_CFGLOCKR1` writer
pub struct W(crate::W<MPCBB2_CFGLOCKR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPCBB2_CFGLOCKR1_SPEC>;
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
impl From<crate::W<MPCBB2_CFGLOCKR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPCBB2_CFGLOCKR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SPLCK0` reader - SPLCK0
pub type SPLCK0_R = crate::BitReader<bool>;
///Field `SPLCK0` writer - SPLCK0
pub type SPLCK0_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_CFGLOCKR1_SPEC, bool, O>;
///Field `SPLCK1` reader - SPLCK1
pub type SPLCK1_R = crate::BitReader<bool>;
///Field `SPLCK1` writer - SPLCK1
pub type SPLCK1_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_CFGLOCKR1_SPEC, bool, O>;
///Field `SPLCK2` reader - SPLCK2
pub type SPLCK2_R = crate::BitReader<bool>;
///Field `SPLCK2` writer - SPLCK2
pub type SPLCK2_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_CFGLOCKR1_SPEC, bool, O>;
///Field `SPLCK3` reader - SPLCK3
pub type SPLCK3_R = crate::BitReader<bool>;
///Field `SPLCK3` writer - SPLCK3
pub type SPLCK3_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_CFGLOCKR1_SPEC, bool, O>;
///Field `SPLCK4` reader - SPLCK4
pub type SPLCK4_R = crate::BitReader<bool>;
///Field `SPLCK4` writer - SPLCK4
pub type SPLCK4_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_CFGLOCKR1_SPEC, bool, O>;
///Field `SPLCK5` reader - SPLCK5
pub type SPLCK5_R = crate::BitReader<bool>;
///Field `SPLCK5` writer - SPLCK5
pub type SPLCK5_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_CFGLOCKR1_SPEC, bool, O>;
///Field `SPLCK6` reader - SPLCK6
pub type SPLCK6_R = crate::BitReader<bool>;
///Field `SPLCK6` writer - SPLCK6
pub type SPLCK6_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_CFGLOCKR1_SPEC, bool, O>;
///Field `SPLCK7` reader - SPLCK7
pub type SPLCK7_R = crate::BitReader<bool>;
///Field `SPLCK7` writer - SPLCK7
pub type SPLCK7_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_CFGLOCKR1_SPEC, bool, O>;
///Field `SPLCK8` reader - SPLCK8
pub type SPLCK8_R = crate::BitReader<bool>;
///Field `SPLCK8` writer - SPLCK8
pub type SPLCK8_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_CFGLOCKR1_SPEC, bool, O>;
///Field `SPLCK9` reader - SPLCK9
pub type SPLCK9_R = crate::BitReader<bool>;
///Field `SPLCK9` writer - SPLCK9
pub type SPLCK9_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_CFGLOCKR1_SPEC, bool, O>;
///Field `SPLCK10` reader - SPLCK10
pub type SPLCK10_R = crate::BitReader<bool>;
///Field `SPLCK10` writer - SPLCK10
pub type SPLCK10_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_CFGLOCKR1_SPEC, bool, O>;
///Field `SPLCK11` reader - SPLCK11
pub type SPLCK11_R = crate::BitReader<bool>;
///Field `SPLCK11` writer - SPLCK11
pub type SPLCK11_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_CFGLOCKR1_SPEC, bool, O>;
///Field `SPLCK12` reader - SPLCK12
pub type SPLCK12_R = crate::BitReader<bool>;
///Field `SPLCK12` writer - SPLCK12
pub type SPLCK12_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_CFGLOCKR1_SPEC, bool, O>;
///Field `SPLCK13` reader - SPLCK13
pub type SPLCK13_R = crate::BitReader<bool>;
///Field `SPLCK13` writer - SPLCK13
pub type SPLCK13_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_CFGLOCKR1_SPEC, bool, O>;
///Field `SPLCK14` reader - SPLCK14
pub type SPLCK14_R = crate::BitReader<bool>;
///Field `SPLCK14` writer - SPLCK14
pub type SPLCK14_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_CFGLOCKR1_SPEC, bool, O>;
///Field `SPLCK15` reader - SPLCK15
pub type SPLCK15_R = crate::BitReader<bool>;
///Field `SPLCK15` writer - SPLCK15
pub type SPLCK15_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_CFGLOCKR1_SPEC, bool, O>;
///Field `SPLCK16` reader - SPLCK16
pub type SPLCK16_R = crate::BitReader<bool>;
///Field `SPLCK16` writer - SPLCK16
pub type SPLCK16_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_CFGLOCKR1_SPEC, bool, O>;
///Field `SPLCK17` reader - SPLCK17
pub type SPLCK17_R = crate::BitReader<bool>;
///Field `SPLCK17` writer - SPLCK17
pub type SPLCK17_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_CFGLOCKR1_SPEC, bool, O>;
///Field `SPLCK18` reader - SPLCK18
pub type SPLCK18_R = crate::BitReader<bool>;
///Field `SPLCK18` writer - SPLCK18
pub type SPLCK18_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_CFGLOCKR1_SPEC, bool, O>;
///Field `SPLCK19` reader - SPLCK19
pub type SPLCK19_R = crate::BitReader<bool>;
///Field `SPLCK19` writer - SPLCK19
pub type SPLCK19_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_CFGLOCKR1_SPEC, bool, O>;
///Field `SPLCK20` reader - SPLCK20
pub type SPLCK20_R = crate::BitReader<bool>;
///Field `SPLCK20` writer - SPLCK20
pub type SPLCK20_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_CFGLOCKR1_SPEC, bool, O>;
///Field `SPLCK21` reader - SPLCK21
pub type SPLCK21_R = crate::BitReader<bool>;
///Field `SPLCK21` writer - SPLCK21
pub type SPLCK21_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_CFGLOCKR1_SPEC, bool, O>;
///Field `SPLCK22` reader - SPLCK22
pub type SPLCK22_R = crate::BitReader<bool>;
///Field `SPLCK22` writer - SPLCK22
pub type SPLCK22_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_CFGLOCKR1_SPEC, bool, O>;
///Field `SPLCK23` reader - SPLCK23
pub type SPLCK23_R = crate::BitReader<bool>;
///Field `SPLCK23` writer - SPLCK23
pub type SPLCK23_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_CFGLOCKR1_SPEC, bool, O>;
///Field `SPLCK24` reader - SPLCK24
pub type SPLCK24_R = crate::BitReader<bool>;
///Field `SPLCK24` writer - SPLCK24
pub type SPLCK24_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_CFGLOCKR1_SPEC, bool, O>;
///Field `SPLCK25` reader - SPLCK25
pub type SPLCK25_R = crate::BitReader<bool>;
///Field `SPLCK25` writer - SPLCK25
pub type SPLCK25_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_CFGLOCKR1_SPEC, bool, O>;
///Field `SPLCK26` reader - SPLCK26
pub type SPLCK26_R = crate::BitReader<bool>;
///Field `SPLCK26` writer - SPLCK26
pub type SPLCK26_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_CFGLOCKR1_SPEC, bool, O>;
///Field `SPLCK27` reader - SPLCK27
pub type SPLCK27_R = crate::BitReader<bool>;
///Field `SPLCK27` writer - SPLCK27
pub type SPLCK27_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_CFGLOCKR1_SPEC, bool, O>;
///Field `SPLCK28` reader - SPLCK28
pub type SPLCK28_R = crate::BitReader<bool>;
///Field `SPLCK28` writer - SPLCK28
pub type SPLCK28_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_CFGLOCKR1_SPEC, bool, O>;
///Field `SPLCK29` reader - SPLCK29
pub type SPLCK29_R = crate::BitReader<bool>;
///Field `SPLCK29` writer - SPLCK29
pub type SPLCK29_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_CFGLOCKR1_SPEC, bool, O>;
///Field `SPLCK30` reader - SPLCK30
pub type SPLCK30_R = crate::BitReader<bool>;
///Field `SPLCK30` writer - SPLCK30
pub type SPLCK30_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_CFGLOCKR1_SPEC, bool, O>;
///Field `SPLCK31` reader - SPLCK31
pub type SPLCK31_R = crate::BitReader<bool>;
///Field `SPLCK31` writer - SPLCK31
pub type SPLCK31_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_CFGLOCKR1_SPEC, bool, O>;
impl R {
    ///Bit 0 - SPLCK0
    #[inline(always)]
    pub fn splck0(&self) -> SPLCK0_R {
        SPLCK0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - SPLCK1
    #[inline(always)]
    pub fn splck1(&self) -> SPLCK1_R {
        SPLCK1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - SPLCK2
    #[inline(always)]
    pub fn splck2(&self) -> SPLCK2_R {
        SPLCK2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - SPLCK3
    #[inline(always)]
    pub fn splck3(&self) -> SPLCK3_R {
        SPLCK3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - SPLCK4
    #[inline(always)]
    pub fn splck4(&self) -> SPLCK4_R {
        SPLCK4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - SPLCK5
    #[inline(always)]
    pub fn splck5(&self) -> SPLCK5_R {
        SPLCK5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - SPLCK6
    #[inline(always)]
    pub fn splck6(&self) -> SPLCK6_R {
        SPLCK6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - SPLCK7
    #[inline(always)]
    pub fn splck7(&self) -> SPLCK7_R {
        SPLCK7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - SPLCK8
    #[inline(always)]
    pub fn splck8(&self) -> SPLCK8_R {
        SPLCK8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - SPLCK9
    #[inline(always)]
    pub fn splck9(&self) -> SPLCK9_R {
        SPLCK9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - SPLCK10
    #[inline(always)]
    pub fn splck10(&self) -> SPLCK10_R {
        SPLCK10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - SPLCK11
    #[inline(always)]
    pub fn splck11(&self) -> SPLCK11_R {
        SPLCK11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - SPLCK12
    #[inline(always)]
    pub fn splck12(&self) -> SPLCK12_R {
        SPLCK12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - SPLCK13
    #[inline(always)]
    pub fn splck13(&self) -> SPLCK13_R {
        SPLCK13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - SPLCK14
    #[inline(always)]
    pub fn splck14(&self) -> SPLCK14_R {
        SPLCK14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - SPLCK15
    #[inline(always)]
    pub fn splck15(&self) -> SPLCK15_R {
        SPLCK15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - SPLCK16
    #[inline(always)]
    pub fn splck16(&self) -> SPLCK16_R {
        SPLCK16_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - SPLCK17
    #[inline(always)]
    pub fn splck17(&self) -> SPLCK17_R {
        SPLCK17_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - SPLCK18
    #[inline(always)]
    pub fn splck18(&self) -> SPLCK18_R {
        SPLCK18_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - SPLCK19
    #[inline(always)]
    pub fn splck19(&self) -> SPLCK19_R {
        SPLCK19_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - SPLCK20
    #[inline(always)]
    pub fn splck20(&self) -> SPLCK20_R {
        SPLCK20_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - SPLCK21
    #[inline(always)]
    pub fn splck21(&self) -> SPLCK21_R {
        SPLCK21_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - SPLCK22
    #[inline(always)]
    pub fn splck22(&self) -> SPLCK22_R {
        SPLCK22_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - SPLCK23
    #[inline(always)]
    pub fn splck23(&self) -> SPLCK23_R {
        SPLCK23_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - SPLCK24
    #[inline(always)]
    pub fn splck24(&self) -> SPLCK24_R {
        SPLCK24_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - SPLCK25
    #[inline(always)]
    pub fn splck25(&self) -> SPLCK25_R {
        SPLCK25_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - SPLCK26
    #[inline(always)]
    pub fn splck26(&self) -> SPLCK26_R {
        SPLCK26_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - SPLCK27
    #[inline(always)]
    pub fn splck27(&self) -> SPLCK27_R {
        SPLCK27_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - SPLCK28
    #[inline(always)]
    pub fn splck28(&self) -> SPLCK28_R {
        SPLCK28_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - SPLCK29
    #[inline(always)]
    pub fn splck29(&self) -> SPLCK29_R {
        SPLCK29_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - SPLCK30
    #[inline(always)]
    pub fn splck30(&self) -> SPLCK30_R {
        SPLCK30_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - SPLCK31
    #[inline(always)]
    pub fn splck31(&self) -> SPLCK31_R {
        SPLCK31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - SPLCK0
    #[inline(always)]
    #[must_use]
    pub fn splck0(&mut self) -> SPLCK0_W<0> {
        SPLCK0_W::new(self)
    }
    ///Bit 1 - SPLCK1
    #[inline(always)]
    #[must_use]
    pub fn splck1(&mut self) -> SPLCK1_W<1> {
        SPLCK1_W::new(self)
    }
    ///Bit 2 - SPLCK2
    #[inline(always)]
    #[must_use]
    pub fn splck2(&mut self) -> SPLCK2_W<2> {
        SPLCK2_W::new(self)
    }
    ///Bit 3 - SPLCK3
    #[inline(always)]
    #[must_use]
    pub fn splck3(&mut self) -> SPLCK3_W<3> {
        SPLCK3_W::new(self)
    }
    ///Bit 4 - SPLCK4
    #[inline(always)]
    #[must_use]
    pub fn splck4(&mut self) -> SPLCK4_W<4> {
        SPLCK4_W::new(self)
    }
    ///Bit 5 - SPLCK5
    #[inline(always)]
    #[must_use]
    pub fn splck5(&mut self) -> SPLCK5_W<5> {
        SPLCK5_W::new(self)
    }
    ///Bit 6 - SPLCK6
    #[inline(always)]
    #[must_use]
    pub fn splck6(&mut self) -> SPLCK6_W<6> {
        SPLCK6_W::new(self)
    }
    ///Bit 7 - SPLCK7
    #[inline(always)]
    #[must_use]
    pub fn splck7(&mut self) -> SPLCK7_W<7> {
        SPLCK7_W::new(self)
    }
    ///Bit 8 - SPLCK8
    #[inline(always)]
    #[must_use]
    pub fn splck8(&mut self) -> SPLCK8_W<8> {
        SPLCK8_W::new(self)
    }
    ///Bit 9 - SPLCK9
    #[inline(always)]
    #[must_use]
    pub fn splck9(&mut self) -> SPLCK9_W<9> {
        SPLCK9_W::new(self)
    }
    ///Bit 10 - SPLCK10
    #[inline(always)]
    #[must_use]
    pub fn splck10(&mut self) -> SPLCK10_W<10> {
        SPLCK10_W::new(self)
    }
    ///Bit 11 - SPLCK11
    #[inline(always)]
    #[must_use]
    pub fn splck11(&mut self) -> SPLCK11_W<11> {
        SPLCK11_W::new(self)
    }
    ///Bit 12 - SPLCK12
    #[inline(always)]
    #[must_use]
    pub fn splck12(&mut self) -> SPLCK12_W<12> {
        SPLCK12_W::new(self)
    }
    ///Bit 13 - SPLCK13
    #[inline(always)]
    #[must_use]
    pub fn splck13(&mut self) -> SPLCK13_W<13> {
        SPLCK13_W::new(self)
    }
    ///Bit 14 - SPLCK14
    #[inline(always)]
    #[must_use]
    pub fn splck14(&mut self) -> SPLCK14_W<14> {
        SPLCK14_W::new(self)
    }
    ///Bit 15 - SPLCK15
    #[inline(always)]
    #[must_use]
    pub fn splck15(&mut self) -> SPLCK15_W<15> {
        SPLCK15_W::new(self)
    }
    ///Bit 16 - SPLCK16
    #[inline(always)]
    #[must_use]
    pub fn splck16(&mut self) -> SPLCK16_W<16> {
        SPLCK16_W::new(self)
    }
    ///Bit 17 - SPLCK17
    #[inline(always)]
    #[must_use]
    pub fn splck17(&mut self) -> SPLCK17_W<17> {
        SPLCK17_W::new(self)
    }
    ///Bit 18 - SPLCK18
    #[inline(always)]
    #[must_use]
    pub fn splck18(&mut self) -> SPLCK18_W<18> {
        SPLCK18_W::new(self)
    }
    ///Bit 19 - SPLCK19
    #[inline(always)]
    #[must_use]
    pub fn splck19(&mut self) -> SPLCK19_W<19> {
        SPLCK19_W::new(self)
    }
    ///Bit 20 - SPLCK20
    #[inline(always)]
    #[must_use]
    pub fn splck20(&mut self) -> SPLCK20_W<20> {
        SPLCK20_W::new(self)
    }
    ///Bit 21 - SPLCK21
    #[inline(always)]
    #[must_use]
    pub fn splck21(&mut self) -> SPLCK21_W<21> {
        SPLCK21_W::new(self)
    }
    ///Bit 22 - SPLCK22
    #[inline(always)]
    #[must_use]
    pub fn splck22(&mut self) -> SPLCK22_W<22> {
        SPLCK22_W::new(self)
    }
    ///Bit 23 - SPLCK23
    #[inline(always)]
    #[must_use]
    pub fn splck23(&mut self) -> SPLCK23_W<23> {
        SPLCK23_W::new(self)
    }
    ///Bit 24 - SPLCK24
    #[inline(always)]
    #[must_use]
    pub fn splck24(&mut self) -> SPLCK24_W<24> {
        SPLCK24_W::new(self)
    }
    ///Bit 25 - SPLCK25
    #[inline(always)]
    #[must_use]
    pub fn splck25(&mut self) -> SPLCK25_W<25> {
        SPLCK25_W::new(self)
    }
    ///Bit 26 - SPLCK26
    #[inline(always)]
    #[must_use]
    pub fn splck26(&mut self) -> SPLCK26_W<26> {
        SPLCK26_W::new(self)
    }
    ///Bit 27 - SPLCK27
    #[inline(always)]
    #[must_use]
    pub fn splck27(&mut self) -> SPLCK27_W<27> {
        SPLCK27_W::new(self)
    }
    ///Bit 28 - SPLCK28
    #[inline(always)]
    #[must_use]
    pub fn splck28(&mut self) -> SPLCK28_W<28> {
        SPLCK28_W::new(self)
    }
    ///Bit 29 - SPLCK29
    #[inline(always)]
    #[must_use]
    pub fn splck29(&mut self) -> SPLCK29_W<29> {
        SPLCK29_W::new(self)
    }
    ///Bit 30 - SPLCK30
    #[inline(always)]
    #[must_use]
    pub fn splck30(&mut self) -> SPLCK30_W<30> {
        SPLCK30_W::new(self)
    }
    ///Bit 31 - SPLCK31
    #[inline(always)]
    #[must_use]
    pub fn splck31(&mut self) -> SPLCK31_W<31> {
        SPLCK31_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///GTZC1 SRAMz MPCBB configuration lock register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mpcbb2_cfglockr1](index.html) module
pub struct MPCBB2_CFGLOCKR1_SPEC;
impl crate::RegisterSpec for MPCBB2_CFGLOCKR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [mpcbb2_cfglockr1::R](R) reader structure
impl crate::Readable for MPCBB2_CFGLOCKR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [mpcbb2_cfglockr1::W](W) writer structure
impl crate::Writable for MPCBB2_CFGLOCKR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets MPCBB2_CFGLOCKR1 to value 0
impl crate::Resettable for MPCBB2_CFGLOCKR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
