///Register `TIM6_ARR` reader
pub struct R(crate::R<TIM6_ARR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM6_ARR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM6_ARR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM6_ARR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TIM6_ARR` writer
pub struct W(crate::W<TIM6_ARR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM6_ARR_SPEC>;
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
impl From<crate::W<TIM6_ARR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM6_ARR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ARR` reader - ARR
pub type ARR_R = crate::FieldReader<u16, u16>;
///Field `ARR` writer - ARR
pub type ARR_W<'a, const O: u8> = crate::FieldWriter<'a, u16, TIM6_ARR_SPEC, u16, u16, 16, O>;
impl R {
    ///Bits 0:15 - ARR
    #[inline(always)]
    pub fn arr(&self) -> ARR_R {
        ARR_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:15 - ARR
    #[inline(always)]
    #[must_use]
    pub fn arr(&mut self) -> ARR_W<0> {
        ARR_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TIM6 auto-reload register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tim6_arr](index.html) module
pub struct TIM6_ARR_SPEC;
impl crate::RegisterSpec for TIM6_ARR_SPEC {
    type Ux = u16;
}
///`read()` method returns [tim6_arr::R](R) reader structure
impl crate::Readable for TIM6_ARR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [tim6_arr::W](W) writer structure
impl crate::Writable for TIM6_ARR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets TIM6_ARR to value 0xffff
impl crate::Resettable for TIM6_ARR_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
