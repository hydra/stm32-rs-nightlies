///Register `RCC_SRDAMR` reader
pub struct R(crate::R<RCC_SRDAMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_SRDAMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_SRDAMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_SRDAMR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RCC_SRDAMR` writer
pub struct W(crate::W<RCC_SRDAMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_SRDAMR_SPEC>;
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
impl From<crate::W<RCC_SRDAMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_SRDAMR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SPI3AMEN` reader - SPI3 autonomous mode enable in Stop 0,1, 2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type SPI3AMEN_R = crate::BitReader<bool>;
///Field `SPI3AMEN` writer - SPI3 autonomous mode enable in Stop 0,1, 2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type SPI3AMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_SRDAMR_SPEC, bool, O>;
///Field `LPUART1AMEN` reader - LPUART1 autonomous mode enable in Stop 0,1, 2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type LPUART1AMEN_R = crate::BitReader<bool>;
///Field `LPUART1AMEN` writer - LPUART1 autonomous mode enable in Stop 0,1, 2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type LPUART1AMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_SRDAMR_SPEC, bool, O>;
///Field `I2C3AMEN` reader - I2C3 autonomous mode enable in Stop 0,1,2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type I2C3AMEN_R = crate::BitReader<bool>;
///Field `I2C3AMEN` writer - I2C3 autonomous mode enable in Stop 0,1,2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type I2C3AMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_SRDAMR_SPEC, bool, O>;
///Field `LPTIM1AMEN` reader - LPTIM1 autonomous mode enable in Stop 0,1,2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type LPTIM1AMEN_R = crate::BitReader<bool>;
///Field `LPTIM1AMEN` writer - LPTIM1 autonomous mode enable in Stop 0,1,2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type LPTIM1AMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_SRDAMR_SPEC, bool, O>;
///Field `LPTIM3AMEN` reader - LPTIM3 autonomous mode enable in Stop 0,1,2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type LPTIM3AMEN_R = crate::BitReader<bool>;
///Field `LPTIM3AMEN` writer - LPTIM3 autonomous mode enable in Stop 0,1,2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type LPTIM3AMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_SRDAMR_SPEC, bool, O>;
///Field `LPTIM4AMEN` reader - LPTIM4 autonomous mode enable in Stop 0,1,2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type LPTIM4AMEN_R = crate::BitReader<bool>;
///Field `LPTIM4AMEN` writer - LPTIM4 autonomous mode enable in Stop 0,1,2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type LPTIM4AMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_SRDAMR_SPEC, bool, O>;
///Field `OPAMPAMEN` reader - OPAMP autonomous mode enable in Stop 0,1,2 mode Set and cleared by software.
pub type OPAMPAMEN_R = crate::BitReader<bool>;
///Field `OPAMPAMEN` writer - OPAMP autonomous mode enable in Stop 0,1,2 mode Set and cleared by software.
pub type OPAMPAMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_SRDAMR_SPEC, bool, O>;
///Field `COMPAMEN` reader - COMP autonomous mode enable in Stop 0,1,2 mode Set and cleared by software.
pub type COMPAMEN_R = crate::BitReader<bool>;
///Field `COMPAMEN` writer - COMP autonomous mode enable in Stop 0,1,2 mode Set and cleared by software.
pub type COMPAMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_SRDAMR_SPEC, bool, O>;
///Field `VREFAMEN` reader - VREFBUF autonomous mode enable in Stop 0,1,2 mode Set and cleared by software.
pub type VREFAMEN_R = crate::BitReader<bool>;
///Field `VREFAMEN` writer - VREFBUF autonomous mode enable in Stop 0,1,2 mode Set and cleared by software.
pub type VREFAMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_SRDAMR_SPEC, bool, O>;
///Field `RTCAPBAMEN` reader - RTC and TAMP autonomous mode enable in Stop 0,1,2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type RTCAPBAMEN_R = crate::BitReader<bool>;
///Field `RTCAPBAMEN` writer - RTC and TAMP autonomous mode enable in Stop 0,1,2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type RTCAPBAMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_SRDAMR_SPEC, bool, O>;
///Field `ADC4AMEN` reader - ADC4 autonomous mode enable in Stop 0,1,2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type ADC4AMEN_R = crate::BitReader<bool>;
///Field `ADC4AMEN` writer - ADC4 autonomous mode enable in Stop 0,1,2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type ADC4AMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_SRDAMR_SPEC, bool, O>;
///Field `LPGPIO1AMEN` reader - LPGPIO1 autonomous mode enable in Stop 0,1,2 mode Set and cleared by software.
pub type LPGPIO1AMEN_R = crate::BitReader<bool>;
///Field `LPGPIO1AMEN` writer - LPGPIO1 autonomous mode enable in Stop 0,1,2 mode Set and cleared by software.
pub type LPGPIO1AMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_SRDAMR_SPEC, bool, O>;
///Field `DAC1AMEN` reader - DAC1 autonomous mode enable in Stop 0,1,2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type DAC1AMEN_R = crate::BitReader<bool>;
///Field `DAC1AMEN` writer - DAC1 autonomous mode enable in Stop 0,1,2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type DAC1AMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_SRDAMR_SPEC, bool, O>;
///Field `LPDMA1AMEN` reader - LPDMA1 autonomous mode enable in Stop 0,1,2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type LPDMA1AMEN_R = crate::BitReader<bool>;
///Field `LPDMA1AMEN` writer - LPDMA1 autonomous mode enable in Stop 0,1,2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type LPDMA1AMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_SRDAMR_SPEC, bool, O>;
///Field `ADF1AMEN` reader - ADF1 autonomous mode enable in Stop 0,1,2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type ADF1AMEN_R = crate::BitReader<bool>;
///Field `ADF1AMEN` writer - ADF1 autonomous mode enable in Stop 0,1,2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type ADF1AMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_SRDAMR_SPEC, bool, O>;
///Field `SRAM4AMEN` reader - SRAM4 autonomous mode enable in Stop 0,1,2 mode Set and cleared by software.
pub type SRAM4AMEN_R = crate::BitReader<bool>;
///Field `SRAM4AMEN` writer - SRAM4 autonomous mode enable in Stop 0,1,2 mode Set and cleared by software.
pub type SRAM4AMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_SRDAMR_SPEC, bool, O>;
impl R {
    ///Bit 5 - SPI3 autonomous mode enable in Stop 0,1, 2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn spi3amen(&self) -> SPI3AMEN_R {
        SPI3AMEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - LPUART1 autonomous mode enable in Stop 0,1, 2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn lpuart1amen(&self) -> LPUART1AMEN_R {
        LPUART1AMEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - I2C3 autonomous mode enable in Stop 0,1,2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn i2c3amen(&self) -> I2C3AMEN_R {
        I2C3AMEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 11 - LPTIM1 autonomous mode enable in Stop 0,1,2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn lptim1amen(&self) -> LPTIM1AMEN_R {
        LPTIM1AMEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - LPTIM3 autonomous mode enable in Stop 0,1,2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn lptim3amen(&self) -> LPTIM3AMEN_R {
        LPTIM3AMEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - LPTIM4 autonomous mode enable in Stop 0,1,2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn lptim4amen(&self) -> LPTIM4AMEN_R {
        LPTIM4AMEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - OPAMP autonomous mode enable in Stop 0,1,2 mode Set and cleared by software.
    #[inline(always)]
    pub fn opampamen(&self) -> OPAMPAMEN_R {
        OPAMPAMEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - COMP autonomous mode enable in Stop 0,1,2 mode Set and cleared by software.
    #[inline(always)]
    pub fn compamen(&self) -> COMPAMEN_R {
        COMPAMEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 20 - VREFBUF autonomous mode enable in Stop 0,1,2 mode Set and cleared by software.
    #[inline(always)]
    pub fn vrefamen(&self) -> VREFAMEN_R {
        VREFAMEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - RTC and TAMP autonomous mode enable in Stop 0,1,2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn rtcapbamen(&self) -> RTCAPBAMEN_R {
        RTCAPBAMEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 25 - ADC4 autonomous mode enable in Stop 0,1,2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn adc4amen(&self) -> ADC4AMEN_R {
        ADC4AMEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - LPGPIO1 autonomous mode enable in Stop 0,1,2 mode Set and cleared by software.
    #[inline(always)]
    pub fn lpgpio1amen(&self) -> LPGPIO1AMEN_R {
        LPGPIO1AMEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - DAC1 autonomous mode enable in Stop 0,1,2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn dac1amen(&self) -> DAC1AMEN_R {
        DAC1AMEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - LPDMA1 autonomous mode enable in Stop 0,1,2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn lpdma1amen(&self) -> LPDMA1AMEN_R {
        LPDMA1AMEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - ADF1 autonomous mode enable in Stop 0,1,2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn adf1amen(&self) -> ADF1AMEN_R {
        ADF1AMEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 31 - SRAM4 autonomous mode enable in Stop 0,1,2 mode Set and cleared by software.
    #[inline(always)]
    pub fn sram4amen(&self) -> SRAM4AMEN_R {
        SRAM4AMEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 5 - SPI3 autonomous mode enable in Stop 0,1, 2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    #[must_use]
    pub fn spi3amen(&mut self) -> SPI3AMEN_W<5> {
        SPI3AMEN_W::new(self)
    }
    ///Bit 6 - LPUART1 autonomous mode enable in Stop 0,1, 2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    #[must_use]
    pub fn lpuart1amen(&mut self) -> LPUART1AMEN_W<6> {
        LPUART1AMEN_W::new(self)
    }
    ///Bit 7 - I2C3 autonomous mode enable in Stop 0,1,2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    #[must_use]
    pub fn i2c3amen(&mut self) -> I2C3AMEN_W<7> {
        I2C3AMEN_W::new(self)
    }
    ///Bit 11 - LPTIM1 autonomous mode enable in Stop 0,1,2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    #[must_use]
    pub fn lptim1amen(&mut self) -> LPTIM1AMEN_W<11> {
        LPTIM1AMEN_W::new(self)
    }
    ///Bit 12 - LPTIM3 autonomous mode enable in Stop 0,1,2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    #[must_use]
    pub fn lptim3amen(&mut self) -> LPTIM3AMEN_W<12> {
        LPTIM3AMEN_W::new(self)
    }
    ///Bit 13 - LPTIM4 autonomous mode enable in Stop 0,1,2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    #[must_use]
    pub fn lptim4amen(&mut self) -> LPTIM4AMEN_W<13> {
        LPTIM4AMEN_W::new(self)
    }
    ///Bit 14 - OPAMP autonomous mode enable in Stop 0,1,2 mode Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn opampamen(&mut self) -> OPAMPAMEN_W<14> {
        OPAMPAMEN_W::new(self)
    }
    ///Bit 15 - COMP autonomous mode enable in Stop 0,1,2 mode Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn compamen(&mut self) -> COMPAMEN_W<15> {
        COMPAMEN_W::new(self)
    }
    ///Bit 20 - VREFBUF autonomous mode enable in Stop 0,1,2 mode Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn vrefamen(&mut self) -> VREFAMEN_W<20> {
        VREFAMEN_W::new(self)
    }
    ///Bit 21 - RTC and TAMP autonomous mode enable in Stop 0,1,2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    #[must_use]
    pub fn rtcapbamen(&mut self) -> RTCAPBAMEN_W<21> {
        RTCAPBAMEN_W::new(self)
    }
    ///Bit 25 - ADC4 autonomous mode enable in Stop 0,1,2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    #[must_use]
    pub fn adc4amen(&mut self) -> ADC4AMEN_W<25> {
        ADC4AMEN_W::new(self)
    }
    ///Bit 26 - LPGPIO1 autonomous mode enable in Stop 0,1,2 mode Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn lpgpio1amen(&mut self) -> LPGPIO1AMEN_W<26> {
        LPGPIO1AMEN_W::new(self)
    }
    ///Bit 27 - DAC1 autonomous mode enable in Stop 0,1,2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    #[must_use]
    pub fn dac1amen(&mut self) -> DAC1AMEN_W<27> {
        DAC1AMEN_W::new(self)
    }
    ///Bit 28 - LPDMA1 autonomous mode enable in Stop 0,1,2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    #[must_use]
    pub fn lpdma1amen(&mut self) -> LPDMA1AMEN_W<28> {
        LPDMA1AMEN_W::new(self)
    }
    ///Bit 29 - ADF1 autonomous mode enable in Stop 0,1,2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    #[must_use]
    pub fn adf1amen(&mut self) -> ADF1AMEN_W<29> {
        ADF1AMEN_W::new(self)
    }
    ///Bit 31 - SRAM4 autonomous mode enable in Stop 0,1,2 mode Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn sram4amen(&mut self) -> SRAM4AMEN_W<31> {
        SRAM4AMEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RCC SmartRun domain peripheral autonomous mode register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rcc_srdamr](index.html) module
pub struct RCC_SRDAMR_SPEC;
impl crate::RegisterSpec for RCC_SRDAMR_SPEC {
    type Ux = u32;
}
///`read()` method returns [rcc_srdamr::R](R) reader structure
impl crate::Readable for RCC_SRDAMR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rcc_srdamr::W](W) writer structure
impl crate::Writable for RCC_SRDAMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RCC_SRDAMR to value 0
impl crate::Resettable for RCC_SRDAMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
