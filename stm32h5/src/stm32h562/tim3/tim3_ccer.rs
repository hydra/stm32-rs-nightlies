///Register `TIM3_CCER` reader
pub struct R(crate::R<TIM3_CCER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM3_CCER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM3_CCER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM3_CCER_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TIM3_CCER` writer
pub struct W(crate::W<TIM3_CCER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM3_CCER_SPEC>;
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
impl From<crate::W<TIM3_CCER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM3_CCER_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CC1E` reader - Capture/Compare 1 output enable.
pub type CC1E_R = crate::BitReader<bool>;
///Field `CC1E` writer - Capture/Compare 1 output enable.
pub type CC1E_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM3_CCER_SPEC, bool, O>;
///Field `CC1P` reader - Capture/Compare 1 output Polarity. When CC1 channel is configured as input, both CC1NP/CC1P bits select the active polarity of TI1FP1 and TI2FP1 for trigger or capture operations. CC1NP=0, CC1P=0: non-inverted/rising edge. The circuit is sensitive to TIxFP1 rising edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger operation in gated mode or encoder mode). CC1NP=0, CC1P=1: inverted/falling edge. The circuit is sensitive to TIxFP1 falling edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is inverted (trigger operation in gated mode or encoder mode). CC1NP=1, CC1P=1: non-inverted/both edges. The circuit is sensitive to both TIxFP1 rising and falling edges (capture or trigger operations in reset, external clock or trigger mode), TIxFP1is not inverted (trigger operation in gated mode). This configuration must not be used in encoder mode. CC1NP=1, CC1P=0: this configuration is reserved, it must not be used.
pub type CC1P_R = crate::BitReader<bool>;
///Field `CC1P` writer - Capture/Compare 1 output Polarity. When CC1 channel is configured as input, both CC1NP/CC1P bits select the active polarity of TI1FP1 and TI2FP1 for trigger or capture operations. CC1NP=0, CC1P=0: non-inverted/rising edge. The circuit is sensitive to TIxFP1 rising edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger operation in gated mode or encoder mode). CC1NP=0, CC1P=1: inverted/falling edge. The circuit is sensitive to TIxFP1 falling edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is inverted (trigger operation in gated mode or encoder mode). CC1NP=1, CC1P=1: non-inverted/both edges. The circuit is sensitive to both TIxFP1 rising and falling edges (capture or trigger operations in reset, external clock or trigger mode), TIxFP1is not inverted (trigger operation in gated mode). This configuration must not be used in encoder mode. CC1NP=1, CC1P=0: this configuration is reserved, it must not be used.
pub type CC1P_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM3_CCER_SPEC, bool, O>;
///Field `CC1NP` reader - Capture/Compare 1 output Polarity. CC1 channel configured as output: CC1NP must be kept cleared in this case. CC1 channel configured as input: This bit is used in conjunction with CC1P to define tim_ti1fp1/tim_ti2fp1 polarity. refer to CC1P description.
pub type CC1NP_R = crate::BitReader<bool>;
///Field `CC1NP` writer - Capture/Compare 1 output Polarity. CC1 channel configured as output: CC1NP must be kept cleared in this case. CC1 channel configured as input: This bit is used in conjunction with CC1P to define tim_ti1fp1/tim_ti2fp1 polarity. refer to CC1P description.
pub type CC1NP_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM3_CCER_SPEC, bool, O>;
///Field `CC2E` reader - Capture/Compare 2 output enable. Refer to CC1E description
pub type CC2E_R = crate::BitReader<bool>;
///Field `CC2E` writer - Capture/Compare 2 output enable. Refer to CC1E description
pub type CC2E_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM3_CCER_SPEC, bool, O>;
///Field `CC2P` reader - Capture/Compare 2 output Polarity. refer to CC1P description
pub type CC2P_R = crate::BitReader<bool>;
///Field `CC2P` writer - Capture/Compare 2 output Polarity. refer to CC1P description
pub type CC2P_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM3_CCER_SPEC, bool, O>;
///Field `CC2NP` reader - Capture/Compare 2 output Polarity. Refer to CC1NP description
pub type CC2NP_R = crate::BitReader<bool>;
///Field `CC2NP` writer - Capture/Compare 2 output Polarity. Refer to CC1NP description
pub type CC2NP_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM3_CCER_SPEC, bool, O>;
///Field `CC3E` reader - Capture/Compare 3 output enable. Refer to CC1E description
pub type CC3E_R = crate::BitReader<bool>;
///Field `CC3E` writer - Capture/Compare 3 output enable. Refer to CC1E description
pub type CC3E_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM3_CCER_SPEC, bool, O>;
///Field `CC3P` reader - Capture/Compare 3 output Polarity. Refer to CC1P description
pub type CC3P_R = crate::BitReader<bool>;
///Field `CC3P` writer - Capture/Compare 3 output Polarity. Refer to CC1P description
pub type CC3P_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM3_CCER_SPEC, bool, O>;
///Field `CC3NP` reader - Capture/Compare 3 output Polarity. Refer to CC1NP description
pub type CC3NP_R = crate::BitReader<bool>;
///Field `CC3NP` writer - Capture/Compare 3 output Polarity. Refer to CC1NP description
pub type CC3NP_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM3_CCER_SPEC, bool, O>;
///Field `CC4E` reader - Capture/Compare 4 output enable. refer to CC1E description
pub type CC4E_R = crate::BitReader<bool>;
///Field `CC4E` writer - Capture/Compare 4 output enable. refer to CC1E description
pub type CC4E_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM3_CCER_SPEC, bool, O>;
///Field `CC4P` reader - Capture/Compare 4 output Polarity. Refer to CC1P description
pub type CC4P_R = crate::BitReader<bool>;
///Field `CC4P` writer - Capture/Compare 4 output Polarity. Refer to CC1P description
pub type CC4P_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM3_CCER_SPEC, bool, O>;
///Field `CC4NP` reader - Capture/Compare 4 output Polarity. Refer to CC1NP description
pub type CC4NP_R = crate::BitReader<bool>;
///Field `CC4NP` writer - Capture/Compare 4 output Polarity. Refer to CC1NP description
pub type CC4NP_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM3_CCER_SPEC, bool, O>;
impl R {
    ///Bit 0 - Capture/Compare 1 output enable.
    #[inline(always)]
    pub fn cc1e(&self) -> CC1E_R {
        CC1E_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Capture/Compare 1 output Polarity. When CC1 channel is configured as input, both CC1NP/CC1P bits select the active polarity of TI1FP1 and TI2FP1 for trigger or capture operations. CC1NP=0, CC1P=0: non-inverted/rising edge. The circuit is sensitive to TIxFP1 rising edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger operation in gated mode or encoder mode). CC1NP=0, CC1P=1: inverted/falling edge. The circuit is sensitive to TIxFP1 falling edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is inverted (trigger operation in gated mode or encoder mode). CC1NP=1, CC1P=1: non-inverted/both edges. The circuit is sensitive to both TIxFP1 rising and falling edges (capture or trigger operations in reset, external clock or trigger mode), TIxFP1is not inverted (trigger operation in gated mode). This configuration must not be used in encoder mode. CC1NP=1, CC1P=0: this configuration is reserved, it must not be used.
    #[inline(always)]
    pub fn cc1p(&self) -> CC1P_R {
        CC1P_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - Capture/Compare 1 output Polarity. CC1 channel configured as output: CC1NP must be kept cleared in this case. CC1 channel configured as input: This bit is used in conjunction with CC1P to define tim_ti1fp1/tim_ti2fp1 polarity. refer to CC1P description.
    #[inline(always)]
    pub fn cc1np(&self) -> CC1NP_R {
        CC1NP_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Capture/Compare 2 output enable. Refer to CC1E description
    #[inline(always)]
    pub fn cc2e(&self) -> CC2E_R {
        CC2E_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Capture/Compare 2 output Polarity. refer to CC1P description
    #[inline(always)]
    pub fn cc2p(&self) -> CC2P_R {
        CC2P_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 7 - Capture/Compare 2 output Polarity. Refer to CC1NP description
    #[inline(always)]
    pub fn cc2np(&self) -> CC2NP_R {
        CC2NP_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Capture/Compare 3 output enable. Refer to CC1E description
    #[inline(always)]
    pub fn cc3e(&self) -> CC3E_R {
        CC3E_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Capture/Compare 3 output Polarity. Refer to CC1P description
    #[inline(always)]
    pub fn cc3p(&self) -> CC3P_R {
        CC3P_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - Capture/Compare 3 output Polarity. Refer to CC1NP description
    #[inline(always)]
    pub fn cc3np(&self) -> CC3NP_R {
        CC3NP_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Capture/Compare 4 output enable. refer to CC1E description
    #[inline(always)]
    pub fn cc4e(&self) -> CC4E_R {
        CC4E_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Capture/Compare 4 output Polarity. Refer to CC1P description
    #[inline(always)]
    pub fn cc4p(&self) -> CC4P_R {
        CC4P_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 15 - Capture/Compare 4 output Polarity. Refer to CC1NP description
    #[inline(always)]
    pub fn cc4np(&self) -> CC4NP_R {
        CC4NP_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Capture/Compare 1 output enable.
    #[inline(always)]
    #[must_use]
    pub fn cc1e(&mut self) -> CC1E_W<0> {
        CC1E_W::new(self)
    }
    ///Bit 1 - Capture/Compare 1 output Polarity. When CC1 channel is configured as input, both CC1NP/CC1P bits select the active polarity of TI1FP1 and TI2FP1 for trigger or capture operations. CC1NP=0, CC1P=0: non-inverted/rising edge. The circuit is sensitive to TIxFP1 rising edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger operation in gated mode or encoder mode). CC1NP=0, CC1P=1: inverted/falling edge. The circuit is sensitive to TIxFP1 falling edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is inverted (trigger operation in gated mode or encoder mode). CC1NP=1, CC1P=1: non-inverted/both edges. The circuit is sensitive to both TIxFP1 rising and falling edges (capture or trigger operations in reset, external clock or trigger mode), TIxFP1is not inverted (trigger operation in gated mode). This configuration must not be used in encoder mode. CC1NP=1, CC1P=0: this configuration is reserved, it must not be used.
    #[inline(always)]
    #[must_use]
    pub fn cc1p(&mut self) -> CC1P_W<1> {
        CC1P_W::new(self)
    }
    ///Bit 3 - Capture/Compare 1 output Polarity. CC1 channel configured as output: CC1NP must be kept cleared in this case. CC1 channel configured as input: This bit is used in conjunction with CC1P to define tim_ti1fp1/tim_ti2fp1 polarity. refer to CC1P description.
    #[inline(always)]
    #[must_use]
    pub fn cc1np(&mut self) -> CC1NP_W<3> {
        CC1NP_W::new(self)
    }
    ///Bit 4 - Capture/Compare 2 output enable. Refer to CC1E description
    #[inline(always)]
    #[must_use]
    pub fn cc2e(&mut self) -> CC2E_W<4> {
        CC2E_W::new(self)
    }
    ///Bit 5 - Capture/Compare 2 output Polarity. refer to CC1P description
    #[inline(always)]
    #[must_use]
    pub fn cc2p(&mut self) -> CC2P_W<5> {
        CC2P_W::new(self)
    }
    ///Bit 7 - Capture/Compare 2 output Polarity. Refer to CC1NP description
    #[inline(always)]
    #[must_use]
    pub fn cc2np(&mut self) -> CC2NP_W<7> {
        CC2NP_W::new(self)
    }
    ///Bit 8 - Capture/Compare 3 output enable. Refer to CC1E description
    #[inline(always)]
    #[must_use]
    pub fn cc3e(&mut self) -> CC3E_W<8> {
        CC3E_W::new(self)
    }
    ///Bit 9 - Capture/Compare 3 output Polarity. Refer to CC1P description
    #[inline(always)]
    #[must_use]
    pub fn cc3p(&mut self) -> CC3P_W<9> {
        CC3P_W::new(self)
    }
    ///Bit 11 - Capture/Compare 3 output Polarity. Refer to CC1NP description
    #[inline(always)]
    #[must_use]
    pub fn cc3np(&mut self) -> CC3NP_W<11> {
        CC3NP_W::new(self)
    }
    ///Bit 12 - Capture/Compare 4 output enable. refer to CC1E description
    #[inline(always)]
    #[must_use]
    pub fn cc4e(&mut self) -> CC4E_W<12> {
        CC4E_W::new(self)
    }
    ///Bit 13 - Capture/Compare 4 output Polarity. Refer to CC1P description
    #[inline(always)]
    #[must_use]
    pub fn cc4p(&mut self) -> CC4P_W<13> {
        CC4P_W::new(self)
    }
    ///Bit 15 - Capture/Compare 4 output Polarity. Refer to CC1NP description
    #[inline(always)]
    #[must_use]
    pub fn cc4np(&mut self) -> CC4NP_W<15> {
        CC4NP_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TIM3 capture/compare enable register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tim3_ccer](index.html) module
pub struct TIM3_CCER_SPEC;
impl crate::RegisterSpec for TIM3_CCER_SPEC {
    type Ux = u16;
}
///`read()` method returns [tim3_ccer::R](R) reader structure
impl crate::Readable for TIM3_CCER_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [tim3_ccer::W](W) writer structure
impl crate::Writable for TIM3_CCER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets TIM3_CCER to value 0
impl crate::Resettable for TIM3_CCER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
