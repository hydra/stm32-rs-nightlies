///Register `LPGPIO_ODR` reader
pub struct R(crate::R<LPGPIO_ODR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPGPIO_ODR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPGPIO_ODR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPGPIO_ODR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `LPGPIO_ODR` writer
pub struct W(crate::W<LPGPIO_ODR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPGPIO_ODR_SPEC>;
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
impl From<crate::W<LPGPIO_ODR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPGPIO_ODR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ODy0` reader - ODy0
pub type ODY0_R = crate::BitReader<bool>;
///Field `ODy0` writer - ODy0
pub type ODY0_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPGPIO_ODR_SPEC, bool, O>;
///Field `ODy1` reader - ODy1
pub type ODY1_R = crate::BitReader<bool>;
///Field `ODy1` writer - ODy1
pub type ODY1_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPGPIO_ODR_SPEC, bool, O>;
///Field `ODy2` reader - ODy2
pub type ODY2_R = crate::BitReader<bool>;
///Field `ODy2` writer - ODy2
pub type ODY2_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPGPIO_ODR_SPEC, bool, O>;
///Field `ODy3` reader - ODy3
pub type ODY3_R = crate::BitReader<bool>;
///Field `ODy3` writer - ODy3
pub type ODY3_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPGPIO_ODR_SPEC, bool, O>;
///Field `ODy4` reader - ODy4
pub type ODY4_R = crate::BitReader<bool>;
///Field `ODy4` writer - ODy4
pub type ODY4_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPGPIO_ODR_SPEC, bool, O>;
///Field `ODy5` reader - ODy5
pub type ODY5_R = crate::BitReader<bool>;
///Field `ODy5` writer - ODy5
pub type ODY5_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPGPIO_ODR_SPEC, bool, O>;
///Field `ODy6` reader - ODy6
pub type ODY6_R = crate::BitReader<bool>;
///Field `ODy6` writer - ODy6
pub type ODY6_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPGPIO_ODR_SPEC, bool, O>;
///Field `ODy7` reader - ODy7
pub type ODY7_R = crate::BitReader<bool>;
///Field `ODy7` writer - ODy7
pub type ODY7_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPGPIO_ODR_SPEC, bool, O>;
///Field `ODy8` reader - ODy8
pub type ODY8_R = crate::BitReader<bool>;
///Field `ODy8` writer - ODy8
pub type ODY8_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPGPIO_ODR_SPEC, bool, O>;
///Field `ODy9` reader - ODy9
pub type ODY9_R = crate::BitReader<bool>;
///Field `ODy9` writer - ODy9
pub type ODY9_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPGPIO_ODR_SPEC, bool, O>;
///Field `ODy10` reader - ODy10
pub type ODY10_R = crate::BitReader<bool>;
///Field `ODy10` writer - ODy10
pub type ODY10_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPGPIO_ODR_SPEC, bool, O>;
///Field `ODy11` reader - ODy11
pub type ODY11_R = crate::BitReader<bool>;
///Field `ODy11` writer - ODy11
pub type ODY11_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPGPIO_ODR_SPEC, bool, O>;
///Field `ODy12` reader - ODy12
pub type ODY12_R = crate::BitReader<bool>;
///Field `ODy12` writer - ODy12
pub type ODY12_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPGPIO_ODR_SPEC, bool, O>;
///Field `ODy13` reader - ODy13
pub type ODY13_R = crate::BitReader<bool>;
///Field `ODy13` writer - ODy13
pub type ODY13_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPGPIO_ODR_SPEC, bool, O>;
///Field `ODy14` reader - ODy14
pub type ODY14_R = crate::BitReader<bool>;
///Field `ODy14` writer - ODy14
pub type ODY14_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPGPIO_ODR_SPEC, bool, O>;
///Field `ODy15` reader - ODy15
pub type ODY15_R = crate::BitReader<bool>;
///Field `ODy15` writer - ODy15
pub type ODY15_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPGPIO_ODR_SPEC, bool, O>;
impl R {
    ///Bit 0 - ODy0
    #[inline(always)]
    pub fn ody0(&self) -> ODY0_R {
        ODY0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - ODy1
    #[inline(always)]
    pub fn ody1(&self) -> ODY1_R {
        ODY1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - ODy2
    #[inline(always)]
    pub fn ody2(&self) -> ODY2_R {
        ODY2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - ODy3
    #[inline(always)]
    pub fn ody3(&self) -> ODY3_R {
        ODY3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - ODy4
    #[inline(always)]
    pub fn ody4(&self) -> ODY4_R {
        ODY4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - ODy5
    #[inline(always)]
    pub fn ody5(&self) -> ODY5_R {
        ODY5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - ODy6
    #[inline(always)]
    pub fn ody6(&self) -> ODY6_R {
        ODY6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - ODy7
    #[inline(always)]
    pub fn ody7(&self) -> ODY7_R {
        ODY7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - ODy8
    #[inline(always)]
    pub fn ody8(&self) -> ODY8_R {
        ODY8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - ODy9
    #[inline(always)]
    pub fn ody9(&self) -> ODY9_R {
        ODY9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - ODy10
    #[inline(always)]
    pub fn ody10(&self) -> ODY10_R {
        ODY10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - ODy11
    #[inline(always)]
    pub fn ody11(&self) -> ODY11_R {
        ODY11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - ODy12
    #[inline(always)]
    pub fn ody12(&self) -> ODY12_R {
        ODY12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - ODy13
    #[inline(always)]
    pub fn ody13(&self) -> ODY13_R {
        ODY13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - ODy14
    #[inline(always)]
    pub fn ody14(&self) -> ODY14_R {
        ODY14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - ODy15
    #[inline(always)]
    pub fn ody15(&self) -> ODY15_R {
        ODY15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - ODy0
    #[inline(always)]
    #[must_use]
    pub fn ody0(&mut self) -> ODY0_W<0> {
        ODY0_W::new(self)
    }
    ///Bit 1 - ODy1
    #[inline(always)]
    #[must_use]
    pub fn ody1(&mut self) -> ODY1_W<1> {
        ODY1_W::new(self)
    }
    ///Bit 2 - ODy2
    #[inline(always)]
    #[must_use]
    pub fn ody2(&mut self) -> ODY2_W<2> {
        ODY2_W::new(self)
    }
    ///Bit 3 - ODy3
    #[inline(always)]
    #[must_use]
    pub fn ody3(&mut self) -> ODY3_W<3> {
        ODY3_W::new(self)
    }
    ///Bit 4 - ODy4
    #[inline(always)]
    #[must_use]
    pub fn ody4(&mut self) -> ODY4_W<4> {
        ODY4_W::new(self)
    }
    ///Bit 5 - ODy5
    #[inline(always)]
    #[must_use]
    pub fn ody5(&mut self) -> ODY5_W<5> {
        ODY5_W::new(self)
    }
    ///Bit 6 - ODy6
    #[inline(always)]
    #[must_use]
    pub fn ody6(&mut self) -> ODY6_W<6> {
        ODY6_W::new(self)
    }
    ///Bit 7 - ODy7
    #[inline(always)]
    #[must_use]
    pub fn ody7(&mut self) -> ODY7_W<7> {
        ODY7_W::new(self)
    }
    ///Bit 8 - ODy8
    #[inline(always)]
    #[must_use]
    pub fn ody8(&mut self) -> ODY8_W<8> {
        ODY8_W::new(self)
    }
    ///Bit 9 - ODy9
    #[inline(always)]
    #[must_use]
    pub fn ody9(&mut self) -> ODY9_W<9> {
        ODY9_W::new(self)
    }
    ///Bit 10 - ODy10
    #[inline(always)]
    #[must_use]
    pub fn ody10(&mut self) -> ODY10_W<10> {
        ODY10_W::new(self)
    }
    ///Bit 11 - ODy11
    #[inline(always)]
    #[must_use]
    pub fn ody11(&mut self) -> ODY11_W<11> {
        ODY11_W::new(self)
    }
    ///Bit 12 - ODy12
    #[inline(always)]
    #[must_use]
    pub fn ody12(&mut self) -> ODY12_W<12> {
        ODY12_W::new(self)
    }
    ///Bit 13 - ODy13
    #[inline(always)]
    #[must_use]
    pub fn ody13(&mut self) -> ODY13_W<13> {
        ODY13_W::new(self)
    }
    ///Bit 14 - ODy14
    #[inline(always)]
    #[must_use]
    pub fn ody14(&mut self) -> ODY14_W<14> {
        ODY14_W::new(self)
    }
    ///Bit 15 - ODy15
    #[inline(always)]
    #[must_use]
    pub fn ody15(&mut self) -> ODY15_W<15> {
        ODY15_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///LPGPIO port output data register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [lpgpio_odr](index.html) module
pub struct LPGPIO_ODR_SPEC;
impl crate::RegisterSpec for LPGPIO_ODR_SPEC {
    type Ux = u32;
}
///`read()` method returns [lpgpio_odr::R](R) reader structure
impl crate::Readable for LPGPIO_ODR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [lpgpio_odr::W](W) writer structure
impl crate::Writable for LPGPIO_ODR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets LPGPIO_ODR to value 0
impl crate::Resettable for LPGPIO_ODR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
