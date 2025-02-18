///Register `FCCAN_CCU_CWD` reader
pub struct R(crate::R<FCCAN_CCU_CWD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCCAN_CCU_CWD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCCAN_CCU_CWD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCCAN_CCU_CWD_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FCCAN_CCU_CWD` writer
pub struct W(crate::W<FCCAN_CCU_CWD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCCAN_CCU_CWD_SPEC>;
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
impl From<crate::W<FCCAN_CCU_CWD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCCAN_CCU_CWD_SPEC>) -> Self {
        W(writer)
    }
}
///Field `WDC` reader - WDC
pub type WDC_R = crate::FieldReader<u16, u16>;
///Field `WDC` writer - WDC
pub type WDC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FCCAN_CCU_CWD_SPEC, u16, u16, 16, O>;
///Field `WDV` reader - WDV
pub type WDV_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:15 - WDC
    #[inline(always)]
    pub fn wdc(&self) -> WDC_R {
        WDC_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - WDV
    #[inline(always)]
    pub fn wdv(&self) -> WDV_R {
        WDV_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - WDC
    #[inline(always)]
    #[must_use]
    pub fn wdc(&mut self) -> WDC_W<0> {
        WDC_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///The calibration watchdog is started after the first falling edge when the calibration FSM is in state Not_Calibrated (CCU_CSTAT.CALS = 00). In this state the calibration watchdog monitors the message received. In case no message was received until the calibration watchdog has counted down to 0, the calibration FSM stays in state Not_Calibrated (CCU_CSTAT.CALS = 00), the counter is reloaded with FDCAN_RWD.WDC and basic calibration is restarted after the next falling edge. When in state Basic_Calibrated (CCU_CSTAT.CALS = 01), the calibration watchdog is restarted with each received message . In case no message was received until the calibration watchdog has counted down to 0, the calibration FSM returns to state Not_Calibrated (CCU_CSTAT.CALS = 00), the counter is reloaded with FDCAN_RWD.WDC and basic calibration is restarted after the next falling edge. When a quartz message is received, state Precision_Calibrated (CCU_CSTAT.CALS = 10) is entered and the calibration watchdog is restarted. In this state the calibration watchdog monitors the quartz message received input. In case no message from a quartz controlled node is received by the attached TTCAN until the calibration watchdog has counted down to 0, the calibration FSM transits back to state Basic_Calibrated (CCU_CSTAT.CALS = 01). The signal is active when the CAN protocol engine on the attached TTCAN is started i.e. when the INIT bit is reset. A calibration watchdog event also sets interrupt flag CCU_IR.CWE. If enabled by CCU_IE.CWEE, interrupt line is activated (set to high). Interrupt line remains active until interrupt flag CCU_IR.CWE is reset.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fccan_ccu_cwd](index.html) module
pub struct FCCAN_CCU_CWD_SPEC;
impl crate::RegisterSpec for FCCAN_CCU_CWD_SPEC {
    type Ux = u32;
}
///`read()` method returns [fccan_ccu_cwd::R](R) reader structure
impl crate::Readable for FCCAN_CCU_CWD_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [fccan_ccu_cwd::W](W) writer structure
impl crate::Writable for FCCAN_CCU_CWD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets FCCAN_CCU_CWD to value 0
impl crate::Resettable for FCCAN_CCU_CWD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
