///Register `TIM15_CCR2` reader
pub struct R(crate::R<TIM15_CCR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM15_CCR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM15_CCR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM15_CCR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TIM15_CCR2` writer
pub struct W(crate::W<TIM15_CCR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM15_CCR2_SPEC>;
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
impl From<crate::W<TIM15_CCR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM15_CCR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CCR2` reader - Capture/compare 2 value If channel CC2 is configured as output: CCR2 is the value to be loaded in the actual capture/compare 2 register (preload value). It is loaded permanently if the preload feature is not selected in the TIM15_CCMR2 register (bit OC2PE). Else the preload value is copied in the active capture/compare 2 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIM15_CNT and signalled on tim_oc2 output. Non-dithering mode (DITHEN = 0) The register holds the compare value in CCR2\[15:0\]. The CCR2\[19:16\]
///bits are reset. Dithering mode (DITHEN = 1) The register holds the integer part in CCR2\[19:4\]. The CCR2\[3:0\]
///bitfield contains the dithered part. If channel CC2 is configured as input: CCR2 is the counter value transferred by the last input capture 1 event (tim_ic2). The TIMx_CCR2 register is read-only and cannot be programmed. Non-dithering mode (DITHEN = 0) The register holds the capture value in CCR2\[15:0\]. The CCR2\[19:16\]
///bits are reset. Dithering mode (DITHEN = 1) The register holds the capture in CCR2\[19:4\]. The CCR2\[3:0\]
///bits are reset.
pub type CCR2_R = crate::FieldReader<u32, u32>;
///Field `CCR2` writer - Capture/compare 2 value If channel CC2 is configured as output: CCR2 is the value to be loaded in the actual capture/compare 2 register (preload value). It is loaded permanently if the preload feature is not selected in the TIM15_CCMR2 register (bit OC2PE). Else the preload value is copied in the active capture/compare 2 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIM15_CNT and signalled on tim_oc2 output. Non-dithering mode (DITHEN = 0) The register holds the compare value in CCR2\[15:0\]. The CCR2\[19:16\]
///bits are reset. Dithering mode (DITHEN = 1) The register holds the integer part in CCR2\[19:4\]. The CCR2\[3:0\]
///bitfield contains the dithered part. If channel CC2 is configured as input: CCR2 is the counter value transferred by the last input capture 1 event (tim_ic2). The TIMx_CCR2 register is read-only and cannot be programmed. Non-dithering mode (DITHEN = 0) The register holds the capture value in CCR2\[15:0\]. The CCR2\[19:16\]
///bits are reset. Dithering mode (DITHEN = 1) The register holds the capture in CCR2\[19:4\]. The CCR2\[3:0\]
///bits are reset.
pub type CCR2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM15_CCR2_SPEC, u32, u32, 20, O>;
impl R {
    ///Bits 0:19 - Capture/compare 2 value If channel CC2 is configured as output: CCR2 is the value to be loaded in the actual capture/compare 2 register (preload value). It is loaded permanently if the preload feature is not selected in the TIM15_CCMR2 register (bit OC2PE). Else the preload value is copied in the active capture/compare 2 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIM15_CNT and signalled on tim_oc2 output. Non-dithering mode (DITHEN = 0) The register holds the compare value in CCR2\[15:0\]. The CCR2\[19:16\]
    ///bits are reset. Dithering mode (DITHEN = 1) The register holds the integer part in CCR2\[19:4\]. The CCR2\[3:0\]
    ///bitfield contains the dithered part. If channel CC2 is configured as input: CCR2 is the counter value transferred by the last input capture 1 event (tim_ic2). The TIMx_CCR2 register is read-only and cannot be programmed. Non-dithering mode (DITHEN = 0) The register holds the capture value in CCR2\[15:0\]. The CCR2\[19:16\]
    ///bits are reset. Dithering mode (DITHEN = 1) The register holds the capture in CCR2\[19:4\]. The CCR2\[3:0\]
    ///bits are reset.
    #[inline(always)]
    pub fn ccr2(&self) -> CCR2_R {
        CCR2_R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    ///Bits 0:19 - Capture/compare 2 value If channel CC2 is configured as output: CCR2 is the value to be loaded in the actual capture/compare 2 register (preload value). It is loaded permanently if the preload feature is not selected in the TIM15_CCMR2 register (bit OC2PE). Else the preload value is copied in the active capture/compare 2 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIM15_CNT and signalled on tim_oc2 output. Non-dithering mode (DITHEN = 0) The register holds the compare value in CCR2\[15:0\]. The CCR2\[19:16\]
    ///bits are reset. Dithering mode (DITHEN = 1) The register holds the integer part in CCR2\[19:4\]. The CCR2\[3:0\]
    ///bitfield contains the dithered part. If channel CC2 is configured as input: CCR2 is the counter value transferred by the last input capture 1 event (tim_ic2). The TIMx_CCR2 register is read-only and cannot be programmed. Non-dithering mode (DITHEN = 0) The register holds the capture value in CCR2\[15:0\]. The CCR2\[19:16\]
    ///bits are reset. Dithering mode (DITHEN = 1) The register holds the capture in CCR2\[19:4\]. The CCR2\[3:0\]
    ///bits are reset.
    #[inline(always)]
    #[must_use]
    pub fn ccr2(&mut self) -> CCR2_W<0> {
        CCR2_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TIM15 capture/compare register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tim15_ccr2](index.html) module
pub struct TIM15_CCR2_SPEC;
impl crate::RegisterSpec for TIM15_CCR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [tim15_ccr2::R](R) reader structure
impl crate::Readable for TIM15_CCR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [tim15_ccr2::W](W) writer structure
impl crate::Writable for TIM15_CCR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets TIM15_CCR2 to value 0
impl crate::Resettable for TIM15_CCR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
