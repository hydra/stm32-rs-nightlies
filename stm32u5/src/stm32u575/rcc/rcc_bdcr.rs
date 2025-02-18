///Register `RCC_BDCR` reader
pub struct R(crate::R<RCC_BDCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_BDCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_BDCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_BDCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RCC_BDCR` writer
pub struct W(crate::W<RCC_BDCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_BDCR_SPEC>;
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
impl From<crate::W<RCC_BDCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_BDCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `LSEON` reader - LSE oscillator enable Set and cleared by software.
pub type LSEON_R = crate::BitReader<bool>;
///Field `LSEON` writer - LSE oscillator enable Set and cleared by software.
pub type LSEON_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_BDCR_SPEC, bool, O>;
///Field `LSERDY` reader - LSE oscillator ready Set and cleared by hardware to indicate when the external 32 kHz oscillator is stable. After the LSEON bit is cleared, LSERDY goes low after six external low-speed oscillator clock cycles.
pub type LSERDY_R = crate::BitReader<bool>;
///Field `LSEBYP` reader - LSE oscillator bypass Set and cleared by software to bypass oscillator in debug mode. This bit can be written only when the external 32 kHz oscillator is disabled (LSEON = 0 and LSERDY = 0).
pub type LSEBYP_R = crate::BitReader<bool>;
///Field `LSEBYP` writer - LSE oscillator bypass Set and cleared by software to bypass oscillator in debug mode. This bit can be written only when the external 32 kHz oscillator is disabled (LSEON = 0 and LSERDY = 0).
pub type LSEBYP_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_BDCR_SPEC, bool, O>;
///Field `LSEDRV` reader - LSE oscillator drive capability Set by software to modulate the drive capability of the LSE oscillator. This field can be written only when the external 32 kHz oscillator is disabled (LSEON = 0 and LSERDY = 0). Note: The oscillator is in 'Xtal mode when it is not in bypass mode.
pub type LSEDRV_R = crate::FieldReader<u8, u8>;
///Field `LSEDRV` writer - LSE oscillator drive capability Set by software to modulate the drive capability of the LSE oscillator. This field can be written only when the external 32 kHz oscillator is disabled (LSEON = 0 and LSERDY = 0). Note: The oscillator is in 'Xtal mode when it is not in bypass mode.
pub type LSEDRV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_BDCR_SPEC, u8, u8, 2, O>;
///Field `LSECSSON` reader - CSS on LSE enable Set by software to enable the CSS on LSE. LSECSSON must be enabled after the LSE oscillator is enabled (LSEON bit enabled) and ready (LSERDY flag set by hardware), and after the RTCSEL bit is selected. Once enabled, this bit cannot be disabled, except after a LSE failure detection (LSECSSD = 1). In that case, the software must disable the LSECSSON bit.
pub type LSECSSON_R = crate::BitReader<bool>;
///Field `LSECSSON` writer - CSS on LSE enable Set by software to enable the CSS on LSE. LSECSSON must be enabled after the LSE oscillator is enabled (LSEON bit enabled) and ready (LSERDY flag set by hardware), and after the RTCSEL bit is selected. Once enabled, this bit cannot be disabled, except after a LSE failure detection (LSECSSD = 1). In that case, the software must disable the LSECSSON bit.
pub type LSECSSON_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_BDCR_SPEC, bool, O>;
///Field `LSECSSD` reader - CSS on LSE failure Detection Set by hardware to indicate when a failure is detected by the CCS on the external 32 kHz oscillator (LSE).
pub type LSECSSD_R = crate::BitReader<bool>;
///Field `LSESYSEN` reader - LSE system clock (LSESYS) enable Set by software to enable always the LSE system clock generated by RCC. This clock can be used by any peripheral when its source clock is the LSE or at system level in case of one of the LSCOSEL, MCO, MSI PLL mode or CSS on LSE is needed. The LSESYS clock can be generated even if LSESYSEN= 0 if the LSE clock is requested by the CSS on LSE, by a peripheral or any other source clock using LSE.
pub type LSESYSEN_R = crate::BitReader<bool>;
///Field `LSESYSEN` writer - LSE system clock (LSESYS) enable Set by software to enable always the LSE system clock generated by RCC. This clock can be used by any peripheral when its source clock is the LSE or at system level in case of one of the LSCOSEL, MCO, MSI PLL mode or CSS on LSE is needed. The LSESYS clock can be generated even if LSESYSEN= 0 if the LSE clock is requested by the CSS on LSE, by a peripheral or any other source clock using LSE.
pub type LSESYSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_BDCR_SPEC, bool, O>;
///Field `RTCSEL` reader - RTC and TAMP clock source selection Set by software to select the clock source for the RTC and TAMP . Once the RTC and TAMP clock source has been selected, it cannot be changed anymore unless the Backup domain is reset, or unless a failure is detected on LSE (LSECSSD is set). The BDRST bit can be used to reset them.
pub type RTCSEL_R = crate::FieldReader<u8, u8>;
///Field `RTCSEL` writer - RTC and TAMP clock source selection Set by software to select the clock source for the RTC and TAMP . Once the RTC and TAMP clock source has been selected, it cannot be changed anymore unless the Backup domain is reset, or unless a failure is detected on LSE (LSECSSD is set). The BDRST bit can be used to reset them.
pub type RTCSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_BDCR_SPEC, u8, u8, 2, O>;
///Field `LSESYSRDY` reader - LSE system clock (LSESYS) ready Set and cleared by hardware to indicate when the LSE system clock is stable.When the LSESYSEN bit is set, the LSESYSRDY flag is set after two LSE clock cycles. The LSE clock must be already enabled and stable (LSEON and LSERDY are set). When the LSEON bit is cleared, LSERDY goes low after six external low-speed oscillator clock cycles.
pub type LSESYSRDY_R = crate::BitReader<bool>;
///Field `LSEGFON` reader - LSE clock glitch filter enable Set and cleared by hardware to enable the LSE glitch filter. This bit can be written only when the LSE is disabled (LSEON = 0 and LSERDY = 0)
pub type LSEGFON_R = crate::BitReader<bool>;
///Field `LSEGFON` writer - LSE clock glitch filter enable Set and cleared by hardware to enable the LSE glitch filter. This bit can be written only when the LSE is disabled (LSEON = 0 and LSERDY = 0)
pub type LSEGFON_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_BDCR_SPEC, bool, O>;
///Field `RTCEN` reader - RTC and TAMP clock enable Set and cleared by software.
pub type RTCEN_R = crate::BitReader<bool>;
///Field `RTCEN` writer - RTC and TAMP clock enable Set and cleared by software.
pub type RTCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_BDCR_SPEC, bool, O>;
///Field `BDRST` reader - Backup domain software reset Set and cleared by software.
pub type BDRST_R = crate::BitReader<bool>;
///Field `BDRST` writer - Backup domain software reset Set and cleared by software.
pub type BDRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_BDCR_SPEC, bool, O>;
///Field `LSCOEN` reader - Low-speed clock output (LSCO) enable Set and cleared by software.
pub type LSCOEN_R = crate::BitReader<bool>;
///Field `LSCOEN` writer - Low-speed clock output (LSCO) enable Set and cleared by software.
pub type LSCOEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_BDCR_SPEC, bool, O>;
///Field `LSCOSEL` reader - Low-speed clock output selection Set and cleared by software.
pub type LSCOSEL_R = crate::BitReader<bool>;
///Field `LSCOSEL` writer - Low-speed clock output selection Set and cleared by software.
pub type LSCOSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_BDCR_SPEC, bool, O>;
///Field `LSION` reader - LSI oscillator enable Set and cleared by software.
pub type LSION_R = crate::BitReader<bool>;
///Field `LSION` writer - LSI oscillator enable Set and cleared by software.
pub type LSION_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_BDCR_SPEC, bool, O>;
///Field `LSIRDY` reader - LSI oscillator ready Set and cleared by hardware to indicate when the LSI oscillator is stable. After the LSION bit is cleared, LSIRDY goes low after three internal low-speed oscillator clock cycles. This bit is set when the LSI is used by IWDG or RTC, even if LSION = 0.
pub type LSIRDY_R = crate::BitReader<bool>;
///Field `LSIRDY` writer - LSI oscillator ready Set and cleared by hardware to indicate when the LSI oscillator is stable. After the LSION bit is cleared, LSIRDY goes low after three internal low-speed oscillator clock cycles. This bit is set when the LSI is used by IWDG or RTC, even if LSION = 0.
pub type LSIRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_BDCR_SPEC, bool, O>;
///Field `LSIPREDIV` reader - Low-speed clock divider configuration Set and cleared by software to enable the LSI division. This bit can be written only when the LSI is disabled (LSION = 0 and LSIRDY = 0). If the LSI was previously enabled, it is necessary to wait for at least 60 Î¼s after clearing LSION bit (synchronization time for LSI to be really disabled), before writing LSIPREDIV. The LSIPREDIV cannot be changed if the LSI is used by the IWDG or by the RTC.
pub type LSIPREDIV_R = crate::BitReader<bool>;
///Field `LSIPREDIV` writer - Low-speed clock divider configuration Set and cleared by software to enable the LSI division. This bit can be written only when the LSI is disabled (LSION = 0 and LSIRDY = 0). If the LSI was previously enabled, it is necessary to wait for at least 60 Î¼s after clearing LSION bit (synchronization time for LSI to be really disabled), before writing LSIPREDIV. The LSIPREDIV cannot be changed if the LSI is used by the IWDG or by the RTC.
pub type LSIPREDIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_BDCR_SPEC, bool, O>;
impl R {
    ///Bit 0 - LSE oscillator enable Set and cleared by software.
    #[inline(always)]
    pub fn lseon(&self) -> LSEON_R {
        LSEON_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LSE oscillator ready Set and cleared by hardware to indicate when the external 32 kHz oscillator is stable. After the LSEON bit is cleared, LSERDY goes low after six external low-speed oscillator clock cycles.
    #[inline(always)]
    pub fn lserdy(&self) -> LSERDY_R {
        LSERDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - LSE oscillator bypass Set and cleared by software to bypass oscillator in debug mode. This bit can be written only when the external 32 kHz oscillator is disabled (LSEON = 0 and LSERDY = 0).
    #[inline(always)]
    pub fn lsebyp(&self) -> LSEBYP_R {
        LSEBYP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:4 - LSE oscillator drive capability Set by software to modulate the drive capability of the LSE oscillator. This field can be written only when the external 32 kHz oscillator is disabled (LSEON = 0 and LSERDY = 0). Note: The oscillator is in 'Xtal mode when it is not in bypass mode.
    #[inline(always)]
    pub fn lsedrv(&self) -> LSEDRV_R {
        LSEDRV_R::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bit 5 - CSS on LSE enable Set by software to enable the CSS on LSE. LSECSSON must be enabled after the LSE oscillator is enabled (LSEON bit enabled) and ready (LSERDY flag set by hardware), and after the RTCSEL bit is selected. Once enabled, this bit cannot be disabled, except after a LSE failure detection (LSECSSD = 1). In that case, the software must disable the LSECSSON bit.
    #[inline(always)]
    pub fn lsecsson(&self) -> LSECSSON_R {
        LSECSSON_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - CSS on LSE failure Detection Set by hardware to indicate when a failure is detected by the CCS on the external 32 kHz oscillator (LSE).
    #[inline(always)]
    pub fn lsecssd(&self) -> LSECSSD_R {
        LSECSSD_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - LSE system clock (LSESYS) enable Set by software to enable always the LSE system clock generated by RCC. This clock can be used by any peripheral when its source clock is the LSE or at system level in case of one of the LSCOSEL, MCO, MSI PLL mode or CSS on LSE is needed. The LSESYS clock can be generated even if LSESYSEN= 0 if the LSE clock is requested by the CSS on LSE, by a peripheral or any other source clock using LSE.
    #[inline(always)]
    pub fn lsesysen(&self) -> LSESYSEN_R {
        LSESYSEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:9 - RTC and TAMP clock source selection Set by software to select the clock source for the RTC and TAMP . Once the RTC and TAMP clock source has been selected, it cannot be changed anymore unless the Backup domain is reset, or unless a failure is detected on LSE (LSECSSD is set). The BDRST bit can be used to reset them.
    #[inline(always)]
    pub fn rtcsel(&self) -> RTCSEL_R {
        RTCSEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 11 - LSE system clock (LSESYS) ready Set and cleared by hardware to indicate when the LSE system clock is stable.When the LSESYSEN bit is set, the LSESYSRDY flag is set after two LSE clock cycles. The LSE clock must be already enabled and stable (LSEON and LSERDY are set). When the LSEON bit is cleared, LSERDY goes low after six external low-speed oscillator clock cycles.
    #[inline(always)]
    pub fn lsesysrdy(&self) -> LSESYSRDY_R {
        LSESYSRDY_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - LSE clock glitch filter enable Set and cleared by hardware to enable the LSE glitch filter. This bit can be written only when the LSE is disabled (LSEON = 0 and LSERDY = 0)
    #[inline(always)]
    pub fn lsegfon(&self) -> LSEGFON_R {
        LSEGFON_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 15 - RTC and TAMP clock enable Set and cleared by software.
    #[inline(always)]
    pub fn rtcen(&self) -> RTCEN_R {
        RTCEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Backup domain software reset Set and cleared by software.
    #[inline(always)]
    pub fn bdrst(&self) -> BDRST_R {
        BDRST_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 24 - Low-speed clock output (LSCO) enable Set and cleared by software.
    #[inline(always)]
    pub fn lscoen(&self) -> LSCOEN_R {
        LSCOEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Low-speed clock output selection Set and cleared by software.
    #[inline(always)]
    pub fn lscosel(&self) -> LSCOSEL_R {
        LSCOSEL_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - LSI oscillator enable Set and cleared by software.
    #[inline(always)]
    pub fn lsion(&self) -> LSION_R {
        LSION_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - LSI oscillator ready Set and cleared by hardware to indicate when the LSI oscillator is stable. After the LSION bit is cleared, LSIRDY goes low after three internal low-speed oscillator clock cycles. This bit is set when the LSI is used by IWDG or RTC, even if LSION = 0.
    #[inline(always)]
    pub fn lsirdy(&self) -> LSIRDY_R {
        LSIRDY_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Low-speed clock divider configuration Set and cleared by software to enable the LSI division. This bit can be written only when the LSI is disabled (LSION = 0 and LSIRDY = 0). If the LSI was previously enabled, it is necessary to wait for at least 60 Î¼s after clearing LSION bit (synchronization time for LSI to be really disabled), before writing LSIPREDIV. The LSIPREDIV cannot be changed if the LSI is used by the IWDG or by the RTC.
    #[inline(always)]
    pub fn lsiprediv(&self) -> LSIPREDIV_R {
        LSIPREDIV_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - LSE oscillator enable Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn lseon(&mut self) -> LSEON_W<0> {
        LSEON_W::new(self)
    }
    ///Bit 2 - LSE oscillator bypass Set and cleared by software to bypass oscillator in debug mode. This bit can be written only when the external 32 kHz oscillator is disabled (LSEON = 0 and LSERDY = 0).
    #[inline(always)]
    #[must_use]
    pub fn lsebyp(&mut self) -> LSEBYP_W<2> {
        LSEBYP_W::new(self)
    }
    ///Bits 3:4 - LSE oscillator drive capability Set by software to modulate the drive capability of the LSE oscillator. This field can be written only when the external 32 kHz oscillator is disabled (LSEON = 0 and LSERDY = 0). Note: The oscillator is in 'Xtal mode when it is not in bypass mode.
    #[inline(always)]
    #[must_use]
    pub fn lsedrv(&mut self) -> LSEDRV_W<3> {
        LSEDRV_W::new(self)
    }
    ///Bit 5 - CSS on LSE enable Set by software to enable the CSS on LSE. LSECSSON must be enabled after the LSE oscillator is enabled (LSEON bit enabled) and ready (LSERDY flag set by hardware), and after the RTCSEL bit is selected. Once enabled, this bit cannot be disabled, except after a LSE failure detection (LSECSSD = 1). In that case, the software must disable the LSECSSON bit.
    #[inline(always)]
    #[must_use]
    pub fn lsecsson(&mut self) -> LSECSSON_W<5> {
        LSECSSON_W::new(self)
    }
    ///Bit 7 - LSE system clock (LSESYS) enable Set by software to enable always the LSE system clock generated by RCC. This clock can be used by any peripheral when its source clock is the LSE or at system level in case of one of the LSCOSEL, MCO, MSI PLL mode or CSS on LSE is needed. The LSESYS clock can be generated even if LSESYSEN= 0 if the LSE clock is requested by the CSS on LSE, by a peripheral or any other source clock using LSE.
    #[inline(always)]
    #[must_use]
    pub fn lsesysen(&mut self) -> LSESYSEN_W<7> {
        LSESYSEN_W::new(self)
    }
    ///Bits 8:9 - RTC and TAMP clock source selection Set by software to select the clock source for the RTC and TAMP . Once the RTC and TAMP clock source has been selected, it cannot be changed anymore unless the Backup domain is reset, or unless a failure is detected on LSE (LSECSSD is set). The BDRST bit can be used to reset them.
    #[inline(always)]
    #[must_use]
    pub fn rtcsel(&mut self) -> RTCSEL_W<8> {
        RTCSEL_W::new(self)
    }
    ///Bit 12 - LSE clock glitch filter enable Set and cleared by hardware to enable the LSE glitch filter. This bit can be written only when the LSE is disabled (LSEON = 0 and LSERDY = 0)
    #[inline(always)]
    #[must_use]
    pub fn lsegfon(&mut self) -> LSEGFON_W<12> {
        LSEGFON_W::new(self)
    }
    ///Bit 15 - RTC and TAMP clock enable Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn rtcen(&mut self) -> RTCEN_W<15> {
        RTCEN_W::new(self)
    }
    ///Bit 16 - Backup domain software reset Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn bdrst(&mut self) -> BDRST_W<16> {
        BDRST_W::new(self)
    }
    ///Bit 24 - Low-speed clock output (LSCO) enable Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn lscoen(&mut self) -> LSCOEN_W<24> {
        LSCOEN_W::new(self)
    }
    ///Bit 25 - Low-speed clock output selection Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn lscosel(&mut self) -> LSCOSEL_W<25> {
        LSCOSEL_W::new(self)
    }
    ///Bit 26 - LSI oscillator enable Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn lsion(&mut self) -> LSION_W<26> {
        LSION_W::new(self)
    }
    ///Bit 27 - LSI oscillator ready Set and cleared by hardware to indicate when the LSI oscillator is stable. After the LSION bit is cleared, LSIRDY goes low after three internal low-speed oscillator clock cycles. This bit is set when the LSI is used by IWDG or RTC, even if LSION = 0.
    #[inline(always)]
    #[must_use]
    pub fn lsirdy(&mut self) -> LSIRDY_W<27> {
        LSIRDY_W::new(self)
    }
    ///Bit 28 - Low-speed clock divider configuration Set and cleared by software to enable the LSI division. This bit can be written only when the LSI is disabled (LSION = 0 and LSIRDY = 0). If the LSI was previously enabled, it is necessary to wait for at least 60 Î¼s after clearing LSION bit (synchronization time for LSI to be really disabled), before writing LSIPREDIV. The LSIPREDIV cannot be changed if the LSI is used by the IWDG or by the RTC.
    #[inline(always)]
    #[must_use]
    pub fn lsiprediv(&mut self) -> LSIPREDIV_W<28> {
        LSIPREDIV_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RCC Backup domain control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rcc_bdcr](index.html) module
pub struct RCC_BDCR_SPEC;
impl crate::RegisterSpec for RCC_BDCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [rcc_bdcr::R](R) reader structure
impl crate::Readable for RCC_BDCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rcc_bdcr::W](W) writer structure
impl crate::Writable for RCC_BDCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RCC_BDCR to value 0
impl crate::Resettable for RCC_BDCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
