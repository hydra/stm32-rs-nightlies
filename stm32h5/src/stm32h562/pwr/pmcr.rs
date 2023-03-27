///Register `PMCR` reader
pub struct R(crate::R<PMCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PMCR` writer
pub struct W(crate::W<PMCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMCR_SPEC>;
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
impl From<crate::W<PMCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `LPMS` reader - low-power mode selection This bit defines the Deepsleep mode.
pub type LPMS_R = crate::BitReader<bool>;
///Field `LPMS` writer - low-power mode selection This bit defines the Deepsleep mode.
pub type LPMS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMCR_SPEC, bool, O>;
///Field `SVOS` reader - system Stop mode voltage scaling selection These bits control the V&lt;sub>CORE&lt;/sub> voltage level in system Stop mode, to obtain the best trade-off between power consumption and performance.
pub type SVOS_R = crate::FieldReader<u8, u8>;
///Field `SVOS` writer - system Stop mode voltage scaling selection These bits control the V&lt;sub>CORE&lt;/sub> voltage level in system Stop mode, to obtain the best trade-off between power consumption and performance.
pub type SVOS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PMCR_SPEC, u8, u8, 2, O>;
///Field `CSSF` reader - clear Standby and Stop flags (always read as 0) This bit is cleared to 0 by hardware.
pub type CSSF_R = crate::BitReader<bool>;
///Field `CSSF` writer - clear Standby and Stop flags (always read as 0) This bit is cleared to 0 by hardware.
pub type CSSF_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMCR_SPEC, bool, O>;
///Field `FLPS` reader - Flash memory low-power mode in Stop mode This bit is used to obtain the best trade-off between low-power consumption and restart time when exiting from Stop mode. When it is set, the Flash memory enters low-power mode when the CPU domain is in Stop mode. Note: When system enters stop mode with SVOS5 enabled, Flash memory is automatically forced in low-power mode.
pub type FLPS_R = crate::BitReader<bool>;
///Field `FLPS` writer - Flash memory low-power mode in Stop mode This bit is used to obtain the best trade-off between low-power consumption and restart time when exiting from Stop mode. When it is set, the Flash memory enters low-power mode when the CPU domain is in Stop mode. Note: When system enters stop mode with SVOS5 enabled, Flash memory is automatically forced in low-power mode.
pub type FLPS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMCR_SPEC, bool, O>;
///Field `BOOSTE` reader - analog switch V&lt;sub>BOOST&lt;/sub> control This bit enables the booster to guarantee the analog switch AC performance when the V&lt;sub>DD&lt;/sub> supply voltage is below 2.7 V (reduction of the total harmonic distortion to have the same switch performance over the full supply voltage range) The V&lt;sub>DD&lt;/sub> supply voltage can be monitored through the PVD and the PLS bits.
pub type BOOSTE_R = crate::BitReader<bool>;
///Field `BOOSTE` writer - analog switch V&lt;sub>BOOST&lt;/sub> control This bit enables the booster to guarantee the analog switch AC performance when the V&lt;sub>DD&lt;/sub> supply voltage is below 2.7 V (reduction of the total harmonic distortion to have the same switch performance over the full supply voltage range) The V&lt;sub>DD&lt;/sub> supply voltage can be monitored through the PVD and the PLS bits.
pub type BOOSTE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMCR_SPEC, bool, O>;
///Field `AVD_READY` reader - analog voltage ready This bit is only used when the analog switch boost needs to be enabled (see BOOSTE bit). It must be set by software when the expected V&lt;sub>DDA&lt;/sub> analog supply level is available. The correct analog supply level is indicated by the AVDO bit (PWR_VMSR register) after setting the AVDEN bit (PWR_VMCR register) and selecting the supply level to be monitored (ALS bits).
pub type AVD_READY_R = crate::BitReader<bool>;
///Field `AVD_READY` writer - analog voltage ready This bit is only used when the analog switch boost needs to be enabled (see BOOSTE bit). It must be set by software when the expected V&lt;sub>DDA&lt;/sub> analog supply level is available. The correct analog supply level is indicated by the AVDO bit (PWR_VMSR register) after setting the AVDEN bit (PWR_VMCR register) and selecting the supply level to be monitored (ALS bits).
pub type AVD_READY_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMCR_SPEC, bool, O>;
///Field `SRAM3SO` reader - AHB SRAM3 shut-off in Stop mode.
pub type SRAM3SO_R = crate::BitReader<bool>;
///Field `SRAM3SO` writer - AHB SRAM3 shut-off in Stop mode.
pub type SRAM3SO_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMCR_SPEC, bool, O>;
///Field `SRAM2_16SO` reader - AHB SRAM2 16-Kbyte shut-off in Stop mode.
pub type SRAM2_16SO_R = crate::BitReader<bool>;
///Field `SRAM2_16SO` writer - AHB SRAM2 16-Kbyte shut-off in Stop mode.
pub type SRAM2_16SO_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMCR_SPEC, bool, O>;
///Field `SRAM2_48SO` reader - AHB SRAM2 48-Kbyte shut-off in Stop mode.
pub type SRAM2_48SO_R = crate::BitReader<bool>;
///Field `SRAM2_48SO` writer - AHB SRAM2 48-Kbyte shut-off in Stop mode.
pub type SRAM2_48SO_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMCR_SPEC, bool, O>;
///Field `SRAM1SO` reader - AHB SRAM1 shut-off in Stop mode
pub type SRAM1SO_R = crate::BitReader<bool>;
///Field `SRAM1SO` writer - AHB SRAM1 shut-off in Stop mode
pub type SRAM1SO_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMCR_SPEC, bool, O>;
impl R {
    ///Bit 0 - low-power mode selection This bit defines the Deepsleep mode.
    #[inline(always)]
    pub fn lpms(&self) -> LPMS_R {
        LPMS_R::new((self.bits & 1) != 0)
    }
    ///Bits 2:3 - system Stop mode voltage scaling selection These bits control the V&lt;sub>CORE&lt;/sub> voltage level in system Stop mode, to obtain the best trade-off between power consumption and performance.
    #[inline(always)]
    pub fn svos(&self) -> SVOS_R {
        SVOS_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bit 7 - clear Standby and Stop flags (always read as 0) This bit is cleared to 0 by hardware.
    #[inline(always)]
    pub fn cssf(&self) -> CSSF_R {
        CSSF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 9 - Flash memory low-power mode in Stop mode This bit is used to obtain the best trade-off between low-power consumption and restart time when exiting from Stop mode. When it is set, the Flash memory enters low-power mode when the CPU domain is in Stop mode. Note: When system enters stop mode with SVOS5 enabled, Flash memory is automatically forced in low-power mode.
    #[inline(always)]
    pub fn flps(&self) -> FLPS_R {
        FLPS_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 12 - analog switch V&lt;sub>BOOST&lt;/sub> control This bit enables the booster to guarantee the analog switch AC performance when the V&lt;sub>DD&lt;/sub> supply voltage is below 2.7 V (reduction of the total harmonic distortion to have the same switch performance over the full supply voltage range) The V&lt;sub>DD&lt;/sub> supply voltage can be monitored through the PVD and the PLS bits.
    #[inline(always)]
    pub fn booste(&self) -> BOOSTE_R {
        BOOSTE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - analog voltage ready This bit is only used when the analog switch boost needs to be enabled (see BOOSTE bit). It must be set by software when the expected V&lt;sub>DDA&lt;/sub> analog supply level is available. The correct analog supply level is indicated by the AVDO bit (PWR_VMSR register) after setting the AVDEN bit (PWR_VMCR register) and selecting the supply level to be monitored (ALS bits).
    #[inline(always)]
    pub fn avd_ready(&self) -> AVD_READY_R {
        AVD_READY_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 23 - AHB SRAM3 shut-off in Stop mode.
    #[inline(always)]
    pub fn sram3so(&self) -> SRAM3SO_R {
        SRAM3SO_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - AHB SRAM2 16-Kbyte shut-off in Stop mode.
    #[inline(always)]
    pub fn sram2_16so(&self) -> SRAM2_16SO_R {
        SRAM2_16SO_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - AHB SRAM2 48-Kbyte shut-off in Stop mode.
    #[inline(always)]
    pub fn sram2_48so(&self) -> SRAM2_48SO_R {
        SRAM2_48SO_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - AHB SRAM1 shut-off in Stop mode
    #[inline(always)]
    pub fn sram1so(&self) -> SRAM1SO_R {
        SRAM1SO_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - low-power mode selection This bit defines the Deepsleep mode.
    #[inline(always)]
    #[must_use]
    pub fn lpms(&mut self) -> LPMS_W<0> {
        LPMS_W::new(self)
    }
    ///Bits 2:3 - system Stop mode voltage scaling selection These bits control the V&lt;sub>CORE&lt;/sub> voltage level in system Stop mode, to obtain the best trade-off between power consumption and performance.
    #[inline(always)]
    #[must_use]
    pub fn svos(&mut self) -> SVOS_W<2> {
        SVOS_W::new(self)
    }
    ///Bit 7 - clear Standby and Stop flags (always read as 0) This bit is cleared to 0 by hardware.
    #[inline(always)]
    #[must_use]
    pub fn cssf(&mut self) -> CSSF_W<7> {
        CSSF_W::new(self)
    }
    ///Bit 9 - Flash memory low-power mode in Stop mode This bit is used to obtain the best trade-off between low-power consumption and restart time when exiting from Stop mode. When it is set, the Flash memory enters low-power mode when the CPU domain is in Stop mode. Note: When system enters stop mode with SVOS5 enabled, Flash memory is automatically forced in low-power mode.
    #[inline(always)]
    #[must_use]
    pub fn flps(&mut self) -> FLPS_W<9> {
        FLPS_W::new(self)
    }
    ///Bit 12 - analog switch V&lt;sub>BOOST&lt;/sub> control This bit enables the booster to guarantee the analog switch AC performance when the V&lt;sub>DD&lt;/sub> supply voltage is below 2.7 V (reduction of the total harmonic distortion to have the same switch performance over the full supply voltage range) The V&lt;sub>DD&lt;/sub> supply voltage can be monitored through the PVD and the PLS bits.
    #[inline(always)]
    #[must_use]
    pub fn booste(&mut self) -> BOOSTE_W<12> {
        BOOSTE_W::new(self)
    }
    ///Bit 13 - analog voltage ready This bit is only used when the analog switch boost needs to be enabled (see BOOSTE bit). It must be set by software when the expected V&lt;sub>DDA&lt;/sub> analog supply level is available. The correct analog supply level is indicated by the AVDO bit (PWR_VMSR register) after setting the AVDEN bit (PWR_VMCR register) and selecting the supply level to be monitored (ALS bits).
    #[inline(always)]
    #[must_use]
    pub fn avd_ready(&mut self) -> AVD_READY_W<13> {
        AVD_READY_W::new(self)
    }
    ///Bit 23 - AHB SRAM3 shut-off in Stop mode.
    #[inline(always)]
    #[must_use]
    pub fn sram3so(&mut self) -> SRAM3SO_W<23> {
        SRAM3SO_W::new(self)
    }
    ///Bit 24 - AHB SRAM2 16-Kbyte shut-off in Stop mode.
    #[inline(always)]
    #[must_use]
    pub fn sram2_16so(&mut self) -> SRAM2_16SO_W<24> {
        SRAM2_16SO_W::new(self)
    }
    ///Bit 25 - AHB SRAM2 48-Kbyte shut-off in Stop mode.
    #[inline(always)]
    #[must_use]
    pub fn sram2_48so(&mut self) -> SRAM2_48SO_W<25> {
        SRAM2_48SO_W::new(self)
    }
    ///Bit 26 - AHB SRAM1 shut-off in Stop mode
    #[inline(always)]
    #[must_use]
    pub fn sram1so(&mut self) -> SRAM1SO_W<26> {
        SRAM1SO_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///PWR power mode control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pmcr](index.html) module
pub struct PMCR_SPEC;
impl crate::RegisterSpec for PMCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [pmcr::R](R) reader structure
impl crate::Readable for PMCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [pmcr::W](W) writer structure
impl crate::Writable for PMCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets PMCR to value 0x0c
impl crate::Resettable for PMCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0c;
}
