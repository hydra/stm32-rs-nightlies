///Register `LPGPIO_BSRR` writer
pub struct W(crate::W<LPGPIO_BSRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPGPIO_BSRR_SPEC>;
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
impl From<crate::W<LPGPIO_BSRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPGPIO_BSRR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `BSy0` writer - BSy0
pub type BSY0_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPGPIO_BSRR_SPEC, bool, O>;
///Field `BSy1` writer - BSy1
pub type BSY1_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPGPIO_BSRR_SPEC, bool, O>;
///Field `BSy2` writer - BSy2
pub type BSY2_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPGPIO_BSRR_SPEC, bool, O>;
///Field `BSy3` writer - BSy3
pub type BSY3_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPGPIO_BSRR_SPEC, bool, O>;
///Field `BSy4` writer - BSy4
pub type BSY4_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPGPIO_BSRR_SPEC, bool, O>;
///Field `BSy5` writer - BSy5
pub type BSY5_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPGPIO_BSRR_SPEC, bool, O>;
///Field `BSy6` writer - BSy6
pub type BSY6_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPGPIO_BSRR_SPEC, bool, O>;
///Field `BSy7` writer - BSy7
pub type BSY7_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPGPIO_BSRR_SPEC, bool, O>;
///Field `BSy8` writer - BSy8
pub type BSY8_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPGPIO_BSRR_SPEC, bool, O>;
///Field `BSy9` writer - BSy9
pub type BSY9_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPGPIO_BSRR_SPEC, bool, O>;
///Field `BSy10` writer - BSy10
pub type BSY10_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPGPIO_BSRR_SPEC, bool, O>;
///Field `BSy11` writer - BSy11
pub type BSY11_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPGPIO_BSRR_SPEC, bool, O>;
///Field `BSy12` writer - BSy12
pub type BSY12_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPGPIO_BSRR_SPEC, bool, O>;
///Field `BSy13` writer - BSy13
pub type BSY13_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPGPIO_BSRR_SPEC, bool, O>;
///Field `BSy14` writer - BSy14
pub type BSY14_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPGPIO_BSRR_SPEC, bool, O>;
///Field `BSy15` writer - BSy15
pub type BSY15_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPGPIO_BSRR_SPEC, bool, O>;
///Field `BRy16` writer - BRy16
pub type BRY16_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPGPIO_BSRR_SPEC, bool, O>;
///Field `BRy17` writer - BRy17
pub type BRY17_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPGPIO_BSRR_SPEC, bool, O>;
///Field `BRy18` writer - BRy18
pub type BRY18_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPGPIO_BSRR_SPEC, bool, O>;
///Field `BRy19` writer - BRy19
pub type BRY19_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPGPIO_BSRR_SPEC, bool, O>;
///Field `BRy20` writer - BRy20
pub type BRY20_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPGPIO_BSRR_SPEC, bool, O>;
///Field `BRy21` writer - BRy21
pub type BRY21_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPGPIO_BSRR_SPEC, bool, O>;
///Field `BRy22` writer - BRy22
pub type BRY22_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPGPIO_BSRR_SPEC, bool, O>;
///Field `BRy23` writer - BRy23
pub type BRY23_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPGPIO_BSRR_SPEC, bool, O>;
///Field `BRy24` writer - BRy24
pub type BRY24_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPGPIO_BSRR_SPEC, bool, O>;
///Field `BRy25` writer - BRy25
pub type BRY25_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPGPIO_BSRR_SPEC, bool, O>;
///Field `BRy26` writer - BRy26
pub type BRY26_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPGPIO_BSRR_SPEC, bool, O>;
///Field `BRy27` writer - BRy27
pub type BRY27_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPGPIO_BSRR_SPEC, bool, O>;
///Field `BRy28` writer - BRy28
pub type BRY28_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPGPIO_BSRR_SPEC, bool, O>;
///Field `BRy29` writer - BRy29
pub type BRY29_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPGPIO_BSRR_SPEC, bool, O>;
///Field `BRy30` writer - BRy30
pub type BRY30_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPGPIO_BSRR_SPEC, bool, O>;
///Field `BRy31` writer - BRy31
pub type BRY31_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPGPIO_BSRR_SPEC, bool, O>;
impl W {
    ///Bit 0 - BSy0
    #[inline(always)]
    #[must_use]
    pub fn bsy0(&mut self) -> BSY0_W<0> {
        BSY0_W::new(self)
    }
    ///Bit 1 - BSy1
    #[inline(always)]
    #[must_use]
    pub fn bsy1(&mut self) -> BSY1_W<1> {
        BSY1_W::new(self)
    }
    ///Bit 2 - BSy2
    #[inline(always)]
    #[must_use]
    pub fn bsy2(&mut self) -> BSY2_W<2> {
        BSY2_W::new(self)
    }
    ///Bit 3 - BSy3
    #[inline(always)]
    #[must_use]
    pub fn bsy3(&mut self) -> BSY3_W<3> {
        BSY3_W::new(self)
    }
    ///Bit 4 - BSy4
    #[inline(always)]
    #[must_use]
    pub fn bsy4(&mut self) -> BSY4_W<4> {
        BSY4_W::new(self)
    }
    ///Bit 5 - BSy5
    #[inline(always)]
    #[must_use]
    pub fn bsy5(&mut self) -> BSY5_W<5> {
        BSY5_W::new(self)
    }
    ///Bit 6 - BSy6
    #[inline(always)]
    #[must_use]
    pub fn bsy6(&mut self) -> BSY6_W<6> {
        BSY6_W::new(self)
    }
    ///Bit 7 - BSy7
    #[inline(always)]
    #[must_use]
    pub fn bsy7(&mut self) -> BSY7_W<7> {
        BSY7_W::new(self)
    }
    ///Bit 8 - BSy8
    #[inline(always)]
    #[must_use]
    pub fn bsy8(&mut self) -> BSY8_W<8> {
        BSY8_W::new(self)
    }
    ///Bit 9 - BSy9
    #[inline(always)]
    #[must_use]
    pub fn bsy9(&mut self) -> BSY9_W<9> {
        BSY9_W::new(self)
    }
    ///Bit 10 - BSy10
    #[inline(always)]
    #[must_use]
    pub fn bsy10(&mut self) -> BSY10_W<10> {
        BSY10_W::new(self)
    }
    ///Bit 11 - BSy11
    #[inline(always)]
    #[must_use]
    pub fn bsy11(&mut self) -> BSY11_W<11> {
        BSY11_W::new(self)
    }
    ///Bit 12 - BSy12
    #[inline(always)]
    #[must_use]
    pub fn bsy12(&mut self) -> BSY12_W<12> {
        BSY12_W::new(self)
    }
    ///Bit 13 - BSy13
    #[inline(always)]
    #[must_use]
    pub fn bsy13(&mut self) -> BSY13_W<13> {
        BSY13_W::new(self)
    }
    ///Bit 14 - BSy14
    #[inline(always)]
    #[must_use]
    pub fn bsy14(&mut self) -> BSY14_W<14> {
        BSY14_W::new(self)
    }
    ///Bit 15 - BSy15
    #[inline(always)]
    #[must_use]
    pub fn bsy15(&mut self) -> BSY15_W<15> {
        BSY15_W::new(self)
    }
    ///Bit 16 - BRy16
    #[inline(always)]
    #[must_use]
    pub fn bry16(&mut self) -> BRY16_W<16> {
        BRY16_W::new(self)
    }
    ///Bit 17 - BRy17
    #[inline(always)]
    #[must_use]
    pub fn bry17(&mut self) -> BRY17_W<17> {
        BRY17_W::new(self)
    }
    ///Bit 18 - BRy18
    #[inline(always)]
    #[must_use]
    pub fn bry18(&mut self) -> BRY18_W<18> {
        BRY18_W::new(self)
    }
    ///Bit 19 - BRy19
    #[inline(always)]
    #[must_use]
    pub fn bry19(&mut self) -> BRY19_W<19> {
        BRY19_W::new(self)
    }
    ///Bit 20 - BRy20
    #[inline(always)]
    #[must_use]
    pub fn bry20(&mut self) -> BRY20_W<20> {
        BRY20_W::new(self)
    }
    ///Bit 21 - BRy21
    #[inline(always)]
    #[must_use]
    pub fn bry21(&mut self) -> BRY21_W<21> {
        BRY21_W::new(self)
    }
    ///Bit 22 - BRy22
    #[inline(always)]
    #[must_use]
    pub fn bry22(&mut self) -> BRY22_W<22> {
        BRY22_W::new(self)
    }
    ///Bit 23 - BRy23
    #[inline(always)]
    #[must_use]
    pub fn bry23(&mut self) -> BRY23_W<23> {
        BRY23_W::new(self)
    }
    ///Bit 24 - BRy24
    #[inline(always)]
    #[must_use]
    pub fn bry24(&mut self) -> BRY24_W<24> {
        BRY24_W::new(self)
    }
    ///Bit 25 - BRy25
    #[inline(always)]
    #[must_use]
    pub fn bry25(&mut self) -> BRY25_W<25> {
        BRY25_W::new(self)
    }
    ///Bit 26 - BRy26
    #[inline(always)]
    #[must_use]
    pub fn bry26(&mut self) -> BRY26_W<26> {
        BRY26_W::new(self)
    }
    ///Bit 27 - BRy27
    #[inline(always)]
    #[must_use]
    pub fn bry27(&mut self) -> BRY27_W<27> {
        BRY27_W::new(self)
    }
    ///Bit 28 - BRy28
    #[inline(always)]
    #[must_use]
    pub fn bry28(&mut self) -> BRY28_W<28> {
        BRY28_W::new(self)
    }
    ///Bit 29 - BRy29
    #[inline(always)]
    #[must_use]
    pub fn bry29(&mut self) -> BRY29_W<29> {
        BRY29_W::new(self)
    }
    ///Bit 30 - BRy30
    #[inline(always)]
    #[must_use]
    pub fn bry30(&mut self) -> BRY30_W<30> {
        BRY30_W::new(self)
    }
    ///Bit 31 - BRy31
    #[inline(always)]
    #[must_use]
    pub fn bry31(&mut self) -> BRY31_W<31> {
        BRY31_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///LPGPIO port bit set/reset register
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [lpgpio_bsrr](index.html) module
pub struct LPGPIO_BSRR_SPEC;
impl crate::RegisterSpec for LPGPIO_BSRR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [lpgpio_bsrr::W](W) writer structure
impl crate::Writable for LPGPIO_BSRR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets LPGPIO_BSRR to value 0
impl crate::Resettable for LPGPIO_BSRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
