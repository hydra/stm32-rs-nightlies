///Register `FLASH_PRIV1BBR3` reader
pub struct R(crate::R<FLASH_PRIV1BBR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASH_PRIV1BBR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASH_PRIV1BBR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASH_PRIV1BBR3_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FLASH_PRIV1BBR3` writer
pub struct W(crate::W<FLASH_PRIV1BBR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASH_PRIV1BBR3_SPEC>;
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
impl From<crate::W<FLASH_PRIV1BBR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASH_PRIV1BBR3_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PRIV1BB0` reader - page privileged/unprivileged attribution
pub type PRIV1BB0_R = crate::BitReader<bool>;
///Field `PRIV1BB0` writer - page privileged/unprivileged attribution
pub type PRIV1BB0_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_PRIV1BBR3_SPEC, bool, O>;
///Field `PRIV1BB1` reader - page privileged/unprivileged attribution
pub type PRIV1BB1_R = crate::BitReader<bool>;
///Field `PRIV1BB1` writer - page privileged/unprivileged attribution
pub type PRIV1BB1_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_PRIV1BBR3_SPEC, bool, O>;
///Field `PRIV1BB2` reader - page privileged/unprivileged attribution
pub type PRIV1BB2_R = crate::BitReader<bool>;
///Field `PRIV1BB2` writer - page privileged/unprivileged attribution
pub type PRIV1BB2_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_PRIV1BBR3_SPEC, bool, O>;
///Field `PRIV1BB3` reader - page privileged/unprivileged attribution
pub type PRIV1BB3_R = crate::BitReader<bool>;
///Field `PRIV1BB3` writer - page privileged/unprivileged attribution
pub type PRIV1BB3_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_PRIV1BBR3_SPEC, bool, O>;
///Field `PRIV1BB4` reader - page privileged/unprivileged attribution
pub type PRIV1BB4_R = crate::BitReader<bool>;
///Field `PRIV1BB4` writer - page privileged/unprivileged attribution
pub type PRIV1BB4_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_PRIV1BBR3_SPEC, bool, O>;
///Field `PRIV1BB5` reader - page privileged/unprivileged attribution
pub type PRIV1BB5_R = crate::BitReader<bool>;
///Field `PRIV1BB5` writer - page privileged/unprivileged attribution
pub type PRIV1BB5_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_PRIV1BBR3_SPEC, bool, O>;
///Field `PRIV1BB6` reader - page privileged/unprivileged attribution
pub type PRIV1BB6_R = crate::BitReader<bool>;
///Field `PRIV1BB6` writer - page privileged/unprivileged attribution
pub type PRIV1BB6_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_PRIV1BBR3_SPEC, bool, O>;
///Field `PRIV1BB7` reader - page privileged/unprivileged attribution
pub type PRIV1BB7_R = crate::BitReader<bool>;
///Field `PRIV1BB7` writer - page privileged/unprivileged attribution
pub type PRIV1BB7_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_PRIV1BBR3_SPEC, bool, O>;
///Field `PRIV1BB8` reader - page privileged/unprivileged attribution
pub type PRIV1BB8_R = crate::BitReader<bool>;
///Field `PRIV1BB8` writer - page privileged/unprivileged attribution
pub type PRIV1BB8_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_PRIV1BBR3_SPEC, bool, O>;
///Field `PRIV1BB9` reader - page privileged/unprivileged attribution
pub type PRIV1BB9_R = crate::BitReader<bool>;
///Field `PRIV1BB9` writer - page privileged/unprivileged attribution
pub type PRIV1BB9_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_PRIV1BBR3_SPEC, bool, O>;
///Field `PRIV1BB10` reader - page privileged/unprivileged attribution
pub type PRIV1BB10_R = crate::BitReader<bool>;
///Field `PRIV1BB10` writer - page privileged/unprivileged attribution
pub type PRIV1BB10_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_PRIV1BBR3_SPEC, bool, O>;
///Field `PRIV1BB11` reader - page privileged/unprivileged attribution
pub type PRIV1BB11_R = crate::BitReader<bool>;
///Field `PRIV1BB11` writer - page privileged/unprivileged attribution
pub type PRIV1BB11_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_PRIV1BBR3_SPEC, bool, O>;
///Field `PRIV1BB12` reader - page privileged/unprivileged attribution
pub type PRIV1BB12_R = crate::BitReader<bool>;
///Field `PRIV1BB12` writer - page privileged/unprivileged attribution
pub type PRIV1BB12_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_PRIV1BBR3_SPEC, bool, O>;
///Field `PRIV1BB13` reader - page privileged/unprivileged attribution
pub type PRIV1BB13_R = crate::BitReader<bool>;
///Field `PRIV1BB13` writer - page privileged/unprivileged attribution
pub type PRIV1BB13_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_PRIV1BBR3_SPEC, bool, O>;
///Field `PRIV1BB14` reader - page privileged/unprivileged attribution
pub type PRIV1BB14_R = crate::BitReader<bool>;
///Field `PRIV1BB14` writer - page privileged/unprivileged attribution
pub type PRIV1BB14_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_PRIV1BBR3_SPEC, bool, O>;
///Field `PRIV1BB15` reader - page privileged/unprivileged attribution
pub type PRIV1BB15_R = crate::BitReader<bool>;
///Field `PRIV1BB15` writer - page privileged/unprivileged attribution
pub type PRIV1BB15_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_PRIV1BBR3_SPEC, bool, O>;
///Field `PRIV1BB16` reader - page privileged/unprivileged attribution
pub type PRIV1BB16_R = crate::BitReader<bool>;
///Field `PRIV1BB16` writer - page privileged/unprivileged attribution
pub type PRIV1BB16_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_PRIV1BBR3_SPEC, bool, O>;
///Field `PRIV1BB17` reader - page privileged/unprivileged attribution
pub type PRIV1BB17_R = crate::BitReader<bool>;
///Field `PRIV1BB17` writer - page privileged/unprivileged attribution
pub type PRIV1BB17_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_PRIV1BBR3_SPEC, bool, O>;
///Field `PRIV1BB18` reader - page privileged/unprivileged attribution
pub type PRIV1BB18_R = crate::BitReader<bool>;
///Field `PRIV1BB18` writer - page privileged/unprivileged attribution
pub type PRIV1BB18_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_PRIV1BBR3_SPEC, bool, O>;
///Field `PRIV1BB19` reader - page privileged/unprivileged attribution
pub type PRIV1BB19_R = crate::BitReader<bool>;
///Field `PRIV1BB19` writer - page privileged/unprivileged attribution
pub type PRIV1BB19_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_PRIV1BBR3_SPEC, bool, O>;
///Field `PRIV1BB20` reader - page privileged/unprivileged attribution
pub type PRIV1BB20_R = crate::BitReader<bool>;
///Field `PRIV1BB20` writer - page privileged/unprivileged attribution
pub type PRIV1BB20_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_PRIV1BBR3_SPEC, bool, O>;
///Field `PRIV1BB21` reader - page privileged/unprivileged attribution
pub type PRIV1BB21_R = crate::BitReader<bool>;
///Field `PRIV1BB21` writer - page privileged/unprivileged attribution
pub type PRIV1BB21_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_PRIV1BBR3_SPEC, bool, O>;
///Field `PRIV1BB22` reader - page privileged/unprivileged attribution
pub type PRIV1BB22_R = crate::BitReader<bool>;
///Field `PRIV1BB22` writer - page privileged/unprivileged attribution
pub type PRIV1BB22_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_PRIV1BBR3_SPEC, bool, O>;
///Field `PRIV1BB23` reader - page privileged/unprivileged attribution
pub type PRIV1BB23_R = crate::BitReader<bool>;
///Field `PRIV1BB23` writer - page privileged/unprivileged attribution
pub type PRIV1BB23_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_PRIV1BBR3_SPEC, bool, O>;
///Field `PRIV1BB24` reader - page privileged/unprivileged attribution
pub type PRIV1BB24_R = crate::BitReader<bool>;
///Field `PRIV1BB24` writer - page privileged/unprivileged attribution
pub type PRIV1BB24_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_PRIV1BBR3_SPEC, bool, O>;
///Field `PRIV1BB25` reader - page privileged/unprivileged attribution
pub type PRIV1BB25_R = crate::BitReader<bool>;
///Field `PRIV1BB25` writer - page privileged/unprivileged attribution
pub type PRIV1BB25_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_PRIV1BBR3_SPEC, bool, O>;
///Field `PRIV1BB26` reader - page privileged/unprivileged attribution
pub type PRIV1BB26_R = crate::BitReader<bool>;
///Field `PRIV1BB26` writer - page privileged/unprivileged attribution
pub type PRIV1BB26_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_PRIV1BBR3_SPEC, bool, O>;
///Field `PRIV1BB27` reader - page privileged/unprivileged attribution
pub type PRIV1BB27_R = crate::BitReader<bool>;
///Field `PRIV1BB27` writer - page privileged/unprivileged attribution
pub type PRIV1BB27_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_PRIV1BBR3_SPEC, bool, O>;
///Field `PRIV1BB28` reader - page privileged/unprivileged attribution
pub type PRIV1BB28_R = crate::BitReader<bool>;
///Field `PRIV1BB28` writer - page privileged/unprivileged attribution
pub type PRIV1BB28_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_PRIV1BBR3_SPEC, bool, O>;
///Field `PRIV1BB29` reader - page privileged/unprivileged attribution
pub type PRIV1BB29_R = crate::BitReader<bool>;
///Field `PRIV1BB29` writer - page privileged/unprivileged attribution
pub type PRIV1BB29_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_PRIV1BBR3_SPEC, bool, O>;
///Field `PRIV1BB30` reader - page privileged/unprivileged attribution
pub type PRIV1BB30_R = crate::BitReader<bool>;
///Field `PRIV1BB30` writer - page privileged/unprivileged attribution
pub type PRIV1BB30_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_PRIV1BBR3_SPEC, bool, O>;
///Field `PRIV1BB31` reader - page privileged/unprivileged attribution
pub type PRIV1BB31_R = crate::BitReader<bool>;
///Field `PRIV1BB31` writer - page privileged/unprivileged attribution
pub type PRIV1BB31_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_PRIV1BBR3_SPEC, bool, O>;
impl R {
    ///Bit 0 - page privileged/unprivileged attribution
    #[inline(always)]
    pub fn priv1bb0(&self) -> PRIV1BB0_R {
        PRIV1BB0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - page privileged/unprivileged attribution
    #[inline(always)]
    pub fn priv1bb1(&self) -> PRIV1BB1_R {
        PRIV1BB1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - page privileged/unprivileged attribution
    #[inline(always)]
    pub fn priv1bb2(&self) -> PRIV1BB2_R {
        PRIV1BB2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - page privileged/unprivileged attribution
    #[inline(always)]
    pub fn priv1bb3(&self) -> PRIV1BB3_R {
        PRIV1BB3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - page privileged/unprivileged attribution
    #[inline(always)]
    pub fn priv1bb4(&self) -> PRIV1BB4_R {
        PRIV1BB4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - page privileged/unprivileged attribution
    #[inline(always)]
    pub fn priv1bb5(&self) -> PRIV1BB5_R {
        PRIV1BB5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - page privileged/unprivileged attribution
    #[inline(always)]
    pub fn priv1bb6(&self) -> PRIV1BB6_R {
        PRIV1BB6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - page privileged/unprivileged attribution
    #[inline(always)]
    pub fn priv1bb7(&self) -> PRIV1BB7_R {
        PRIV1BB7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - page privileged/unprivileged attribution
    #[inline(always)]
    pub fn priv1bb8(&self) -> PRIV1BB8_R {
        PRIV1BB8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - page privileged/unprivileged attribution
    #[inline(always)]
    pub fn priv1bb9(&self) -> PRIV1BB9_R {
        PRIV1BB9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - page privileged/unprivileged attribution
    #[inline(always)]
    pub fn priv1bb10(&self) -> PRIV1BB10_R {
        PRIV1BB10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - page privileged/unprivileged attribution
    #[inline(always)]
    pub fn priv1bb11(&self) -> PRIV1BB11_R {
        PRIV1BB11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - page privileged/unprivileged attribution
    #[inline(always)]
    pub fn priv1bb12(&self) -> PRIV1BB12_R {
        PRIV1BB12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - page privileged/unprivileged attribution
    #[inline(always)]
    pub fn priv1bb13(&self) -> PRIV1BB13_R {
        PRIV1BB13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - page privileged/unprivileged attribution
    #[inline(always)]
    pub fn priv1bb14(&self) -> PRIV1BB14_R {
        PRIV1BB14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - page privileged/unprivileged attribution
    #[inline(always)]
    pub fn priv1bb15(&self) -> PRIV1BB15_R {
        PRIV1BB15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - page privileged/unprivileged attribution
    #[inline(always)]
    pub fn priv1bb16(&self) -> PRIV1BB16_R {
        PRIV1BB16_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - page privileged/unprivileged attribution
    #[inline(always)]
    pub fn priv1bb17(&self) -> PRIV1BB17_R {
        PRIV1BB17_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - page privileged/unprivileged attribution
    #[inline(always)]
    pub fn priv1bb18(&self) -> PRIV1BB18_R {
        PRIV1BB18_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - page privileged/unprivileged attribution
    #[inline(always)]
    pub fn priv1bb19(&self) -> PRIV1BB19_R {
        PRIV1BB19_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - page privileged/unprivileged attribution
    #[inline(always)]
    pub fn priv1bb20(&self) -> PRIV1BB20_R {
        PRIV1BB20_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - page privileged/unprivileged attribution
    #[inline(always)]
    pub fn priv1bb21(&self) -> PRIV1BB21_R {
        PRIV1BB21_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - page privileged/unprivileged attribution
    #[inline(always)]
    pub fn priv1bb22(&self) -> PRIV1BB22_R {
        PRIV1BB22_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - page privileged/unprivileged attribution
    #[inline(always)]
    pub fn priv1bb23(&self) -> PRIV1BB23_R {
        PRIV1BB23_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - page privileged/unprivileged attribution
    #[inline(always)]
    pub fn priv1bb24(&self) -> PRIV1BB24_R {
        PRIV1BB24_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - page privileged/unprivileged attribution
    #[inline(always)]
    pub fn priv1bb25(&self) -> PRIV1BB25_R {
        PRIV1BB25_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - page privileged/unprivileged attribution
    #[inline(always)]
    pub fn priv1bb26(&self) -> PRIV1BB26_R {
        PRIV1BB26_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - page privileged/unprivileged attribution
    #[inline(always)]
    pub fn priv1bb27(&self) -> PRIV1BB27_R {
        PRIV1BB27_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - page privileged/unprivileged attribution
    #[inline(always)]
    pub fn priv1bb28(&self) -> PRIV1BB28_R {
        PRIV1BB28_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - page privileged/unprivileged attribution
    #[inline(always)]
    pub fn priv1bb29(&self) -> PRIV1BB29_R {
        PRIV1BB29_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - page privileged/unprivileged attribution
    #[inline(always)]
    pub fn priv1bb30(&self) -> PRIV1BB30_R {
        PRIV1BB30_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - page privileged/unprivileged attribution
    #[inline(always)]
    pub fn priv1bb31(&self) -> PRIV1BB31_R {
        PRIV1BB31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - page privileged/unprivileged attribution
    #[inline(always)]
    #[must_use]
    pub fn priv1bb0(&mut self) -> PRIV1BB0_W<0> {
        PRIV1BB0_W::new(self)
    }
    ///Bit 1 - page privileged/unprivileged attribution
    #[inline(always)]
    #[must_use]
    pub fn priv1bb1(&mut self) -> PRIV1BB1_W<1> {
        PRIV1BB1_W::new(self)
    }
    ///Bit 2 - page privileged/unprivileged attribution
    #[inline(always)]
    #[must_use]
    pub fn priv1bb2(&mut self) -> PRIV1BB2_W<2> {
        PRIV1BB2_W::new(self)
    }
    ///Bit 3 - page privileged/unprivileged attribution
    #[inline(always)]
    #[must_use]
    pub fn priv1bb3(&mut self) -> PRIV1BB3_W<3> {
        PRIV1BB3_W::new(self)
    }
    ///Bit 4 - page privileged/unprivileged attribution
    #[inline(always)]
    #[must_use]
    pub fn priv1bb4(&mut self) -> PRIV1BB4_W<4> {
        PRIV1BB4_W::new(self)
    }
    ///Bit 5 - page privileged/unprivileged attribution
    #[inline(always)]
    #[must_use]
    pub fn priv1bb5(&mut self) -> PRIV1BB5_W<5> {
        PRIV1BB5_W::new(self)
    }
    ///Bit 6 - page privileged/unprivileged attribution
    #[inline(always)]
    #[must_use]
    pub fn priv1bb6(&mut self) -> PRIV1BB6_W<6> {
        PRIV1BB6_W::new(self)
    }
    ///Bit 7 - page privileged/unprivileged attribution
    #[inline(always)]
    #[must_use]
    pub fn priv1bb7(&mut self) -> PRIV1BB7_W<7> {
        PRIV1BB7_W::new(self)
    }
    ///Bit 8 - page privileged/unprivileged attribution
    #[inline(always)]
    #[must_use]
    pub fn priv1bb8(&mut self) -> PRIV1BB8_W<8> {
        PRIV1BB8_W::new(self)
    }
    ///Bit 9 - page privileged/unprivileged attribution
    #[inline(always)]
    #[must_use]
    pub fn priv1bb9(&mut self) -> PRIV1BB9_W<9> {
        PRIV1BB9_W::new(self)
    }
    ///Bit 10 - page privileged/unprivileged attribution
    #[inline(always)]
    #[must_use]
    pub fn priv1bb10(&mut self) -> PRIV1BB10_W<10> {
        PRIV1BB10_W::new(self)
    }
    ///Bit 11 - page privileged/unprivileged attribution
    #[inline(always)]
    #[must_use]
    pub fn priv1bb11(&mut self) -> PRIV1BB11_W<11> {
        PRIV1BB11_W::new(self)
    }
    ///Bit 12 - page privileged/unprivileged attribution
    #[inline(always)]
    #[must_use]
    pub fn priv1bb12(&mut self) -> PRIV1BB12_W<12> {
        PRIV1BB12_W::new(self)
    }
    ///Bit 13 - page privileged/unprivileged attribution
    #[inline(always)]
    #[must_use]
    pub fn priv1bb13(&mut self) -> PRIV1BB13_W<13> {
        PRIV1BB13_W::new(self)
    }
    ///Bit 14 - page privileged/unprivileged attribution
    #[inline(always)]
    #[must_use]
    pub fn priv1bb14(&mut self) -> PRIV1BB14_W<14> {
        PRIV1BB14_W::new(self)
    }
    ///Bit 15 - page privileged/unprivileged attribution
    #[inline(always)]
    #[must_use]
    pub fn priv1bb15(&mut self) -> PRIV1BB15_W<15> {
        PRIV1BB15_W::new(self)
    }
    ///Bit 16 - page privileged/unprivileged attribution
    #[inline(always)]
    #[must_use]
    pub fn priv1bb16(&mut self) -> PRIV1BB16_W<16> {
        PRIV1BB16_W::new(self)
    }
    ///Bit 17 - page privileged/unprivileged attribution
    #[inline(always)]
    #[must_use]
    pub fn priv1bb17(&mut self) -> PRIV1BB17_W<17> {
        PRIV1BB17_W::new(self)
    }
    ///Bit 18 - page privileged/unprivileged attribution
    #[inline(always)]
    #[must_use]
    pub fn priv1bb18(&mut self) -> PRIV1BB18_W<18> {
        PRIV1BB18_W::new(self)
    }
    ///Bit 19 - page privileged/unprivileged attribution
    #[inline(always)]
    #[must_use]
    pub fn priv1bb19(&mut self) -> PRIV1BB19_W<19> {
        PRIV1BB19_W::new(self)
    }
    ///Bit 20 - page privileged/unprivileged attribution
    #[inline(always)]
    #[must_use]
    pub fn priv1bb20(&mut self) -> PRIV1BB20_W<20> {
        PRIV1BB20_W::new(self)
    }
    ///Bit 21 - page privileged/unprivileged attribution
    #[inline(always)]
    #[must_use]
    pub fn priv1bb21(&mut self) -> PRIV1BB21_W<21> {
        PRIV1BB21_W::new(self)
    }
    ///Bit 22 - page privileged/unprivileged attribution
    #[inline(always)]
    #[must_use]
    pub fn priv1bb22(&mut self) -> PRIV1BB22_W<22> {
        PRIV1BB22_W::new(self)
    }
    ///Bit 23 - page privileged/unprivileged attribution
    #[inline(always)]
    #[must_use]
    pub fn priv1bb23(&mut self) -> PRIV1BB23_W<23> {
        PRIV1BB23_W::new(self)
    }
    ///Bit 24 - page privileged/unprivileged attribution
    #[inline(always)]
    #[must_use]
    pub fn priv1bb24(&mut self) -> PRIV1BB24_W<24> {
        PRIV1BB24_W::new(self)
    }
    ///Bit 25 - page privileged/unprivileged attribution
    #[inline(always)]
    #[must_use]
    pub fn priv1bb25(&mut self) -> PRIV1BB25_W<25> {
        PRIV1BB25_W::new(self)
    }
    ///Bit 26 - page privileged/unprivileged attribution
    #[inline(always)]
    #[must_use]
    pub fn priv1bb26(&mut self) -> PRIV1BB26_W<26> {
        PRIV1BB26_W::new(self)
    }
    ///Bit 27 - page privileged/unprivileged attribution
    #[inline(always)]
    #[must_use]
    pub fn priv1bb27(&mut self) -> PRIV1BB27_W<27> {
        PRIV1BB27_W::new(self)
    }
    ///Bit 28 - page privileged/unprivileged attribution
    #[inline(always)]
    #[must_use]
    pub fn priv1bb28(&mut self) -> PRIV1BB28_W<28> {
        PRIV1BB28_W::new(self)
    }
    ///Bit 29 - page privileged/unprivileged attribution
    #[inline(always)]
    #[must_use]
    pub fn priv1bb29(&mut self) -> PRIV1BB29_W<29> {
        PRIV1BB29_W::new(self)
    }
    ///Bit 30 - page privileged/unprivileged attribution
    #[inline(always)]
    #[must_use]
    pub fn priv1bb30(&mut self) -> PRIV1BB30_W<30> {
        PRIV1BB30_W::new(self)
    }
    ///Bit 31 - page privileged/unprivileged attribution
    #[inline(always)]
    #[must_use]
    pub fn priv1bb31(&mut self) -> PRIV1BB31_W<31> {
        PRIV1BB31_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FLASH privilege block based bank 1 register 3
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [flash_priv1bbr3](index.html) module
pub struct FLASH_PRIV1BBR3_SPEC;
impl crate::RegisterSpec for FLASH_PRIV1BBR3_SPEC {
    type Ux = u32;
}
///`read()` method returns [flash_priv1bbr3::R](R) reader structure
impl crate::Readable for FLASH_PRIV1BBR3_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [flash_priv1bbr3::W](W) writer structure
impl crate::Writable for FLASH_PRIV1BBR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets FLASH_PRIV1BBR3 to value 0
impl crate::Resettable for FLASH_PRIV1BBR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
