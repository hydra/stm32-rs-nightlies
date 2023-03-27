///Register `RCC_AHB3ENR` reader
pub struct R(crate::R<RCC_AHB3ENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_AHB3ENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_AHB3ENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_AHB3ENR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RCC_AHB3ENR` writer
pub struct W(crate::W<RCC_AHB3ENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_AHB3ENR_SPEC>;
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
impl From<crate::W<RCC_AHB3ENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_AHB3ENR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `LPGPIO1EN` reader - LPGPIO1 enable Set and cleared by software.
pub type LPGPIO1EN_R = crate::BitReader<bool>;
///Field `LPGPIO1EN` writer - LPGPIO1 enable Set and cleared by software.
pub type LPGPIO1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB3ENR_SPEC, bool, O>;
///Field `PWREN` reader - PWR clock enable Set and cleared by software.
pub type PWREN_R = crate::BitReader<bool>;
///Field `PWREN` writer - PWR clock enable Set and cleared by software.
pub type PWREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB3ENR_SPEC, bool, O>;
///Field `ADC4EN` reader - ADC4 clock enable Set and cleared by software.
pub type ADC4EN_R = crate::BitReader<bool>;
///Field `ADC4EN` writer - ADC4 clock enable Set and cleared by software.
pub type ADC4EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB3ENR_SPEC, bool, O>;
///Field `DAC1EN` reader - DAC1 clock enable Set and cleared by software.
pub type DAC1EN_R = crate::BitReader<bool>;
///Field `DAC1EN` writer - DAC1 clock enable Set and cleared by software.
pub type DAC1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB3ENR_SPEC, bool, O>;
///Field `LPDMA1EN` reader - LPDMA1 clock enable Set and cleared by software.
pub type LPDMA1EN_R = crate::BitReader<bool>;
///Field `LPDMA1EN` writer - LPDMA1 clock enable Set and cleared by software.
pub type LPDMA1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB3ENR_SPEC, bool, O>;
///Field `ADF1EN` reader - ADF1 clock enable Set and cleared by software.
pub type ADF1EN_R = crate::BitReader<bool>;
///Field `ADF1EN` writer - ADF1 clock enable Set and cleared by software.
pub type ADF1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB3ENR_SPEC, bool, O>;
///Field `GTZC2EN` reader - GTZC2 clock enable Set and cleared by software.
pub type GTZC2EN_R = crate::BitReader<bool>;
///Field `GTZC2EN` writer - GTZC2 clock enable Set and cleared by software.
pub type GTZC2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB3ENR_SPEC, bool, O>;
///Field `SRAM4EN` reader - SRAM4 clock enable Set and reset by software.
pub type SRAM4EN_R = crate::BitReader<bool>;
///Field `SRAM4EN` writer - SRAM4 clock enable Set and reset by software.
pub type SRAM4EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB3ENR_SPEC, bool, O>;
impl R {
    ///Bit 0 - LPGPIO1 enable Set and cleared by software.
    #[inline(always)]
    pub fn lpgpio1en(&self) -> LPGPIO1EN_R {
        LPGPIO1EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - PWR clock enable Set and cleared by software.
    #[inline(always)]
    pub fn pwren(&self) -> PWREN_R {
        PWREN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 5 - ADC4 clock enable Set and cleared by software.
    #[inline(always)]
    pub fn adc4en(&self) -> ADC4EN_R {
        ADC4EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - DAC1 clock enable Set and cleared by software.
    #[inline(always)]
    pub fn dac1en(&self) -> DAC1EN_R {
        DAC1EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 9 - LPDMA1 clock enable Set and cleared by software.
    #[inline(always)]
    pub fn lpdma1en(&self) -> LPDMA1EN_R {
        LPDMA1EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - ADF1 clock enable Set and cleared by software.
    #[inline(always)]
    pub fn adf1en(&self) -> ADF1EN_R {
        ADF1EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 12 - GTZC2 clock enable Set and cleared by software.
    #[inline(always)]
    pub fn gtzc2en(&self) -> GTZC2EN_R {
        GTZC2EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 31 - SRAM4 clock enable Set and reset by software.
    #[inline(always)]
    pub fn sram4en(&self) -> SRAM4EN_R {
        SRAM4EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - LPGPIO1 enable Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn lpgpio1en(&mut self) -> LPGPIO1EN_W<0> {
        LPGPIO1EN_W::new(self)
    }
    ///Bit 2 - PWR clock enable Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn pwren(&mut self) -> PWREN_W<2> {
        PWREN_W::new(self)
    }
    ///Bit 5 - ADC4 clock enable Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn adc4en(&mut self) -> ADC4EN_W<5> {
        ADC4EN_W::new(self)
    }
    ///Bit 6 - DAC1 clock enable Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn dac1en(&mut self) -> DAC1EN_W<6> {
        DAC1EN_W::new(self)
    }
    ///Bit 9 - LPDMA1 clock enable Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn lpdma1en(&mut self) -> LPDMA1EN_W<9> {
        LPDMA1EN_W::new(self)
    }
    ///Bit 10 - ADF1 clock enable Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn adf1en(&mut self) -> ADF1EN_W<10> {
        ADF1EN_W::new(self)
    }
    ///Bit 12 - GTZC2 clock enable Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn gtzc2en(&mut self) -> GTZC2EN_W<12> {
        GTZC2EN_W::new(self)
    }
    ///Bit 31 - SRAM4 clock enable Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn sram4en(&mut self) -> SRAM4EN_W<31> {
        SRAM4EN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RCC AHB3 peripheral clock enable register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rcc_ahb3enr](index.html) module
pub struct RCC_AHB3ENR_SPEC;
impl crate::RegisterSpec for RCC_AHB3ENR_SPEC {
    type Ux = u32;
}
///`read()` method returns [rcc_ahb3enr::R](R) reader structure
impl crate::Readable for RCC_AHB3ENR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rcc_ahb3enr::W](W) writer structure
impl crate::Writable for RCC_AHB3ENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RCC_AHB3ENR to value 0x8000_0000
impl crate::Resettable for RCC_AHB3ENR_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000_0000;
}
