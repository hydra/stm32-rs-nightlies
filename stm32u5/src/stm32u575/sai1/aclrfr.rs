///Register `ACLRFR` writer
pub struct W(crate::W<ACLRFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACLRFR_SPEC>;
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
impl From<crate::W<ACLRFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACLRFR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `COVRUDR` writer - Clear overrun / underrun
pub type COVRUDR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACLRFR_SPEC, bool, O>;
///Field `CMUTEDET` writer - Mute detection flag
pub type CMUTEDET_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACLRFR_SPEC, bool, O>;
///Field `CWCKCFG` writer - Clear wrong clock configuration flag
pub type CWCKCFG_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACLRFR_SPEC, bool, O>;
///Field `CCNRDY` writer - Clear codec not ready flag
pub type CCNRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACLRFR_SPEC, bool, O>;
///Field `CAFSDET` writer - Clear anticipated frame synchronization detection flag
pub type CAFSDET_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACLRFR_SPEC, bool, O>;
///Field `CLFSDET` writer - Clear late frame synchronization detection flag
pub type CLFSDET_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACLRFR_SPEC, bool, O>;
impl W {
    ///Bit 0 - Clear overrun / underrun
    #[inline(always)]
    #[must_use]
    pub fn covrudr(&mut self) -> COVRUDR_W<0> {
        COVRUDR_W::new(self)
    }
    ///Bit 1 - Mute detection flag
    #[inline(always)]
    #[must_use]
    pub fn cmutedet(&mut self) -> CMUTEDET_W<1> {
        CMUTEDET_W::new(self)
    }
    ///Bit 2 - Clear wrong clock configuration flag
    #[inline(always)]
    #[must_use]
    pub fn cwckcfg(&mut self) -> CWCKCFG_W<2> {
        CWCKCFG_W::new(self)
    }
    ///Bit 4 - Clear codec not ready flag
    #[inline(always)]
    #[must_use]
    pub fn ccnrdy(&mut self) -> CCNRDY_W<4> {
        CCNRDY_W::new(self)
    }
    ///Bit 5 - Clear anticipated frame synchronization detection flag
    #[inline(always)]
    #[must_use]
    pub fn cafsdet(&mut self) -> CAFSDET_W<5> {
        CAFSDET_W::new(self)
    }
    ///Bit 6 - Clear late frame synchronization detection flag
    #[inline(always)]
    #[must_use]
    pub fn clfsdet(&mut self) -> CLFSDET_W<6> {
        CLFSDET_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///A Clear flag register
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [aclrfr](index.html) module
pub struct ACLRFR_SPEC;
impl crate::RegisterSpec for ACLRFR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [aclrfr::W](W) writer structure
impl crate::Writable for ACLRFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ACLRFR to value 0
impl crate::Resettable for ACLRFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
