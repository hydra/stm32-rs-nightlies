///Register `LPGPIO_MODER` reader
pub struct R(crate::R<LPGPIO_MODER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPGPIO_MODER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPGPIO_MODER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPGPIO_MODER_SPEC>) -> Self {
        R(reader)
    }
}
///Register `LPGPIO_MODER` writer
pub struct W(crate::W<LPGPIO_MODER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPGPIO_MODER_SPEC>;
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
impl From<crate::W<LPGPIO_MODER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPGPIO_MODER_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MODE0` reader - MODE0
pub type MODE0_R = crate::BitReader<bool>;
///Field `MODE0` writer - MODE0
pub type MODE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPGPIO_MODER_SPEC, bool, O>;
///Field `MODE1` reader - MODE1
pub type MODE1_R = crate::BitReader<bool>;
///Field `MODE1` writer - MODE1
pub type MODE1_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPGPIO_MODER_SPEC, bool, O>;
///Field `MODE2` reader - MODE2
pub type MODE2_R = crate::BitReader<bool>;
///Field `MODE2` writer - MODE2
pub type MODE2_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPGPIO_MODER_SPEC, bool, O>;
///Field `MODE3` reader - MODE3
pub type MODE3_R = crate::BitReader<bool>;
///Field `MODE3` writer - MODE3
pub type MODE3_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPGPIO_MODER_SPEC, bool, O>;
///Field `MODE4` reader - MODE4
pub type MODE4_R = crate::BitReader<bool>;
///Field `MODE4` writer - MODE4
pub type MODE4_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPGPIO_MODER_SPEC, bool, O>;
///Field `MODE5` reader - MODE5
pub type MODE5_R = crate::BitReader<bool>;
///Field `MODE5` writer - MODE5
pub type MODE5_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPGPIO_MODER_SPEC, bool, O>;
///Field `MODE6` reader - MODE6
pub type MODE6_R = crate::BitReader<bool>;
///Field `MODE6` writer - MODE6
pub type MODE6_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPGPIO_MODER_SPEC, bool, O>;
///Field `MODE7` reader - MODE7
pub type MODE7_R = crate::BitReader<bool>;
///Field `MODE7` writer - MODE7
pub type MODE7_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPGPIO_MODER_SPEC, bool, O>;
///Field `MODE8` reader - MODE8
pub type MODE8_R = crate::BitReader<bool>;
///Field `MODE8` writer - MODE8
pub type MODE8_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPGPIO_MODER_SPEC, bool, O>;
///Field `MODE9` reader - MODE9
pub type MODE9_R = crate::BitReader<bool>;
///Field `MODE9` writer - MODE9
pub type MODE9_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPGPIO_MODER_SPEC, bool, O>;
///Field `MODE10` reader - MODE10
pub type MODE10_R = crate::BitReader<bool>;
///Field `MODE10` writer - MODE10
pub type MODE10_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPGPIO_MODER_SPEC, bool, O>;
///Field `MODE11` reader - MODE11
pub type MODE11_R = crate::BitReader<bool>;
///Field `MODE11` writer - MODE11
pub type MODE11_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPGPIO_MODER_SPEC, bool, O>;
///Field `MODE12` reader - MODE12
pub type MODE12_R = crate::BitReader<bool>;
///Field `MODE12` writer - MODE12
pub type MODE12_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPGPIO_MODER_SPEC, bool, O>;
///Field `MODE13` reader - MODE13
pub type MODE13_R = crate::BitReader<bool>;
///Field `MODE13` writer - MODE13
pub type MODE13_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPGPIO_MODER_SPEC, bool, O>;
///Field `MODE14` reader - MODE14
pub type MODE14_R = crate::BitReader<bool>;
///Field `MODE14` writer - MODE14
pub type MODE14_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPGPIO_MODER_SPEC, bool, O>;
///Field `MODE15` reader - MODE15
pub type MODE15_R = crate::BitReader<bool>;
///Field `MODE15` writer - MODE15
pub type MODE15_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPGPIO_MODER_SPEC, bool, O>;
impl R {
    ///Bit 0 - MODE0
    #[inline(always)]
    pub fn mode0(&self) -> MODE0_R {
        MODE0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - MODE1
    #[inline(always)]
    pub fn mode1(&self) -> MODE1_R {
        MODE1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - MODE2
    #[inline(always)]
    pub fn mode2(&self) -> MODE2_R {
        MODE2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - MODE3
    #[inline(always)]
    pub fn mode3(&self) -> MODE3_R {
        MODE3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - MODE4
    #[inline(always)]
    pub fn mode4(&self) -> MODE4_R {
        MODE4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - MODE5
    #[inline(always)]
    pub fn mode5(&self) -> MODE5_R {
        MODE5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - MODE6
    #[inline(always)]
    pub fn mode6(&self) -> MODE6_R {
        MODE6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - MODE7
    #[inline(always)]
    pub fn mode7(&self) -> MODE7_R {
        MODE7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - MODE8
    #[inline(always)]
    pub fn mode8(&self) -> MODE8_R {
        MODE8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - MODE9
    #[inline(always)]
    pub fn mode9(&self) -> MODE9_R {
        MODE9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - MODE10
    #[inline(always)]
    pub fn mode10(&self) -> MODE10_R {
        MODE10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - MODE11
    #[inline(always)]
    pub fn mode11(&self) -> MODE11_R {
        MODE11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - MODE12
    #[inline(always)]
    pub fn mode12(&self) -> MODE12_R {
        MODE12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - MODE13
    #[inline(always)]
    pub fn mode13(&self) -> MODE13_R {
        MODE13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - MODE14
    #[inline(always)]
    pub fn mode14(&self) -> MODE14_R {
        MODE14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - MODE15
    #[inline(always)]
    pub fn mode15(&self) -> MODE15_R {
        MODE15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - MODE0
    #[inline(always)]
    #[must_use]
    pub fn mode0(&mut self) -> MODE0_W<0> {
        MODE0_W::new(self)
    }
    ///Bit 1 - MODE1
    #[inline(always)]
    #[must_use]
    pub fn mode1(&mut self) -> MODE1_W<1> {
        MODE1_W::new(self)
    }
    ///Bit 2 - MODE2
    #[inline(always)]
    #[must_use]
    pub fn mode2(&mut self) -> MODE2_W<2> {
        MODE2_W::new(self)
    }
    ///Bit 3 - MODE3
    #[inline(always)]
    #[must_use]
    pub fn mode3(&mut self) -> MODE3_W<3> {
        MODE3_W::new(self)
    }
    ///Bit 4 - MODE4
    #[inline(always)]
    #[must_use]
    pub fn mode4(&mut self) -> MODE4_W<4> {
        MODE4_W::new(self)
    }
    ///Bit 5 - MODE5
    #[inline(always)]
    #[must_use]
    pub fn mode5(&mut self) -> MODE5_W<5> {
        MODE5_W::new(self)
    }
    ///Bit 6 - MODE6
    #[inline(always)]
    #[must_use]
    pub fn mode6(&mut self) -> MODE6_W<6> {
        MODE6_W::new(self)
    }
    ///Bit 7 - MODE7
    #[inline(always)]
    #[must_use]
    pub fn mode7(&mut self) -> MODE7_W<7> {
        MODE7_W::new(self)
    }
    ///Bit 8 - MODE8
    #[inline(always)]
    #[must_use]
    pub fn mode8(&mut self) -> MODE8_W<8> {
        MODE8_W::new(self)
    }
    ///Bit 9 - MODE9
    #[inline(always)]
    #[must_use]
    pub fn mode9(&mut self) -> MODE9_W<9> {
        MODE9_W::new(self)
    }
    ///Bit 10 - MODE10
    #[inline(always)]
    #[must_use]
    pub fn mode10(&mut self) -> MODE10_W<10> {
        MODE10_W::new(self)
    }
    ///Bit 11 - MODE11
    #[inline(always)]
    #[must_use]
    pub fn mode11(&mut self) -> MODE11_W<11> {
        MODE11_W::new(self)
    }
    ///Bit 12 - MODE12
    #[inline(always)]
    #[must_use]
    pub fn mode12(&mut self) -> MODE12_W<12> {
        MODE12_W::new(self)
    }
    ///Bit 13 - MODE13
    #[inline(always)]
    #[must_use]
    pub fn mode13(&mut self) -> MODE13_W<13> {
        MODE13_W::new(self)
    }
    ///Bit 14 - MODE14
    #[inline(always)]
    #[must_use]
    pub fn mode14(&mut self) -> MODE14_W<14> {
        MODE14_W::new(self)
    }
    ///Bit 15 - MODE15
    #[inline(always)]
    #[must_use]
    pub fn mode15(&mut self) -> MODE15_W<15> {
        MODE15_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///LPGPIO port mode register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [lpgpio_moder](index.html) module
pub struct LPGPIO_MODER_SPEC;
impl crate::RegisterSpec for LPGPIO_MODER_SPEC {
    type Ux = u32;
}
///`read()` method returns [lpgpio_moder::R](R) reader structure
impl crate::Readable for LPGPIO_MODER_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [lpgpio_moder::W](W) writer structure
impl crate::Writable for LPGPIO_MODER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets LPGPIO_MODER to value 0
impl crate::Resettable for LPGPIO_MODER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
