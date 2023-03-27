///Register `GPDMA_PRIVCFGR` reader
pub struct R(crate::R<GPDMA_PRIVCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPDMA_PRIVCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPDMA_PRIVCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPDMA_PRIVCFGR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `GPDMA_PRIVCFGR` writer
pub struct W(crate::W<GPDMA_PRIVCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPDMA_PRIVCFGR_SPEC>;
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
impl From<crate::W<GPDMA_PRIVCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPDMA_PRIVCFGR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PRIV0` reader - PRIV0
pub type PRIV0_R = crate::BitReader<bool>;
///Field `PRIV0` writer - PRIV0
pub type PRIV0_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPDMA_PRIVCFGR_SPEC, bool, O>;
///Field `PRIV1` reader - PRIV1
pub type PRIV1_R = crate::BitReader<bool>;
///Field `PRIV1` writer - PRIV1
pub type PRIV1_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPDMA_PRIVCFGR_SPEC, bool, O>;
///Field `PRIV2` reader - PRIV2
pub type PRIV2_R = crate::BitReader<bool>;
///Field `PRIV2` writer - PRIV2
pub type PRIV2_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPDMA_PRIVCFGR_SPEC, bool, O>;
///Field `PRIV3` reader - PRIV3
pub type PRIV3_R = crate::BitReader<bool>;
///Field `PRIV3` writer - PRIV3
pub type PRIV3_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPDMA_PRIVCFGR_SPEC, bool, O>;
///Field `PRIV4` reader - PRIV4
pub type PRIV4_R = crate::BitReader<bool>;
///Field `PRIV4` writer - PRIV4
pub type PRIV4_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPDMA_PRIVCFGR_SPEC, bool, O>;
///Field `PRIV5` reader - PRIV5
pub type PRIV5_R = crate::BitReader<bool>;
///Field `PRIV5` writer - PRIV5
pub type PRIV5_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPDMA_PRIVCFGR_SPEC, bool, O>;
///Field `PRIV6` reader - PRIV6
pub type PRIV6_R = crate::BitReader<bool>;
///Field `PRIV6` writer - PRIV6
pub type PRIV6_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPDMA_PRIVCFGR_SPEC, bool, O>;
///Field `PRIV7` reader - PRIV7
pub type PRIV7_R = crate::BitReader<bool>;
///Field `PRIV7` writer - PRIV7
pub type PRIV7_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPDMA_PRIVCFGR_SPEC, bool, O>;
///Field `PRIV8` reader - PRIV8
pub type PRIV8_R = crate::BitReader<bool>;
///Field `PRIV8` writer - PRIV8
pub type PRIV8_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPDMA_PRIVCFGR_SPEC, bool, O>;
///Field `PRIV9` reader - PRIV9
pub type PRIV9_R = crate::BitReader<bool>;
///Field `PRIV9` writer - PRIV9
pub type PRIV9_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPDMA_PRIVCFGR_SPEC, bool, O>;
///Field `PRIV10` reader - PRIV10
pub type PRIV10_R = crate::BitReader<bool>;
///Field `PRIV10` writer - PRIV10
pub type PRIV10_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPDMA_PRIVCFGR_SPEC, bool, O>;
///Field `PRIV11` reader - PRIV11
pub type PRIV11_R = crate::BitReader<bool>;
///Field `PRIV11` writer - PRIV11
pub type PRIV11_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPDMA_PRIVCFGR_SPEC, bool, O>;
///Field `PRIV12` reader - PRIV12
pub type PRIV12_R = crate::BitReader<bool>;
///Field `PRIV12` writer - PRIV12
pub type PRIV12_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPDMA_PRIVCFGR_SPEC, bool, O>;
///Field `PRIV13` reader - PRIV13
pub type PRIV13_R = crate::BitReader<bool>;
///Field `PRIV13` writer - PRIV13
pub type PRIV13_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPDMA_PRIVCFGR_SPEC, bool, O>;
///Field `PRIV14` reader - PRIV14
pub type PRIV14_R = crate::BitReader<bool>;
///Field `PRIV14` writer - PRIV14
pub type PRIV14_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPDMA_PRIVCFGR_SPEC, bool, O>;
///Field `PRIV15` reader - PRIV15
pub type PRIV15_R = crate::BitReader<bool>;
///Field `PRIV15` writer - PRIV15
pub type PRIV15_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPDMA_PRIVCFGR_SPEC, bool, O>;
impl R {
    ///Bit 0 - PRIV0
    #[inline(always)]
    pub fn priv0(&self) -> PRIV0_R {
        PRIV0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - PRIV1
    #[inline(always)]
    pub fn priv1(&self) -> PRIV1_R {
        PRIV1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - PRIV2
    #[inline(always)]
    pub fn priv2(&self) -> PRIV2_R {
        PRIV2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - PRIV3
    #[inline(always)]
    pub fn priv3(&self) -> PRIV3_R {
        PRIV3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - PRIV4
    #[inline(always)]
    pub fn priv4(&self) -> PRIV4_R {
        PRIV4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - PRIV5
    #[inline(always)]
    pub fn priv5(&self) -> PRIV5_R {
        PRIV5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - PRIV6
    #[inline(always)]
    pub fn priv6(&self) -> PRIV6_R {
        PRIV6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - PRIV7
    #[inline(always)]
    pub fn priv7(&self) -> PRIV7_R {
        PRIV7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - PRIV8
    #[inline(always)]
    pub fn priv8(&self) -> PRIV8_R {
        PRIV8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - PRIV9
    #[inline(always)]
    pub fn priv9(&self) -> PRIV9_R {
        PRIV9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - PRIV10
    #[inline(always)]
    pub fn priv10(&self) -> PRIV10_R {
        PRIV10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - PRIV11
    #[inline(always)]
    pub fn priv11(&self) -> PRIV11_R {
        PRIV11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - PRIV12
    #[inline(always)]
    pub fn priv12(&self) -> PRIV12_R {
        PRIV12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - PRIV13
    #[inline(always)]
    pub fn priv13(&self) -> PRIV13_R {
        PRIV13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - PRIV14
    #[inline(always)]
    pub fn priv14(&self) -> PRIV14_R {
        PRIV14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - PRIV15
    #[inline(always)]
    pub fn priv15(&self) -> PRIV15_R {
        PRIV15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - PRIV0
    #[inline(always)]
    #[must_use]
    pub fn priv0(&mut self) -> PRIV0_W<0> {
        PRIV0_W::new(self)
    }
    ///Bit 1 - PRIV1
    #[inline(always)]
    #[must_use]
    pub fn priv1(&mut self) -> PRIV1_W<1> {
        PRIV1_W::new(self)
    }
    ///Bit 2 - PRIV2
    #[inline(always)]
    #[must_use]
    pub fn priv2(&mut self) -> PRIV2_W<2> {
        PRIV2_W::new(self)
    }
    ///Bit 3 - PRIV3
    #[inline(always)]
    #[must_use]
    pub fn priv3(&mut self) -> PRIV3_W<3> {
        PRIV3_W::new(self)
    }
    ///Bit 4 - PRIV4
    #[inline(always)]
    #[must_use]
    pub fn priv4(&mut self) -> PRIV4_W<4> {
        PRIV4_W::new(self)
    }
    ///Bit 5 - PRIV5
    #[inline(always)]
    #[must_use]
    pub fn priv5(&mut self) -> PRIV5_W<5> {
        PRIV5_W::new(self)
    }
    ///Bit 6 - PRIV6
    #[inline(always)]
    #[must_use]
    pub fn priv6(&mut self) -> PRIV6_W<6> {
        PRIV6_W::new(self)
    }
    ///Bit 7 - PRIV7
    #[inline(always)]
    #[must_use]
    pub fn priv7(&mut self) -> PRIV7_W<7> {
        PRIV7_W::new(self)
    }
    ///Bit 8 - PRIV8
    #[inline(always)]
    #[must_use]
    pub fn priv8(&mut self) -> PRIV8_W<8> {
        PRIV8_W::new(self)
    }
    ///Bit 9 - PRIV9
    #[inline(always)]
    #[must_use]
    pub fn priv9(&mut self) -> PRIV9_W<9> {
        PRIV9_W::new(self)
    }
    ///Bit 10 - PRIV10
    #[inline(always)]
    #[must_use]
    pub fn priv10(&mut self) -> PRIV10_W<10> {
        PRIV10_W::new(self)
    }
    ///Bit 11 - PRIV11
    #[inline(always)]
    #[must_use]
    pub fn priv11(&mut self) -> PRIV11_W<11> {
        PRIV11_W::new(self)
    }
    ///Bit 12 - PRIV12
    #[inline(always)]
    #[must_use]
    pub fn priv12(&mut self) -> PRIV12_W<12> {
        PRIV12_W::new(self)
    }
    ///Bit 13 - PRIV13
    #[inline(always)]
    #[must_use]
    pub fn priv13(&mut self) -> PRIV13_W<13> {
        PRIV13_W::new(self)
    }
    ///Bit 14 - PRIV14
    #[inline(always)]
    #[must_use]
    pub fn priv14(&mut self) -> PRIV14_W<14> {
        PRIV14_W::new(self)
    }
    ///Bit 15 - PRIV15
    #[inline(always)]
    #[must_use]
    pub fn priv15(&mut self) -> PRIV15_W<15> {
        PRIV15_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///GPDMA privileged configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [gpdma_privcfgr](index.html) module
pub struct GPDMA_PRIVCFGR_SPEC;
impl crate::RegisterSpec for GPDMA_PRIVCFGR_SPEC {
    type Ux = u32;
}
///`read()` method returns [gpdma_privcfgr::R](R) reader structure
impl crate::Readable for GPDMA_PRIVCFGR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [gpdma_privcfgr::W](W) writer structure
impl crate::Writable for GPDMA_PRIVCFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets GPDMA_PRIVCFGR to value 0
impl crate::Resettable for GPDMA_PRIVCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
