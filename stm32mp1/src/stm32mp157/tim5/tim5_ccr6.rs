///Register `TIM5_CCR6` reader
pub struct R(crate::R<TIM5_CCR6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM5_CCR6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM5_CCR6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM5_CCR6_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TIM5_CCR6` writer
pub struct W(crate::W<TIM5_CCR6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM5_CCR6_SPEC>;
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
impl From<crate::W<TIM5_CCR6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM5_CCR6_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CCR6` reader - CCR6
pub type CCR6_R = crate::FieldReader<u16, u16>;
///Field `CCR6` writer - CCR6
pub type CCR6_W<'a, const O: u8> = crate::FieldWriter<'a, u16, TIM5_CCR6_SPEC, u16, u16, 16, O>;
impl R {
    ///Bits 0:15 - CCR6
    #[inline(always)]
    pub fn ccr6(&self) -> CCR6_R {
        CCR6_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:15 - CCR6
    #[inline(always)]
    #[must_use]
    pub fn ccr6(&mut self) -> CCR6_W<0> {
        CCR6_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TIM5 capture/compare register 6
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tim5_ccr6](index.html) module
pub struct TIM5_CCR6_SPEC;
impl crate::RegisterSpec for TIM5_CCR6_SPEC {
    type Ux = u16;
}
///`read()` method returns [tim5_ccr6::R](R) reader structure
impl crate::Readable for TIM5_CCR6_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [tim5_ccr6::W](W) writer structure
impl crate::Writable for TIM5_CCR6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets TIM5_CCR6 to value 0
impl crate::Resettable for TIM5_CCR6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
