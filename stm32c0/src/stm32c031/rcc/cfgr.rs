///Register `CFGR` reader
pub struct R(crate::R<CFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CFGR` writer
pub struct W(crate::W<CFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR_SPEC>;
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
impl From<crate::W<CFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SW` reader - System clock switch This bitfield is controlled by software and hardware. The bitfield selects the clock for SYSCLK as follows: Others: Reserved The setting is forced by hardware to 000 (HSISYS selected) when the MCU exits Stop, or Standby, or Shutdown mode, or when the setting is 001 (HSE selected) and HSE oscillator failure is detected.
pub type SW_R = crate::FieldReader<u8, u8>;
///Field `SW` writer - System clock switch This bitfield is controlled by software and hardware. The bitfield selects the clock for SYSCLK as follows: Others: Reserved The setting is forced by hardware to 000 (HSISYS selected) when the MCU exits Stop, or Standby, or Shutdown mode, or when the setting is 001 (HSE selected) and HSE oscillator failure is detected.
pub type SW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, u8, 3, O>;
///Field `SWS` reader - System clock switch status This bitfield is controlled by hardware to indicate the clock source used as system clock: Others: Reserved
pub type SWS_R = crate::FieldReader<u8, u8>;
///Field `HPRE` reader - AHB prescaler This bitfield is controlled by software. To produce HCLK clock, it sets the division factor of SYSCLK clock as follows: 0xxx: 1
pub type HPRE_R = crate::FieldReader<u8, u8>;
///Field `HPRE` writer - AHB prescaler This bitfield is controlled by software. To produce HCLK clock, it sets the division factor of SYSCLK clock as follows: 0xxx: 1
pub type HPRE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, u8, 4, O>;
///Field `PPRE` reader - APB prescaler This bitfield is controlled by software. To produce PCLK clock, it sets the division factor of HCLK clock as follows: 0xx: 1
pub type PPRE_R = crate::FieldReader<u8, u8>;
///Field `PPRE` writer - APB prescaler This bitfield is controlled by software. To produce PCLK clock, it sets the division factor of HCLK clock as follows: 0xx: 1
pub type PPRE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, u8, 3, O>;
///Field `MCO2SEL` reader - Microcontroller clock output 2 clock selector This bitfield is controlled by software. It sets the clock selector for MCO2 output as follows: This bitfield is controlled by software. It sets the clock selector for MCO output as follows: Note: This clock output may have some truncated cycles at startup or during MCO2 clock source switching.
pub type MCO2SEL_R = crate::FieldReader<u8, u8>;
///Field `MCO2SEL` writer - Microcontroller clock output 2 clock selector This bitfield is controlled by software. It sets the clock selector for MCO2 output as follows: This bitfield is controlled by software. It sets the clock selector for MCO output as follows: Note: This clock output may have some truncated cycles at startup or during MCO2 clock source switching.
pub type MCO2SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, u8, 4, O>;
///Field `MCO2PRE` reader - Microcontroller clock output 2 prescaler This bitfield is controlled by software. It sets the division factor of the clock sent to the MCO2 output as follows: ... It is highly recommended to set this field before the MCO2 output is enabled.
pub type MCO2PRE_R = crate::FieldReader<u8, u8>;
///Field `MCO2PRE` writer - Microcontroller clock output 2 prescaler This bitfield is controlled by software. It sets the division factor of the clock sent to the MCO2 output as follows: ... It is highly recommended to set this field before the MCO2 output is enabled.
pub type MCO2PRE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, u8, 4, O>;
///Field `MCOSEL` reader - Microcontroller clock output clock selector This bitfield is controlled by software. It sets the clock selector for MCO output as follows: Note: This clock output may have some truncated cycles at startup or during MCO clock source switching. Any other value means no clock on MCO.
pub type MCOSEL_R = crate::FieldReader<u8, u8>;
///Field `MCOSEL` writer - Microcontroller clock output clock selector This bitfield is controlled by software. It sets the clock selector for MCO output as follows: Note: This clock output may have some truncated cycles at startup or during MCO clock source switching. Any other value means no clock on MCO.
pub type MCOSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, u8, 4, O>;
///Field `MCOPRE` reader - Microcontroller clock output prescaler This bitfield is controlled by software. It sets the division factor of the clock sent to the MCO output as follows: ... It is highly recommended to set this field before the MCO output is enabled.
pub type MCOPRE_R = crate::FieldReader<u8, u8>;
///Field `MCOPRE` writer - Microcontroller clock output prescaler This bitfield is controlled by software. It sets the division factor of the clock sent to the MCO output as follows: ... It is highly recommended to set this field before the MCO output is enabled.
pub type MCOPRE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, u8, 4, O>;
impl R {
    ///Bits 0:2 - System clock switch This bitfield is controlled by software and hardware. The bitfield selects the clock for SYSCLK as follows: Others: Reserved The setting is forced by hardware to 000 (HSISYS selected) when the MCU exits Stop, or Standby, or Shutdown mode, or when the setting is 001 (HSE selected) and HSE oscillator failure is detected.
    #[inline(always)]
    pub fn sw(&self) -> SW_R {
        SW_R::new((self.bits & 7) as u8)
    }
    ///Bits 3:5 - System clock switch status This bitfield is controlled by hardware to indicate the clock source used as system clock: Others: Reserved
    #[inline(always)]
    pub fn sws(&self) -> SWS_R {
        SWS_R::new(((self.bits >> 3) & 7) as u8)
    }
    ///Bits 8:11 - AHB prescaler This bitfield is controlled by software. To produce HCLK clock, it sets the division factor of SYSCLK clock as follows: 0xxx: 1
    #[inline(always)]
    pub fn hpre(&self) -> HPRE_R {
        HPRE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:14 - APB prescaler This bitfield is controlled by software. To produce PCLK clock, it sets the division factor of HCLK clock as follows: 0xx: 1
    #[inline(always)]
    pub fn ppre(&self) -> PPRE_R {
        PPRE_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bits 16:19 - Microcontroller clock output 2 clock selector This bitfield is controlled by software. It sets the clock selector for MCO2 output as follows: This bitfield is controlled by software. It sets the clock selector for MCO output as follows: Note: This clock output may have some truncated cycles at startup or during MCO2 clock source switching.
    #[inline(always)]
    pub fn mco2sel(&self) -> MCO2SEL_R {
        MCO2SEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - Microcontroller clock output 2 prescaler This bitfield is controlled by software. It sets the division factor of the clock sent to the MCO2 output as follows: ... It is highly recommended to set this field before the MCO2 output is enabled.
    #[inline(always)]
    pub fn mco2pre(&self) -> MCO2PRE_R {
        MCO2PRE_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:27 - Microcontroller clock output clock selector This bitfield is controlled by software. It sets the clock selector for MCO output as follows: Note: This clock output may have some truncated cycles at startup or during MCO clock source switching. Any other value means no clock on MCO.
    #[inline(always)]
    pub fn mcosel(&self) -> MCOSEL_R {
        MCOSEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:31 - Microcontroller clock output prescaler This bitfield is controlled by software. It sets the division factor of the clock sent to the MCO output as follows: ... It is highly recommended to set this field before the MCO output is enabled.
    #[inline(always)]
    pub fn mcopre(&self) -> MCOPRE_R {
        MCOPRE_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    ///Bits 0:2 - System clock switch This bitfield is controlled by software and hardware. The bitfield selects the clock for SYSCLK as follows: Others: Reserved The setting is forced by hardware to 000 (HSISYS selected) when the MCU exits Stop, or Standby, or Shutdown mode, or when the setting is 001 (HSE selected) and HSE oscillator failure is detected.
    #[inline(always)]
    #[must_use]
    pub fn sw(&mut self) -> SW_W<0> {
        SW_W::new(self)
    }
    ///Bits 8:11 - AHB prescaler This bitfield is controlled by software. To produce HCLK clock, it sets the division factor of SYSCLK clock as follows: 0xxx: 1
    #[inline(always)]
    #[must_use]
    pub fn hpre(&mut self) -> HPRE_W<8> {
        HPRE_W::new(self)
    }
    ///Bits 12:14 - APB prescaler This bitfield is controlled by software. To produce PCLK clock, it sets the division factor of HCLK clock as follows: 0xx: 1
    #[inline(always)]
    #[must_use]
    pub fn ppre(&mut self) -> PPRE_W<12> {
        PPRE_W::new(self)
    }
    ///Bits 16:19 - Microcontroller clock output 2 clock selector This bitfield is controlled by software. It sets the clock selector for MCO2 output as follows: This bitfield is controlled by software. It sets the clock selector for MCO output as follows: Note: This clock output may have some truncated cycles at startup or during MCO2 clock source switching.
    #[inline(always)]
    #[must_use]
    pub fn mco2sel(&mut self) -> MCO2SEL_W<16> {
        MCO2SEL_W::new(self)
    }
    ///Bits 20:23 - Microcontroller clock output 2 prescaler This bitfield is controlled by software. It sets the division factor of the clock sent to the MCO2 output as follows: ... It is highly recommended to set this field before the MCO2 output is enabled.
    #[inline(always)]
    #[must_use]
    pub fn mco2pre(&mut self) -> MCO2PRE_W<20> {
        MCO2PRE_W::new(self)
    }
    ///Bits 24:27 - Microcontroller clock output clock selector This bitfield is controlled by software. It sets the clock selector for MCO output as follows: Note: This clock output may have some truncated cycles at startup or during MCO clock source switching. Any other value means no clock on MCO.
    #[inline(always)]
    #[must_use]
    pub fn mcosel(&mut self) -> MCOSEL_W<24> {
        MCOSEL_W::new(self)
    }
    ///Bits 28:31 - Microcontroller clock output prescaler This bitfield is controlled by software. It sets the division factor of the clock sent to the MCO output as follows: ... It is highly recommended to set this field before the MCO output is enabled.
    #[inline(always)]
    #[must_use]
    pub fn mcopre(&mut self) -> MCOPRE_W<28> {
        MCOPRE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RCC clock configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cfgr](index.html) module
pub struct CFGR_SPEC;
impl crate::RegisterSpec for CFGR_SPEC {
    type Ux = u32;
}
///`read()` method returns [cfgr::R](R) reader structure
impl crate::Readable for CFGR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cfgr::W](W) writer structure
impl crate::Writable for CFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CFGR to value 0
impl crate::Resettable for CFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
