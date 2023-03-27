///Register `PRIVCFGR1` reader
pub struct R(crate::R<PRIVCFGR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRIVCFGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRIVCFGR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRIVCFGR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PRIVCFGR1` writer
pub struct W(crate::W<PRIVCFGR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRIVCFGR1_SPEC>;
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
impl From<crate::W<PRIVCFGR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRIVCFGR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PRIV0` reader - Privilege enable on event input x (x = 17 to 0)
pub type PRIV0_R = crate::BitReader<bool>;
///Field `PRIV0` writer - Privilege enable on event input x (x = 17 to 0)
pub type PRIV0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR1_SPEC, bool, O>;
///Field `PRIV1` reader - Privilege enable on event input x (x = 17 to 0)
pub type PRIV1_R = crate::BitReader<bool>;
///Field `PRIV1` writer - Privilege enable on event input x (x = 17 to 0)
pub type PRIV1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR1_SPEC, bool, O>;
///Field `PRIV2` reader - Privilege enable on event input x (x = 17 to 0)
pub type PRIV2_R = crate::BitReader<bool>;
///Field `PRIV2` writer - Privilege enable on event input x (x = 17 to 0)
pub type PRIV2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR1_SPEC, bool, O>;
///Field `PRIV3` reader - Privilege enable on event input x (x = 17 to 0)
pub type PRIV3_R = crate::BitReader<bool>;
///Field `PRIV3` writer - Privilege enable on event input x (x = 17 to 0)
pub type PRIV3_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR1_SPEC, bool, O>;
///Field `PRIV4` reader - Privilege enable on event input x (x = 17 to 0)
pub type PRIV4_R = crate::BitReader<bool>;
///Field `PRIV4` writer - Privilege enable on event input x (x = 17 to 0)
pub type PRIV4_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR1_SPEC, bool, O>;
///Field `PRIV5` reader - Privilege enable on event input x (x = 17 to 0)
pub type PRIV5_R = crate::BitReader<bool>;
///Field `PRIV5` writer - Privilege enable on event input x (x = 17 to 0)
pub type PRIV5_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR1_SPEC, bool, O>;
///Field `PRIV6` reader - Privilege enable on event input x (x = 17 to 0)
pub type PRIV6_R = crate::BitReader<bool>;
///Field `PRIV6` writer - Privilege enable on event input x (x = 17 to 0)
pub type PRIV6_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR1_SPEC, bool, O>;
///Field `PRIV7` reader - Privilege enable on event input x (x = 17 to 0)
pub type PRIV7_R = crate::BitReader<bool>;
///Field `PRIV7` writer - Privilege enable on event input x (x = 17 to 0)
pub type PRIV7_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR1_SPEC, bool, O>;
///Field `PRIV8` reader - Privilege enable on event input x (x = 17 to 0)
pub type PRIV8_R = crate::BitReader<bool>;
///Field `PRIV8` writer - Privilege enable on event input x (x = 17 to 0)
pub type PRIV8_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR1_SPEC, bool, O>;
///Field `PRIV9` reader - Privilege enable on event input x (x = 17 to 0)
pub type PRIV9_R = crate::BitReader<bool>;
///Field `PRIV9` writer - Privilege enable on event input x (x = 17 to 0)
pub type PRIV9_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR1_SPEC, bool, O>;
///Field `PRIV10` reader - Privilege enable on event input x (x = 17 to 0)
pub type PRIV10_R = crate::BitReader<bool>;
///Field `PRIV10` writer - Privilege enable on event input x (x = 17 to 0)
pub type PRIV10_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR1_SPEC, bool, O>;
///Field `PRIV11` reader - Privilege enable on event input x (x = 17 to 0)
pub type PRIV11_R = crate::BitReader<bool>;
///Field `PRIV11` writer - Privilege enable on event input x (x = 17 to 0)
pub type PRIV11_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR1_SPEC, bool, O>;
///Field `PRIV12` reader - Privilege enable on event input x (x = 17 to 0)
pub type PRIV12_R = crate::BitReader<bool>;
///Field `PRIV12` writer - Privilege enable on event input x (x = 17 to 0)
pub type PRIV12_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR1_SPEC, bool, O>;
///Field `PRIV13` reader - Privilege enable on event input x (x = 17 to 0)
pub type PRIV13_R = crate::BitReader<bool>;
///Field `PRIV13` writer - Privilege enable on event input x (x = 17 to 0)
pub type PRIV13_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR1_SPEC, bool, O>;
///Field `PRIV14` reader - Privilege enable on event input x (x = 17 to 0)
pub type PRIV14_R = crate::BitReader<bool>;
///Field `PRIV14` writer - Privilege enable on event input x (x = 17 to 0)
pub type PRIV14_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR1_SPEC, bool, O>;
///Field `PRIV15` reader - Privilege enable on event input x (x = 17 to 0)
pub type PRIV15_R = crate::BitReader<bool>;
///Field `PRIV15` writer - Privilege enable on event input x (x = 17 to 0)
pub type PRIV15_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR1_SPEC, bool, O>;
///Field `PRIV16` reader - Privilege enable on event input x (x = 17 to 0)
pub type PRIV16_R = crate::BitReader<bool>;
///Field `PRIV16` writer - Privilege enable on event input x (x = 17 to 0)
pub type PRIV16_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR1_SPEC, bool, O>;
///Field `PRIV17` reader - Privilege enable on event input x (x = 17 to 0)
pub type PRIV17_R = crate::BitReader<bool>;
///Field `PRIV17` writer - Privilege enable on event input x (x = 17 to 0)
pub type PRIV17_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR1_SPEC, bool, O>;
///Field `PRIV19` reader - Privilege enable on event input 19
pub type PRIV19_R = crate::BitReader<bool>;
///Field `PRIV19` writer - Privilege enable on event input 19
pub type PRIV19_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR1_SPEC, bool, O>;
///Field `PRIV21` reader - Privilege enable on event input x (x = 22 to 21)
pub type PRIV21_R = crate::BitReader<bool>;
///Field `PRIV21` writer - Privilege enable on event input x (x = 22 to 21)
pub type PRIV21_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR1_SPEC, bool, O>;
///Field `PRIV22` reader - Privilege enable on event input x (x = 22 to 21)
pub type PRIV22_R = crate::BitReader<bool>;
///Field `PRIV22` writer - Privilege enable on event input x (x = 22 to 21)
pub type PRIV22_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR1_SPEC, bool, O>;
///Field `PRIV24` reader - Privilege enable on event input x (x = 29 to 24)
pub type PRIV24_R = crate::BitReader<bool>;
///Field `PRIV24` writer - Privilege enable on event input x (x = 29 to 24)
pub type PRIV24_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR1_SPEC, bool, O>;
///Field `PRIV25` reader - Privilege enable on event input x (x = 29 to 24)
pub type PRIV25_R = crate::BitReader<bool>;
///Field `PRIV25` writer - Privilege enable on event input x (x = 29 to 24)
pub type PRIV25_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR1_SPEC, bool, O>;
///Field `PRIV26` reader - Privilege enable on event input x (x = 29 to 24)
pub type PRIV26_R = crate::BitReader<bool>;
///Field `PRIV26` writer - Privilege enable on event input x (x = 29 to 24)
pub type PRIV26_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR1_SPEC, bool, O>;
///Field `PRIV27` reader - Privilege enable on event input x (x = 29 to 24)
pub type PRIV27_R = crate::BitReader<bool>;
///Field `PRIV27` writer - Privilege enable on event input x (x = 29 to 24)
pub type PRIV27_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR1_SPEC, bool, O>;
///Field `PRIV28` reader - Privilege enable on event input x (x = 29 to 24)
pub type PRIV28_R = crate::BitReader<bool>;
///Field `PRIV28` writer - Privilege enable on event input x (x = 29 to 24)
pub type PRIV28_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR1_SPEC, bool, O>;
///Field `PRIV29` reader - Privilege enable on event input x (x = 29 to 24)
pub type PRIV29_R = crate::BitReader<bool>;
///Field `PRIV29` writer - Privilege enable on event input x (x = 29 to 24)
pub type PRIV29_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR1_SPEC, bool, O>;
impl R {
    ///Bit 0 - Privilege enable on event input x (x = 17 to 0)
    #[inline(always)]
    pub fn priv0(&self) -> PRIV0_R {
        PRIV0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Privilege enable on event input x (x = 17 to 0)
    #[inline(always)]
    pub fn priv1(&self) -> PRIV1_R {
        PRIV1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Privilege enable on event input x (x = 17 to 0)
    #[inline(always)]
    pub fn priv2(&self) -> PRIV2_R {
        PRIV2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Privilege enable on event input x (x = 17 to 0)
    #[inline(always)]
    pub fn priv3(&self) -> PRIV3_R {
        PRIV3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Privilege enable on event input x (x = 17 to 0)
    #[inline(always)]
    pub fn priv4(&self) -> PRIV4_R {
        PRIV4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Privilege enable on event input x (x = 17 to 0)
    #[inline(always)]
    pub fn priv5(&self) -> PRIV5_R {
        PRIV5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Privilege enable on event input x (x = 17 to 0)
    #[inline(always)]
    pub fn priv6(&self) -> PRIV6_R {
        PRIV6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Privilege enable on event input x (x = 17 to 0)
    #[inline(always)]
    pub fn priv7(&self) -> PRIV7_R {
        PRIV7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Privilege enable on event input x (x = 17 to 0)
    #[inline(always)]
    pub fn priv8(&self) -> PRIV8_R {
        PRIV8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Privilege enable on event input x (x = 17 to 0)
    #[inline(always)]
    pub fn priv9(&self) -> PRIV9_R {
        PRIV9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Privilege enable on event input x (x = 17 to 0)
    #[inline(always)]
    pub fn priv10(&self) -> PRIV10_R {
        PRIV10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Privilege enable on event input x (x = 17 to 0)
    #[inline(always)]
    pub fn priv11(&self) -> PRIV11_R {
        PRIV11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Privilege enable on event input x (x = 17 to 0)
    #[inline(always)]
    pub fn priv12(&self) -> PRIV12_R {
        PRIV12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Privilege enable on event input x (x = 17 to 0)
    #[inline(always)]
    pub fn priv13(&self) -> PRIV13_R {
        PRIV13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Privilege enable on event input x (x = 17 to 0)
    #[inline(always)]
    pub fn priv14(&self) -> PRIV14_R {
        PRIV14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Privilege enable on event input x (x = 17 to 0)
    #[inline(always)]
    pub fn priv15(&self) -> PRIV15_R {
        PRIV15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Privilege enable on event input x (x = 17 to 0)
    #[inline(always)]
    pub fn priv16(&self) -> PRIV16_R {
        PRIV16_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Privilege enable on event input x (x = 17 to 0)
    #[inline(always)]
    pub fn priv17(&self) -> PRIV17_R {
        PRIV17_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 19 - Privilege enable on event input 19
    #[inline(always)]
    pub fn priv19(&self) -> PRIV19_R {
        PRIV19_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 21 - Privilege enable on event input x (x = 22 to 21)
    #[inline(always)]
    pub fn priv21(&self) -> PRIV21_R {
        PRIV21_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Privilege enable on event input x (x = 22 to 21)
    #[inline(always)]
    pub fn priv22(&self) -> PRIV22_R {
        PRIV22_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 24 - Privilege enable on event input x (x = 29 to 24)
    #[inline(always)]
    pub fn priv24(&self) -> PRIV24_R {
        PRIV24_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Privilege enable on event input x (x = 29 to 24)
    #[inline(always)]
    pub fn priv25(&self) -> PRIV25_R {
        PRIV25_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Privilege enable on event input x (x = 29 to 24)
    #[inline(always)]
    pub fn priv26(&self) -> PRIV26_R {
        PRIV26_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Privilege enable on event input x (x = 29 to 24)
    #[inline(always)]
    pub fn priv27(&self) -> PRIV27_R {
        PRIV27_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Privilege enable on event input x (x = 29 to 24)
    #[inline(always)]
    pub fn priv28(&self) -> PRIV28_R {
        PRIV28_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Privilege enable on event input x (x = 29 to 24)
    #[inline(always)]
    pub fn priv29(&self) -> PRIV29_R {
        PRIV29_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Privilege enable on event input x (x = 17 to 0)
    #[inline(always)]
    #[must_use]
    pub fn priv0(&mut self) -> PRIV0_W<0> {
        PRIV0_W::new(self)
    }
    ///Bit 1 - Privilege enable on event input x (x = 17 to 0)
    #[inline(always)]
    #[must_use]
    pub fn priv1(&mut self) -> PRIV1_W<1> {
        PRIV1_W::new(self)
    }
    ///Bit 2 - Privilege enable on event input x (x = 17 to 0)
    #[inline(always)]
    #[must_use]
    pub fn priv2(&mut self) -> PRIV2_W<2> {
        PRIV2_W::new(self)
    }
    ///Bit 3 - Privilege enable on event input x (x = 17 to 0)
    #[inline(always)]
    #[must_use]
    pub fn priv3(&mut self) -> PRIV3_W<3> {
        PRIV3_W::new(self)
    }
    ///Bit 4 - Privilege enable on event input x (x = 17 to 0)
    #[inline(always)]
    #[must_use]
    pub fn priv4(&mut self) -> PRIV4_W<4> {
        PRIV4_W::new(self)
    }
    ///Bit 5 - Privilege enable on event input x (x = 17 to 0)
    #[inline(always)]
    #[must_use]
    pub fn priv5(&mut self) -> PRIV5_W<5> {
        PRIV5_W::new(self)
    }
    ///Bit 6 - Privilege enable on event input x (x = 17 to 0)
    #[inline(always)]
    #[must_use]
    pub fn priv6(&mut self) -> PRIV6_W<6> {
        PRIV6_W::new(self)
    }
    ///Bit 7 - Privilege enable on event input x (x = 17 to 0)
    #[inline(always)]
    #[must_use]
    pub fn priv7(&mut self) -> PRIV7_W<7> {
        PRIV7_W::new(self)
    }
    ///Bit 8 - Privilege enable on event input x (x = 17 to 0)
    #[inline(always)]
    #[must_use]
    pub fn priv8(&mut self) -> PRIV8_W<8> {
        PRIV8_W::new(self)
    }
    ///Bit 9 - Privilege enable on event input x (x = 17 to 0)
    #[inline(always)]
    #[must_use]
    pub fn priv9(&mut self) -> PRIV9_W<9> {
        PRIV9_W::new(self)
    }
    ///Bit 10 - Privilege enable on event input x (x = 17 to 0)
    #[inline(always)]
    #[must_use]
    pub fn priv10(&mut self) -> PRIV10_W<10> {
        PRIV10_W::new(self)
    }
    ///Bit 11 - Privilege enable on event input x (x = 17 to 0)
    #[inline(always)]
    #[must_use]
    pub fn priv11(&mut self) -> PRIV11_W<11> {
        PRIV11_W::new(self)
    }
    ///Bit 12 - Privilege enable on event input x (x = 17 to 0)
    #[inline(always)]
    #[must_use]
    pub fn priv12(&mut self) -> PRIV12_W<12> {
        PRIV12_W::new(self)
    }
    ///Bit 13 - Privilege enable on event input x (x = 17 to 0)
    #[inline(always)]
    #[must_use]
    pub fn priv13(&mut self) -> PRIV13_W<13> {
        PRIV13_W::new(self)
    }
    ///Bit 14 - Privilege enable on event input x (x = 17 to 0)
    #[inline(always)]
    #[must_use]
    pub fn priv14(&mut self) -> PRIV14_W<14> {
        PRIV14_W::new(self)
    }
    ///Bit 15 - Privilege enable on event input x (x = 17 to 0)
    #[inline(always)]
    #[must_use]
    pub fn priv15(&mut self) -> PRIV15_W<15> {
        PRIV15_W::new(self)
    }
    ///Bit 16 - Privilege enable on event input x (x = 17 to 0)
    #[inline(always)]
    #[must_use]
    pub fn priv16(&mut self) -> PRIV16_W<16> {
        PRIV16_W::new(self)
    }
    ///Bit 17 - Privilege enable on event input x (x = 17 to 0)
    #[inline(always)]
    #[must_use]
    pub fn priv17(&mut self) -> PRIV17_W<17> {
        PRIV17_W::new(self)
    }
    ///Bit 19 - Privilege enable on event input 19
    #[inline(always)]
    #[must_use]
    pub fn priv19(&mut self) -> PRIV19_W<19> {
        PRIV19_W::new(self)
    }
    ///Bit 21 - Privilege enable on event input x (x = 22 to 21)
    #[inline(always)]
    #[must_use]
    pub fn priv21(&mut self) -> PRIV21_W<21> {
        PRIV21_W::new(self)
    }
    ///Bit 22 - Privilege enable on event input x (x = 22 to 21)
    #[inline(always)]
    #[must_use]
    pub fn priv22(&mut self) -> PRIV22_W<22> {
        PRIV22_W::new(self)
    }
    ///Bit 24 - Privilege enable on event input x (x = 29 to 24)
    #[inline(always)]
    #[must_use]
    pub fn priv24(&mut self) -> PRIV24_W<24> {
        PRIV24_W::new(self)
    }
    ///Bit 25 - Privilege enable on event input x (x = 29 to 24)
    #[inline(always)]
    #[must_use]
    pub fn priv25(&mut self) -> PRIV25_W<25> {
        PRIV25_W::new(self)
    }
    ///Bit 26 - Privilege enable on event input x (x = 29 to 24)
    #[inline(always)]
    #[must_use]
    pub fn priv26(&mut self) -> PRIV26_W<26> {
        PRIV26_W::new(self)
    }
    ///Bit 27 - Privilege enable on event input x (x = 29 to 24)
    #[inline(always)]
    #[must_use]
    pub fn priv27(&mut self) -> PRIV27_W<27> {
        PRIV27_W::new(self)
    }
    ///Bit 28 - Privilege enable on event input x (x = 29 to 24)
    #[inline(always)]
    #[must_use]
    pub fn priv28(&mut self) -> PRIV28_W<28> {
        PRIV28_W::new(self)
    }
    ///Bit 29 - Privilege enable on event input x (x = 29 to 24)
    #[inline(always)]
    #[must_use]
    pub fn priv29(&mut self) -> PRIV29_W<29> {
        PRIV29_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///EXTI privilege configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [privcfgr1](index.html) module
pub struct PRIVCFGR1_SPEC;
impl crate::RegisterSpec for PRIVCFGR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [privcfgr1::R](R) reader structure
impl crate::Readable for PRIVCFGR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [privcfgr1::W](W) writer structure
impl crate::Writable for PRIVCFGR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets PRIVCFGR1 to value 0
impl crate::Resettable for PRIVCFGR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
