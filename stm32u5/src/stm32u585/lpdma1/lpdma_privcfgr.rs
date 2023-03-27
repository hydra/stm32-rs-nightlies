///Register `LPDMA_PRIVCFGR` reader
pub struct R(crate::R<LPDMA_PRIVCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPDMA_PRIVCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPDMA_PRIVCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPDMA_PRIVCFGR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `LPDMA_PRIVCFGR` writer
pub struct W(crate::W<LPDMA_PRIVCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPDMA_PRIVCFGR_SPEC>;
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
impl From<crate::W<LPDMA_PRIVCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPDMA_PRIVCFGR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PRIV0` reader - PRIV0
pub type PRIV0_R = crate::BitReader<bool>;
///Field `PRIV0` writer - PRIV0
pub type PRIV0_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPDMA_PRIVCFGR_SPEC, bool, O>;
///Field `PRIV1` reader - PRIV1
pub type PRIV1_R = crate::BitReader<bool>;
///Field `PRIV1` writer - PRIV1
pub type PRIV1_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPDMA_PRIVCFGR_SPEC, bool, O>;
///Field `PRIV2` reader - PRIV2
pub type PRIV2_R = crate::BitReader<bool>;
///Field `PRIV2` writer - PRIV2
pub type PRIV2_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPDMA_PRIVCFGR_SPEC, bool, O>;
///Field `PRIV3` reader - PRIV3
pub type PRIV3_R = crate::BitReader<bool>;
///Field `PRIV3` writer - PRIV3
pub type PRIV3_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPDMA_PRIVCFGR_SPEC, bool, O>;
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
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///LPDMA privileged configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [lpdma_privcfgr](index.html) module
pub struct LPDMA_PRIVCFGR_SPEC;
impl crate::RegisterSpec for LPDMA_PRIVCFGR_SPEC {
    type Ux = u32;
}
///`read()` method returns [lpdma_privcfgr::R](R) reader structure
impl crate::Readable for LPDMA_PRIVCFGR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [lpdma_privcfgr::W](W) writer structure
impl crate::Writable for LPDMA_PRIVCFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets LPDMA_PRIVCFGR to value 0
impl crate::Resettable for LPDMA_PRIVCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
