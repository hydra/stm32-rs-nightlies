///Register `TIM4_CNT` reader
pub struct R(crate::R<TIM4_CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM4_CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM4_CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM4_CNT_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TIM4_CNT` writer
pub struct W(crate::W<TIM4_CNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM4_CNT_SPEC>;
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
impl From<crate::W<TIM4_CNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM4_CNT_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CNT` reader - Counter value‘ Non-dithering mode (DITHEN = 0) The register holds the counter value. Dithering mode (DITHEN = 1) The register holds the non-dithered part in CNT\[15:0\]. The fractional part is not available.
pub type CNT_R = crate::FieldReader<u16, u16>;
///Field `CNT` writer - Counter value‘ Non-dithering mode (DITHEN = 0) The register holds the counter value. Dithering mode (DITHEN = 1) The register holds the non-dithered part in CNT\[15:0\]. The fractional part is not available.
pub type CNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM4_CNT_SPEC, u16, u16, 16, O>;
///Field `UIFCPY` reader - Value depends on IUFREMAP in TIMx_CR1. If UIFREMAP = 0 Reserved If UIFREMAP = 1 UIFCPY: UIF Copy This bit is a read-only copy of the UIF bit of the TIMx_ISR register
pub type UIFCPY_R = crate::BitReader<bool>;
///Field `UIFCPY` writer - Value depends on IUFREMAP in TIMx_CR1. If UIFREMAP = 0 Reserved If UIFREMAP = 1 UIFCPY: UIF Copy This bit is a read-only copy of the UIF bit of the TIMx_ISR register
pub type UIFCPY_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM4_CNT_SPEC, bool, O>;
impl R {
    ///Bits 0:15 - Counter value‘ Non-dithering mode (DITHEN = 0) The register holds the counter value. Dithering mode (DITHEN = 1) The register holds the non-dithered part in CNT\[15:0\]. The fractional part is not available.
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0xffff) as u16)
    }
    ///Bit 31 - Value depends on IUFREMAP in TIMx_CR1. If UIFREMAP = 0 Reserved If UIFREMAP = 1 UIFCPY: UIF Copy This bit is a read-only copy of the UIF bit of the TIMx_ISR register
    #[inline(always)]
    pub fn uifcpy(&self) -> UIFCPY_R {
        UIFCPY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 0:15 - Counter value‘ Non-dithering mode (DITHEN = 0) The register holds the counter value. Dithering mode (DITHEN = 1) The register holds the non-dithered part in CNT\[15:0\]. The fractional part is not available.
    #[inline(always)]
    #[must_use]
    pub fn cnt(&mut self) -> CNT_W<0> {
        CNT_W::new(self)
    }
    ///Bit 31 - Value depends on IUFREMAP in TIMx_CR1. If UIFREMAP = 0 Reserved If UIFREMAP = 1 UIFCPY: UIF Copy This bit is a read-only copy of the UIF bit of the TIMx_ISR register
    #[inline(always)]
    #[must_use]
    pub fn uifcpy(&mut self) -> UIFCPY_W<31> {
        UIFCPY_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TIM4 counter
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tim4_cnt](index.html) module
pub struct TIM4_CNT_SPEC;
impl crate::RegisterSpec for TIM4_CNT_SPEC {
    type Ux = u32;
}
///`read()` method returns [tim4_cnt::R](R) reader structure
impl crate::Readable for TIM4_CNT_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [tim4_cnt::W](W) writer structure
impl crate::Writable for TIM4_CNT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets TIM4_CNT to value 0
impl crate::Resettable for TIM4_CNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
