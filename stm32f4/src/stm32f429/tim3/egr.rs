///Register `EGR` writer
pub struct W(crate::W<EGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EGR_SPEC>;
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
impl From<crate::W<EGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EGR_SPEC>) -> Self {
        W(writer)
    }
}
///Update generation
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UG_AW {
    ///1: Re-initializes the timer counter and generates an update of the registers.
    Update = 1,
}
impl From<UG_AW> for bool {
    #[inline(always)]
    fn from(variant: UG_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `UG` writer - Update generation
pub type UG_W<'a, const O: u8> = crate::BitWriter<'a, u32, EGR_SPEC, UG_AW, O>;
impl<'a, const O: u8> UG_W<'a, O> {
    ///Re-initializes the timer counter and generates an update of the registers.
    #[inline(always)]
    pub fn update(self) -> &'a mut W {
        self.variant(UG_AW::Update)
    }
}
///Capture/compare 1 generation
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1GW_AW {
    ///1: If CC1 is an output: CC1IF flag is set, Corresponding interrupt or DMA request is sent if enabled. If CC1 is an input: The current value of the counter is captured in TIMx_CCR1 register.
    Trigger = 1,
}
impl From<CC1GW_AW> for bool {
    #[inline(always)]
    fn from(variant: CC1GW_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `CC1G` writer - Capture/compare 1 generation
pub type CC1G_W<'a, const O: u8> = crate::BitWriter<'a, u32, EGR_SPEC, CC1GW_AW, O>;
impl<'a, const O: u8> CC1G_W<'a, O> {
    ///If CC1 is an output: CC1IF flag is set, Corresponding interrupt or DMA request is sent if enabled. If CC1 is an input: The current value of the counter is captured in TIMx_CCR1 register.
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(CC1GW_AW::Trigger)
    }
}
///Field `CC2G` writer - Capture/compare 2 generation
pub use CC1G_W as CC2G_W;
///Field `CC3G` writer - Capture/compare 3 generation
pub use CC1G_W as CC3G_W;
///Field `CC4G` writer - Capture/compare 4 generation
pub use CC1G_W as CC4G_W;
///Trigger generation
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TGW_AW {
    ///1: The TIF flag is set in TIMx_SR register. Related interrupt or DMA transfer can occur if enabled.
    Trigger = 1,
}
impl From<TGW_AW> for bool {
    #[inline(always)]
    fn from(variant: TGW_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `TG` writer - Trigger generation
pub type TG_W<'a, const O: u8> = crate::BitWriter<'a, u32, EGR_SPEC, TGW_AW, O>;
impl<'a, const O: u8> TG_W<'a, O> {
    ///The TIF flag is set in TIMx_SR register. Related interrupt or DMA transfer can occur if enabled.
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TGW_AW::Trigger)
    }
}
impl W {
    ///Bit 0 - Update generation
    #[inline(always)]
    #[must_use]
    pub fn ug(&mut self) -> UG_W<0> {
        UG_W::new(self)
    }
    ///Bit 1 - Capture/compare 1 generation
    #[inline(always)]
    #[must_use]
    pub fn cc1g(&mut self) -> CC1G_W<1> {
        CC1G_W::new(self)
    }
    ///Bit 2 - Capture/compare 2 generation
    #[inline(always)]
    #[must_use]
    pub fn cc2g(&mut self) -> CC2G_W<2> {
        CC2G_W::new(self)
    }
    ///Bit 3 - Capture/compare 3 generation
    #[inline(always)]
    #[must_use]
    pub fn cc3g(&mut self) -> CC3G_W<3> {
        CC3G_W::new(self)
    }
    ///Bit 4 - Capture/compare 4 generation
    #[inline(always)]
    #[must_use]
    pub fn cc4g(&mut self) -> CC4G_W<4> {
        CC4G_W::new(self)
    }
    ///Bit 6 - Trigger generation
    #[inline(always)]
    #[must_use]
    pub fn tg(&mut self) -> TG_W<6> {
        TG_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///event generation register
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [egr](index.html) module
pub struct EGR_SPEC;
impl crate::RegisterSpec for EGR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [egr::W](W) writer structure
impl crate::Writable for EGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets EGR to value 0
impl crate::Resettable for EGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
