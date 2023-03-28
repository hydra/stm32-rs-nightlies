///Register `PRIVCFGR` reader
pub struct R(crate::R<PRIVCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRIVCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRIVCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRIVCFGR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PRIVCFGR` writer
pub struct W(crate::W<PRIVCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRIVCFGR_SPEC>;
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
impl From<crate::W<PRIVCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRIVCFGR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PRIV0` reader - privileged state of channel x
pub type PRIV0_R = crate::BitReader<bool>;
///Field `PRIV0` writer - privileged state of channel x
pub type PRIV0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR_SPEC, bool, O>;
///Field `PRIV1` reader - privileged state of channel x
pub type PRIV1_R = crate::BitReader<bool>;
///Field `PRIV1` writer - privileged state of channel x
pub type PRIV1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR_SPEC, bool, O>;
///Field `PRIV2` reader - privileged state of channel x
pub type PRIV2_R = crate::BitReader<bool>;
///Field `PRIV2` writer - privileged state of channel x
pub type PRIV2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR_SPEC, bool, O>;
///Field `PRIV3` reader - privileged state of channel x
pub type PRIV3_R = crate::BitReader<bool>;
///Field `PRIV3` writer - privileged state of channel x
pub type PRIV3_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR_SPEC, bool, O>;
///Field `PRIV4` reader - privileged state of channel x
pub type PRIV4_R = crate::BitReader<bool>;
///Field `PRIV4` writer - privileged state of channel x
pub type PRIV4_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR_SPEC, bool, O>;
///Field `PRIV5` reader - privileged state of channel x
pub type PRIV5_R = crate::BitReader<bool>;
///Field `PRIV5` writer - privileged state of channel x
pub type PRIV5_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR_SPEC, bool, O>;
///Field `PRIV6` reader - privileged state of channel x
pub type PRIV6_R = crate::BitReader<bool>;
///Field `PRIV6` writer - privileged state of channel x
pub type PRIV6_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR_SPEC, bool, O>;
///Field `PRIV7` reader - privileged state of channel x
pub type PRIV7_R = crate::BitReader<bool>;
///Field `PRIV7` writer - privileged state of channel x
pub type PRIV7_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR_SPEC, bool, O>;
impl R {
    ///Bit 0 - privileged state of channel x
    #[inline(always)]
    pub fn priv0(&self) -> PRIV0_R {
        PRIV0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - privileged state of channel x
    #[inline(always)]
    pub fn priv1(&self) -> PRIV1_R {
        PRIV1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - privileged state of channel x
    #[inline(always)]
    pub fn priv2(&self) -> PRIV2_R {
        PRIV2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - privileged state of channel x
    #[inline(always)]
    pub fn priv3(&self) -> PRIV3_R {
        PRIV3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - privileged state of channel x
    #[inline(always)]
    pub fn priv4(&self) -> PRIV4_R {
        PRIV4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - privileged state of channel x
    #[inline(always)]
    pub fn priv5(&self) -> PRIV5_R {
        PRIV5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - privileged state of channel x
    #[inline(always)]
    pub fn priv6(&self) -> PRIV6_R {
        PRIV6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - privileged state of channel x
    #[inline(always)]
    pub fn priv7(&self) -> PRIV7_R {
        PRIV7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - privileged state of channel x
    #[inline(always)]
    #[must_use]
    pub fn priv0(&mut self) -> PRIV0_W<0> {
        PRIV0_W::new(self)
    }
    ///Bit 1 - privileged state of channel x
    #[inline(always)]
    #[must_use]
    pub fn priv1(&mut self) -> PRIV1_W<1> {
        PRIV1_W::new(self)
    }
    ///Bit 2 - privileged state of channel x
    #[inline(always)]
    #[must_use]
    pub fn priv2(&mut self) -> PRIV2_W<2> {
        PRIV2_W::new(self)
    }
    ///Bit 3 - privileged state of channel x
    #[inline(always)]
    #[must_use]
    pub fn priv3(&mut self) -> PRIV3_W<3> {
        PRIV3_W::new(self)
    }
    ///Bit 4 - privileged state of channel x
    #[inline(always)]
    #[must_use]
    pub fn priv4(&mut self) -> PRIV4_W<4> {
        PRIV4_W::new(self)
    }
    ///Bit 5 - privileged state of channel x
    #[inline(always)]
    #[must_use]
    pub fn priv5(&mut self) -> PRIV5_W<5> {
        PRIV5_W::new(self)
    }
    ///Bit 6 - privileged state of channel x
    #[inline(always)]
    #[must_use]
    pub fn priv6(&mut self) -> PRIV6_W<6> {
        PRIV6_W::new(self)
    }
    ///Bit 7 - privileged state of channel x
    #[inline(always)]
    #[must_use]
    pub fn priv7(&mut self) -> PRIV7_W<7> {
        PRIV7_W::new(self)
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
///For information about available fields see [privcfgr](index.html) module
pub struct PRIVCFGR_SPEC;
impl crate::RegisterSpec for PRIVCFGR_SPEC {
    type Ux = u32;
}
///`read()` method returns [privcfgr::R](R) reader structure
impl crate::Readable for PRIVCFGR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [privcfgr::W](W) writer structure
impl crate::Writable for PRIVCFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets PRIVCFGR to value 0
impl crate::Resettable for PRIVCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
