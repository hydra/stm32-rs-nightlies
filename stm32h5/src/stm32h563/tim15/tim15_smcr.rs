///Register `TIM15_SMCR` reader
pub struct R(crate::R<TIM15_SMCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM15_SMCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM15_SMCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM15_SMCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TIM15_SMCR` writer
pub struct W(crate::W<TIM15_SMCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM15_SMCR_SPEC>;
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
impl From<crate::W<TIM15_SMCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM15_SMCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SMS1` reader - Slave mode selection When external signals are selected the active edge of the trigger signal (tim_trgi) is linked to the polarity selected on the external input (see Input Control register and Control Register description. Others: Reserved. Note: The gated mode must not be used if tim_ti1f_ed is selected as the trigger input (TS=’00100’). Indeed, tim_ti1f_ed outputs 1 pulse for each transition on tim_ti1f, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the tim_trgo signal must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer.
pub type SMS1_R = crate::FieldReader<u8, u8>;
///Field `SMS1` writer - Slave mode selection When external signals are selected the active edge of the trigger signal (tim_trgi) is linked to the polarity selected on the external input (see Input Control register and Control Register description. Others: Reserved. Note: The gated mode must not be used if tim_ti1f_ed is selected as the trigger input (TS=’00100’). Indeed, tim_ti1f_ed outputs 1 pulse for each transition on tim_ti1f, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the tim_trgo signal must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer.
pub type SMS1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM15_SMCR_SPEC, u8, u8, 3, O>;
///Field `TS1` reader - Trigger selection This bit field selects the trigger input to be used to synchronize the counter. Others: Reserved See for more details on tim_itrx meaning for each timer. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition.
pub type TS1_R = crate::FieldReader<u8, u8>;
///Field `TS1` writer - Trigger selection This bit field selects the trigger input to be used to synchronize the counter. Others: Reserved See for more details on tim_itrx meaning for each timer. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition.
pub type TS1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM15_SMCR_SPEC, u8, u8, 3, O>;
///Field `MSM` reader - Master/slave mode
pub type MSM_R = crate::BitReader<bool>;
///Field `MSM` writer - Master/slave mode
pub type MSM_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM15_SMCR_SPEC, bool, O>;
///Field `SMS2` reader - Slave mode selection When external signals are selected the active edge of the trigger signal (tim_trgi) is linked to the polarity selected on the external input (see Input Control register and Control Register description. Others: Reserved. Note: The gated mode must not be used if tim_ti1f_ed is selected as the trigger input (TS=’00100’). Indeed, tim_ti1f_ed outputs 1 pulse for each transition on tim_ti1f, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the tim_trgo signal must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer.
pub type SMS2_R = crate::BitReader<bool>;
///Field `SMS2` writer - Slave mode selection When external signals are selected the active edge of the trigger signal (tim_trgi) is linked to the polarity selected on the external input (see Input Control register and Control Register description. Others: Reserved. Note: The gated mode must not be used if tim_ti1f_ed is selected as the trigger input (TS=’00100’). Indeed, tim_ti1f_ed outputs 1 pulse for each transition on tim_ti1f, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the tim_trgo signal must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer.
pub type SMS2_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM15_SMCR_SPEC, bool, O>;
///Field `TS2` reader - Trigger selection This bit field selects the trigger input to be used to synchronize the counter. Others: Reserved See for more details on tim_itrx meaning for each timer. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition.
pub type TS2_R = crate::FieldReader<u8, u8>;
///Field `TS2` writer - Trigger selection This bit field selects the trigger input to be used to synchronize the counter. Others: Reserved See for more details on tim_itrx meaning for each timer. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition.
pub type TS2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM15_SMCR_SPEC, u8, u8, 2, O>;
impl R {
    ///Bits 0:2 - Slave mode selection When external signals are selected the active edge of the trigger signal (tim_trgi) is linked to the polarity selected on the external input (see Input Control register and Control Register description. Others: Reserved. Note: The gated mode must not be used if tim_ti1f_ed is selected as the trigger input (TS=’00100’). Indeed, tim_ti1f_ed outputs 1 pulse for each transition on tim_ti1f, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the tim_trgo signal must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer.
    #[inline(always)]
    pub fn sms1(&self) -> SMS1_R {
        SMS1_R::new((self.bits & 7) as u8)
    }
    ///Bits 4:6 - Trigger selection This bit field selects the trigger input to be used to synchronize the counter. Others: Reserved See for more details on tim_itrx meaning for each timer. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition.
    #[inline(always)]
    pub fn ts1(&self) -> TS1_R {
        TS1_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 7 - Master/slave mode
    #[inline(always)]
    pub fn msm(&self) -> MSM_R {
        MSM_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 16 - Slave mode selection When external signals are selected the active edge of the trigger signal (tim_trgi) is linked to the polarity selected on the external input (see Input Control register and Control Register description. Others: Reserved. Note: The gated mode must not be used if tim_ti1f_ed is selected as the trigger input (TS=’00100’). Indeed, tim_ti1f_ed outputs 1 pulse for each transition on tim_ti1f, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the tim_trgo signal must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer.
    #[inline(always)]
    pub fn sms2(&self) -> SMS2_R {
        SMS2_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 20:21 - Trigger selection This bit field selects the trigger input to be used to synchronize the counter. Others: Reserved See for more details on tim_itrx meaning for each timer. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition.
    #[inline(always)]
    pub fn ts2(&self) -> TS2_R {
        TS2_R::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    ///Bits 0:2 - Slave mode selection When external signals are selected the active edge of the trigger signal (tim_trgi) is linked to the polarity selected on the external input (see Input Control register and Control Register description. Others: Reserved. Note: The gated mode must not be used if tim_ti1f_ed is selected as the trigger input (TS=’00100’). Indeed, tim_ti1f_ed outputs 1 pulse for each transition on tim_ti1f, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the tim_trgo signal must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer.
    #[inline(always)]
    #[must_use]
    pub fn sms1(&mut self) -> SMS1_W<0> {
        SMS1_W::new(self)
    }
    ///Bits 4:6 - Trigger selection This bit field selects the trigger input to be used to synchronize the counter. Others: Reserved See for more details on tim_itrx meaning for each timer. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition.
    #[inline(always)]
    #[must_use]
    pub fn ts1(&mut self) -> TS1_W<4> {
        TS1_W::new(self)
    }
    ///Bit 7 - Master/slave mode
    #[inline(always)]
    #[must_use]
    pub fn msm(&mut self) -> MSM_W<7> {
        MSM_W::new(self)
    }
    ///Bit 16 - Slave mode selection When external signals are selected the active edge of the trigger signal (tim_trgi) is linked to the polarity selected on the external input (see Input Control register and Control Register description. Others: Reserved. Note: The gated mode must not be used if tim_ti1f_ed is selected as the trigger input (TS=’00100’). Indeed, tim_ti1f_ed outputs 1 pulse for each transition on tim_ti1f, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the tim_trgo signal must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer.
    #[inline(always)]
    #[must_use]
    pub fn sms2(&mut self) -> SMS2_W<16> {
        SMS2_W::new(self)
    }
    ///Bits 20:21 - Trigger selection This bit field selects the trigger input to be used to synchronize the counter. Others: Reserved See for more details on tim_itrx meaning for each timer. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition.
    #[inline(always)]
    #[must_use]
    pub fn ts2(&mut self) -> TS2_W<20> {
        TS2_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TIM15 slave mode control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tim15_smcr](index.html) module
pub struct TIM15_SMCR_SPEC;
impl crate::RegisterSpec for TIM15_SMCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [tim15_smcr::R](R) reader structure
impl crate::Readable for TIM15_SMCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [tim15_smcr::W](W) writer structure
impl crate::Writable for TIM15_SMCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets TIM15_SMCR to value 0
impl crate::Resettable for TIM15_SMCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
