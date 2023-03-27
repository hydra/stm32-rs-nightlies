///Register `FLASH_SEC1BBR4` reader
pub struct R(crate::R<FLASH_SEC1BBR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASH_SEC1BBR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASH_SEC1BBR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASH_SEC1BBR4_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FLASH_SEC1BBR4` writer
pub struct W(crate::W<FLASH_SEC1BBR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASH_SEC1BBR4_SPEC>;
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
impl From<crate::W<FLASH_SEC1BBR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASH_SEC1BBR4_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SEC1BB0` reader - page secure/non-secure attribution
pub type SEC1BB0_R = crate::BitReader<bool>;
///Field `SEC1BB0` writer - page secure/non-secure attribution
pub type SEC1BB0_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_SEC1BBR4_SPEC, bool, O>;
///Field `SEC1BB1` reader - page secure/non-secure attribution
pub type SEC1BB1_R = crate::BitReader<bool>;
///Field `SEC1BB1` writer - page secure/non-secure attribution
pub type SEC1BB1_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_SEC1BBR4_SPEC, bool, O>;
///Field `SEC1BB2` reader - page secure/non-secure attribution
pub type SEC1BB2_R = crate::BitReader<bool>;
///Field `SEC1BB2` writer - page secure/non-secure attribution
pub type SEC1BB2_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_SEC1BBR4_SPEC, bool, O>;
///Field `SEC1BB3` reader - page secure/non-secure attribution
pub type SEC1BB3_R = crate::BitReader<bool>;
///Field `SEC1BB3` writer - page secure/non-secure attribution
pub type SEC1BB3_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_SEC1BBR4_SPEC, bool, O>;
///Field `SEC1BB4` reader - page secure/non-secure attribution
pub type SEC1BB4_R = crate::BitReader<bool>;
///Field `SEC1BB4` writer - page secure/non-secure attribution
pub type SEC1BB4_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_SEC1BBR4_SPEC, bool, O>;
///Field `SEC1BB5` reader - page secure/non-secure attribution
pub type SEC1BB5_R = crate::BitReader<bool>;
///Field `SEC1BB5` writer - page secure/non-secure attribution
pub type SEC1BB5_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_SEC1BBR4_SPEC, bool, O>;
///Field `SEC1BB6` reader - page secure/non-secure attribution
pub type SEC1BB6_R = crate::BitReader<bool>;
///Field `SEC1BB6` writer - page secure/non-secure attribution
pub type SEC1BB6_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_SEC1BBR4_SPEC, bool, O>;
///Field `SEC1BB7` reader - page secure/non-secure attribution
pub type SEC1BB7_R = crate::BitReader<bool>;
///Field `SEC1BB7` writer - page secure/non-secure attribution
pub type SEC1BB7_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_SEC1BBR4_SPEC, bool, O>;
///Field `SEC1BB8` reader - page secure/non-secure attribution
pub type SEC1BB8_R = crate::BitReader<bool>;
///Field `SEC1BB8` writer - page secure/non-secure attribution
pub type SEC1BB8_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_SEC1BBR4_SPEC, bool, O>;
///Field `SEC1BB9` reader - page secure/non-secure attribution
pub type SEC1BB9_R = crate::BitReader<bool>;
///Field `SEC1BB9` writer - page secure/non-secure attribution
pub type SEC1BB9_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_SEC1BBR4_SPEC, bool, O>;
///Field `SEC1BB10` reader - page secure/non-secure attribution
pub type SEC1BB10_R = crate::BitReader<bool>;
///Field `SEC1BB10` writer - page secure/non-secure attribution
pub type SEC1BB10_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_SEC1BBR4_SPEC, bool, O>;
///Field `SEC1BB11` reader - page secure/non-secure attribution
pub type SEC1BB11_R = crate::BitReader<bool>;
///Field `SEC1BB11` writer - page secure/non-secure attribution
pub type SEC1BB11_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_SEC1BBR4_SPEC, bool, O>;
///Field `SEC1BB12` reader - page secure/non-secure attribution
pub type SEC1BB12_R = crate::BitReader<bool>;
///Field `SEC1BB12` writer - page secure/non-secure attribution
pub type SEC1BB12_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_SEC1BBR4_SPEC, bool, O>;
///Field `SEC1BB13` reader - page secure/non-secure attribution
pub type SEC1BB13_R = crate::BitReader<bool>;
///Field `SEC1BB13` writer - page secure/non-secure attribution
pub type SEC1BB13_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_SEC1BBR4_SPEC, bool, O>;
///Field `SEC1BB14` reader - page secure/non-secure attribution
pub type SEC1BB14_R = crate::BitReader<bool>;
///Field `SEC1BB14` writer - page secure/non-secure attribution
pub type SEC1BB14_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_SEC1BBR4_SPEC, bool, O>;
///Field `SEC1BB15` reader - page secure/non-secure attribution
pub type SEC1BB15_R = crate::BitReader<bool>;
///Field `SEC1BB15` writer - page secure/non-secure attribution
pub type SEC1BB15_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_SEC1BBR4_SPEC, bool, O>;
///Field `SEC1BB16` reader - page secure/non-secure attribution
pub type SEC1BB16_R = crate::BitReader<bool>;
///Field `SEC1BB16` writer - page secure/non-secure attribution
pub type SEC1BB16_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_SEC1BBR4_SPEC, bool, O>;
///Field `SEC1BB17` reader - page secure/non-secure attribution
pub type SEC1BB17_R = crate::BitReader<bool>;
///Field `SEC1BB17` writer - page secure/non-secure attribution
pub type SEC1BB17_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_SEC1BBR4_SPEC, bool, O>;
///Field `SEC1BB18` reader - page secure/non-secure attribution
pub type SEC1BB18_R = crate::BitReader<bool>;
///Field `SEC1BB18` writer - page secure/non-secure attribution
pub type SEC1BB18_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_SEC1BBR4_SPEC, bool, O>;
///Field `SEC1BB19` reader - page secure/non-secure attribution
pub type SEC1BB19_R = crate::BitReader<bool>;
///Field `SEC1BB19` writer - page secure/non-secure attribution
pub type SEC1BB19_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_SEC1BBR4_SPEC, bool, O>;
///Field `SEC1BB20` reader - page secure/non-secure attribution
pub type SEC1BB20_R = crate::BitReader<bool>;
///Field `SEC1BB20` writer - page secure/non-secure attribution
pub type SEC1BB20_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_SEC1BBR4_SPEC, bool, O>;
///Field `SEC1BB21` reader - page secure/non-secure attribution
pub type SEC1BB21_R = crate::BitReader<bool>;
///Field `SEC1BB21` writer - page secure/non-secure attribution
pub type SEC1BB21_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_SEC1BBR4_SPEC, bool, O>;
///Field `SEC1BB22` reader - page secure/non-secure attribution
pub type SEC1BB22_R = crate::BitReader<bool>;
///Field `SEC1BB22` writer - page secure/non-secure attribution
pub type SEC1BB22_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_SEC1BBR4_SPEC, bool, O>;
///Field `SEC1BB23` reader - page secure/non-secure attribution
pub type SEC1BB23_R = crate::BitReader<bool>;
///Field `SEC1BB23` writer - page secure/non-secure attribution
pub type SEC1BB23_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_SEC1BBR4_SPEC, bool, O>;
///Field `SEC1BB24` reader - page secure/non-secure attribution
pub type SEC1BB24_R = crate::BitReader<bool>;
///Field `SEC1BB24` writer - page secure/non-secure attribution
pub type SEC1BB24_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_SEC1BBR4_SPEC, bool, O>;
///Field `SEC1BB25` reader - page secure/non-secure attribution
pub type SEC1BB25_R = crate::BitReader<bool>;
///Field `SEC1BB25` writer - page secure/non-secure attribution
pub type SEC1BB25_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_SEC1BBR4_SPEC, bool, O>;
///Field `SEC1BB26` reader - page secure/non-secure attribution
pub type SEC1BB26_R = crate::BitReader<bool>;
///Field `SEC1BB26` writer - page secure/non-secure attribution
pub type SEC1BB26_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_SEC1BBR4_SPEC, bool, O>;
///Field `SEC1BB27` reader - page secure/non-secure attribution
pub type SEC1BB27_R = crate::BitReader<bool>;
///Field `SEC1BB27` writer - page secure/non-secure attribution
pub type SEC1BB27_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_SEC1BBR4_SPEC, bool, O>;
///Field `SEC1BB28` reader - page secure/non-secure attribution
pub type SEC1BB28_R = crate::BitReader<bool>;
///Field `SEC1BB28` writer - page secure/non-secure attribution
pub type SEC1BB28_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_SEC1BBR4_SPEC, bool, O>;
///Field `SEC1BB29` reader - page secure/non-secure attribution
pub type SEC1BB29_R = crate::BitReader<bool>;
///Field `SEC1BB29` writer - page secure/non-secure attribution
pub type SEC1BB29_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_SEC1BBR4_SPEC, bool, O>;
///Field `SEC1BB30` reader - page secure/non-secure attribution
pub type SEC1BB30_R = crate::BitReader<bool>;
///Field `SEC1BB30` writer - page secure/non-secure attribution
pub type SEC1BB30_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_SEC1BBR4_SPEC, bool, O>;
///Field `SEC1BB31` reader - page secure/non-secure attribution
pub type SEC1BB31_R = crate::BitReader<bool>;
///Field `SEC1BB31` writer - page secure/non-secure attribution
pub type SEC1BB31_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_SEC1BBR4_SPEC, bool, O>;
impl R {
    ///Bit 0 - page secure/non-secure attribution
    #[inline(always)]
    pub fn sec1bb0(&self) -> SEC1BB0_R {
        SEC1BB0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - page secure/non-secure attribution
    #[inline(always)]
    pub fn sec1bb1(&self) -> SEC1BB1_R {
        SEC1BB1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - page secure/non-secure attribution
    #[inline(always)]
    pub fn sec1bb2(&self) -> SEC1BB2_R {
        SEC1BB2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - page secure/non-secure attribution
    #[inline(always)]
    pub fn sec1bb3(&self) -> SEC1BB3_R {
        SEC1BB3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - page secure/non-secure attribution
    #[inline(always)]
    pub fn sec1bb4(&self) -> SEC1BB4_R {
        SEC1BB4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - page secure/non-secure attribution
    #[inline(always)]
    pub fn sec1bb5(&self) -> SEC1BB5_R {
        SEC1BB5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - page secure/non-secure attribution
    #[inline(always)]
    pub fn sec1bb6(&self) -> SEC1BB6_R {
        SEC1BB6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - page secure/non-secure attribution
    #[inline(always)]
    pub fn sec1bb7(&self) -> SEC1BB7_R {
        SEC1BB7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - page secure/non-secure attribution
    #[inline(always)]
    pub fn sec1bb8(&self) -> SEC1BB8_R {
        SEC1BB8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - page secure/non-secure attribution
    #[inline(always)]
    pub fn sec1bb9(&self) -> SEC1BB9_R {
        SEC1BB9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - page secure/non-secure attribution
    #[inline(always)]
    pub fn sec1bb10(&self) -> SEC1BB10_R {
        SEC1BB10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - page secure/non-secure attribution
    #[inline(always)]
    pub fn sec1bb11(&self) -> SEC1BB11_R {
        SEC1BB11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - page secure/non-secure attribution
    #[inline(always)]
    pub fn sec1bb12(&self) -> SEC1BB12_R {
        SEC1BB12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - page secure/non-secure attribution
    #[inline(always)]
    pub fn sec1bb13(&self) -> SEC1BB13_R {
        SEC1BB13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - page secure/non-secure attribution
    #[inline(always)]
    pub fn sec1bb14(&self) -> SEC1BB14_R {
        SEC1BB14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - page secure/non-secure attribution
    #[inline(always)]
    pub fn sec1bb15(&self) -> SEC1BB15_R {
        SEC1BB15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - page secure/non-secure attribution
    #[inline(always)]
    pub fn sec1bb16(&self) -> SEC1BB16_R {
        SEC1BB16_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - page secure/non-secure attribution
    #[inline(always)]
    pub fn sec1bb17(&self) -> SEC1BB17_R {
        SEC1BB17_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - page secure/non-secure attribution
    #[inline(always)]
    pub fn sec1bb18(&self) -> SEC1BB18_R {
        SEC1BB18_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - page secure/non-secure attribution
    #[inline(always)]
    pub fn sec1bb19(&self) -> SEC1BB19_R {
        SEC1BB19_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - page secure/non-secure attribution
    #[inline(always)]
    pub fn sec1bb20(&self) -> SEC1BB20_R {
        SEC1BB20_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - page secure/non-secure attribution
    #[inline(always)]
    pub fn sec1bb21(&self) -> SEC1BB21_R {
        SEC1BB21_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - page secure/non-secure attribution
    #[inline(always)]
    pub fn sec1bb22(&self) -> SEC1BB22_R {
        SEC1BB22_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - page secure/non-secure attribution
    #[inline(always)]
    pub fn sec1bb23(&self) -> SEC1BB23_R {
        SEC1BB23_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - page secure/non-secure attribution
    #[inline(always)]
    pub fn sec1bb24(&self) -> SEC1BB24_R {
        SEC1BB24_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - page secure/non-secure attribution
    #[inline(always)]
    pub fn sec1bb25(&self) -> SEC1BB25_R {
        SEC1BB25_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - page secure/non-secure attribution
    #[inline(always)]
    pub fn sec1bb26(&self) -> SEC1BB26_R {
        SEC1BB26_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - page secure/non-secure attribution
    #[inline(always)]
    pub fn sec1bb27(&self) -> SEC1BB27_R {
        SEC1BB27_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - page secure/non-secure attribution
    #[inline(always)]
    pub fn sec1bb28(&self) -> SEC1BB28_R {
        SEC1BB28_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - page secure/non-secure attribution
    #[inline(always)]
    pub fn sec1bb29(&self) -> SEC1BB29_R {
        SEC1BB29_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - page secure/non-secure attribution
    #[inline(always)]
    pub fn sec1bb30(&self) -> SEC1BB30_R {
        SEC1BB30_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - page secure/non-secure attribution
    #[inline(always)]
    pub fn sec1bb31(&self) -> SEC1BB31_R {
        SEC1BB31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - page secure/non-secure attribution
    #[inline(always)]
    #[must_use]
    pub fn sec1bb0(&mut self) -> SEC1BB0_W<0> {
        SEC1BB0_W::new(self)
    }
    ///Bit 1 - page secure/non-secure attribution
    #[inline(always)]
    #[must_use]
    pub fn sec1bb1(&mut self) -> SEC1BB1_W<1> {
        SEC1BB1_W::new(self)
    }
    ///Bit 2 - page secure/non-secure attribution
    #[inline(always)]
    #[must_use]
    pub fn sec1bb2(&mut self) -> SEC1BB2_W<2> {
        SEC1BB2_W::new(self)
    }
    ///Bit 3 - page secure/non-secure attribution
    #[inline(always)]
    #[must_use]
    pub fn sec1bb3(&mut self) -> SEC1BB3_W<3> {
        SEC1BB3_W::new(self)
    }
    ///Bit 4 - page secure/non-secure attribution
    #[inline(always)]
    #[must_use]
    pub fn sec1bb4(&mut self) -> SEC1BB4_W<4> {
        SEC1BB4_W::new(self)
    }
    ///Bit 5 - page secure/non-secure attribution
    #[inline(always)]
    #[must_use]
    pub fn sec1bb5(&mut self) -> SEC1BB5_W<5> {
        SEC1BB5_W::new(self)
    }
    ///Bit 6 - page secure/non-secure attribution
    #[inline(always)]
    #[must_use]
    pub fn sec1bb6(&mut self) -> SEC1BB6_W<6> {
        SEC1BB6_W::new(self)
    }
    ///Bit 7 - page secure/non-secure attribution
    #[inline(always)]
    #[must_use]
    pub fn sec1bb7(&mut self) -> SEC1BB7_W<7> {
        SEC1BB7_W::new(self)
    }
    ///Bit 8 - page secure/non-secure attribution
    #[inline(always)]
    #[must_use]
    pub fn sec1bb8(&mut self) -> SEC1BB8_W<8> {
        SEC1BB8_W::new(self)
    }
    ///Bit 9 - page secure/non-secure attribution
    #[inline(always)]
    #[must_use]
    pub fn sec1bb9(&mut self) -> SEC1BB9_W<9> {
        SEC1BB9_W::new(self)
    }
    ///Bit 10 - page secure/non-secure attribution
    #[inline(always)]
    #[must_use]
    pub fn sec1bb10(&mut self) -> SEC1BB10_W<10> {
        SEC1BB10_W::new(self)
    }
    ///Bit 11 - page secure/non-secure attribution
    #[inline(always)]
    #[must_use]
    pub fn sec1bb11(&mut self) -> SEC1BB11_W<11> {
        SEC1BB11_W::new(self)
    }
    ///Bit 12 - page secure/non-secure attribution
    #[inline(always)]
    #[must_use]
    pub fn sec1bb12(&mut self) -> SEC1BB12_W<12> {
        SEC1BB12_W::new(self)
    }
    ///Bit 13 - page secure/non-secure attribution
    #[inline(always)]
    #[must_use]
    pub fn sec1bb13(&mut self) -> SEC1BB13_W<13> {
        SEC1BB13_W::new(self)
    }
    ///Bit 14 - page secure/non-secure attribution
    #[inline(always)]
    #[must_use]
    pub fn sec1bb14(&mut self) -> SEC1BB14_W<14> {
        SEC1BB14_W::new(self)
    }
    ///Bit 15 - page secure/non-secure attribution
    #[inline(always)]
    #[must_use]
    pub fn sec1bb15(&mut self) -> SEC1BB15_W<15> {
        SEC1BB15_W::new(self)
    }
    ///Bit 16 - page secure/non-secure attribution
    #[inline(always)]
    #[must_use]
    pub fn sec1bb16(&mut self) -> SEC1BB16_W<16> {
        SEC1BB16_W::new(self)
    }
    ///Bit 17 - page secure/non-secure attribution
    #[inline(always)]
    #[must_use]
    pub fn sec1bb17(&mut self) -> SEC1BB17_W<17> {
        SEC1BB17_W::new(self)
    }
    ///Bit 18 - page secure/non-secure attribution
    #[inline(always)]
    #[must_use]
    pub fn sec1bb18(&mut self) -> SEC1BB18_W<18> {
        SEC1BB18_W::new(self)
    }
    ///Bit 19 - page secure/non-secure attribution
    #[inline(always)]
    #[must_use]
    pub fn sec1bb19(&mut self) -> SEC1BB19_W<19> {
        SEC1BB19_W::new(self)
    }
    ///Bit 20 - page secure/non-secure attribution
    #[inline(always)]
    #[must_use]
    pub fn sec1bb20(&mut self) -> SEC1BB20_W<20> {
        SEC1BB20_W::new(self)
    }
    ///Bit 21 - page secure/non-secure attribution
    #[inline(always)]
    #[must_use]
    pub fn sec1bb21(&mut self) -> SEC1BB21_W<21> {
        SEC1BB21_W::new(self)
    }
    ///Bit 22 - page secure/non-secure attribution
    #[inline(always)]
    #[must_use]
    pub fn sec1bb22(&mut self) -> SEC1BB22_W<22> {
        SEC1BB22_W::new(self)
    }
    ///Bit 23 - page secure/non-secure attribution
    #[inline(always)]
    #[must_use]
    pub fn sec1bb23(&mut self) -> SEC1BB23_W<23> {
        SEC1BB23_W::new(self)
    }
    ///Bit 24 - page secure/non-secure attribution
    #[inline(always)]
    #[must_use]
    pub fn sec1bb24(&mut self) -> SEC1BB24_W<24> {
        SEC1BB24_W::new(self)
    }
    ///Bit 25 - page secure/non-secure attribution
    #[inline(always)]
    #[must_use]
    pub fn sec1bb25(&mut self) -> SEC1BB25_W<25> {
        SEC1BB25_W::new(self)
    }
    ///Bit 26 - page secure/non-secure attribution
    #[inline(always)]
    #[must_use]
    pub fn sec1bb26(&mut self) -> SEC1BB26_W<26> {
        SEC1BB26_W::new(self)
    }
    ///Bit 27 - page secure/non-secure attribution
    #[inline(always)]
    #[must_use]
    pub fn sec1bb27(&mut self) -> SEC1BB27_W<27> {
        SEC1BB27_W::new(self)
    }
    ///Bit 28 - page secure/non-secure attribution
    #[inline(always)]
    #[must_use]
    pub fn sec1bb28(&mut self) -> SEC1BB28_W<28> {
        SEC1BB28_W::new(self)
    }
    ///Bit 29 - page secure/non-secure attribution
    #[inline(always)]
    #[must_use]
    pub fn sec1bb29(&mut self) -> SEC1BB29_W<29> {
        SEC1BB29_W::new(self)
    }
    ///Bit 30 - page secure/non-secure attribution
    #[inline(always)]
    #[must_use]
    pub fn sec1bb30(&mut self) -> SEC1BB30_W<30> {
        SEC1BB30_W::new(self)
    }
    ///Bit 31 - page secure/non-secure attribution
    #[inline(always)]
    #[must_use]
    pub fn sec1bb31(&mut self) -> SEC1BB31_W<31> {
        SEC1BB31_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FLASH secure block based bank 1 register 4
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [flash_sec1bbr4](index.html) module
pub struct FLASH_SEC1BBR4_SPEC;
impl crate::RegisterSpec for FLASH_SEC1BBR4_SPEC {
    type Ux = u32;
}
///`read()` method returns [flash_sec1bbr4::R](R) reader structure
impl crate::Readable for FLASH_SEC1BBR4_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [flash_sec1bbr4::W](W) writer structure
impl crate::Writable for FLASH_SEC1BBR4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets FLASH_SEC1BBR4 to value 0
impl crate::Resettable for FLASH_SEC1BBR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
