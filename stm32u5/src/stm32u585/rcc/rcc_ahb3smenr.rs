///Register `RCC_AHB3SMENR` reader
pub struct R(crate::R<RCC_AHB3SMENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_AHB3SMENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_AHB3SMENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_AHB3SMENR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RCC_AHB3SMENR` writer
pub struct W(crate::W<RCC_AHB3SMENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_AHB3SMENR_SPEC>;
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
impl From<crate::W<RCC_AHB3SMENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_AHB3SMENR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `LPGPIO1SMEN` reader - LPGPIO1 enable during Sleep and Stop modes Set and cleared by software.
pub type LPGPIO1SMEN_R = crate::BitReader<bool>;
///Field `LPGPIO1SMEN` writer - LPGPIO1 enable during Sleep and Stop modes Set and cleared by software.
pub type LPGPIO1SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB3SMENR_SPEC, bool, O>;
///Field `PWRSMEN` reader - PWR clock enable during Sleep and Stop modes Set and cleared by software.
pub type PWRSMEN_R = crate::BitReader<bool>;
///Field `PWRSMEN` writer - PWR clock enable during Sleep and Stop modes Set and cleared by software.
pub type PWRSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB3SMENR_SPEC, bool, O>;
///Field `ADC4SMEN` reader - ADC4 clock enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type ADC4SMEN_R = crate::BitReader<bool>;
///Field `ADC4SMEN` writer - ADC4 clock enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type ADC4SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB3SMENR_SPEC, bool, O>;
///Field `DAC1SMEN` reader - DAC1 clock enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type DAC1SMEN_R = crate::BitReader<bool>;
///Field `DAC1SMEN` writer - DAC1 clock enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type DAC1SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB3SMENR_SPEC, bool, O>;
///Field `LPDMA1SMEN` reader - LPDMA1 clock enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type LPDMA1SMEN_R = crate::BitReader<bool>;
///Field `LPDMA1SMEN` writer - LPDMA1 clock enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type LPDMA1SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB3SMENR_SPEC, bool, O>;
///Field `ADF1SMEN` reader - ADF1 clock enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type ADF1SMEN_R = crate::BitReader<bool>;
///Field `ADF1SMEN` writer - ADF1 clock enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type ADF1SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB3SMENR_SPEC, bool, O>;
///Field `GTZC2SMEN` reader - GTZC2 clock enable during Sleep and Stop modes Set and cleared by software.
pub type GTZC2SMEN_R = crate::BitReader<bool>;
///Field `GTZC2SMEN` writer - GTZC2 clock enable during Sleep and Stop modes Set and cleared by software.
pub type GTZC2SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB3SMENR_SPEC, bool, O>;
///Field `SRAM4SMEN` reader - SRAM4 clocks enable during Sleep and Stop modes Set and cleared by software.
pub type SRAM4SMEN_R = crate::BitReader<bool>;
///Field `SRAM4SMEN` writer - SRAM4 clocks enable during Sleep and Stop modes Set and cleared by software.
pub type SRAM4SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB3SMENR_SPEC, bool, O>;
impl R {
    ///Bit 0 - LPGPIO1 enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn lpgpio1smen(&self) -> LPGPIO1SMEN_R {
        LPGPIO1SMEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - PWR clock enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn pwrsmen(&self) -> PWRSMEN_R {
        PWRSMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 5 - ADC4 clock enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn adc4smen(&self) -> ADC4SMEN_R {
        ADC4SMEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - DAC1 clock enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn dac1smen(&self) -> DAC1SMEN_R {
        DAC1SMEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 9 - LPDMA1 clock enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn lpdma1smen(&self) -> LPDMA1SMEN_R {
        LPDMA1SMEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - ADF1 clock enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn adf1smen(&self) -> ADF1SMEN_R {
        ADF1SMEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 12 - GTZC2 clock enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn gtzc2smen(&self) -> GTZC2SMEN_R {
        GTZC2SMEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 31 - SRAM4 clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn sram4smen(&self) -> SRAM4SMEN_R {
        SRAM4SMEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - LPGPIO1 enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn lpgpio1smen(&mut self) -> LPGPIO1SMEN_W<0> {
        LPGPIO1SMEN_W::new(self)
    }
    ///Bit 2 - PWR clock enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn pwrsmen(&mut self) -> PWRSMEN_W<2> {
        PWRSMEN_W::new(self)
    }
    ///Bit 5 - ADC4 clock enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    #[must_use]
    pub fn adc4smen(&mut self) -> ADC4SMEN_W<5> {
        ADC4SMEN_W::new(self)
    }
    ///Bit 6 - DAC1 clock enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    #[must_use]
    pub fn dac1smen(&mut self) -> DAC1SMEN_W<6> {
        DAC1SMEN_W::new(self)
    }
    ///Bit 9 - LPDMA1 clock enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    #[must_use]
    pub fn lpdma1smen(&mut self) -> LPDMA1SMEN_W<9> {
        LPDMA1SMEN_W::new(self)
    }
    ///Bit 10 - ADF1 clock enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    #[must_use]
    pub fn adf1smen(&mut self) -> ADF1SMEN_W<10> {
        ADF1SMEN_W::new(self)
    }
    ///Bit 12 - GTZC2 clock enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn gtzc2smen(&mut self) -> GTZC2SMEN_W<12> {
        GTZC2SMEN_W::new(self)
    }
    ///Bit 31 - SRAM4 clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn sram4smen(&mut self) -> SRAM4SMEN_W<31> {
        SRAM4SMEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RCC AHB3 peripheral clocks enable in Sleep and Stop modes register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rcc_ahb3smenr](index.html) module
pub struct RCC_AHB3SMENR_SPEC;
impl crate::RegisterSpec for RCC_AHB3SMENR_SPEC {
    type Ux = u32;
}
///`read()` method returns [rcc_ahb3smenr::R](R) reader structure
impl crate::Readable for RCC_AHB3SMENR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rcc_ahb3smenr::W](W) writer structure
impl crate::Writable for RCC_AHB3SMENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RCC_AHB3SMENR to value 0xffff_ffff
impl crate::Resettable for RCC_AHB3SMENR_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
