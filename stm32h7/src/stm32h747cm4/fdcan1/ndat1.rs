///Register `NDAT1` reader
pub struct R(crate::R<NDAT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NDAT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NDAT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NDAT1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `NDAT1` writer
pub struct W(crate::W<NDAT1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NDAT1_SPEC>;
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
impl From<crate::W<NDAT1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NDAT1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ND0` reader - New data
pub type ND0_R = crate::BitReader<bool>;
///Field `ND0` writer - New data
pub type ND0_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT1_SPEC, bool, O>;
///Field `ND1` reader - New data
pub type ND1_R = crate::BitReader<bool>;
///Field `ND1` writer - New data
pub type ND1_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT1_SPEC, bool, O>;
///Field `ND2` reader - New data
pub type ND2_R = crate::BitReader<bool>;
///Field `ND2` writer - New data
pub type ND2_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT1_SPEC, bool, O>;
///Field `ND3` reader - New data
pub type ND3_R = crate::BitReader<bool>;
///Field `ND3` writer - New data
pub type ND3_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT1_SPEC, bool, O>;
///Field `ND4` reader - New data
pub type ND4_R = crate::BitReader<bool>;
///Field `ND4` writer - New data
pub type ND4_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT1_SPEC, bool, O>;
///Field `ND5` reader - New data
pub type ND5_R = crate::BitReader<bool>;
///Field `ND5` writer - New data
pub type ND5_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT1_SPEC, bool, O>;
///Field `ND6` reader - New data
pub type ND6_R = crate::BitReader<bool>;
///Field `ND6` writer - New data
pub type ND6_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT1_SPEC, bool, O>;
///Field `ND7` reader - New data
pub type ND7_R = crate::BitReader<bool>;
///Field `ND7` writer - New data
pub type ND7_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT1_SPEC, bool, O>;
///Field `ND8` reader - New data
pub type ND8_R = crate::BitReader<bool>;
///Field `ND8` writer - New data
pub type ND8_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT1_SPEC, bool, O>;
///Field `ND9` reader - New data
pub type ND9_R = crate::BitReader<bool>;
///Field `ND9` writer - New data
pub type ND9_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT1_SPEC, bool, O>;
///Field `ND10` reader - New data
pub type ND10_R = crate::BitReader<bool>;
///Field `ND10` writer - New data
pub type ND10_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT1_SPEC, bool, O>;
///Field `ND11` reader - New data
pub type ND11_R = crate::BitReader<bool>;
///Field `ND11` writer - New data
pub type ND11_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT1_SPEC, bool, O>;
///Field `ND12` reader - New data
pub type ND12_R = crate::BitReader<bool>;
///Field `ND12` writer - New data
pub type ND12_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT1_SPEC, bool, O>;
///Field `ND13` reader - New data
pub type ND13_R = crate::BitReader<bool>;
///Field `ND13` writer - New data
pub type ND13_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT1_SPEC, bool, O>;
///Field `ND14` reader - New data
pub type ND14_R = crate::BitReader<bool>;
///Field `ND14` writer - New data
pub type ND14_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT1_SPEC, bool, O>;
///Field `ND15` reader - New data
pub type ND15_R = crate::BitReader<bool>;
///Field `ND15` writer - New data
pub type ND15_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT1_SPEC, bool, O>;
///Field `ND16` reader - New data
pub type ND16_R = crate::BitReader<bool>;
///Field `ND16` writer - New data
pub type ND16_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT1_SPEC, bool, O>;
///Field `ND17` reader - New data
pub type ND17_R = crate::BitReader<bool>;
///Field `ND17` writer - New data
pub type ND17_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT1_SPEC, bool, O>;
///Field `ND18` reader - New data
pub type ND18_R = crate::BitReader<bool>;
///Field `ND18` writer - New data
pub type ND18_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT1_SPEC, bool, O>;
///Field `ND19` reader - New data
pub type ND19_R = crate::BitReader<bool>;
///Field `ND19` writer - New data
pub type ND19_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT1_SPEC, bool, O>;
///Field `ND20` reader - New data
pub type ND20_R = crate::BitReader<bool>;
///Field `ND20` writer - New data
pub type ND20_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT1_SPEC, bool, O>;
///Field `ND21` reader - New data
pub type ND21_R = crate::BitReader<bool>;
///Field `ND21` writer - New data
pub type ND21_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT1_SPEC, bool, O>;
///Field `ND22` reader - New data
pub type ND22_R = crate::BitReader<bool>;
///Field `ND22` writer - New data
pub type ND22_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT1_SPEC, bool, O>;
///Field `ND23` reader - New data
pub type ND23_R = crate::BitReader<bool>;
///Field `ND23` writer - New data
pub type ND23_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT1_SPEC, bool, O>;
///Field `ND24` reader - New data
pub type ND24_R = crate::BitReader<bool>;
///Field `ND24` writer - New data
pub type ND24_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT1_SPEC, bool, O>;
///Field `ND25` reader - New data
pub type ND25_R = crate::BitReader<bool>;
///Field `ND25` writer - New data
pub type ND25_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT1_SPEC, bool, O>;
///Field `ND26` reader - New data
pub type ND26_R = crate::BitReader<bool>;
///Field `ND26` writer - New data
pub type ND26_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT1_SPEC, bool, O>;
///Field `ND27` reader - New data
pub type ND27_R = crate::BitReader<bool>;
///Field `ND27` writer - New data
pub type ND27_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT1_SPEC, bool, O>;
///Field `ND28` reader - New data
pub type ND28_R = crate::BitReader<bool>;
///Field `ND28` writer - New data
pub type ND28_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT1_SPEC, bool, O>;
///Field `ND29` reader - New data
pub type ND29_R = crate::BitReader<bool>;
///Field `ND29` writer - New data
pub type ND29_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT1_SPEC, bool, O>;
///Field `ND30` reader - New data
pub type ND30_R = crate::BitReader<bool>;
///Field `ND30` writer - New data
pub type ND30_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT1_SPEC, bool, O>;
///Field `ND31` reader - New data
pub type ND31_R = crate::BitReader<bool>;
///Field `ND31` writer - New data
pub type ND31_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT1_SPEC, bool, O>;
impl R {
    ///Bit 0 - New data
    #[inline(always)]
    pub fn nd0(&self) -> ND0_R {
        ND0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - New data
    #[inline(always)]
    pub fn nd1(&self) -> ND1_R {
        ND1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - New data
    #[inline(always)]
    pub fn nd2(&self) -> ND2_R {
        ND2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - New data
    #[inline(always)]
    pub fn nd3(&self) -> ND3_R {
        ND3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - New data
    #[inline(always)]
    pub fn nd4(&self) -> ND4_R {
        ND4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - New data
    #[inline(always)]
    pub fn nd5(&self) -> ND5_R {
        ND5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - New data
    #[inline(always)]
    pub fn nd6(&self) -> ND6_R {
        ND6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - New data
    #[inline(always)]
    pub fn nd7(&self) -> ND7_R {
        ND7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - New data
    #[inline(always)]
    pub fn nd8(&self) -> ND8_R {
        ND8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - New data
    #[inline(always)]
    pub fn nd9(&self) -> ND9_R {
        ND9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - New data
    #[inline(always)]
    pub fn nd10(&self) -> ND10_R {
        ND10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - New data
    #[inline(always)]
    pub fn nd11(&self) -> ND11_R {
        ND11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - New data
    #[inline(always)]
    pub fn nd12(&self) -> ND12_R {
        ND12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - New data
    #[inline(always)]
    pub fn nd13(&self) -> ND13_R {
        ND13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - New data
    #[inline(always)]
    pub fn nd14(&self) -> ND14_R {
        ND14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - New data
    #[inline(always)]
    pub fn nd15(&self) -> ND15_R {
        ND15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - New data
    #[inline(always)]
    pub fn nd16(&self) -> ND16_R {
        ND16_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - New data
    #[inline(always)]
    pub fn nd17(&self) -> ND17_R {
        ND17_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - New data
    #[inline(always)]
    pub fn nd18(&self) -> ND18_R {
        ND18_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - New data
    #[inline(always)]
    pub fn nd19(&self) -> ND19_R {
        ND19_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - New data
    #[inline(always)]
    pub fn nd20(&self) -> ND20_R {
        ND20_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - New data
    #[inline(always)]
    pub fn nd21(&self) -> ND21_R {
        ND21_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - New data
    #[inline(always)]
    pub fn nd22(&self) -> ND22_R {
        ND22_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - New data
    #[inline(always)]
    pub fn nd23(&self) -> ND23_R {
        ND23_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - New data
    #[inline(always)]
    pub fn nd24(&self) -> ND24_R {
        ND24_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - New data
    #[inline(always)]
    pub fn nd25(&self) -> ND25_R {
        ND25_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - New data
    #[inline(always)]
    pub fn nd26(&self) -> ND26_R {
        ND26_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - New data
    #[inline(always)]
    pub fn nd27(&self) -> ND27_R {
        ND27_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - New data
    #[inline(always)]
    pub fn nd28(&self) -> ND28_R {
        ND28_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - New data
    #[inline(always)]
    pub fn nd29(&self) -> ND29_R {
        ND29_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - New data
    #[inline(always)]
    pub fn nd30(&self) -> ND30_R {
        ND30_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - New data
    #[inline(always)]
    pub fn nd31(&self) -> ND31_R {
        ND31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - New data
    #[inline(always)]
    #[must_use]
    pub fn nd0(&mut self) -> ND0_W<0> {
        ND0_W::new(self)
    }
    ///Bit 1 - New data
    #[inline(always)]
    #[must_use]
    pub fn nd1(&mut self) -> ND1_W<1> {
        ND1_W::new(self)
    }
    ///Bit 2 - New data
    #[inline(always)]
    #[must_use]
    pub fn nd2(&mut self) -> ND2_W<2> {
        ND2_W::new(self)
    }
    ///Bit 3 - New data
    #[inline(always)]
    #[must_use]
    pub fn nd3(&mut self) -> ND3_W<3> {
        ND3_W::new(self)
    }
    ///Bit 4 - New data
    #[inline(always)]
    #[must_use]
    pub fn nd4(&mut self) -> ND4_W<4> {
        ND4_W::new(self)
    }
    ///Bit 5 - New data
    #[inline(always)]
    #[must_use]
    pub fn nd5(&mut self) -> ND5_W<5> {
        ND5_W::new(self)
    }
    ///Bit 6 - New data
    #[inline(always)]
    #[must_use]
    pub fn nd6(&mut self) -> ND6_W<6> {
        ND6_W::new(self)
    }
    ///Bit 7 - New data
    #[inline(always)]
    #[must_use]
    pub fn nd7(&mut self) -> ND7_W<7> {
        ND7_W::new(self)
    }
    ///Bit 8 - New data
    #[inline(always)]
    #[must_use]
    pub fn nd8(&mut self) -> ND8_W<8> {
        ND8_W::new(self)
    }
    ///Bit 9 - New data
    #[inline(always)]
    #[must_use]
    pub fn nd9(&mut self) -> ND9_W<9> {
        ND9_W::new(self)
    }
    ///Bit 10 - New data
    #[inline(always)]
    #[must_use]
    pub fn nd10(&mut self) -> ND10_W<10> {
        ND10_W::new(self)
    }
    ///Bit 11 - New data
    #[inline(always)]
    #[must_use]
    pub fn nd11(&mut self) -> ND11_W<11> {
        ND11_W::new(self)
    }
    ///Bit 12 - New data
    #[inline(always)]
    #[must_use]
    pub fn nd12(&mut self) -> ND12_W<12> {
        ND12_W::new(self)
    }
    ///Bit 13 - New data
    #[inline(always)]
    #[must_use]
    pub fn nd13(&mut self) -> ND13_W<13> {
        ND13_W::new(self)
    }
    ///Bit 14 - New data
    #[inline(always)]
    #[must_use]
    pub fn nd14(&mut self) -> ND14_W<14> {
        ND14_W::new(self)
    }
    ///Bit 15 - New data
    #[inline(always)]
    #[must_use]
    pub fn nd15(&mut self) -> ND15_W<15> {
        ND15_W::new(self)
    }
    ///Bit 16 - New data
    #[inline(always)]
    #[must_use]
    pub fn nd16(&mut self) -> ND16_W<16> {
        ND16_W::new(self)
    }
    ///Bit 17 - New data
    #[inline(always)]
    #[must_use]
    pub fn nd17(&mut self) -> ND17_W<17> {
        ND17_W::new(self)
    }
    ///Bit 18 - New data
    #[inline(always)]
    #[must_use]
    pub fn nd18(&mut self) -> ND18_W<18> {
        ND18_W::new(self)
    }
    ///Bit 19 - New data
    #[inline(always)]
    #[must_use]
    pub fn nd19(&mut self) -> ND19_W<19> {
        ND19_W::new(self)
    }
    ///Bit 20 - New data
    #[inline(always)]
    #[must_use]
    pub fn nd20(&mut self) -> ND20_W<20> {
        ND20_W::new(self)
    }
    ///Bit 21 - New data
    #[inline(always)]
    #[must_use]
    pub fn nd21(&mut self) -> ND21_W<21> {
        ND21_W::new(self)
    }
    ///Bit 22 - New data
    #[inline(always)]
    #[must_use]
    pub fn nd22(&mut self) -> ND22_W<22> {
        ND22_W::new(self)
    }
    ///Bit 23 - New data
    #[inline(always)]
    #[must_use]
    pub fn nd23(&mut self) -> ND23_W<23> {
        ND23_W::new(self)
    }
    ///Bit 24 - New data
    #[inline(always)]
    #[must_use]
    pub fn nd24(&mut self) -> ND24_W<24> {
        ND24_W::new(self)
    }
    ///Bit 25 - New data
    #[inline(always)]
    #[must_use]
    pub fn nd25(&mut self) -> ND25_W<25> {
        ND25_W::new(self)
    }
    ///Bit 26 - New data
    #[inline(always)]
    #[must_use]
    pub fn nd26(&mut self) -> ND26_W<26> {
        ND26_W::new(self)
    }
    ///Bit 27 - New data
    #[inline(always)]
    #[must_use]
    pub fn nd27(&mut self) -> ND27_W<27> {
        ND27_W::new(self)
    }
    ///Bit 28 - New data
    #[inline(always)]
    #[must_use]
    pub fn nd28(&mut self) -> ND28_W<28> {
        ND28_W::new(self)
    }
    ///Bit 29 - New data
    #[inline(always)]
    #[must_use]
    pub fn nd29(&mut self) -> ND29_W<29> {
        ND29_W::new(self)
    }
    ///Bit 30 - New data
    #[inline(always)]
    #[must_use]
    pub fn nd30(&mut self) -> ND30_W<30> {
        ND30_W::new(self)
    }
    ///Bit 31 - New data
    #[inline(always)]
    #[must_use]
    pub fn nd31(&mut self) -> ND31_W<31> {
        ND31_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FDCAN New Data 1 Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ndat1](index.html) module
pub struct NDAT1_SPEC;
impl crate::RegisterSpec for NDAT1_SPEC {
    type Ux = u32;
}
///`read()` method returns [ndat1::R](R) reader structure
impl crate::Readable for NDAT1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ndat1::W](W) writer structure
impl crate::Writable for NDAT1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets NDAT1 to value 0
impl crate::Resettable for NDAT1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
