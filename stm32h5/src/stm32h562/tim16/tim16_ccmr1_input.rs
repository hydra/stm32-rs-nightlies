///Register `TIM16_CCMR1_Input` reader
pub struct R(crate::R<TIM16_CCMR1_INPUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM16_CCMR1_INPUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM16_CCMR1_INPUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM16_CCMR1_INPUT_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TIM16_CCMR1_Input` writer
pub struct W(crate::W<TIM16_CCMR1_INPUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM16_CCMR1_INPUT_SPEC>;
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
impl From<crate::W<TIM16_CCMR1_INPUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM16_CCMR1_INPUT_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CC1S` reader - Capture/Compare 1 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Others: Reserved Note: CC1S bits are writable only when the channel is OFF (CC1E = ‘0’ in TIMx_CCER).
pub type CC1S_R = crate::FieldReader<u8, u8>;
///Field `CC1S` writer - Capture/Compare 1 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Others: Reserved Note: CC1S bits are writable only when the channel is OFF (CC1E = ‘0’ in TIMx_CCER).
pub type CC1S_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TIM16_CCMR1_INPUT_SPEC, u8, u8, 2, O>;
///Field `IC1PSC` reader - Input capture 1 prescaler This bit-field defines the ratio of the prescaler acting on CC1 input (tim_ic1). The prescaler is reset as soon as CC1E=’0’ (TIMx_CCER register).
pub type IC1PSC_R = crate::FieldReader<u8, u8>;
///Field `IC1PSC` writer - Input capture 1 prescaler This bit-field defines the ratio of the prescaler acting on CC1 input (tim_ic1). The prescaler is reset as soon as CC1E=’0’ (TIMx_CCER register).
pub type IC1PSC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TIM16_CCMR1_INPUT_SPEC, u8, u8, 2, O>;
///Field `IC1F` reader - Input capture 1 filter This bit-field defines the frequency used to sample tim_ti1 input and the length of the digital filter applied to tim_ti1. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:
pub type IC1F_R = crate::FieldReader<u8, u8>;
///Field `IC1F` writer - Input capture 1 filter This bit-field defines the frequency used to sample tim_ti1 input and the length of the digital filter applied to tim_ti1. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:
pub type IC1F_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TIM16_CCMR1_INPUT_SPEC, u8, u8, 4, O>;
impl R {
    ///Bits 0:1 - Capture/Compare 1 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Others: Reserved Note: CC1S bits are writable only when the channel is OFF (CC1E = ‘0’ in TIMx_CCER).
    #[inline(always)]
    pub fn cc1s(&self) -> CC1S_R {
        CC1S_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - Input capture 1 prescaler This bit-field defines the ratio of the prescaler acting on CC1 input (tim_ic1). The prescaler is reset as soon as CC1E=’0’ (TIMx_CCER register).
    #[inline(always)]
    pub fn ic1psc(&self) -> IC1PSC_R {
        IC1PSC_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:7 - Input capture 1 filter This bit-field defines the frequency used to sample tim_ti1 input and the length of the digital filter applied to tim_ti1. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:
    #[inline(always)]
    pub fn ic1f(&self) -> IC1F_R {
        IC1F_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    ///Bits 0:1 - Capture/Compare 1 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Others: Reserved Note: CC1S bits are writable only when the channel is OFF (CC1E = ‘0’ in TIMx_CCER).
    #[inline(always)]
    #[must_use]
    pub fn cc1s(&mut self) -> CC1S_W<0> {
        CC1S_W::new(self)
    }
    ///Bits 2:3 - Input capture 1 prescaler This bit-field defines the ratio of the prescaler acting on CC1 input (tim_ic1). The prescaler is reset as soon as CC1E=’0’ (TIMx_CCER register).
    #[inline(always)]
    #[must_use]
    pub fn ic1psc(&mut self) -> IC1PSC_W<2> {
        IC1PSC_W::new(self)
    }
    ///Bits 4:7 - Input capture 1 filter This bit-field defines the frequency used to sample tim_ti1 input and the length of the digital filter applied to tim_ti1. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:
    #[inline(always)]
    #[must_use]
    pub fn ic1f(&mut self) -> IC1F_W<4> {
        IC1F_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TIM16 capture/compare mode register 1 \[alternate\]
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tim16_ccmr1_input](index.html) module
pub struct TIM16_CCMR1_INPUT_SPEC;
impl crate::RegisterSpec for TIM16_CCMR1_INPUT_SPEC {
    type Ux = u32;
}
///`read()` method returns [tim16_ccmr1_input::R](R) reader structure
impl crate::Readable for TIM16_CCMR1_INPUT_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [tim16_ccmr1_input::W](W) writer structure
impl crate::Writable for TIM16_CCMR1_INPUT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets TIM16_CCMR1_Input to value 0
impl crate::Resettable for TIM16_CCMR1_INPUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
