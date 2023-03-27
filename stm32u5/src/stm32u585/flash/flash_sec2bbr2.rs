///Register `FLASH_SEC2BBR2` reader
pub struct R(crate::R<FLASH_SEC2BBR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASH_SEC2BBR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASH_SEC2BBR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASH_SEC2BBR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FLASH_SEC2BBR2` writer
pub struct W(crate::W<FLASH_SEC2BBR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASH_SEC2BBR2_SPEC>;
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
impl From<crate::W<FLASH_SEC2BBR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASH_SEC2BBR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SEC2BB0` reader - page secure/non-secure attribution
pub type SEC2BB0_R = crate::BitReader<bool>;
///Field `SEC2BB0` writer - page secure/non-secure attribution
pub type SEC2BB0_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_SEC2BBR2_SPEC, bool, O>;
///Field `SEC2BB1` reader - page secure/non-secure attribution
pub type SEC2BB1_R = crate::BitReader<bool>;
///Field `SEC2BB1` writer - page secure/non-secure attribution
pub type SEC2BB1_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_SEC2BBR2_SPEC, bool, O>;
///Field `SEC2BB2` reader - page secure/non-secure attribution
pub type SEC2BB2_R = crate::BitReader<bool>;
///Field `SEC2BB2` writer - page secure/non-secure attribution
pub type SEC2BB2_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_SEC2BBR2_SPEC, bool, O>;
///Field `SEC2BB3` reader - page secure/non-secure attribution
pub type SEC2BB3_R = crate::BitReader<bool>;
///Field `SEC2BB3` writer - page secure/non-secure attribution
pub type SEC2BB3_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_SEC2BBR2_SPEC, bool, O>;
///Field `SEC2BB4` reader - page secure/non-secure attribution
pub type SEC2BB4_R = crate::BitReader<bool>;
///Field `SEC2BB4` writer - page secure/non-secure attribution
pub type SEC2BB4_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_SEC2BBR2_SPEC, bool, O>;
///Field `SEC2BB5` reader - page secure/non-secure attribution
pub type SEC2BB5_R = crate::BitReader<bool>;
///Field `SEC2BB5` writer - page secure/non-secure attribution
pub type SEC2BB5_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_SEC2BBR2_SPEC, bool, O>;
///Field `SEC2BB6` reader - page secure/non-secure attribution
pub type SEC2BB6_R = crate::BitReader<bool>;
///Field `SEC2BB6` writer - page secure/non-secure attribution
pub type SEC2BB6_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_SEC2BBR2_SPEC, bool, O>;
///Field `SEC2BB7` reader - page secure/non-secure attribution
pub type SEC2BB7_R = crate::BitReader<bool>;
///Field `SEC2BB7` writer - page secure/non-secure attribution
pub type SEC2BB7_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_SEC2BBR2_SPEC, bool, O>;
///Field `SEC2BB8` reader - page secure/non-secure attribution
pub type SEC2BB8_R = crate::BitReader<bool>;
///Field `SEC2BB8` writer - page secure/non-secure attribution
pub type SEC2BB8_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_SEC2BBR2_SPEC, bool, O>;
///Field `SEC2BB9` reader - page secure/non-secure attribution
pub type SEC2BB9_R = crate::BitReader<bool>;
///Field `SEC2BB9` writer - page secure/non-secure attribution
pub type SEC2BB9_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_SEC2BBR2_SPEC, bool, O>;
///Field `SEC2BB10` reader - page secure/non-secure attribution
pub type SEC2BB10_R = crate::BitReader<bool>;
///Field `SEC2BB10` writer - page secure/non-secure attribution
pub type SEC2BB10_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_SEC2BBR2_SPEC, bool, O>;
///Field `SEC2BB11` reader - page secure/non-secure attribution
pub type SEC2BB11_R = crate::BitReader<bool>;
///Field `SEC2BB11` writer - page secure/non-secure attribution
pub type SEC2BB11_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_SEC2BBR2_SPEC, bool, O>;
///Field `SEC2BB12` reader - page secure/non-secure attribution
pub type SEC2BB12_R = crate::BitReader<bool>;
///Field `SEC2BB12` writer - page secure/non-secure attribution
pub type SEC2BB12_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_SEC2BBR2_SPEC, bool, O>;
///Field `SEC2BB13` reader - page secure/non-secure attribution
pub type SEC2BB13_R = crate::BitReader<bool>;
///Field `SEC2BB13` writer - page secure/non-secure attribution
pub type SEC2BB13_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_SEC2BBR2_SPEC, bool, O>;
///Field `SEC2BB14` reader - page secure/non-secure attribution
pub type SEC2BB14_R = crate::BitReader<bool>;
///Field `SEC2BB14` writer - page secure/non-secure attribution
pub type SEC2BB14_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_SEC2BBR2_SPEC, bool, O>;
///Field `SEC2BB15` reader - page secure/non-secure attribution
pub type SEC2BB15_R = crate::BitReader<bool>;
///Field `SEC2BB15` writer - page secure/non-secure attribution
pub type SEC2BB15_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_SEC2BBR2_SPEC, bool, O>;
///Field `SEC2BB16` reader - page secure/non-secure attribution
pub type SEC2BB16_R = crate::BitReader<bool>;
///Field `SEC2BB16` writer - page secure/non-secure attribution
pub type SEC2BB16_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_SEC2BBR2_SPEC, bool, O>;
///Field `SEC2BB17` reader - page secure/non-secure attribution
pub type SEC2BB17_R = crate::BitReader<bool>;
///Field `SEC2BB17` writer - page secure/non-secure attribution
pub type SEC2BB17_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_SEC2BBR2_SPEC, bool, O>;
///Field `SEC2BB18` reader - page secure/non-secure attribution
pub type SEC2BB18_R = crate::BitReader<bool>;
///Field `SEC2BB18` writer - page secure/non-secure attribution
pub type SEC2BB18_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_SEC2BBR2_SPEC, bool, O>;
///Field `SEC2BB19` reader - page secure/non-secure attribution
pub type SEC2BB19_R = crate::BitReader<bool>;
///Field `SEC2BB19` writer - page secure/non-secure attribution
pub type SEC2BB19_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_SEC2BBR2_SPEC, bool, O>;
///Field `SEC2BB20` reader - page secure/non-secure attribution
pub type SEC2BB20_R = crate::BitReader<bool>;
///Field `SEC2BB20` writer - page secure/non-secure attribution
pub type SEC2BB20_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_SEC2BBR2_SPEC, bool, O>;
///Field `SEC2BB21` reader - page secure/non-secure attribution
pub type SEC2BB21_R = crate::BitReader<bool>;
///Field `SEC2BB21` writer - page secure/non-secure attribution
pub type SEC2BB21_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_SEC2BBR2_SPEC, bool, O>;
///Field `SEC2BB22` reader - page secure/non-secure attribution
pub type SEC2BB22_R = crate::BitReader<bool>;
///Field `SEC2BB22` writer - page secure/non-secure attribution
pub type SEC2BB22_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_SEC2BBR2_SPEC, bool, O>;
///Field `SEC2BB23` reader - page secure/non-secure attribution
pub type SEC2BB23_R = crate::BitReader<bool>;
///Field `SEC2BB23` writer - page secure/non-secure attribution
pub type SEC2BB23_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_SEC2BBR2_SPEC, bool, O>;
///Field `SEC2BB24` reader - page secure/non-secure attribution
pub type SEC2BB24_R = crate::BitReader<bool>;
///Field `SEC2BB24` writer - page secure/non-secure attribution
pub type SEC2BB24_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_SEC2BBR2_SPEC, bool, O>;
///Field `SEC2BB25` reader - page secure/non-secure attribution
pub type SEC2BB25_R = crate::BitReader<bool>;
///Field `SEC2BB25` writer - page secure/non-secure attribution
pub type SEC2BB25_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_SEC2BBR2_SPEC, bool, O>;
///Field `SEC2BB26` reader - page secure/non-secure attribution
pub type SEC2BB26_R = crate::BitReader<bool>;
///Field `SEC2BB26` writer - page secure/non-secure attribution
pub type SEC2BB26_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_SEC2BBR2_SPEC, bool, O>;
///Field `SEC2BB27` reader - page secure/non-secure attribution
pub type SEC2BB27_R = crate::BitReader<bool>;
///Field `SEC2BB27` writer - page secure/non-secure attribution
pub type SEC2BB27_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_SEC2BBR2_SPEC, bool, O>;
///Field `SEC2BB28` reader - page secure/non-secure attribution
pub type SEC2BB28_R = crate::BitReader<bool>;
///Field `SEC2BB28` writer - page secure/non-secure attribution
pub type SEC2BB28_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_SEC2BBR2_SPEC, bool, O>;
///Field `SEC2BB29` reader - page secure/non-secure attribution
pub type SEC2BB29_R = crate::BitReader<bool>;
///Field `SEC2BB29` writer - page secure/non-secure attribution
pub type SEC2BB29_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_SEC2BBR2_SPEC, bool, O>;
///Field `SEC2BB30` reader - page secure/non-secure attribution
pub type SEC2BB30_R = crate::BitReader<bool>;
///Field `SEC2BB30` writer - page secure/non-secure attribution
pub type SEC2BB30_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_SEC2BBR2_SPEC, bool, O>;
///Field `SEC2BB31` reader - page secure/non-secure attribution
pub type SEC2BB31_R = crate::BitReader<bool>;
///Field `SEC2BB31` writer - page secure/non-secure attribution
pub type SEC2BB31_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_SEC2BBR2_SPEC, bool, O>;
impl R {
    ///Bit 0 - page secure/non-secure attribution
    #[inline(always)]
    pub fn sec2bb0(&self) -> SEC2BB0_R {
        SEC2BB0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - page secure/non-secure attribution
    #[inline(always)]
    pub fn sec2bb1(&self) -> SEC2BB1_R {
        SEC2BB1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - page secure/non-secure attribution
    #[inline(always)]
    pub fn sec2bb2(&self) -> SEC2BB2_R {
        SEC2BB2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - page secure/non-secure attribution
    #[inline(always)]
    pub fn sec2bb3(&self) -> SEC2BB3_R {
        SEC2BB3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - page secure/non-secure attribution
    #[inline(always)]
    pub fn sec2bb4(&self) -> SEC2BB4_R {
        SEC2BB4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - page secure/non-secure attribution
    #[inline(always)]
    pub fn sec2bb5(&self) -> SEC2BB5_R {
        SEC2BB5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - page secure/non-secure attribution
    #[inline(always)]
    pub fn sec2bb6(&self) -> SEC2BB6_R {
        SEC2BB6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - page secure/non-secure attribution
    #[inline(always)]
    pub fn sec2bb7(&self) -> SEC2BB7_R {
        SEC2BB7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - page secure/non-secure attribution
    #[inline(always)]
    pub fn sec2bb8(&self) -> SEC2BB8_R {
        SEC2BB8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - page secure/non-secure attribution
    #[inline(always)]
    pub fn sec2bb9(&self) -> SEC2BB9_R {
        SEC2BB9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - page secure/non-secure attribution
    #[inline(always)]
    pub fn sec2bb10(&self) -> SEC2BB10_R {
        SEC2BB10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - page secure/non-secure attribution
    #[inline(always)]
    pub fn sec2bb11(&self) -> SEC2BB11_R {
        SEC2BB11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - page secure/non-secure attribution
    #[inline(always)]
    pub fn sec2bb12(&self) -> SEC2BB12_R {
        SEC2BB12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - page secure/non-secure attribution
    #[inline(always)]
    pub fn sec2bb13(&self) -> SEC2BB13_R {
        SEC2BB13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - page secure/non-secure attribution
    #[inline(always)]
    pub fn sec2bb14(&self) -> SEC2BB14_R {
        SEC2BB14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - page secure/non-secure attribution
    #[inline(always)]
    pub fn sec2bb15(&self) -> SEC2BB15_R {
        SEC2BB15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - page secure/non-secure attribution
    #[inline(always)]
    pub fn sec2bb16(&self) -> SEC2BB16_R {
        SEC2BB16_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - page secure/non-secure attribution
    #[inline(always)]
    pub fn sec2bb17(&self) -> SEC2BB17_R {
        SEC2BB17_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - page secure/non-secure attribution
    #[inline(always)]
    pub fn sec2bb18(&self) -> SEC2BB18_R {
        SEC2BB18_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - page secure/non-secure attribution
    #[inline(always)]
    pub fn sec2bb19(&self) -> SEC2BB19_R {
        SEC2BB19_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - page secure/non-secure attribution
    #[inline(always)]
    pub fn sec2bb20(&self) -> SEC2BB20_R {
        SEC2BB20_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - page secure/non-secure attribution
    #[inline(always)]
    pub fn sec2bb21(&self) -> SEC2BB21_R {
        SEC2BB21_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - page secure/non-secure attribution
    #[inline(always)]
    pub fn sec2bb22(&self) -> SEC2BB22_R {
        SEC2BB22_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - page secure/non-secure attribution
    #[inline(always)]
    pub fn sec2bb23(&self) -> SEC2BB23_R {
        SEC2BB23_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - page secure/non-secure attribution
    #[inline(always)]
    pub fn sec2bb24(&self) -> SEC2BB24_R {
        SEC2BB24_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - page secure/non-secure attribution
    #[inline(always)]
    pub fn sec2bb25(&self) -> SEC2BB25_R {
        SEC2BB25_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - page secure/non-secure attribution
    #[inline(always)]
    pub fn sec2bb26(&self) -> SEC2BB26_R {
        SEC2BB26_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - page secure/non-secure attribution
    #[inline(always)]
    pub fn sec2bb27(&self) -> SEC2BB27_R {
        SEC2BB27_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - page secure/non-secure attribution
    #[inline(always)]
    pub fn sec2bb28(&self) -> SEC2BB28_R {
        SEC2BB28_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - page secure/non-secure attribution
    #[inline(always)]
    pub fn sec2bb29(&self) -> SEC2BB29_R {
        SEC2BB29_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - page secure/non-secure attribution
    #[inline(always)]
    pub fn sec2bb30(&self) -> SEC2BB30_R {
        SEC2BB30_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - page secure/non-secure attribution
    #[inline(always)]
    pub fn sec2bb31(&self) -> SEC2BB31_R {
        SEC2BB31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - page secure/non-secure attribution
    #[inline(always)]
    #[must_use]
    pub fn sec2bb0(&mut self) -> SEC2BB0_W<0> {
        SEC2BB0_W::new(self)
    }
    ///Bit 1 - page secure/non-secure attribution
    #[inline(always)]
    #[must_use]
    pub fn sec2bb1(&mut self) -> SEC2BB1_W<1> {
        SEC2BB1_W::new(self)
    }
    ///Bit 2 - page secure/non-secure attribution
    #[inline(always)]
    #[must_use]
    pub fn sec2bb2(&mut self) -> SEC2BB2_W<2> {
        SEC2BB2_W::new(self)
    }
    ///Bit 3 - page secure/non-secure attribution
    #[inline(always)]
    #[must_use]
    pub fn sec2bb3(&mut self) -> SEC2BB3_W<3> {
        SEC2BB3_W::new(self)
    }
    ///Bit 4 - page secure/non-secure attribution
    #[inline(always)]
    #[must_use]
    pub fn sec2bb4(&mut self) -> SEC2BB4_W<4> {
        SEC2BB4_W::new(self)
    }
    ///Bit 5 - page secure/non-secure attribution
    #[inline(always)]
    #[must_use]
    pub fn sec2bb5(&mut self) -> SEC2BB5_W<5> {
        SEC2BB5_W::new(self)
    }
    ///Bit 6 - page secure/non-secure attribution
    #[inline(always)]
    #[must_use]
    pub fn sec2bb6(&mut self) -> SEC2BB6_W<6> {
        SEC2BB6_W::new(self)
    }
    ///Bit 7 - page secure/non-secure attribution
    #[inline(always)]
    #[must_use]
    pub fn sec2bb7(&mut self) -> SEC2BB7_W<7> {
        SEC2BB7_W::new(self)
    }
    ///Bit 8 - page secure/non-secure attribution
    #[inline(always)]
    #[must_use]
    pub fn sec2bb8(&mut self) -> SEC2BB8_W<8> {
        SEC2BB8_W::new(self)
    }
    ///Bit 9 - page secure/non-secure attribution
    #[inline(always)]
    #[must_use]
    pub fn sec2bb9(&mut self) -> SEC2BB9_W<9> {
        SEC2BB9_W::new(self)
    }
    ///Bit 10 - page secure/non-secure attribution
    #[inline(always)]
    #[must_use]
    pub fn sec2bb10(&mut self) -> SEC2BB10_W<10> {
        SEC2BB10_W::new(self)
    }
    ///Bit 11 - page secure/non-secure attribution
    #[inline(always)]
    #[must_use]
    pub fn sec2bb11(&mut self) -> SEC2BB11_W<11> {
        SEC2BB11_W::new(self)
    }
    ///Bit 12 - page secure/non-secure attribution
    #[inline(always)]
    #[must_use]
    pub fn sec2bb12(&mut self) -> SEC2BB12_W<12> {
        SEC2BB12_W::new(self)
    }
    ///Bit 13 - page secure/non-secure attribution
    #[inline(always)]
    #[must_use]
    pub fn sec2bb13(&mut self) -> SEC2BB13_W<13> {
        SEC2BB13_W::new(self)
    }
    ///Bit 14 - page secure/non-secure attribution
    #[inline(always)]
    #[must_use]
    pub fn sec2bb14(&mut self) -> SEC2BB14_W<14> {
        SEC2BB14_W::new(self)
    }
    ///Bit 15 - page secure/non-secure attribution
    #[inline(always)]
    #[must_use]
    pub fn sec2bb15(&mut self) -> SEC2BB15_W<15> {
        SEC2BB15_W::new(self)
    }
    ///Bit 16 - page secure/non-secure attribution
    #[inline(always)]
    #[must_use]
    pub fn sec2bb16(&mut self) -> SEC2BB16_W<16> {
        SEC2BB16_W::new(self)
    }
    ///Bit 17 - page secure/non-secure attribution
    #[inline(always)]
    #[must_use]
    pub fn sec2bb17(&mut self) -> SEC2BB17_W<17> {
        SEC2BB17_W::new(self)
    }
    ///Bit 18 - page secure/non-secure attribution
    #[inline(always)]
    #[must_use]
    pub fn sec2bb18(&mut self) -> SEC2BB18_W<18> {
        SEC2BB18_W::new(self)
    }
    ///Bit 19 - page secure/non-secure attribution
    #[inline(always)]
    #[must_use]
    pub fn sec2bb19(&mut self) -> SEC2BB19_W<19> {
        SEC2BB19_W::new(self)
    }
    ///Bit 20 - page secure/non-secure attribution
    #[inline(always)]
    #[must_use]
    pub fn sec2bb20(&mut self) -> SEC2BB20_W<20> {
        SEC2BB20_W::new(self)
    }
    ///Bit 21 - page secure/non-secure attribution
    #[inline(always)]
    #[must_use]
    pub fn sec2bb21(&mut self) -> SEC2BB21_W<21> {
        SEC2BB21_W::new(self)
    }
    ///Bit 22 - page secure/non-secure attribution
    #[inline(always)]
    #[must_use]
    pub fn sec2bb22(&mut self) -> SEC2BB22_W<22> {
        SEC2BB22_W::new(self)
    }
    ///Bit 23 - page secure/non-secure attribution
    #[inline(always)]
    #[must_use]
    pub fn sec2bb23(&mut self) -> SEC2BB23_W<23> {
        SEC2BB23_W::new(self)
    }
    ///Bit 24 - page secure/non-secure attribution
    #[inline(always)]
    #[must_use]
    pub fn sec2bb24(&mut self) -> SEC2BB24_W<24> {
        SEC2BB24_W::new(self)
    }
    ///Bit 25 - page secure/non-secure attribution
    #[inline(always)]
    #[must_use]
    pub fn sec2bb25(&mut self) -> SEC2BB25_W<25> {
        SEC2BB25_W::new(self)
    }
    ///Bit 26 - page secure/non-secure attribution
    #[inline(always)]
    #[must_use]
    pub fn sec2bb26(&mut self) -> SEC2BB26_W<26> {
        SEC2BB26_W::new(self)
    }
    ///Bit 27 - page secure/non-secure attribution
    #[inline(always)]
    #[must_use]
    pub fn sec2bb27(&mut self) -> SEC2BB27_W<27> {
        SEC2BB27_W::new(self)
    }
    ///Bit 28 - page secure/non-secure attribution
    #[inline(always)]
    #[must_use]
    pub fn sec2bb28(&mut self) -> SEC2BB28_W<28> {
        SEC2BB28_W::new(self)
    }
    ///Bit 29 - page secure/non-secure attribution
    #[inline(always)]
    #[must_use]
    pub fn sec2bb29(&mut self) -> SEC2BB29_W<29> {
        SEC2BB29_W::new(self)
    }
    ///Bit 30 - page secure/non-secure attribution
    #[inline(always)]
    #[must_use]
    pub fn sec2bb30(&mut self) -> SEC2BB30_W<30> {
        SEC2BB30_W::new(self)
    }
    ///Bit 31 - page secure/non-secure attribution
    #[inline(always)]
    #[must_use]
    pub fn sec2bb31(&mut self) -> SEC2BB31_W<31> {
        SEC2BB31_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FLASH secure block based bank 2 register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [flash_sec2bbr2](index.html) module
pub struct FLASH_SEC2BBR2_SPEC;
impl crate::RegisterSpec for FLASH_SEC2BBR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [flash_sec2bbr2::R](R) reader structure
impl crate::Readable for FLASH_SEC2BBR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [flash_sec2bbr2::W](W) writer structure
impl crate::Writable for FLASH_SEC2BBR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets FLASH_SEC2BBR2 to value 0
impl crate::Resettable for FLASH_SEC2BBR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
