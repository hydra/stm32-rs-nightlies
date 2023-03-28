///Register `TIM6_SR` reader
pub struct R(crate::R<TIM6_SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM6_SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM6_SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM6_SR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TIM6_SR` writer
pub struct W(crate::W<TIM6_SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM6_SR_SPEC>;
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
impl From<crate::W<TIM6_SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM6_SR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `UIF` reader - Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. On counter overflow if UDIS = 0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in the TIMx_EGR register, if URS = 0 and UDIS = 0 in the TIMx_CR1 register.
pub type UIF_R = crate::BitReader<bool>;
///Field `UIF` writer - Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. On counter overflow if UDIS = 0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in the TIMx_EGR register, if URS = 0 and UDIS = 0 in the TIMx_CR1 register.
pub type UIF_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM6_SR_SPEC, bool, O>;
impl R {
    ///Bit 0 - Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. On counter overflow if UDIS = 0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in the TIMx_EGR register, if URS = 0 and UDIS = 0 in the TIMx_CR1 register.
    #[inline(always)]
    pub fn uif(&self) -> UIF_R {
        UIF_R::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. On counter overflow if UDIS = 0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in the TIMx_EGR register, if URS = 0 and UDIS = 0 in the TIMx_CR1 register.
    #[inline(always)]
    #[must_use]
    pub fn uif(&mut self) -> UIF_W<0> {
        UIF_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TIM6 status register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tim6_sr](index.html) module
pub struct TIM6_SR_SPEC;
impl crate::RegisterSpec for TIM6_SR_SPEC {
    type Ux = u16;
}
///`read()` method returns [tim6_sr::R](R) reader structure
impl crate::Readable for TIM6_SR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [tim6_sr::W](W) writer structure
impl crate::Writable for TIM6_SR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets TIM6_SR to value 0
impl crate::Resettable for TIM6_SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
