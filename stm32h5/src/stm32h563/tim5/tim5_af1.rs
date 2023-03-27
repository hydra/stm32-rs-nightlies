///Register `TIM5_AF1` reader
pub struct R(crate::R<TIM5_AF1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM5_AF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM5_AF1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM5_AF1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TIM5_AF1` writer
pub struct W(crate::W<TIM5_AF1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM5_AF1_SPEC>;
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
impl From<crate::W<TIM5_AF1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM5_AF1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ETRSEL` reader - etr_in source selection These bits select the etr_in input source. ... Refer to for product specific implementation.
pub type ETRSEL_R = crate::FieldReader<u8, u8>;
///Field `ETRSEL` writer - etr_in source selection These bits select the etr_in input source. ... Refer to for product specific implementation.
pub type ETRSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM5_AF1_SPEC, u8, u8, 4, O>;
impl R {
    ///Bits 14:17 - etr_in source selection These bits select the etr_in input source. ... Refer to for product specific implementation.
    #[inline(always)]
    pub fn etrsel(&self) -> ETRSEL_R {
        ETRSEL_R::new(((self.bits >> 14) & 0x0f) as u8)
    }
}
impl W {
    ///Bits 14:17 - etr_in source selection These bits select the etr_in input source. ... Refer to for product specific implementation.
    #[inline(always)]
    #[must_use]
    pub fn etrsel(&mut self) -> ETRSEL_W<14> {
        ETRSEL_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TIM5 alternate function register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tim5_af1](index.html) module
pub struct TIM5_AF1_SPEC;
impl crate::RegisterSpec for TIM5_AF1_SPEC {
    type Ux = u32;
}
///`read()` method returns [tim5_af1::R](R) reader structure
impl crate::Readable for TIM5_AF1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [tim5_af1::W](W) writer structure
impl crate::Writable for TIM5_AF1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets TIM5_AF1 to value 0
impl crate::Resettable for TIM5_AF1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
