///Register `TIM2_CNT` reader
pub struct R(crate::R<TIM2_CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM2_CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM2_CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM2_CNT_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TIM2_CNT` writer
pub struct W(crate::W<TIM2_CNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM2_CNT_SPEC>;
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
impl From<crate::W<TIM2_CNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM2_CNT_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CNT` reader - or UIFCPY: Value depends on IUFREMAP in TIMx_CR1. If UIFREMAP = 0 nullMost significant bit of counter value If UIFREMAP = 1 UIFCPY: UIF Copy This bit is a read-only copy of the UIF bit of the TIMx_ISR register nullLeast significant part of counter value Non-dithering mode (DITHEN = 0) The register holds the counter value. Dithering mode (DITHEN = 1) The register holds the non-dithered part in CNT\[30:0\]. The fractional part is not available.
pub type CNT_R = crate::FieldReader<u32, u32>;
///Field `CNT` writer - or UIFCPY: Value depends on IUFREMAP in TIMx_CR1. If UIFREMAP = 0 nullMost significant bit of counter value If UIFREMAP = 1 UIFCPY: UIF Copy This bit is a read-only copy of the UIF bit of the TIMx_ISR register nullLeast significant part of counter value Non-dithering mode (DITHEN = 0) The register holds the counter value. Dithering mode (DITHEN = 1) The register holds the non-dithered part in CNT\[30:0\]. The fractional part is not available.
pub type CNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM2_CNT_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - or UIFCPY: Value depends on IUFREMAP in TIMx_CR1. If UIFREMAP = 0 nullMost significant bit of counter value If UIFREMAP = 1 UIFCPY: UIF Copy This bit is a read-only copy of the UIF bit of the TIMx_ISR register nullLeast significant part of counter value Non-dithering mode (DITHEN = 0) The register holds the counter value. Dithering mode (DITHEN = 1) The register holds the non-dithered part in CNT\[30:0\]. The fractional part is not available.
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - or UIFCPY: Value depends on IUFREMAP in TIMx_CR1. If UIFREMAP = 0 nullMost significant bit of counter value If UIFREMAP = 1 UIFCPY: UIF Copy This bit is a read-only copy of the UIF bit of the TIMx_ISR register nullLeast significant part of counter value Non-dithering mode (DITHEN = 0) The register holds the counter value. Dithering mode (DITHEN = 1) The register holds the non-dithered part in CNT\[30:0\]. The fractional part is not available.
    #[inline(always)]
    #[must_use]
    pub fn cnt(&mut self) -> CNT_W<0> {
        CNT_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TIM2 counter
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tim2_cnt](index.html) module
pub struct TIM2_CNT_SPEC;
impl crate::RegisterSpec for TIM2_CNT_SPEC {
    type Ux = u32;
}
///`read()` method returns [tim2_cnt::R](R) reader structure
impl crate::Readable for TIM2_CNT_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [tim2_cnt::W](W) writer structure
impl crate::Writable for TIM2_CNT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets TIM2_CNT to value 0
impl crate::Resettable for TIM2_CNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
