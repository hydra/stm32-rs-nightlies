///Register `MPCBB1_VCTR0` reader
pub struct R(crate::R<MPCBB1_VCTR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPCBB1_VCTR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPCBB1_VCTR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPCBB1_VCTR0_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MPCBB1_VCTR0` writer
pub struct W(crate::W<MPCBB1_VCTR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPCBB1_VCTR0_SPEC>;
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
impl From<crate::W<MPCBB1_VCTR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPCBB1_VCTR0_SPEC>) -> Self {
        W(writer)
    }
}
///Field `B0` reader - B0
pub type B0_R = crate::BitReader<bool>;
///Field `B0` writer - B0
pub type B0_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_VCTR0_SPEC, bool, O>;
///Field `B1` reader - B1
pub type B1_R = crate::BitReader<bool>;
///Field `B1` writer - B1
pub type B1_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_VCTR0_SPEC, bool, O>;
///Field `B2` reader - B2
pub type B2_R = crate::BitReader<bool>;
///Field `B2` writer - B2
pub type B2_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_VCTR0_SPEC, bool, O>;
///Field `B3` reader - B3
pub type B3_R = crate::BitReader<bool>;
///Field `B3` writer - B3
pub type B3_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_VCTR0_SPEC, bool, O>;
///Field `B4` reader - B4
pub type B4_R = crate::BitReader<bool>;
///Field `B4` writer - B4
pub type B4_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_VCTR0_SPEC, bool, O>;
///Field `B5` reader - B5
pub type B5_R = crate::BitReader<bool>;
///Field `B5` writer - B5
pub type B5_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_VCTR0_SPEC, bool, O>;
///Field `B6` reader - B6
pub type B6_R = crate::BitReader<bool>;
///Field `B6` writer - B6
pub type B6_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_VCTR0_SPEC, bool, O>;
///Field `B7` reader - B7
pub type B7_R = crate::BitReader<bool>;
///Field `B7` writer - B7
pub type B7_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_VCTR0_SPEC, bool, O>;
///Field `B8` reader - B8
pub type B8_R = crate::BitReader<bool>;
///Field `B8` writer - B8
pub type B8_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_VCTR0_SPEC, bool, O>;
///Field `B9` reader - B9
pub type B9_R = crate::BitReader<bool>;
///Field `B9` writer - B9
pub type B9_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_VCTR0_SPEC, bool, O>;
///Field `B10` reader - B10
pub type B10_R = crate::BitReader<bool>;
///Field `B10` writer - B10
pub type B10_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_VCTR0_SPEC, bool, O>;
///Field `B11` reader - B11
pub type B11_R = crate::BitReader<bool>;
///Field `B11` writer - B11
pub type B11_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_VCTR0_SPEC, bool, O>;
///Field `B12` reader - B12
pub type B12_R = crate::BitReader<bool>;
///Field `B12` writer - B12
pub type B12_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_VCTR0_SPEC, bool, O>;
///Field `B13` reader - B13
pub type B13_R = crate::BitReader<bool>;
///Field `B13` writer - B13
pub type B13_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_VCTR0_SPEC, bool, O>;
///Field `B14` reader - B14
pub type B14_R = crate::BitReader<bool>;
///Field `B14` writer - B14
pub type B14_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_VCTR0_SPEC, bool, O>;
///Field `B15` reader - B15
pub type B15_R = crate::BitReader<bool>;
///Field `B15` writer - B15
pub type B15_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_VCTR0_SPEC, bool, O>;
///Field `B16` reader - B16
pub type B16_R = crate::BitReader<bool>;
///Field `B16` writer - B16
pub type B16_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_VCTR0_SPEC, bool, O>;
///Field `B17` reader - B17
pub type B17_R = crate::BitReader<bool>;
///Field `B17` writer - B17
pub type B17_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_VCTR0_SPEC, bool, O>;
///Field `B18` reader - B18
pub type B18_R = crate::BitReader<bool>;
///Field `B18` writer - B18
pub type B18_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_VCTR0_SPEC, bool, O>;
///Field `B19` reader - B19
pub type B19_R = crate::BitReader<bool>;
///Field `B19` writer - B19
pub type B19_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_VCTR0_SPEC, bool, O>;
///Field `B20` reader - B20
pub type B20_R = crate::BitReader<bool>;
///Field `B20` writer - B20
pub type B20_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_VCTR0_SPEC, bool, O>;
///Field `B21` reader - B21
pub type B21_R = crate::BitReader<bool>;
///Field `B21` writer - B21
pub type B21_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_VCTR0_SPEC, bool, O>;
///Field `B22` reader - B22
pub type B22_R = crate::BitReader<bool>;
///Field `B22` writer - B22
pub type B22_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_VCTR0_SPEC, bool, O>;
///Field `B23` reader - B23
pub type B23_R = crate::BitReader<bool>;
///Field `B23` writer - B23
pub type B23_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_VCTR0_SPEC, bool, O>;
///Field `B24` reader - B24
pub type B24_R = crate::BitReader<bool>;
///Field `B24` writer - B24
pub type B24_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_VCTR0_SPEC, bool, O>;
///Field `B25` reader - B25
pub type B25_R = crate::BitReader<bool>;
///Field `B25` writer - B25
pub type B25_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_VCTR0_SPEC, bool, O>;
///Field `B26` reader - B26
pub type B26_R = crate::BitReader<bool>;
///Field `B26` writer - B26
pub type B26_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_VCTR0_SPEC, bool, O>;
///Field `B27` reader - B27
pub type B27_R = crate::BitReader<bool>;
///Field `B27` writer - B27
pub type B27_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_VCTR0_SPEC, bool, O>;
///Field `B28` reader - B28
pub type B28_R = crate::BitReader<bool>;
///Field `B28` writer - B28
pub type B28_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_VCTR0_SPEC, bool, O>;
///Field `B29` reader - B29
pub type B29_R = crate::BitReader<bool>;
///Field `B29` writer - B29
pub type B29_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_VCTR0_SPEC, bool, O>;
///Field `B30` reader - B30
pub type B30_R = crate::BitReader<bool>;
///Field `B30` writer - B30
pub type B30_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_VCTR0_SPEC, bool, O>;
///Field `B31` reader - B31
pub type B31_R = crate::BitReader<bool>;
///Field `B31` writer - B31
pub type B31_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB1_VCTR0_SPEC, bool, O>;
impl R {
    ///Bit 0 - B0
    #[inline(always)]
    pub fn b0(&self) -> B0_R {
        B0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - B1
    #[inline(always)]
    pub fn b1(&self) -> B1_R {
        B1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - B2
    #[inline(always)]
    pub fn b2(&self) -> B2_R {
        B2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - B3
    #[inline(always)]
    pub fn b3(&self) -> B3_R {
        B3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - B4
    #[inline(always)]
    pub fn b4(&self) -> B4_R {
        B4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - B5
    #[inline(always)]
    pub fn b5(&self) -> B5_R {
        B5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - B6
    #[inline(always)]
    pub fn b6(&self) -> B6_R {
        B6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - B7
    #[inline(always)]
    pub fn b7(&self) -> B7_R {
        B7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - B8
    #[inline(always)]
    pub fn b8(&self) -> B8_R {
        B8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - B9
    #[inline(always)]
    pub fn b9(&self) -> B9_R {
        B9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - B10
    #[inline(always)]
    pub fn b10(&self) -> B10_R {
        B10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - B11
    #[inline(always)]
    pub fn b11(&self) -> B11_R {
        B11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - B12
    #[inline(always)]
    pub fn b12(&self) -> B12_R {
        B12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - B13
    #[inline(always)]
    pub fn b13(&self) -> B13_R {
        B13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - B14
    #[inline(always)]
    pub fn b14(&self) -> B14_R {
        B14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - B15
    #[inline(always)]
    pub fn b15(&self) -> B15_R {
        B15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - B16
    #[inline(always)]
    pub fn b16(&self) -> B16_R {
        B16_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - B17
    #[inline(always)]
    pub fn b17(&self) -> B17_R {
        B17_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - B18
    #[inline(always)]
    pub fn b18(&self) -> B18_R {
        B18_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - B19
    #[inline(always)]
    pub fn b19(&self) -> B19_R {
        B19_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - B20
    #[inline(always)]
    pub fn b20(&self) -> B20_R {
        B20_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - B21
    #[inline(always)]
    pub fn b21(&self) -> B21_R {
        B21_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - B22
    #[inline(always)]
    pub fn b22(&self) -> B22_R {
        B22_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - B23
    #[inline(always)]
    pub fn b23(&self) -> B23_R {
        B23_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - B24
    #[inline(always)]
    pub fn b24(&self) -> B24_R {
        B24_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - B25
    #[inline(always)]
    pub fn b25(&self) -> B25_R {
        B25_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - B26
    #[inline(always)]
    pub fn b26(&self) -> B26_R {
        B26_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - B27
    #[inline(always)]
    pub fn b27(&self) -> B27_R {
        B27_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - B28
    #[inline(always)]
    pub fn b28(&self) -> B28_R {
        B28_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - B29
    #[inline(always)]
    pub fn b29(&self) -> B29_R {
        B29_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - B30
    #[inline(always)]
    pub fn b30(&self) -> B30_R {
        B30_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - B31
    #[inline(always)]
    pub fn b31(&self) -> B31_R {
        B31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - B0
    #[inline(always)]
    #[must_use]
    pub fn b0(&mut self) -> B0_W<0> {
        B0_W::new(self)
    }
    ///Bit 1 - B1
    #[inline(always)]
    #[must_use]
    pub fn b1(&mut self) -> B1_W<1> {
        B1_W::new(self)
    }
    ///Bit 2 - B2
    #[inline(always)]
    #[must_use]
    pub fn b2(&mut self) -> B2_W<2> {
        B2_W::new(self)
    }
    ///Bit 3 - B3
    #[inline(always)]
    #[must_use]
    pub fn b3(&mut self) -> B3_W<3> {
        B3_W::new(self)
    }
    ///Bit 4 - B4
    #[inline(always)]
    #[must_use]
    pub fn b4(&mut self) -> B4_W<4> {
        B4_W::new(self)
    }
    ///Bit 5 - B5
    #[inline(always)]
    #[must_use]
    pub fn b5(&mut self) -> B5_W<5> {
        B5_W::new(self)
    }
    ///Bit 6 - B6
    #[inline(always)]
    #[must_use]
    pub fn b6(&mut self) -> B6_W<6> {
        B6_W::new(self)
    }
    ///Bit 7 - B7
    #[inline(always)]
    #[must_use]
    pub fn b7(&mut self) -> B7_W<7> {
        B7_W::new(self)
    }
    ///Bit 8 - B8
    #[inline(always)]
    #[must_use]
    pub fn b8(&mut self) -> B8_W<8> {
        B8_W::new(self)
    }
    ///Bit 9 - B9
    #[inline(always)]
    #[must_use]
    pub fn b9(&mut self) -> B9_W<9> {
        B9_W::new(self)
    }
    ///Bit 10 - B10
    #[inline(always)]
    #[must_use]
    pub fn b10(&mut self) -> B10_W<10> {
        B10_W::new(self)
    }
    ///Bit 11 - B11
    #[inline(always)]
    #[must_use]
    pub fn b11(&mut self) -> B11_W<11> {
        B11_W::new(self)
    }
    ///Bit 12 - B12
    #[inline(always)]
    #[must_use]
    pub fn b12(&mut self) -> B12_W<12> {
        B12_W::new(self)
    }
    ///Bit 13 - B13
    #[inline(always)]
    #[must_use]
    pub fn b13(&mut self) -> B13_W<13> {
        B13_W::new(self)
    }
    ///Bit 14 - B14
    #[inline(always)]
    #[must_use]
    pub fn b14(&mut self) -> B14_W<14> {
        B14_W::new(self)
    }
    ///Bit 15 - B15
    #[inline(always)]
    #[must_use]
    pub fn b15(&mut self) -> B15_W<15> {
        B15_W::new(self)
    }
    ///Bit 16 - B16
    #[inline(always)]
    #[must_use]
    pub fn b16(&mut self) -> B16_W<16> {
        B16_W::new(self)
    }
    ///Bit 17 - B17
    #[inline(always)]
    #[must_use]
    pub fn b17(&mut self) -> B17_W<17> {
        B17_W::new(self)
    }
    ///Bit 18 - B18
    #[inline(always)]
    #[must_use]
    pub fn b18(&mut self) -> B18_W<18> {
        B18_W::new(self)
    }
    ///Bit 19 - B19
    #[inline(always)]
    #[must_use]
    pub fn b19(&mut self) -> B19_W<19> {
        B19_W::new(self)
    }
    ///Bit 20 - B20
    #[inline(always)]
    #[must_use]
    pub fn b20(&mut self) -> B20_W<20> {
        B20_W::new(self)
    }
    ///Bit 21 - B21
    #[inline(always)]
    #[must_use]
    pub fn b21(&mut self) -> B21_W<21> {
        B21_W::new(self)
    }
    ///Bit 22 - B22
    #[inline(always)]
    #[must_use]
    pub fn b22(&mut self) -> B22_W<22> {
        B22_W::new(self)
    }
    ///Bit 23 - B23
    #[inline(always)]
    #[must_use]
    pub fn b23(&mut self) -> B23_W<23> {
        B23_W::new(self)
    }
    ///Bit 24 - B24
    #[inline(always)]
    #[must_use]
    pub fn b24(&mut self) -> B24_W<24> {
        B24_W::new(self)
    }
    ///Bit 25 - B25
    #[inline(always)]
    #[must_use]
    pub fn b25(&mut self) -> B25_W<25> {
        B25_W::new(self)
    }
    ///Bit 26 - B26
    #[inline(always)]
    #[must_use]
    pub fn b26(&mut self) -> B26_W<26> {
        B26_W::new(self)
    }
    ///Bit 27 - B27
    #[inline(always)]
    #[must_use]
    pub fn b27(&mut self) -> B27_W<27> {
        B27_W::new(self)
    }
    ///Bit 28 - B28
    #[inline(always)]
    #[must_use]
    pub fn b28(&mut self) -> B28_W<28> {
        B28_W::new(self)
    }
    ///Bit 29 - B29
    #[inline(always)]
    #[must_use]
    pub fn b29(&mut self) -> B29_W<29> {
        B29_W::new(self)
    }
    ///Bit 30 - B30
    #[inline(always)]
    #[must_use]
    pub fn b30(&mut self) -> B30_W<30> {
        B30_W::new(self)
    }
    ///Bit 31 - B31
    #[inline(always)]
    #[must_use]
    pub fn b31(&mut self) -> B31_W<31> {
        B31_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///MPCBBx vector register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mpcbb1_vctr0](index.html) module
pub struct MPCBB1_VCTR0_SPEC;
impl crate::RegisterSpec for MPCBB1_VCTR0_SPEC {
    type Ux = u32;
}
///`read()` method returns [mpcbb1_vctr0::R](R) reader structure
impl crate::Readable for MPCBB1_VCTR0_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [mpcbb1_vctr0::W](W) writer structure
impl crate::Writable for MPCBB1_VCTR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets MPCBB1_VCTR0 to value 0xffff_ffff
impl crate::Resettable for MPCBB1_VCTR0_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
