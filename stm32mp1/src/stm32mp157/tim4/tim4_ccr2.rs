///Register `TIM4_CCR2` reader
pub struct R(crate::R<TIM4_CCR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM4_CCR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM4_CCR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM4_CCR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TIM4_CCR2` writer
pub struct W(crate::W<TIM4_CCR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM4_CCR2_SPEC>;
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
impl From<crate::W<TIM4_CCR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM4_CCR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CCR2` reader - CCR2
pub type CCR2_R = crate::FieldReader<u16, u16>;
///Field `CCR2` writer - CCR2
pub type CCR2_W<'a, const O: u8> = crate::FieldWriter<'a, u16, TIM4_CCR2_SPEC, u16, u16, 16, O>;
impl R {
    ///Bits 0:15 - CCR2
    #[inline(always)]
    pub fn ccr2(&self) -> CCR2_R {
        CCR2_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:15 - CCR2
    #[inline(always)]
    #[must_use]
    pub fn ccr2(&mut self) -> CCR2_W<0> {
        CCR2_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TIM4 capture/compare register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tim4_ccr2](index.html) module
pub struct TIM4_CCR2_SPEC;
impl crate::RegisterSpec for TIM4_CCR2_SPEC {
    type Ux = u16;
}
///`read()` method returns [tim4_ccr2::R](R) reader structure
impl crate::Readable for TIM4_CCR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [tim4_ccr2::W](W) writer structure
impl crate::Writable for TIM4_CCR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets TIM4_CCR2 to value 0
impl crate::Resettable for TIM4_CCR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
