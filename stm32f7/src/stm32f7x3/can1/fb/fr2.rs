///Register `FR2` reader
pub struct R(crate::R<FR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FR2` writer
pub struct W(crate::W<FR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FR2_SPEC>;
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
impl From<crate::W<FR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `FB0` reader - Filter bits
pub type FB0_R = crate::BitReader<bool>;
///Field `FB0` writer - Filter bits
pub type FB0_W<'a, const O: u8> = crate::BitWriter<'a, u32, FR2_SPEC, bool, O>;
///Field `FB1` reader - Filter bits
pub type FB1_R = crate::BitReader<bool>;
///Field `FB1` writer - Filter bits
pub type FB1_W<'a, const O: u8> = crate::BitWriter<'a, u32, FR2_SPEC, bool, O>;
///Field `FB2` reader - Filter bits
pub type FB2_R = crate::BitReader<bool>;
///Field `FB2` writer - Filter bits
pub type FB2_W<'a, const O: u8> = crate::BitWriter<'a, u32, FR2_SPEC, bool, O>;
///Field `FB3` reader - Filter bits
pub type FB3_R = crate::BitReader<bool>;
///Field `FB3` writer - Filter bits
pub type FB3_W<'a, const O: u8> = crate::BitWriter<'a, u32, FR2_SPEC, bool, O>;
///Field `FB4` reader - Filter bits
pub type FB4_R = crate::BitReader<bool>;
///Field `FB4` writer - Filter bits
pub type FB4_W<'a, const O: u8> = crate::BitWriter<'a, u32, FR2_SPEC, bool, O>;
///Field `FB5` reader - Filter bits
pub type FB5_R = crate::BitReader<bool>;
///Field `FB5` writer - Filter bits
pub type FB5_W<'a, const O: u8> = crate::BitWriter<'a, u32, FR2_SPEC, bool, O>;
///Field `FB6` reader - Filter bits
pub type FB6_R = crate::BitReader<bool>;
///Field `FB6` writer - Filter bits
pub type FB6_W<'a, const O: u8> = crate::BitWriter<'a, u32, FR2_SPEC, bool, O>;
///Field `FB7` reader - Filter bits
pub type FB7_R = crate::BitReader<bool>;
///Field `FB7` writer - Filter bits
pub type FB7_W<'a, const O: u8> = crate::BitWriter<'a, u32, FR2_SPEC, bool, O>;
///Field `FB8` reader - Filter bits
pub type FB8_R = crate::BitReader<bool>;
///Field `FB8` writer - Filter bits
pub type FB8_W<'a, const O: u8> = crate::BitWriter<'a, u32, FR2_SPEC, bool, O>;
///Field `FB9` reader - Filter bits
pub type FB9_R = crate::BitReader<bool>;
///Field `FB9` writer - Filter bits
pub type FB9_W<'a, const O: u8> = crate::BitWriter<'a, u32, FR2_SPEC, bool, O>;
///Field `FB10` reader - Filter bits
pub type FB10_R = crate::BitReader<bool>;
///Field `FB10` writer - Filter bits
pub type FB10_W<'a, const O: u8> = crate::BitWriter<'a, u32, FR2_SPEC, bool, O>;
///Field `FB11` reader - Filter bits
pub type FB11_R = crate::BitReader<bool>;
///Field `FB11` writer - Filter bits
pub type FB11_W<'a, const O: u8> = crate::BitWriter<'a, u32, FR2_SPEC, bool, O>;
///Field `FB12` reader - Filter bits
pub type FB12_R = crate::BitReader<bool>;
///Field `FB12` writer - Filter bits
pub type FB12_W<'a, const O: u8> = crate::BitWriter<'a, u32, FR2_SPEC, bool, O>;
///Field `FB13` reader - Filter bits
pub type FB13_R = crate::BitReader<bool>;
///Field `FB13` writer - Filter bits
pub type FB13_W<'a, const O: u8> = crate::BitWriter<'a, u32, FR2_SPEC, bool, O>;
///Field `FB14` reader - Filter bits
pub type FB14_R = crate::BitReader<bool>;
///Field `FB14` writer - Filter bits
pub type FB14_W<'a, const O: u8> = crate::BitWriter<'a, u32, FR2_SPEC, bool, O>;
///Field `FB15` reader - Filter bits
pub type FB15_R = crate::BitReader<bool>;
///Field `FB15` writer - Filter bits
pub type FB15_W<'a, const O: u8> = crate::BitWriter<'a, u32, FR2_SPEC, bool, O>;
///Field `FB16` reader - Filter bits
pub type FB16_R = crate::BitReader<bool>;
///Field `FB16` writer - Filter bits
pub type FB16_W<'a, const O: u8> = crate::BitWriter<'a, u32, FR2_SPEC, bool, O>;
///Field `FB17` reader - Filter bits
pub type FB17_R = crate::BitReader<bool>;
///Field `FB17` writer - Filter bits
pub type FB17_W<'a, const O: u8> = crate::BitWriter<'a, u32, FR2_SPEC, bool, O>;
///Field `FB18` reader - Filter bits
pub type FB18_R = crate::BitReader<bool>;
///Field `FB18` writer - Filter bits
pub type FB18_W<'a, const O: u8> = crate::BitWriter<'a, u32, FR2_SPEC, bool, O>;
///Field `FB19` reader - Filter bits
pub type FB19_R = crate::BitReader<bool>;
///Field `FB19` writer - Filter bits
pub type FB19_W<'a, const O: u8> = crate::BitWriter<'a, u32, FR2_SPEC, bool, O>;
///Field `FB20` reader - Filter bits
pub type FB20_R = crate::BitReader<bool>;
///Field `FB20` writer - Filter bits
pub type FB20_W<'a, const O: u8> = crate::BitWriter<'a, u32, FR2_SPEC, bool, O>;
///Field `FB21` reader - Filter bits
pub type FB21_R = crate::BitReader<bool>;
///Field `FB21` writer - Filter bits
pub type FB21_W<'a, const O: u8> = crate::BitWriter<'a, u32, FR2_SPEC, bool, O>;
///Field `FB22` reader - Filter bits
pub type FB22_R = crate::BitReader<bool>;
///Field `FB22` writer - Filter bits
pub type FB22_W<'a, const O: u8> = crate::BitWriter<'a, u32, FR2_SPEC, bool, O>;
///Field `FB23` reader - Filter bits
pub type FB23_R = crate::BitReader<bool>;
///Field `FB23` writer - Filter bits
pub type FB23_W<'a, const O: u8> = crate::BitWriter<'a, u32, FR2_SPEC, bool, O>;
///Field `FB24` reader - Filter bits
pub type FB24_R = crate::BitReader<bool>;
///Field `FB24` writer - Filter bits
pub type FB24_W<'a, const O: u8> = crate::BitWriter<'a, u32, FR2_SPEC, bool, O>;
///Field `FB25` reader - Filter bits
pub type FB25_R = crate::BitReader<bool>;
///Field `FB25` writer - Filter bits
pub type FB25_W<'a, const O: u8> = crate::BitWriter<'a, u32, FR2_SPEC, bool, O>;
///Field `FB26` reader - Filter bits
pub type FB26_R = crate::BitReader<bool>;
///Field `FB26` writer - Filter bits
pub type FB26_W<'a, const O: u8> = crate::BitWriter<'a, u32, FR2_SPEC, bool, O>;
///Field `FB27` reader - Filter bits
pub type FB27_R = crate::BitReader<bool>;
///Field `FB27` writer - Filter bits
pub type FB27_W<'a, const O: u8> = crate::BitWriter<'a, u32, FR2_SPEC, bool, O>;
///Field `FB28` reader - Filter bits
pub type FB28_R = crate::BitReader<bool>;
///Field `FB28` writer - Filter bits
pub type FB28_W<'a, const O: u8> = crate::BitWriter<'a, u32, FR2_SPEC, bool, O>;
///Field `FB29` reader - Filter bits
pub type FB29_R = crate::BitReader<bool>;
///Field `FB29` writer - Filter bits
pub type FB29_W<'a, const O: u8> = crate::BitWriter<'a, u32, FR2_SPEC, bool, O>;
///Field `FB30` reader - Filter bits
pub type FB30_R = crate::BitReader<bool>;
///Field `FB30` writer - Filter bits
pub type FB30_W<'a, const O: u8> = crate::BitWriter<'a, u32, FR2_SPEC, bool, O>;
///Field `FB31` reader - Filter bits
pub type FB31_R = crate::BitReader<bool>;
///Field `FB31` writer - Filter bits
pub type FB31_W<'a, const O: u8> = crate::BitWriter<'a, u32, FR2_SPEC, bool, O>;
impl R {
    ///Bit 0 - Filter bits
    #[inline(always)]
    pub fn fb0(&self) -> FB0_R {
        FB0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Filter bits
    #[inline(always)]
    pub fn fb1(&self) -> FB1_R {
        FB1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Filter bits
    #[inline(always)]
    pub fn fb2(&self) -> FB2_R {
        FB2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Filter bits
    #[inline(always)]
    pub fn fb3(&self) -> FB3_R {
        FB3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Filter bits
    #[inline(always)]
    pub fn fb4(&self) -> FB4_R {
        FB4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Filter bits
    #[inline(always)]
    pub fn fb5(&self) -> FB5_R {
        FB5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Filter bits
    #[inline(always)]
    pub fn fb6(&self) -> FB6_R {
        FB6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Filter bits
    #[inline(always)]
    pub fn fb7(&self) -> FB7_R {
        FB7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Filter bits
    #[inline(always)]
    pub fn fb8(&self) -> FB8_R {
        FB8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Filter bits
    #[inline(always)]
    pub fn fb9(&self) -> FB9_R {
        FB9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Filter bits
    #[inline(always)]
    pub fn fb10(&self) -> FB10_R {
        FB10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Filter bits
    #[inline(always)]
    pub fn fb11(&self) -> FB11_R {
        FB11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Filter bits
    #[inline(always)]
    pub fn fb12(&self) -> FB12_R {
        FB12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Filter bits
    #[inline(always)]
    pub fn fb13(&self) -> FB13_R {
        FB13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Filter bits
    #[inline(always)]
    pub fn fb14(&self) -> FB14_R {
        FB14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Filter bits
    #[inline(always)]
    pub fn fb15(&self) -> FB15_R {
        FB15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Filter bits
    #[inline(always)]
    pub fn fb16(&self) -> FB16_R {
        FB16_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Filter bits
    #[inline(always)]
    pub fn fb17(&self) -> FB17_R {
        FB17_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Filter bits
    #[inline(always)]
    pub fn fb18(&self) -> FB18_R {
        FB18_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Filter bits
    #[inline(always)]
    pub fn fb19(&self) -> FB19_R {
        FB19_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Filter bits
    #[inline(always)]
    pub fn fb20(&self) -> FB20_R {
        FB20_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Filter bits
    #[inline(always)]
    pub fn fb21(&self) -> FB21_R {
        FB21_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Filter bits
    #[inline(always)]
    pub fn fb22(&self) -> FB22_R {
        FB22_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Filter bits
    #[inline(always)]
    pub fn fb23(&self) -> FB23_R {
        FB23_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Filter bits
    #[inline(always)]
    pub fn fb24(&self) -> FB24_R {
        FB24_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Filter bits
    #[inline(always)]
    pub fn fb25(&self) -> FB25_R {
        FB25_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Filter bits
    #[inline(always)]
    pub fn fb26(&self) -> FB26_R {
        FB26_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Filter bits
    #[inline(always)]
    pub fn fb27(&self) -> FB27_R {
        FB27_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Filter bits
    #[inline(always)]
    pub fn fb28(&self) -> FB28_R {
        FB28_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Filter bits
    #[inline(always)]
    pub fn fb29(&self) -> FB29_R {
        FB29_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Filter bits
    #[inline(always)]
    pub fn fb30(&self) -> FB30_R {
        FB30_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Filter bits
    #[inline(always)]
    pub fn fb31(&self) -> FB31_R {
        FB31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Filter bits
    #[inline(always)]
    #[must_use]
    pub fn fb0(&mut self) -> FB0_W<0> {
        FB0_W::new(self)
    }
    ///Bit 1 - Filter bits
    #[inline(always)]
    #[must_use]
    pub fn fb1(&mut self) -> FB1_W<1> {
        FB1_W::new(self)
    }
    ///Bit 2 - Filter bits
    #[inline(always)]
    #[must_use]
    pub fn fb2(&mut self) -> FB2_W<2> {
        FB2_W::new(self)
    }
    ///Bit 3 - Filter bits
    #[inline(always)]
    #[must_use]
    pub fn fb3(&mut self) -> FB3_W<3> {
        FB3_W::new(self)
    }
    ///Bit 4 - Filter bits
    #[inline(always)]
    #[must_use]
    pub fn fb4(&mut self) -> FB4_W<4> {
        FB4_W::new(self)
    }
    ///Bit 5 - Filter bits
    #[inline(always)]
    #[must_use]
    pub fn fb5(&mut self) -> FB5_W<5> {
        FB5_W::new(self)
    }
    ///Bit 6 - Filter bits
    #[inline(always)]
    #[must_use]
    pub fn fb6(&mut self) -> FB6_W<6> {
        FB6_W::new(self)
    }
    ///Bit 7 - Filter bits
    #[inline(always)]
    #[must_use]
    pub fn fb7(&mut self) -> FB7_W<7> {
        FB7_W::new(self)
    }
    ///Bit 8 - Filter bits
    #[inline(always)]
    #[must_use]
    pub fn fb8(&mut self) -> FB8_W<8> {
        FB8_W::new(self)
    }
    ///Bit 9 - Filter bits
    #[inline(always)]
    #[must_use]
    pub fn fb9(&mut self) -> FB9_W<9> {
        FB9_W::new(self)
    }
    ///Bit 10 - Filter bits
    #[inline(always)]
    #[must_use]
    pub fn fb10(&mut self) -> FB10_W<10> {
        FB10_W::new(self)
    }
    ///Bit 11 - Filter bits
    #[inline(always)]
    #[must_use]
    pub fn fb11(&mut self) -> FB11_W<11> {
        FB11_W::new(self)
    }
    ///Bit 12 - Filter bits
    #[inline(always)]
    #[must_use]
    pub fn fb12(&mut self) -> FB12_W<12> {
        FB12_W::new(self)
    }
    ///Bit 13 - Filter bits
    #[inline(always)]
    #[must_use]
    pub fn fb13(&mut self) -> FB13_W<13> {
        FB13_W::new(self)
    }
    ///Bit 14 - Filter bits
    #[inline(always)]
    #[must_use]
    pub fn fb14(&mut self) -> FB14_W<14> {
        FB14_W::new(self)
    }
    ///Bit 15 - Filter bits
    #[inline(always)]
    #[must_use]
    pub fn fb15(&mut self) -> FB15_W<15> {
        FB15_W::new(self)
    }
    ///Bit 16 - Filter bits
    #[inline(always)]
    #[must_use]
    pub fn fb16(&mut self) -> FB16_W<16> {
        FB16_W::new(self)
    }
    ///Bit 17 - Filter bits
    #[inline(always)]
    #[must_use]
    pub fn fb17(&mut self) -> FB17_W<17> {
        FB17_W::new(self)
    }
    ///Bit 18 - Filter bits
    #[inline(always)]
    #[must_use]
    pub fn fb18(&mut self) -> FB18_W<18> {
        FB18_W::new(self)
    }
    ///Bit 19 - Filter bits
    #[inline(always)]
    #[must_use]
    pub fn fb19(&mut self) -> FB19_W<19> {
        FB19_W::new(self)
    }
    ///Bit 20 - Filter bits
    #[inline(always)]
    #[must_use]
    pub fn fb20(&mut self) -> FB20_W<20> {
        FB20_W::new(self)
    }
    ///Bit 21 - Filter bits
    #[inline(always)]
    #[must_use]
    pub fn fb21(&mut self) -> FB21_W<21> {
        FB21_W::new(self)
    }
    ///Bit 22 - Filter bits
    #[inline(always)]
    #[must_use]
    pub fn fb22(&mut self) -> FB22_W<22> {
        FB22_W::new(self)
    }
    ///Bit 23 - Filter bits
    #[inline(always)]
    #[must_use]
    pub fn fb23(&mut self) -> FB23_W<23> {
        FB23_W::new(self)
    }
    ///Bit 24 - Filter bits
    #[inline(always)]
    #[must_use]
    pub fn fb24(&mut self) -> FB24_W<24> {
        FB24_W::new(self)
    }
    ///Bit 25 - Filter bits
    #[inline(always)]
    #[must_use]
    pub fn fb25(&mut self) -> FB25_W<25> {
        FB25_W::new(self)
    }
    ///Bit 26 - Filter bits
    #[inline(always)]
    #[must_use]
    pub fn fb26(&mut self) -> FB26_W<26> {
        FB26_W::new(self)
    }
    ///Bit 27 - Filter bits
    #[inline(always)]
    #[must_use]
    pub fn fb27(&mut self) -> FB27_W<27> {
        FB27_W::new(self)
    }
    ///Bit 28 - Filter bits
    #[inline(always)]
    #[must_use]
    pub fn fb28(&mut self) -> FB28_W<28> {
        FB28_W::new(self)
    }
    ///Bit 29 - Filter bits
    #[inline(always)]
    #[must_use]
    pub fn fb29(&mut self) -> FB29_W<29> {
        FB29_W::new(self)
    }
    ///Bit 30 - Filter bits
    #[inline(always)]
    #[must_use]
    pub fn fb30(&mut self) -> FB30_W<30> {
        FB30_W::new(self)
    }
    ///Bit 31 - Filter bits
    #[inline(always)]
    #[must_use]
    pub fn fb31(&mut self) -> FB31_W<31> {
        FB31_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Filter bank 0 register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fr2](index.html) module
pub struct FR2_SPEC;
impl crate::RegisterSpec for FR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [fr2::R](R) reader structure
impl crate::Readable for FR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [fr2::W](W) writer structure
impl crate::Writable for FR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets FR2 to value 0
impl crate::Resettable for FR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
