///Register `RCC_CR` reader
pub struct R(crate::R<RCC_CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_CR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RCC_CR` writer
pub struct W(crate::W<RCC_CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_CR_SPEC>;
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
impl From<crate::W<RCC_CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_CR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MSISON` reader - MSIS clock enable Set and cleared by software. Cleared by hardware to stop the MSIS oscillator when entering Stop, Standby or Shutdown mode. Set by hardware to force the MSIS oscillator ON when exiting Standby or Shutdown mode. Set by hardware to force the MSIS oscillator ON when STOPWUCK = 0 when exiting Stop modes or in case of a failure of the HSE oscillator. Set by hardware when used directly or indirectly as system clock.
pub type MSISON_R = crate::BitReader<bool>;
///Field `MSISON` writer - MSIS clock enable Set and cleared by software. Cleared by hardware to stop the MSIS oscillator when entering Stop, Standby or Shutdown mode. Set by hardware to force the MSIS oscillator ON when exiting Standby or Shutdown mode. Set by hardware to force the MSIS oscillator ON when STOPWUCK = 0 when exiting Stop modes or in case of a failure of the HSE oscillator. Set by hardware when used directly or indirectly as system clock.
pub type MSISON_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_CR_SPEC, bool, O>;
///Field `MSIKERON` reader - MSI enable for some peripheral kernels Set and cleared by software to force MSI ON even in Stop modes. Keeping the MSI ON in Stop mode allows the communication speed not to be reduced by the MSI startup time. This bit has no effect on MSISON and MSIKON values (see autonomous mode for more details). The MSIKERON must be configured at 0 before entering Stop 3 mode.
pub type MSIKERON_R = crate::BitReader<bool>;
///Field `MSIKERON` writer - MSI enable for some peripheral kernels Set and cleared by software to force MSI ON even in Stop modes. Keeping the MSI ON in Stop mode allows the communication speed not to be reduced by the MSI startup time. This bit has no effect on MSISON and MSIKON values (see autonomous mode for more details). The MSIKERON must be configured at 0 before entering Stop 3 mode.
pub type MSIKERON_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_CR_SPEC, bool, O>;
///Field `MSISRDY` reader - MSIS clock ready flag Set by hardware to indicate that the MSIS oscillator is stable. This bit is set only when MSIS is enabled by software by setting MSISON. Note: Once the MSISON bit is cleared, MSISRDY goes low after six MSIS clock cycles.
pub type MSISRDY_R = crate::BitReader<bool>;
///Field `MSIPLLEN` reader - MSI clock PLL-mode enable Set and cleared by software to enable/disable the PLL part of the MSI clock source. MSIPLLEN must be enabled after LSE is enabled (LSEON enabled) and ready (LSERDY set by hardware). A hardware protection prevents from enabling MSIPLLEN if LSE is not ready. This bit is cleared by hardware when LSE is disabled (LSEON = 0) or when the CSS on LSE detects a LSE failure (see RCC_CSR).
pub type MSIPLLEN_R = crate::BitReader<bool>;
///Field `MSIPLLEN` writer - MSI clock PLL-mode enable Set and cleared by software to enable/disable the PLL part of the MSI clock source. MSIPLLEN must be enabled after LSE is enabled (LSEON enabled) and ready (LSERDY set by hardware). A hardware protection prevents from enabling MSIPLLEN if LSE is not ready. This bit is cleared by hardware when LSE is disabled (LSEON = 0) or when the CSS on LSE detects a LSE failure (see RCC_CSR).
pub type MSIPLLEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_CR_SPEC, bool, O>;
///Field `MSIKON` reader - MSIK clock enable Set and cleared by software. Cleared by hardware to stop the MSIK when entering Stop, Standby or Shutdown mode. Set by hardware to force the MSIK oscillator ON when exiting Standby or Shutdown mode. Set by hardware to force the MSIK oscillator ON when STOPWUCK = 0 or STOPKERWUCK = 0 when exiting Stop modes or in case of a failure of the HSE oscillator.
pub type MSIKON_R = crate::BitReader<bool>;
///Field `MSIKON` writer - MSIK clock enable Set and cleared by software. Cleared by hardware to stop the MSIK when entering Stop, Standby or Shutdown mode. Set by hardware to force the MSIK oscillator ON when exiting Standby or Shutdown mode. Set by hardware to force the MSIK oscillator ON when STOPWUCK = 0 or STOPKERWUCK = 0 when exiting Stop modes or in case of a failure of the HSE oscillator.
pub type MSIKON_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_CR_SPEC, bool, O>;
///Field `MSIKRDY` reader - MSIK clock ready flag Set by hardware to indicate that the MSIK is stable. This bit is set only when MSI kernel oscillator is enabled by software by setting MSIKON. Note: Once the MSIKON bit is cleared, MSIKRDY goes low after six MSIK oscillator clock cycles.
pub type MSIKRDY_R = crate::BitReader<bool>;
///Field `MSIPLLSEL` reader - MSI clock with PLL mode selection Set and cleared by software to select which MSI output clock uses the PLL mode. This bit can be written only when the MSI PLL mode is disabled (MSIPLLEN = 0). Note: If the MSI kernel clock output uses the same oscillator source than the MSI system clock output, then the PLL mode is applied to the both clocks outputs.
pub type MSIPLLSEL_R = crate::BitReader<bool>;
///Field `MSIPLLSEL` writer - MSI clock with PLL mode selection Set and cleared by software to select which MSI output clock uses the PLL mode. This bit can be written only when the MSI PLL mode is disabled (MSIPLLEN = 0). Note: If the MSI kernel clock output uses the same oscillator source than the MSI system clock output, then the PLL mode is applied to the both clocks outputs.
pub type MSIPLLSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_CR_SPEC, bool, O>;
///Field `MSIPLLFAST` reader - MSI PLL mode fast startup Set and reset by software to enable/disable the fast PLL mode start-up of the MSI clock source. This bit is used only if PLL mode is selected (MSIPLLEN = 1). The fast start-up feature is not active the first time the PLL mode is selected. The fast start-up is active when the MSI in PLL mode returns from switch off.
pub type MSIPLLFAST_R = crate::BitReader<bool>;
///Field `MSIPLLFAST` writer - MSI PLL mode fast startup Set and reset by software to enable/disable the fast PLL mode start-up of the MSI clock source. This bit is used only if PLL mode is selected (MSIPLLEN = 1). The fast start-up feature is not active the first time the PLL mode is selected. The fast start-up is active when the MSI in PLL mode returns from switch off.
pub type MSIPLLFAST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_CR_SPEC, bool, O>;
///Field `HSION` reader - HSI16 clock enable Set and cleared by software. Cleared by hardware to stop the HSI16 oscillator when entering Stop, Standby or Shutdown mode. Set by hardware to force the HSI16 oscillator ON when STOPWUCK = 1 when leaving Stop modes, or in case of failure of the HSE crystal oscillator. This bit is set by hardware if the HSI16 is used directly or indirectly as system clock.
pub type HSION_R = crate::BitReader<bool>;
///Field `HSION` writer - HSI16 clock enable Set and cleared by software. Cleared by hardware to stop the HSI16 oscillator when entering Stop, Standby or Shutdown mode. Set by hardware to force the HSI16 oscillator ON when STOPWUCK = 1 when leaving Stop modes, or in case of failure of the HSE crystal oscillator. This bit is set by hardware if the HSI16 is used directly or indirectly as system clock.
pub type HSION_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_CR_SPEC, bool, O>;
///Field `HSIKERON` reader - HSI16 enable for some peripheral kernels Set and cleared by software to force HSI16 ON even in Stop modes. Keeping the HSI16 ON in Stop mode allows the communication speed not to be reduced by the HSI16 startup time. This bit has no effect on HSION value. Refer to for more details. The HSIKERON must be configured at 0 before entering Stop 3 mode.
pub type HSIKERON_R = crate::BitReader<bool>;
///Field `HSIKERON` writer - HSI16 enable for some peripheral kernels Set and cleared by software to force HSI16 ON even in Stop modes. Keeping the HSI16 ON in Stop mode allows the communication speed not to be reduced by the HSI16 startup time. This bit has no effect on HSION value. Refer to for more details. The HSIKERON must be configured at 0 before entering Stop 3 mode.
pub type HSIKERON_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_CR_SPEC, bool, O>;
///Field `HSIRDY` reader - HSI16 clock ready flag Set by hardware to indicate that HSI16 oscillator is stable. This bit is set only when HSI16 is enabled by software by setting HSION. Note: Once the HSION bit is cleared, HSIRDY goes low after six HSI16 clock cycles.
pub type HSIRDY_R = crate::BitReader<bool>;
///Field `HSI48ON` reader - HSI48 clock enable Set and cleared by software. Cleared by hardware to stop the HSI48 when entering in Stop, Standby or Shutdown modes.
pub type HSI48ON_R = crate::BitReader<bool>;
///Field `HSI48ON` writer - HSI48 clock enable Set and cleared by software. Cleared by hardware to stop the HSI48 when entering in Stop, Standby or Shutdown modes.
pub type HSI48ON_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_CR_SPEC, bool, O>;
///Field `HSI48RDY` reader - HSI48 clock ready flag Set by hardware to indicate that HSI48 oscillator is stable. This bit is set only when HSI48 is enabled by software by setting HSI48ON.
pub type HSI48RDY_R = crate::BitReader<bool>;
///Field `SHSION` reader - SHSI clock enable Set and cleared by software. Cleared by hardware to stop the SHSI when entering in Stop, Standby or Shutdown modes.
pub type SHSION_R = crate::BitReader<bool>;
///Field `SHSION` writer - SHSI clock enable Set and cleared by software. Cleared by hardware to stop the SHSI when entering in Stop, Standby or Shutdown modes.
pub type SHSION_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_CR_SPEC, bool, O>;
///Field `SHSIRDY` reader - SHSI clock ready flag Set by hardware to indicate that the SHSI oscillator is stable. This bit is set only when SHSI is enabled by software by setting SHSION. Note: Once the SHSION bit is cleared, SHSIRDY goes low after six SHSI clock cycles.
pub type SHSIRDY_R = crate::BitReader<bool>;
///Field `HSEON` reader - HSE clock enable Set and cleared by software. Cleared by hardware to stop the HSE oscillator when entering Stop, Standby or Shutdown mode. This bit cannot be reset if the HSE oscillator is used directly or indirectly as the system clock.
pub type HSEON_R = crate::BitReader<bool>;
///Field `HSEON` writer - HSE clock enable Set and cleared by software. Cleared by hardware to stop the HSE oscillator when entering Stop, Standby or Shutdown mode. This bit cannot be reset if the HSE oscillator is used directly or indirectly as the system clock.
pub type HSEON_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_CR_SPEC, bool, O>;
///Field `HSERDY` reader - HSE clock ready flag Set by hardware to indicate that the HSE oscillator is stable. Note: Once the HSEON bit is cleared, HSERDY goes low after six HSE clock cycles.
pub type HSERDY_R = crate::BitReader<bool>;
///Field `HSEBYP` reader - HSE crystal oscillator bypass Set and cleared by software to bypass the oscillator with an external clock. The external clock must be enabled with the HSEON bit set, to be used by the device. The HSEBYP bit can be written only if the HSE oscillator is disabled.
pub type HSEBYP_R = crate::BitReader<bool>;
///Field `HSEBYP` writer - HSE crystal oscillator bypass Set and cleared by software to bypass the oscillator with an external clock. The external clock must be enabled with the HSEON bit set, to be used by the device. The HSEBYP bit can be written only if the HSE oscillator is disabled.
pub type HSEBYP_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_CR_SPEC, bool, O>;
///Field `CSSON` reader - Clock security system enable Set by software to enable the clock security system. When CSSON is set, the clock detector is enabled by hardware when the HSE oscillator is ready, and disabled by hardware if a HSE clock failure is detected. This bit is set only and is cleared by reset.
pub type CSSON_R = crate::BitReader<bool>;
///Field `CSSON` writer - Clock security system enable Set by software to enable the clock security system. When CSSON is set, the clock detector is enabled by hardware when the HSE oscillator is ready, and disabled by hardware if a HSE clock failure is detected. This bit is set only and is cleared by reset.
pub type CSSON_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_CR_SPEC, bool, O>;
///Field `HSEEXT` reader - HSE external clock bypass mode Set and reset by software to select the external clock mode in bypass mode. External clock mode must be configured with HSEON bit to be used by the device. This bit can be written only if the HSE oscillator is disabled. This bit is active only if the HSE bypass mode is enabled.
pub type HSEEXT_R = crate::BitReader<bool>;
///Field `HSEEXT` writer - HSE external clock bypass mode Set and reset by software to select the external clock mode in bypass mode. External clock mode must be configured with HSEON bit to be used by the device. This bit can be written only if the HSE oscillator is disabled. This bit is active only if the HSE bypass mode is enabled.
pub type HSEEXT_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_CR_SPEC, bool, O>;
///Field `PLL1ON` reader - PLL1 enable Set and cleared by software to enable the main PLL. Cleared by hardware when entering Stop, Standby or Shutdown mode. This bit cannot be reset if the PLL1 clock is used as the system clock.
pub type PLL1ON_R = crate::BitReader<bool>;
///Field `PLL1ON` writer - PLL1 enable Set and cleared by software to enable the main PLL. Cleared by hardware when entering Stop, Standby or Shutdown mode. This bit cannot be reset if the PLL1 clock is used as the system clock.
pub type PLL1ON_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_CR_SPEC, bool, O>;
///Field `PLL1RDY` reader - PLL1 clock ready flag Set by hardware to indicate that the PLL1 is locked.
pub type PLL1RDY_R = crate::BitReader<bool>;
///Field `PLL2ON` reader - PLL2 enable Set and cleared by software to enable PLL2. Cleared by hardware when entering Stop, Standby or Shutdown mode.
pub type PLL2ON_R = crate::BitReader<bool>;
///Field `PLL2ON` writer - PLL2 enable Set and cleared by software to enable PLL2. Cleared by hardware when entering Stop, Standby or Shutdown mode.
pub type PLL2ON_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_CR_SPEC, bool, O>;
///Field `PLL2RDY` reader - PLL2 clock ready flag Set by hardware to indicate that the PLL2 is locked.
pub type PLL2RDY_R = crate::BitReader<bool>;
///Field `PLL3ON` reader - PLL3 enable Set and cleared by software to enable PLL3. Cleared by hardware when entering Stop, Standby or Shutdown mode.
pub type PLL3ON_R = crate::BitReader<bool>;
///Field `PLL3ON` writer - PLL3 enable Set and cleared by software to enable PLL3. Cleared by hardware when entering Stop, Standby or Shutdown mode.
pub type PLL3ON_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_CR_SPEC, bool, O>;
///Field `PLL3RDY` reader - PLL3 clock ready flag Set by hardware to indicate that the PLL3 is locked.
pub type PLL3RDY_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - MSIS clock enable Set and cleared by software. Cleared by hardware to stop the MSIS oscillator when entering Stop, Standby or Shutdown mode. Set by hardware to force the MSIS oscillator ON when exiting Standby or Shutdown mode. Set by hardware to force the MSIS oscillator ON when STOPWUCK = 0 when exiting Stop modes or in case of a failure of the HSE oscillator. Set by hardware when used directly or indirectly as system clock.
    #[inline(always)]
    pub fn msison(&self) -> MSISON_R {
        MSISON_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - MSI enable for some peripheral kernels Set and cleared by software to force MSI ON even in Stop modes. Keeping the MSI ON in Stop mode allows the communication speed not to be reduced by the MSI startup time. This bit has no effect on MSISON and MSIKON values (see autonomous mode for more details). The MSIKERON must be configured at 0 before entering Stop 3 mode.
    #[inline(always)]
    pub fn msikeron(&self) -> MSIKERON_R {
        MSIKERON_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - MSIS clock ready flag Set by hardware to indicate that the MSIS oscillator is stable. This bit is set only when MSIS is enabled by software by setting MSISON. Note: Once the MSISON bit is cleared, MSISRDY goes low after six MSIS clock cycles.
    #[inline(always)]
    pub fn msisrdy(&self) -> MSISRDY_R {
        MSISRDY_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - MSI clock PLL-mode enable Set and cleared by software to enable/disable the PLL part of the MSI clock source. MSIPLLEN must be enabled after LSE is enabled (LSEON enabled) and ready (LSERDY set by hardware). A hardware protection prevents from enabling MSIPLLEN if LSE is not ready. This bit is cleared by hardware when LSE is disabled (LSEON = 0) or when the CSS on LSE detects a LSE failure (see RCC_CSR).
    #[inline(always)]
    pub fn msipllen(&self) -> MSIPLLEN_R {
        MSIPLLEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - MSIK clock enable Set and cleared by software. Cleared by hardware to stop the MSIK when entering Stop, Standby or Shutdown mode. Set by hardware to force the MSIK oscillator ON when exiting Standby or Shutdown mode. Set by hardware to force the MSIK oscillator ON when STOPWUCK = 0 or STOPKERWUCK = 0 when exiting Stop modes or in case of a failure of the HSE oscillator.
    #[inline(always)]
    pub fn msikon(&self) -> MSIKON_R {
        MSIKON_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - MSIK clock ready flag Set by hardware to indicate that the MSIK is stable. This bit is set only when MSI kernel oscillator is enabled by software by setting MSIKON. Note: Once the MSIKON bit is cleared, MSIKRDY goes low after six MSIK oscillator clock cycles.
    #[inline(always)]
    pub fn msikrdy(&self) -> MSIKRDY_R {
        MSIKRDY_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - MSI clock with PLL mode selection Set and cleared by software to select which MSI output clock uses the PLL mode. This bit can be written only when the MSI PLL mode is disabled (MSIPLLEN = 0). Note: If the MSI kernel clock output uses the same oscillator source than the MSI system clock output, then the PLL mode is applied to the both clocks outputs.
    #[inline(always)]
    pub fn msipllsel(&self) -> MSIPLLSEL_R {
        MSIPLLSEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - MSI PLL mode fast startup Set and reset by software to enable/disable the fast PLL mode start-up of the MSI clock source. This bit is used only if PLL mode is selected (MSIPLLEN = 1). The fast start-up feature is not active the first time the PLL mode is selected. The fast start-up is active when the MSI in PLL mode returns from switch off.
    #[inline(always)]
    pub fn msipllfast(&self) -> MSIPLLFAST_R {
        MSIPLLFAST_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - HSI16 clock enable Set and cleared by software. Cleared by hardware to stop the HSI16 oscillator when entering Stop, Standby or Shutdown mode. Set by hardware to force the HSI16 oscillator ON when STOPWUCK = 1 when leaving Stop modes, or in case of failure of the HSE crystal oscillator. This bit is set by hardware if the HSI16 is used directly or indirectly as system clock.
    #[inline(always)]
    pub fn hsion(&self) -> HSION_R {
        HSION_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - HSI16 enable for some peripheral kernels Set and cleared by software to force HSI16 ON even in Stop modes. Keeping the HSI16 ON in Stop mode allows the communication speed not to be reduced by the HSI16 startup time. This bit has no effect on HSION value. Refer to for more details. The HSIKERON must be configured at 0 before entering Stop 3 mode.
    #[inline(always)]
    pub fn hsikeron(&self) -> HSIKERON_R {
        HSIKERON_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - HSI16 clock ready flag Set by hardware to indicate that HSI16 oscillator is stable. This bit is set only when HSI16 is enabled by software by setting HSION. Note: Once the HSION bit is cleared, HSIRDY goes low after six HSI16 clock cycles.
    #[inline(always)]
    pub fn hsirdy(&self) -> HSIRDY_R {
        HSIRDY_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 12 - HSI48 clock enable Set and cleared by software. Cleared by hardware to stop the HSI48 when entering in Stop, Standby or Shutdown modes.
    #[inline(always)]
    pub fn hsi48on(&self) -> HSI48ON_R {
        HSI48ON_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - HSI48 clock ready flag Set by hardware to indicate that HSI48 oscillator is stable. This bit is set only when HSI48 is enabled by software by setting HSI48ON.
    #[inline(always)]
    pub fn hsi48rdy(&self) -> HSI48RDY_R {
        HSI48RDY_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - SHSI clock enable Set and cleared by software. Cleared by hardware to stop the SHSI when entering in Stop, Standby or Shutdown modes.
    #[inline(always)]
    pub fn shsion(&self) -> SHSION_R {
        SHSION_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - SHSI clock ready flag Set by hardware to indicate that the SHSI oscillator is stable. This bit is set only when SHSI is enabled by software by setting SHSION. Note: Once the SHSION bit is cleared, SHSIRDY goes low after six SHSI clock cycles.
    #[inline(always)]
    pub fn shsirdy(&self) -> SHSIRDY_R {
        SHSIRDY_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - HSE clock enable Set and cleared by software. Cleared by hardware to stop the HSE oscillator when entering Stop, Standby or Shutdown mode. This bit cannot be reset if the HSE oscillator is used directly or indirectly as the system clock.
    #[inline(always)]
    pub fn hseon(&self) -> HSEON_R {
        HSEON_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - HSE clock ready flag Set by hardware to indicate that the HSE oscillator is stable. Note: Once the HSEON bit is cleared, HSERDY goes low after six HSE clock cycles.
    #[inline(always)]
    pub fn hserdy(&self) -> HSERDY_R {
        HSERDY_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - HSE crystal oscillator bypass Set and cleared by software to bypass the oscillator with an external clock. The external clock must be enabled with the HSEON bit set, to be used by the device. The HSEBYP bit can be written only if the HSE oscillator is disabled.
    #[inline(always)]
    pub fn hsebyp(&self) -> HSEBYP_R {
        HSEBYP_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Clock security system enable Set by software to enable the clock security system. When CSSON is set, the clock detector is enabled by hardware when the HSE oscillator is ready, and disabled by hardware if a HSE clock failure is detected. This bit is set only and is cleared by reset.
    #[inline(always)]
    pub fn csson(&self) -> CSSON_R {
        CSSON_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - HSE external clock bypass mode Set and reset by software to select the external clock mode in bypass mode. External clock mode must be configured with HSEON bit to be used by the device. This bit can be written only if the HSE oscillator is disabled. This bit is active only if the HSE bypass mode is enabled.
    #[inline(always)]
    pub fn hseext(&self) -> HSEEXT_R {
        HSEEXT_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 24 - PLL1 enable Set and cleared by software to enable the main PLL. Cleared by hardware when entering Stop, Standby or Shutdown mode. This bit cannot be reset if the PLL1 clock is used as the system clock.
    #[inline(always)]
    pub fn pll1on(&self) -> PLL1ON_R {
        PLL1ON_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - PLL1 clock ready flag Set by hardware to indicate that the PLL1 is locked.
    #[inline(always)]
    pub fn pll1rdy(&self) -> PLL1RDY_R {
        PLL1RDY_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - PLL2 enable Set and cleared by software to enable PLL2. Cleared by hardware when entering Stop, Standby or Shutdown mode.
    #[inline(always)]
    pub fn pll2on(&self) -> PLL2ON_R {
        PLL2ON_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - PLL2 clock ready flag Set by hardware to indicate that the PLL2 is locked.
    #[inline(always)]
    pub fn pll2rdy(&self) -> PLL2RDY_R {
        PLL2RDY_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - PLL3 enable Set and cleared by software to enable PLL3. Cleared by hardware when entering Stop, Standby or Shutdown mode.
    #[inline(always)]
    pub fn pll3on(&self) -> PLL3ON_R {
        PLL3ON_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - PLL3 clock ready flag Set by hardware to indicate that the PLL3 is locked.
    #[inline(always)]
    pub fn pll3rdy(&self) -> PLL3RDY_R {
        PLL3RDY_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - MSIS clock enable Set and cleared by software. Cleared by hardware to stop the MSIS oscillator when entering Stop, Standby or Shutdown mode. Set by hardware to force the MSIS oscillator ON when exiting Standby or Shutdown mode. Set by hardware to force the MSIS oscillator ON when STOPWUCK = 0 when exiting Stop modes or in case of a failure of the HSE oscillator. Set by hardware when used directly or indirectly as system clock.
    #[inline(always)]
    #[must_use]
    pub fn msison(&mut self) -> MSISON_W<0> {
        MSISON_W::new(self)
    }
    ///Bit 1 - MSI enable for some peripheral kernels Set and cleared by software to force MSI ON even in Stop modes. Keeping the MSI ON in Stop mode allows the communication speed not to be reduced by the MSI startup time. This bit has no effect on MSISON and MSIKON values (see autonomous mode for more details). The MSIKERON must be configured at 0 before entering Stop 3 mode.
    #[inline(always)]
    #[must_use]
    pub fn msikeron(&mut self) -> MSIKERON_W<1> {
        MSIKERON_W::new(self)
    }
    ///Bit 3 - MSI clock PLL-mode enable Set and cleared by software to enable/disable the PLL part of the MSI clock source. MSIPLLEN must be enabled after LSE is enabled (LSEON enabled) and ready (LSERDY set by hardware). A hardware protection prevents from enabling MSIPLLEN if LSE is not ready. This bit is cleared by hardware when LSE is disabled (LSEON = 0) or when the CSS on LSE detects a LSE failure (see RCC_CSR).
    #[inline(always)]
    #[must_use]
    pub fn msipllen(&mut self) -> MSIPLLEN_W<3> {
        MSIPLLEN_W::new(self)
    }
    ///Bit 4 - MSIK clock enable Set and cleared by software. Cleared by hardware to stop the MSIK when entering Stop, Standby or Shutdown mode. Set by hardware to force the MSIK oscillator ON when exiting Standby or Shutdown mode. Set by hardware to force the MSIK oscillator ON when STOPWUCK = 0 or STOPKERWUCK = 0 when exiting Stop modes or in case of a failure of the HSE oscillator.
    #[inline(always)]
    #[must_use]
    pub fn msikon(&mut self) -> MSIKON_W<4> {
        MSIKON_W::new(self)
    }
    ///Bit 6 - MSI clock with PLL mode selection Set and cleared by software to select which MSI output clock uses the PLL mode. This bit can be written only when the MSI PLL mode is disabled (MSIPLLEN = 0). Note: If the MSI kernel clock output uses the same oscillator source than the MSI system clock output, then the PLL mode is applied to the both clocks outputs.
    #[inline(always)]
    #[must_use]
    pub fn msipllsel(&mut self) -> MSIPLLSEL_W<6> {
        MSIPLLSEL_W::new(self)
    }
    ///Bit 7 - MSI PLL mode fast startup Set and reset by software to enable/disable the fast PLL mode start-up of the MSI clock source. This bit is used only if PLL mode is selected (MSIPLLEN = 1). The fast start-up feature is not active the first time the PLL mode is selected. The fast start-up is active when the MSI in PLL mode returns from switch off.
    #[inline(always)]
    #[must_use]
    pub fn msipllfast(&mut self) -> MSIPLLFAST_W<7> {
        MSIPLLFAST_W::new(self)
    }
    ///Bit 8 - HSI16 clock enable Set and cleared by software. Cleared by hardware to stop the HSI16 oscillator when entering Stop, Standby or Shutdown mode. Set by hardware to force the HSI16 oscillator ON when STOPWUCK = 1 when leaving Stop modes, or in case of failure of the HSE crystal oscillator. This bit is set by hardware if the HSI16 is used directly or indirectly as system clock.
    #[inline(always)]
    #[must_use]
    pub fn hsion(&mut self) -> HSION_W<8> {
        HSION_W::new(self)
    }
    ///Bit 9 - HSI16 enable for some peripheral kernels Set and cleared by software to force HSI16 ON even in Stop modes. Keeping the HSI16 ON in Stop mode allows the communication speed not to be reduced by the HSI16 startup time. This bit has no effect on HSION value. Refer to for more details. The HSIKERON must be configured at 0 before entering Stop 3 mode.
    #[inline(always)]
    #[must_use]
    pub fn hsikeron(&mut self) -> HSIKERON_W<9> {
        HSIKERON_W::new(self)
    }
    ///Bit 12 - HSI48 clock enable Set and cleared by software. Cleared by hardware to stop the HSI48 when entering in Stop, Standby or Shutdown modes.
    #[inline(always)]
    #[must_use]
    pub fn hsi48on(&mut self) -> HSI48ON_W<12> {
        HSI48ON_W::new(self)
    }
    ///Bit 14 - SHSI clock enable Set and cleared by software. Cleared by hardware to stop the SHSI when entering in Stop, Standby or Shutdown modes.
    #[inline(always)]
    #[must_use]
    pub fn shsion(&mut self) -> SHSION_W<14> {
        SHSION_W::new(self)
    }
    ///Bit 16 - HSE clock enable Set and cleared by software. Cleared by hardware to stop the HSE oscillator when entering Stop, Standby or Shutdown mode. This bit cannot be reset if the HSE oscillator is used directly or indirectly as the system clock.
    #[inline(always)]
    #[must_use]
    pub fn hseon(&mut self) -> HSEON_W<16> {
        HSEON_W::new(self)
    }
    ///Bit 18 - HSE crystal oscillator bypass Set and cleared by software to bypass the oscillator with an external clock. The external clock must be enabled with the HSEON bit set, to be used by the device. The HSEBYP bit can be written only if the HSE oscillator is disabled.
    #[inline(always)]
    #[must_use]
    pub fn hsebyp(&mut self) -> HSEBYP_W<18> {
        HSEBYP_W::new(self)
    }
    ///Bit 19 - Clock security system enable Set by software to enable the clock security system. When CSSON is set, the clock detector is enabled by hardware when the HSE oscillator is ready, and disabled by hardware if a HSE clock failure is detected. This bit is set only and is cleared by reset.
    #[inline(always)]
    #[must_use]
    pub fn csson(&mut self) -> CSSON_W<19> {
        CSSON_W::new(self)
    }
    ///Bit 20 - HSE external clock bypass mode Set and reset by software to select the external clock mode in bypass mode. External clock mode must be configured with HSEON bit to be used by the device. This bit can be written only if the HSE oscillator is disabled. This bit is active only if the HSE bypass mode is enabled.
    #[inline(always)]
    #[must_use]
    pub fn hseext(&mut self) -> HSEEXT_W<20> {
        HSEEXT_W::new(self)
    }
    ///Bit 24 - PLL1 enable Set and cleared by software to enable the main PLL. Cleared by hardware when entering Stop, Standby or Shutdown mode. This bit cannot be reset if the PLL1 clock is used as the system clock.
    #[inline(always)]
    #[must_use]
    pub fn pll1on(&mut self) -> PLL1ON_W<24> {
        PLL1ON_W::new(self)
    }
    ///Bit 26 - PLL2 enable Set and cleared by software to enable PLL2. Cleared by hardware when entering Stop, Standby or Shutdown mode.
    #[inline(always)]
    #[must_use]
    pub fn pll2on(&mut self) -> PLL2ON_W<26> {
        PLL2ON_W::new(self)
    }
    ///Bit 28 - PLL3 enable Set and cleared by software to enable PLL3. Cleared by hardware when entering Stop, Standby or Shutdown mode.
    #[inline(always)]
    #[must_use]
    pub fn pll3on(&mut self) -> PLL3ON_W<28> {
        PLL3ON_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RCC clock control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rcc_cr](index.html) module
pub struct RCC_CR_SPEC;
impl crate::RegisterSpec for RCC_CR_SPEC {
    type Ux = u32;
}
///`read()` method returns [rcc_cr::R](R) reader structure
impl crate::Readable for RCC_CR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rcc_cr::W](W) writer structure
impl crate::Writable for RCC_CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RCC_CR to value 0x35
impl crate::Resettable for RCC_CR_SPEC {
    const RESET_VALUE: Self::Ux = 0x35;
}
