///Register `FLASH_PRIV2BBR3` reader
pub struct R(crate::R<FLASH_PRIV2BBR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASH_PRIV2BBR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASH_PRIV2BBR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASH_PRIV2BBR3_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FLASH_PRIV2BBR3` writer
pub struct W(crate::W<FLASH_PRIV2BBR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASH_PRIV2BBR3_SPEC>;
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
impl From<crate::W<FLASH_PRIV2BBR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASH_PRIV2BBR3_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PRIV2BB0` reader - page privileged/unprivileged attribution
pub type PRIV2BB0_R = crate::BitReader<bool>;
///Field `PRIV2BB0` writer - page privileged/unprivileged attribution
pub type PRIV2BB0_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_PRIV2BBR3_SPEC, bool, O>;
///Field `PRIV2BB1` reader - page privileged/unprivileged attribution
pub type PRIV2BB1_R = crate::BitReader<bool>;
///Field `PRIV2BB1` writer - page privileged/unprivileged attribution
pub type PRIV2BB1_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_PRIV2BBR3_SPEC, bool, O>;
///Field `PRIV2BB2` reader - page privileged/unprivileged attribution
pub type PRIV2BB2_R = crate::BitReader<bool>;
///Field `PRIV2BB2` writer - page privileged/unprivileged attribution
pub type PRIV2BB2_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_PRIV2BBR3_SPEC, bool, O>;
///Field `PRIV2BB3` reader - page privileged/unprivileged attribution
pub type PRIV2BB3_R = crate::BitReader<bool>;
///Field `PRIV2BB3` writer - page privileged/unprivileged attribution
pub type PRIV2BB3_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_PRIV2BBR3_SPEC, bool, O>;
///Field `PRIV2BB4` reader - page privileged/unprivileged attribution
pub type PRIV2BB4_R = crate::BitReader<bool>;
///Field `PRIV2BB4` writer - page privileged/unprivileged attribution
pub type PRIV2BB4_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_PRIV2BBR3_SPEC, bool, O>;
///Field `PRIV2BB5` reader - page privileged/unprivileged attribution
pub type PRIV2BB5_R = crate::BitReader<bool>;
///Field `PRIV2BB5` writer - page privileged/unprivileged attribution
pub type PRIV2BB5_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_PRIV2BBR3_SPEC, bool, O>;
///Field `PRIV2BB6` reader - page privileged/unprivileged attribution
pub type PRIV2BB6_R = crate::BitReader<bool>;
///Field `PRIV2BB6` writer - page privileged/unprivileged attribution
pub type PRIV2BB6_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_PRIV2BBR3_SPEC, bool, O>;
///Field `PRIV2BB7` reader - page privileged/unprivileged attribution
pub type PRIV2BB7_R = crate::BitReader<bool>;
///Field `PRIV2BB7` writer - page privileged/unprivileged attribution
pub type PRIV2BB7_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_PRIV2BBR3_SPEC, bool, O>;
///Field `PRIV2BB8` reader - page privileged/unprivileged attribution
pub type PRIV2BB8_R = crate::BitReader<bool>;
///Field `PRIV2BB8` writer - page privileged/unprivileged attribution
pub type PRIV2BB8_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_PRIV2BBR3_SPEC, bool, O>;
///Field `PRIV2BB9` reader - page privileged/unprivileged attribution
pub type PRIV2BB9_R = crate::BitReader<bool>;
///Field `PRIV2BB9` writer - page privileged/unprivileged attribution
pub type PRIV2BB9_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_PRIV2BBR3_SPEC, bool, O>;
///Field `PRIV2BB10` reader - page privileged/unprivileged attribution
pub type PRIV2BB10_R = crate::BitReader<bool>;
///Field `PRIV2BB10` writer - page privileged/unprivileged attribution
pub type PRIV2BB10_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_PRIV2BBR3_SPEC, bool, O>;
///Field `PRIV2BB11` reader - page privileged/unprivileged attribution
pub type PRIV2BB11_R = crate::BitReader<bool>;
///Field `PRIV2BB11` writer - page privileged/unprivileged attribution
pub type PRIV2BB11_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_PRIV2BBR3_SPEC, bool, O>;
///Field `PRIV2BB12` reader - page privileged/unprivileged attribution
pub type PRIV2BB12_R = crate::BitReader<bool>;
///Field `PRIV2BB12` writer - page privileged/unprivileged attribution
pub type PRIV2BB12_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_PRIV2BBR3_SPEC, bool, O>;
///Field `PRIV2BB13` reader - page privileged/unprivileged attribution
pub type PRIV2BB13_R = crate::BitReader<bool>;
///Field `PRIV2BB13` writer - page privileged/unprivileged attribution
pub type PRIV2BB13_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_PRIV2BBR3_SPEC, bool, O>;
///Field `PRIV2BB14` reader - page privileged/unprivileged attribution
pub type PRIV2BB14_R = crate::BitReader<bool>;
///Field `PRIV2BB14` writer - page privileged/unprivileged attribution
pub type PRIV2BB14_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_PRIV2BBR3_SPEC, bool, O>;
///Field `PRIV2BB15` reader - page privileged/unprivileged attribution
pub type PRIV2BB15_R = crate::BitReader<bool>;
///Field `PRIV2BB15` writer - page privileged/unprivileged attribution
pub type PRIV2BB15_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_PRIV2BBR3_SPEC, bool, O>;
///Field `PRIV2BB16` reader - page privileged/unprivileged attribution
pub type PRIV2BB16_R = crate::BitReader<bool>;
///Field `PRIV2BB16` writer - page privileged/unprivileged attribution
pub type PRIV2BB16_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_PRIV2BBR3_SPEC, bool, O>;
///Field `PRIV2BB17` reader - page privileged/unprivileged attribution
pub type PRIV2BB17_R = crate::BitReader<bool>;
///Field `PRIV2BB17` writer - page privileged/unprivileged attribution
pub type PRIV2BB17_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_PRIV2BBR3_SPEC, bool, O>;
///Field `PRIV2BB18` reader - page privileged/unprivileged attribution
pub type PRIV2BB18_R = crate::BitReader<bool>;
///Field `PRIV2BB18` writer - page privileged/unprivileged attribution
pub type PRIV2BB18_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_PRIV2BBR3_SPEC, bool, O>;
///Field `PRIV2BB19` reader - page privileged/unprivileged attribution
pub type PRIV2BB19_R = crate::BitReader<bool>;
///Field `PRIV2BB19` writer - page privileged/unprivileged attribution
pub type PRIV2BB19_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_PRIV2BBR3_SPEC, bool, O>;
///Field `PRIV2BB20` reader - page privileged/unprivileged attribution
pub type PRIV2BB20_R = crate::BitReader<bool>;
///Field `PRIV2BB20` writer - page privileged/unprivileged attribution
pub type PRIV2BB20_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_PRIV2BBR3_SPEC, bool, O>;
///Field `PRIV2BB21` reader - page privileged/unprivileged attribution
pub type PRIV2BB21_R = crate::BitReader<bool>;
///Field `PRIV2BB21` writer - page privileged/unprivileged attribution
pub type PRIV2BB21_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_PRIV2BBR3_SPEC, bool, O>;
///Field `PRIV2BB22` reader - page privileged/unprivileged attribution
pub type PRIV2BB22_R = crate::BitReader<bool>;
///Field `PRIV2BB22` writer - page privileged/unprivileged attribution
pub type PRIV2BB22_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_PRIV2BBR3_SPEC, bool, O>;
///Field `PRIV2BB23` reader - page privileged/unprivileged attribution
pub type PRIV2BB23_R = crate::BitReader<bool>;
///Field `PRIV2BB23` writer - page privileged/unprivileged attribution
pub type PRIV2BB23_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_PRIV2BBR3_SPEC, bool, O>;
///Field `PRIV2BB24` reader - page privileged/unprivileged attribution
pub type PRIV2BB24_R = crate::BitReader<bool>;
///Field `PRIV2BB24` writer - page privileged/unprivileged attribution
pub type PRIV2BB24_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_PRIV2BBR3_SPEC, bool, O>;
///Field `PRIV2BB25` reader - page privileged/unprivileged attribution
pub type PRIV2BB25_R = crate::BitReader<bool>;
///Field `PRIV2BB25` writer - page privileged/unprivileged attribution
pub type PRIV2BB25_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_PRIV2BBR3_SPEC, bool, O>;
///Field `PRIV2BB26` reader - page privileged/unprivileged attribution
pub type PRIV2BB26_R = crate::BitReader<bool>;
///Field `PRIV2BB26` writer - page privileged/unprivileged attribution
pub type PRIV2BB26_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_PRIV2BBR3_SPEC, bool, O>;
///Field `PRIV2BB27` reader - page privileged/unprivileged attribution
pub type PRIV2BB27_R = crate::BitReader<bool>;
///Field `PRIV2BB27` writer - page privileged/unprivileged attribution
pub type PRIV2BB27_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_PRIV2BBR3_SPEC, bool, O>;
///Field `PRIV2BB28` reader - page privileged/unprivileged attribution
pub type PRIV2BB28_R = crate::BitReader<bool>;
///Field `PRIV2BB28` writer - page privileged/unprivileged attribution
pub type PRIV2BB28_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_PRIV2BBR3_SPEC, bool, O>;
///Field `PRIV2BB29` reader - page privileged/unprivileged attribution
pub type PRIV2BB29_R = crate::BitReader<bool>;
///Field `PRIV2BB29` writer - page privileged/unprivileged attribution
pub type PRIV2BB29_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_PRIV2BBR3_SPEC, bool, O>;
///Field `PRIV2BB30` reader - page privileged/unprivileged attribution
pub type PRIV2BB30_R = crate::BitReader<bool>;
///Field `PRIV2BB30` writer - page privileged/unprivileged attribution
pub type PRIV2BB30_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_PRIV2BBR3_SPEC, bool, O>;
///Field `PRIV2BB31` reader - page privileged/unprivileged attribution
pub type PRIV2BB31_R = crate::BitReader<bool>;
///Field `PRIV2BB31` writer - page privileged/unprivileged attribution
pub type PRIV2BB31_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_PRIV2BBR3_SPEC, bool, O>;
impl R {
    ///Bit 0 - page privileged/unprivileged attribution
    #[inline(always)]
    pub fn priv2bb0(&self) -> PRIV2BB0_R {
        PRIV2BB0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - page privileged/unprivileged attribution
    #[inline(always)]
    pub fn priv2bb1(&self) -> PRIV2BB1_R {
        PRIV2BB1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - page privileged/unprivileged attribution
    #[inline(always)]
    pub fn priv2bb2(&self) -> PRIV2BB2_R {
        PRIV2BB2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - page privileged/unprivileged attribution
    #[inline(always)]
    pub fn priv2bb3(&self) -> PRIV2BB3_R {
        PRIV2BB3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - page privileged/unprivileged attribution
    #[inline(always)]
    pub fn priv2bb4(&self) -> PRIV2BB4_R {
        PRIV2BB4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - page privileged/unprivileged attribution
    #[inline(always)]
    pub fn priv2bb5(&self) -> PRIV2BB5_R {
        PRIV2BB5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - page privileged/unprivileged attribution
    #[inline(always)]
    pub fn priv2bb6(&self) -> PRIV2BB6_R {
        PRIV2BB6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - page privileged/unprivileged attribution
    #[inline(always)]
    pub fn priv2bb7(&self) -> PRIV2BB7_R {
        PRIV2BB7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - page privileged/unprivileged attribution
    #[inline(always)]
    pub fn priv2bb8(&self) -> PRIV2BB8_R {
        PRIV2BB8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - page privileged/unprivileged attribution
    #[inline(always)]
    pub fn priv2bb9(&self) -> PRIV2BB9_R {
        PRIV2BB9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - page privileged/unprivileged attribution
    #[inline(always)]
    pub fn priv2bb10(&self) -> PRIV2BB10_R {
        PRIV2BB10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - page privileged/unprivileged attribution
    #[inline(always)]
    pub fn priv2bb11(&self) -> PRIV2BB11_R {
        PRIV2BB11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - page privileged/unprivileged attribution
    #[inline(always)]
    pub fn priv2bb12(&self) -> PRIV2BB12_R {
        PRIV2BB12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - page privileged/unprivileged attribution
    #[inline(always)]
    pub fn priv2bb13(&self) -> PRIV2BB13_R {
        PRIV2BB13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - page privileged/unprivileged attribution
    #[inline(always)]
    pub fn priv2bb14(&self) -> PRIV2BB14_R {
        PRIV2BB14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - page privileged/unprivileged attribution
    #[inline(always)]
    pub fn priv2bb15(&self) -> PRIV2BB15_R {
        PRIV2BB15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - page privileged/unprivileged attribution
    #[inline(always)]
    pub fn priv2bb16(&self) -> PRIV2BB16_R {
        PRIV2BB16_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - page privileged/unprivileged attribution
    #[inline(always)]
    pub fn priv2bb17(&self) -> PRIV2BB17_R {
        PRIV2BB17_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - page privileged/unprivileged attribution
    #[inline(always)]
    pub fn priv2bb18(&self) -> PRIV2BB18_R {
        PRIV2BB18_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - page privileged/unprivileged attribution
    #[inline(always)]
    pub fn priv2bb19(&self) -> PRIV2BB19_R {
        PRIV2BB19_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - page privileged/unprivileged attribution
    #[inline(always)]
    pub fn priv2bb20(&self) -> PRIV2BB20_R {
        PRIV2BB20_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - page privileged/unprivileged attribution
    #[inline(always)]
    pub fn priv2bb21(&self) -> PRIV2BB21_R {
        PRIV2BB21_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - page privileged/unprivileged attribution
    #[inline(always)]
    pub fn priv2bb22(&self) -> PRIV2BB22_R {
        PRIV2BB22_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - page privileged/unprivileged attribution
    #[inline(always)]
    pub fn priv2bb23(&self) -> PRIV2BB23_R {
        PRIV2BB23_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - page privileged/unprivileged attribution
    #[inline(always)]
    pub fn priv2bb24(&self) -> PRIV2BB24_R {
        PRIV2BB24_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - page privileged/unprivileged attribution
    #[inline(always)]
    pub fn priv2bb25(&self) -> PRIV2BB25_R {
        PRIV2BB25_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - page privileged/unprivileged attribution
    #[inline(always)]
    pub fn priv2bb26(&self) -> PRIV2BB26_R {
        PRIV2BB26_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - page privileged/unprivileged attribution
    #[inline(always)]
    pub fn priv2bb27(&self) -> PRIV2BB27_R {
        PRIV2BB27_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - page privileged/unprivileged attribution
    #[inline(always)]
    pub fn priv2bb28(&self) -> PRIV2BB28_R {
        PRIV2BB28_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - page privileged/unprivileged attribution
    #[inline(always)]
    pub fn priv2bb29(&self) -> PRIV2BB29_R {
        PRIV2BB29_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - page privileged/unprivileged attribution
    #[inline(always)]
    pub fn priv2bb30(&self) -> PRIV2BB30_R {
        PRIV2BB30_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - page privileged/unprivileged attribution
    #[inline(always)]
    pub fn priv2bb31(&self) -> PRIV2BB31_R {
        PRIV2BB31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - page privileged/unprivileged attribution
    #[inline(always)]
    #[must_use]
    pub fn priv2bb0(&mut self) -> PRIV2BB0_W<0> {
        PRIV2BB0_W::new(self)
    }
    ///Bit 1 - page privileged/unprivileged attribution
    #[inline(always)]
    #[must_use]
    pub fn priv2bb1(&mut self) -> PRIV2BB1_W<1> {
        PRIV2BB1_W::new(self)
    }
    ///Bit 2 - page privileged/unprivileged attribution
    #[inline(always)]
    #[must_use]
    pub fn priv2bb2(&mut self) -> PRIV2BB2_W<2> {
        PRIV2BB2_W::new(self)
    }
    ///Bit 3 - page privileged/unprivileged attribution
    #[inline(always)]
    #[must_use]
    pub fn priv2bb3(&mut self) -> PRIV2BB3_W<3> {
        PRIV2BB3_W::new(self)
    }
    ///Bit 4 - page privileged/unprivileged attribution
    #[inline(always)]
    #[must_use]
    pub fn priv2bb4(&mut self) -> PRIV2BB4_W<4> {
        PRIV2BB4_W::new(self)
    }
    ///Bit 5 - page privileged/unprivileged attribution
    #[inline(always)]
    #[must_use]
    pub fn priv2bb5(&mut self) -> PRIV2BB5_W<5> {
        PRIV2BB5_W::new(self)
    }
    ///Bit 6 - page privileged/unprivileged attribution
    #[inline(always)]
    #[must_use]
    pub fn priv2bb6(&mut self) -> PRIV2BB6_W<6> {
        PRIV2BB6_W::new(self)
    }
    ///Bit 7 - page privileged/unprivileged attribution
    #[inline(always)]
    #[must_use]
    pub fn priv2bb7(&mut self) -> PRIV2BB7_W<7> {
        PRIV2BB7_W::new(self)
    }
    ///Bit 8 - page privileged/unprivileged attribution
    #[inline(always)]
    #[must_use]
    pub fn priv2bb8(&mut self) -> PRIV2BB8_W<8> {
        PRIV2BB8_W::new(self)
    }
    ///Bit 9 - page privileged/unprivileged attribution
    #[inline(always)]
    #[must_use]
    pub fn priv2bb9(&mut self) -> PRIV2BB9_W<9> {
        PRIV2BB9_W::new(self)
    }
    ///Bit 10 - page privileged/unprivileged attribution
    #[inline(always)]
    #[must_use]
    pub fn priv2bb10(&mut self) -> PRIV2BB10_W<10> {
        PRIV2BB10_W::new(self)
    }
    ///Bit 11 - page privileged/unprivileged attribution
    #[inline(always)]
    #[must_use]
    pub fn priv2bb11(&mut self) -> PRIV2BB11_W<11> {
        PRIV2BB11_W::new(self)
    }
    ///Bit 12 - page privileged/unprivileged attribution
    #[inline(always)]
    #[must_use]
    pub fn priv2bb12(&mut self) -> PRIV2BB12_W<12> {
        PRIV2BB12_W::new(self)
    }
    ///Bit 13 - page privileged/unprivileged attribution
    #[inline(always)]
    #[must_use]
    pub fn priv2bb13(&mut self) -> PRIV2BB13_W<13> {
        PRIV2BB13_W::new(self)
    }
    ///Bit 14 - page privileged/unprivileged attribution
    #[inline(always)]
    #[must_use]
    pub fn priv2bb14(&mut self) -> PRIV2BB14_W<14> {
        PRIV2BB14_W::new(self)
    }
    ///Bit 15 - page privileged/unprivileged attribution
    #[inline(always)]
    #[must_use]
    pub fn priv2bb15(&mut self) -> PRIV2BB15_W<15> {
        PRIV2BB15_W::new(self)
    }
    ///Bit 16 - page privileged/unprivileged attribution
    #[inline(always)]
    #[must_use]
    pub fn priv2bb16(&mut self) -> PRIV2BB16_W<16> {
        PRIV2BB16_W::new(self)
    }
    ///Bit 17 - page privileged/unprivileged attribution
    #[inline(always)]
    #[must_use]
    pub fn priv2bb17(&mut self) -> PRIV2BB17_W<17> {
        PRIV2BB17_W::new(self)
    }
    ///Bit 18 - page privileged/unprivileged attribution
    #[inline(always)]
    #[must_use]
    pub fn priv2bb18(&mut self) -> PRIV2BB18_W<18> {
        PRIV2BB18_W::new(self)
    }
    ///Bit 19 - page privileged/unprivileged attribution
    #[inline(always)]
    #[must_use]
    pub fn priv2bb19(&mut self) -> PRIV2BB19_W<19> {
        PRIV2BB19_W::new(self)
    }
    ///Bit 20 - page privileged/unprivileged attribution
    #[inline(always)]
    #[must_use]
    pub fn priv2bb20(&mut self) -> PRIV2BB20_W<20> {
        PRIV2BB20_W::new(self)
    }
    ///Bit 21 - page privileged/unprivileged attribution
    #[inline(always)]
    #[must_use]
    pub fn priv2bb21(&mut self) -> PRIV2BB21_W<21> {
        PRIV2BB21_W::new(self)
    }
    ///Bit 22 - page privileged/unprivileged attribution
    #[inline(always)]
    #[must_use]
    pub fn priv2bb22(&mut self) -> PRIV2BB22_W<22> {
        PRIV2BB22_W::new(self)
    }
    ///Bit 23 - page privileged/unprivileged attribution
    #[inline(always)]
    #[must_use]
    pub fn priv2bb23(&mut self) -> PRIV2BB23_W<23> {
        PRIV2BB23_W::new(self)
    }
    ///Bit 24 - page privileged/unprivileged attribution
    #[inline(always)]
    #[must_use]
    pub fn priv2bb24(&mut self) -> PRIV2BB24_W<24> {
        PRIV2BB24_W::new(self)
    }
    ///Bit 25 - page privileged/unprivileged attribution
    #[inline(always)]
    #[must_use]
    pub fn priv2bb25(&mut self) -> PRIV2BB25_W<25> {
        PRIV2BB25_W::new(self)
    }
    ///Bit 26 - page privileged/unprivileged attribution
    #[inline(always)]
    #[must_use]
    pub fn priv2bb26(&mut self) -> PRIV2BB26_W<26> {
        PRIV2BB26_W::new(self)
    }
    ///Bit 27 - page privileged/unprivileged attribution
    #[inline(always)]
    #[must_use]
    pub fn priv2bb27(&mut self) -> PRIV2BB27_W<27> {
        PRIV2BB27_W::new(self)
    }
    ///Bit 28 - page privileged/unprivileged attribution
    #[inline(always)]
    #[must_use]
    pub fn priv2bb28(&mut self) -> PRIV2BB28_W<28> {
        PRIV2BB28_W::new(self)
    }
    ///Bit 29 - page privileged/unprivileged attribution
    #[inline(always)]
    #[must_use]
    pub fn priv2bb29(&mut self) -> PRIV2BB29_W<29> {
        PRIV2BB29_W::new(self)
    }
    ///Bit 30 - page privileged/unprivileged attribution
    #[inline(always)]
    #[must_use]
    pub fn priv2bb30(&mut self) -> PRIV2BB30_W<30> {
        PRIV2BB30_W::new(self)
    }
    ///Bit 31 - page privileged/unprivileged attribution
    #[inline(always)]
    #[must_use]
    pub fn priv2bb31(&mut self) -> PRIV2BB31_W<31> {
        PRIV2BB31_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FLASH privilege block based bank 2 register 3
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [flash_priv2bbr3](index.html) module
pub struct FLASH_PRIV2BBR3_SPEC;
impl crate::RegisterSpec for FLASH_PRIV2BBR3_SPEC {
    type Ux = u32;
}
///`read()` method returns [flash_priv2bbr3::R](R) reader structure
impl crate::Readable for FLASH_PRIV2BBR3_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [flash_priv2bbr3::W](W) writer structure
impl crate::Writable for FLASH_PRIV2BBR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets FLASH_PRIV2BBR3 to value 0
impl crate::Resettable for FLASH_PRIV2BBR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
