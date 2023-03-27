///Register `RCC_CFGR1` reader
pub struct R(crate::R<RCC_CFGR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_CFGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_CFGR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_CFGR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RCC_CFGR1` writer
pub struct W(crate::W<RCC_CFGR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_CFGR1_SPEC>;
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
impl From<crate::W<RCC_CFGR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_CFGR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SW` reader - system clock switch Set and cleared by software to select system clock source (SYSCLK). Configured by hardware to force MSIS oscillator selection when exiting Standby or Shutdown mode. Configured by hardware to force MSIS or HSI16 oscillator selection when exiting Stop mode or in case of HSE oscillator failure, depending on STOPWUCK value.
pub type SW_R = crate::FieldReader<u8, u8>;
///Field `SW` writer - system clock switch Set and cleared by software to select system clock source (SYSCLK). Configured by hardware to force MSIS oscillator selection when exiting Standby or Shutdown mode. Configured by hardware to force MSIS or HSI16 oscillator selection when exiting Stop mode or in case of HSE oscillator failure, depending on STOPWUCK value.
pub type SW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_CFGR1_SPEC, u8, u8, 2, O>;
///Field `SWS` reader - system clock switch status Set and cleared by hardware to indicate which clock source is used as system clock.
pub type SWS_R = crate::FieldReader<u8, u8>;
///Field `STOPWUCK` reader - wakeup from Stop and CSS backup clock selection Set and cleared by software to select the system clock used when exiting Stop mode. The selected clock is also used as emergency clock for the clock security system on HSE. Warning: STOPWUCK must not be modified when the CSS is enabled by HSECSSON bit in RCC_CR and the system clock is HSE (SWS = 10) or a switch on HSE is requested (SW = 10).
pub type STOPWUCK_R = crate::BitReader<bool>;
///Field `STOPWUCK` writer - wakeup from Stop and CSS backup clock selection Set and cleared by software to select the system clock used when exiting Stop mode. The selected clock is also used as emergency clock for the clock security system on HSE. Warning: STOPWUCK must not be modified when the CSS is enabled by HSECSSON bit in RCC_CR and the system clock is HSE (SWS = 10) or a switch on HSE is requested (SW = 10).
pub type STOPWUCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_CFGR1_SPEC, bool, O>;
///Field `STOPKERWUCK` reader - wakeup from Stop kernel clock automatic enable selection Set and cleared by software to enable automatically another oscillator when exiting Stop mode. This oscillator can be used as independent kernel clock by peripherals.
pub type STOPKERWUCK_R = crate::BitReader<bool>;
///Field `STOPKERWUCK` writer - wakeup from Stop kernel clock automatic enable selection Set and cleared by software to enable automatically another oscillator when exiting Stop mode. This oscillator can be used as independent kernel clock by peripherals.
pub type STOPKERWUCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_CFGR1_SPEC, bool, O>;
///Field `MCOSEL` reader - microcontroller clock output Set and cleared by software. Others: reserved Note: This clock output may have some truncated cycles at startup or during MCO clock source switching.
pub type MCOSEL_R = crate::FieldReader<u8, u8>;
///Field `MCOSEL` writer - microcontroller clock output Set and cleared by software. Others: reserved Note: This clock output may have some truncated cycles at startup or during MCO clock source switching.
pub type MCOSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_CFGR1_SPEC, u8, u8, 4, O>;
///Field `MCOPRE` reader - microcontroller clock output prescaler Set and cleared by software. It is highly recommended to change this prescaler before MCO output is enabled. Others: not allowed
pub type MCOPRE_R = crate::FieldReader<u8, u8>;
///Field `MCOPRE` writer - microcontroller clock output prescaler Set and cleared by software. It is highly recommended to change this prescaler before MCO output is enabled. Others: not allowed
pub type MCOPRE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_CFGR1_SPEC, u8, u8, 3, O>;
impl R {
    ///Bits 0:1 - system clock switch Set and cleared by software to select system clock source (SYSCLK). Configured by hardware to force MSIS oscillator selection when exiting Standby or Shutdown mode. Configured by hardware to force MSIS or HSI16 oscillator selection when exiting Stop mode or in case of HSE oscillator failure, depending on STOPWUCK value.
    #[inline(always)]
    pub fn sw(&self) -> SW_R {
        SW_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - system clock switch status Set and cleared by hardware to indicate which clock source is used as system clock.
    #[inline(always)]
    pub fn sws(&self) -> SWS_R {
        SWS_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bit 4 - wakeup from Stop and CSS backup clock selection Set and cleared by software to select the system clock used when exiting Stop mode. The selected clock is also used as emergency clock for the clock security system on HSE. Warning: STOPWUCK must not be modified when the CSS is enabled by HSECSSON bit in RCC_CR and the system clock is HSE (SWS = 10) or a switch on HSE is requested (SW = 10).
    #[inline(always)]
    pub fn stopwuck(&self) -> STOPWUCK_R {
        STOPWUCK_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - wakeup from Stop kernel clock automatic enable selection Set and cleared by software to enable automatically another oscillator when exiting Stop mode. This oscillator can be used as independent kernel clock by peripherals.
    #[inline(always)]
    pub fn stopkerwuck(&self) -> STOPKERWUCK_R {
        STOPKERWUCK_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 24:27 - microcontroller clock output Set and cleared by software. Others: reserved Note: This clock output may have some truncated cycles at startup or during MCO clock source switching.
    #[inline(always)]
    pub fn mcosel(&self) -> MCOSEL_R {
        MCOSEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:30 - microcontroller clock output prescaler Set and cleared by software. It is highly recommended to change this prescaler before MCO output is enabled. Others: not allowed
    #[inline(always)]
    pub fn mcopre(&self) -> MCOPRE_R {
        MCOPRE_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    ///Bits 0:1 - system clock switch Set and cleared by software to select system clock source (SYSCLK). Configured by hardware to force MSIS oscillator selection when exiting Standby or Shutdown mode. Configured by hardware to force MSIS or HSI16 oscillator selection when exiting Stop mode or in case of HSE oscillator failure, depending on STOPWUCK value.
    #[inline(always)]
    #[must_use]
    pub fn sw(&mut self) -> SW_W<0> {
        SW_W::new(self)
    }
    ///Bit 4 - wakeup from Stop and CSS backup clock selection Set and cleared by software to select the system clock used when exiting Stop mode. The selected clock is also used as emergency clock for the clock security system on HSE. Warning: STOPWUCK must not be modified when the CSS is enabled by HSECSSON bit in RCC_CR and the system clock is HSE (SWS = 10) or a switch on HSE is requested (SW = 10).
    #[inline(always)]
    #[must_use]
    pub fn stopwuck(&mut self) -> STOPWUCK_W<4> {
        STOPWUCK_W::new(self)
    }
    ///Bit 5 - wakeup from Stop kernel clock automatic enable selection Set and cleared by software to enable automatically another oscillator when exiting Stop mode. This oscillator can be used as independent kernel clock by peripherals.
    #[inline(always)]
    #[must_use]
    pub fn stopkerwuck(&mut self) -> STOPKERWUCK_W<5> {
        STOPKERWUCK_W::new(self)
    }
    ///Bits 24:27 - microcontroller clock output Set and cleared by software. Others: reserved Note: This clock output may have some truncated cycles at startup or during MCO clock source switching.
    #[inline(always)]
    #[must_use]
    pub fn mcosel(&mut self) -> MCOSEL_W<24> {
        MCOSEL_W::new(self)
    }
    ///Bits 28:30 - microcontroller clock output prescaler Set and cleared by software. It is highly recommended to change this prescaler before MCO output is enabled. Others: not allowed
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
///RCC clock configuration register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rcc_cfgr1](index.html) module
pub struct RCC_CFGR1_SPEC;
impl crate::RegisterSpec for RCC_CFGR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [rcc_cfgr1::R](R) reader structure
impl crate::Readable for RCC_CFGR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rcc_cfgr1::W](W) writer structure
impl crate::Writable for RCC_CFGR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RCC_CFGR1 to value 0
impl crate::Resettable for RCC_CFGR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
