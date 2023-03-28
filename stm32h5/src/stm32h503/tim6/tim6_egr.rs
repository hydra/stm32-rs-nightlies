///Register `TIM6_EGR` writer
pub struct W(crate::W<TIM6_EGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM6_EGR_SPEC>;
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
impl From<crate::W<TIM6_EGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM6_EGR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `UG` writer - Update generation This bit can be set by software, it is automatically cleared by hardware.
pub type UG_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM6_EGR_SPEC, bool, O>;
impl W {
    ///Bit 0 - Update generation This bit can be set by software, it is automatically cleared by hardware.
    #[inline(always)]
    #[must_use]
    pub fn ug(&mut self) -> UG_W<0> {
        UG_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TIM6 event generation register
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tim6_egr](index.html) module
pub struct TIM6_EGR_SPEC;
impl crate::RegisterSpec for TIM6_EGR_SPEC {
    type Ux = u16;
}
///`write(|w| ..)` method takes [tim6_egr::W](W) writer structure
impl crate::Writable for TIM6_EGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets TIM6_EGR to value 0
impl crate::Resettable for TIM6_EGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
