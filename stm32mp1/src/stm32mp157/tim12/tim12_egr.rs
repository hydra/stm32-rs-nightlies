///Register `TIM12_EGR` writer
pub struct W(crate::W<TIM12_EGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM12_EGR_SPEC>;
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
impl From<crate::W<TIM12_EGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM12_EGR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `UG` writer - UG
pub type UG_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM12_EGR_SPEC, bool, O>;
///Field `CC1G` writer - CC1G
pub type CC1G_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM12_EGR_SPEC, bool, O>;
///Field `CC2G` writer - CC2G
pub type CC2G_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM12_EGR_SPEC, bool, O>;
///Field `TG` writer - TG
pub type TG_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM12_EGR_SPEC, bool, O>;
impl W {
    ///Bit 0 - UG
    #[inline(always)]
    #[must_use]
    pub fn ug(&mut self) -> UG_W<0> {
        UG_W::new(self)
    }
    ///Bit 1 - CC1G
    #[inline(always)]
    #[must_use]
    pub fn cc1g(&mut self) -> CC1G_W<1> {
        CC1G_W::new(self)
    }
    ///Bit 2 - CC2G
    #[inline(always)]
    #[must_use]
    pub fn cc2g(&mut self) -> CC2G_W<2> {
        CC2G_W::new(self)
    }
    ///Bit 6 - TG
    #[inline(always)]
    #[must_use]
    pub fn tg(&mut self) -> TG_W<6> {
        TG_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TIM12 event generation register
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tim12_egr](index.html) module
pub struct TIM12_EGR_SPEC;
impl crate::RegisterSpec for TIM12_EGR_SPEC {
    type Ux = u16;
}
///`write(|w| ..)` method takes [tim12_egr::W](W) writer structure
impl crate::Writable for TIM12_EGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets TIM12_EGR to value 0
impl crate::Resettable for TIM12_EGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
