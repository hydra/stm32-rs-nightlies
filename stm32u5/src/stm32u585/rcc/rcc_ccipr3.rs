///Register `RCC_CCIPR3` reader
pub struct R(crate::R<RCC_CCIPR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_CCIPR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_CCIPR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_CCIPR3_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RCC_CCIPR3` writer
pub struct W(crate::W<RCC_CCIPR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_CCIPR3_SPEC>;
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
impl From<crate::W<RCC_CCIPR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_CCIPR3_SPEC>) -> Self {
        W(writer)
    }
}
///Field `LPUART1SEL` reader - LPUART1 kernel clock source selection These bits are used to select the LPUART1 kernel clock source. others: reserved Note: The LPUART1 is functional in Stop 0, Stop 1 and Stop 2 modes only when the kernel clock is HSI16, LSE or MSIK.
pub type LPUART1SEL_R = crate::FieldReader<u8, u8>;
///Field `LPUART1SEL` writer - LPUART1 kernel clock source selection These bits are used to select the LPUART1 kernel clock source. others: reserved Note: The LPUART1 is functional in Stop 0, Stop 1 and Stop 2 modes only when the kernel clock is HSI16, LSE or MSIK.
pub type LPUART1SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_CCIPR3_SPEC, u8, u8, 3, O>;
///Field `SPI3SEL` reader - SPI3 kernel clock source selection These bits are used to select the SPI3 kernel clock source. Note: The SPI3 is functional in Stop 0, Stop 1 and Stop 2 modes only when the kernel clock is HSI16 or MSIK.
pub type SPI3SEL_R = crate::FieldReader<u8, u8>;
///Field `SPI3SEL` writer - SPI3 kernel clock source selection These bits are used to select the SPI3 kernel clock source. Note: The SPI3 is functional in Stop 0, Stop 1 and Stop 2 modes only when the kernel clock is HSI16 or MSIK.
pub type SPI3SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_CCIPR3_SPEC, u8, u8, 2, O>;
///Field `I2C3SEL` reader - I2C3 kernel clock source selection These bits are used to select the I2C3 kernel clock source. Note: The I2C3 is functional in Stop 0, Stop 1 and Stop 2 modes only when the kernel clock is HSI16 or MSIK.
pub type I2C3SEL_R = crate::FieldReader<u8, u8>;
///Field `I2C3SEL` writer - I2C3 kernel clock source selection These bits are used to select the I2C3 kernel clock source. Note: The I2C3 is functional in Stop 0, Stop 1 and Stop 2 modes only when the kernel clock is HSI16 or MSIK.
pub type I2C3SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_CCIPR3_SPEC, u8, u8, 2, O>;
///Field `LPTIM34SEL` reader - LPTIM3 and LPTIM4 kernel clock source selection These bits are used to select the LPTIM3 and LPTIM4 kernel clock source. Note: The LPTIM3 and LPTIM4 are functional in Stop 0, Stop 1 and Stop 2 modes only when the kernel clock is LSI, LSE, HSI16 with HSIKERON = 1 or MSIK with MSIKERON = 1.
pub type LPTIM34SEL_R = crate::FieldReader<u8, u8>;
///Field `LPTIM34SEL` writer - LPTIM3 and LPTIM4 kernel clock source selection These bits are used to select the LPTIM3 and LPTIM4 kernel clock source. Note: The LPTIM3 and LPTIM4 are functional in Stop 0, Stop 1 and Stop 2 modes only when the kernel clock is LSI, LSE, HSI16 with HSIKERON = 1 or MSIK with MSIKERON = 1.
pub type LPTIM34SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_CCIPR3_SPEC, u8, u8, 2, O>;
///Field `LPTIM1SEL` reader - LPTIM1 kernel clock source selection These bits are used to select the LPTIM1 kernel clock source. Note: The LPTIM1 is functional in Stop 0, Stop 1 and Stop 2 modes only when the kernel clock is LSI, LSE, HSI16 with HSIKERON = 1 or MSIK with MSIKERON = 1.
pub type LPTIM1SEL_R = crate::FieldReader<u8, u8>;
///Field `LPTIM1SEL` writer - LPTIM1 kernel clock source selection These bits are used to select the LPTIM1 kernel clock source. Note: The LPTIM1 is functional in Stop 0, Stop 1 and Stop 2 modes only when the kernel clock is LSI, LSE, HSI16 with HSIKERON = 1 or MSIK with MSIKERON = 1.
pub type LPTIM1SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_CCIPR3_SPEC, u8, u8, 2, O>;
///Field `ADCDACSEL` reader - ADC1, ADC4 and DAC1 kernel clock source selection These bits are used to select the ADC1, ADC4 and DAC1 kernel clock source. others: reserved Note: The ADC1, ADC4 and DAC1 are functional in Stop 0, Stop 1 and Stop 2 modes only when the kernel clock is HSI16 or MSIK (only ADC4 and DAC1 are functional in Stop 2 mode).
pub type ADCDACSEL_R = crate::FieldReader<u8, u8>;
///Field `ADCDACSEL` writer - ADC1, ADC4 and DAC1 kernel clock source selection These bits are used to select the ADC1, ADC4 and DAC1 kernel clock source. others: reserved Note: The ADC1, ADC4 and DAC1 are functional in Stop 0, Stop 1 and Stop 2 modes only when the kernel clock is HSI16 or MSIK (only ADC4 and DAC1 are functional in Stop 2 mode).
pub type ADCDACSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_CCIPR3_SPEC, u8, u8, 3, O>;
///Field `DAC1SEL` reader - DAC1 sample and hold clock source selection This bit is used to select the DAC1 sample and hold clock source.
pub type DAC1SEL_R = crate::BitReader<bool>;
///Field `DAC1SEL` writer - DAC1 sample and hold clock source selection This bit is used to select the DAC1 sample and hold clock source.
pub type DAC1SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_CCIPR3_SPEC, bool, O>;
///Field `ADF1SEL` reader - ADF1 kernel clock source selection These bits are used to select the ADF1 kernel clock source. others: reserved Note: The ADF1 is functional in Stop 0, Stop 1 and Stop 2 modes only when the kernel clock is AUDIOCLK or MSIK.
pub type ADF1SEL_R = crate::FieldReader<u8, u8>;
///Field `ADF1SEL` writer - ADF1 kernel clock source selection These bits are used to select the ADF1 kernel clock source. others: reserved Note: The ADF1 is functional in Stop 0, Stop 1 and Stop 2 modes only when the kernel clock is AUDIOCLK or MSIK.
pub type ADF1SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_CCIPR3_SPEC, u8, u8, 3, O>;
impl R {
    ///Bits 0:2 - LPUART1 kernel clock source selection These bits are used to select the LPUART1 kernel clock source. others: reserved Note: The LPUART1 is functional in Stop 0, Stop 1 and Stop 2 modes only when the kernel clock is HSI16, LSE or MSIK.
    #[inline(always)]
    pub fn lpuart1sel(&self) -> LPUART1SEL_R {
        LPUART1SEL_R::new((self.bits & 7) as u8)
    }
    ///Bits 3:4 - SPI3 kernel clock source selection These bits are used to select the SPI3 kernel clock source. Note: The SPI3 is functional in Stop 0, Stop 1 and Stop 2 modes only when the kernel clock is HSI16 or MSIK.
    #[inline(always)]
    pub fn spi3sel(&self) -> SPI3SEL_R {
        SPI3SEL_R::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bits 6:7 - I2C3 kernel clock source selection These bits are used to select the I2C3 kernel clock source. Note: The I2C3 is functional in Stop 0, Stop 1 and Stop 2 modes only when the kernel clock is HSI16 or MSIK.
    #[inline(always)]
    pub fn i2c3sel(&self) -> I2C3SEL_R {
        I2C3SEL_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:9 - LPTIM3 and LPTIM4 kernel clock source selection These bits are used to select the LPTIM3 and LPTIM4 kernel clock source. Note: The LPTIM3 and LPTIM4 are functional in Stop 0, Stop 1 and Stop 2 modes only when the kernel clock is LSI, LSE, HSI16 with HSIKERON = 1 or MSIK with MSIKERON = 1.
    #[inline(always)]
    pub fn lptim34sel(&self) -> LPTIM34SEL_R {
        LPTIM34SEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - LPTIM1 kernel clock source selection These bits are used to select the LPTIM1 kernel clock source. Note: The LPTIM1 is functional in Stop 0, Stop 1 and Stop 2 modes only when the kernel clock is LSI, LSE, HSI16 with HSIKERON = 1 or MSIK with MSIKERON = 1.
    #[inline(always)]
    pub fn lptim1sel(&self) -> LPTIM1SEL_R {
        LPTIM1SEL_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:14 - ADC1, ADC4 and DAC1 kernel clock source selection These bits are used to select the ADC1, ADC4 and DAC1 kernel clock source. others: reserved Note: The ADC1, ADC4 and DAC1 are functional in Stop 0, Stop 1 and Stop 2 modes only when the kernel clock is HSI16 or MSIK (only ADC4 and DAC1 are functional in Stop 2 mode).
    #[inline(always)]
    pub fn adcdacsel(&self) -> ADCDACSEL_R {
        ADCDACSEL_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bit 15 - DAC1 sample and hold clock source selection This bit is used to select the DAC1 sample and hold clock source.
    #[inline(always)]
    pub fn dac1sel(&self) -> DAC1SEL_R {
        DAC1SEL_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:18 - ADF1 kernel clock source selection These bits are used to select the ADF1 kernel clock source. others: reserved Note: The ADF1 is functional in Stop 0, Stop 1 and Stop 2 modes only when the kernel clock is AUDIOCLK or MSIK.
    #[inline(always)]
    pub fn adf1sel(&self) -> ADF1SEL_R {
        ADF1SEL_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    ///Bits 0:2 - LPUART1 kernel clock source selection These bits are used to select the LPUART1 kernel clock source. others: reserved Note: The LPUART1 is functional in Stop 0, Stop 1 and Stop 2 modes only when the kernel clock is HSI16, LSE or MSIK.
    #[inline(always)]
    #[must_use]
    pub fn lpuart1sel(&mut self) -> LPUART1SEL_W<0> {
        LPUART1SEL_W::new(self)
    }
    ///Bits 3:4 - SPI3 kernel clock source selection These bits are used to select the SPI3 kernel clock source. Note: The SPI3 is functional in Stop 0, Stop 1 and Stop 2 modes only when the kernel clock is HSI16 or MSIK.
    #[inline(always)]
    #[must_use]
    pub fn spi3sel(&mut self) -> SPI3SEL_W<3> {
        SPI3SEL_W::new(self)
    }
    ///Bits 6:7 - I2C3 kernel clock source selection These bits are used to select the I2C3 kernel clock source. Note: The I2C3 is functional in Stop 0, Stop 1 and Stop 2 modes only when the kernel clock is HSI16 or MSIK.
    #[inline(always)]
    #[must_use]
    pub fn i2c3sel(&mut self) -> I2C3SEL_W<6> {
        I2C3SEL_W::new(self)
    }
    ///Bits 8:9 - LPTIM3 and LPTIM4 kernel clock source selection These bits are used to select the LPTIM3 and LPTIM4 kernel clock source. Note: The LPTIM3 and LPTIM4 are functional in Stop 0, Stop 1 and Stop 2 modes only when the kernel clock is LSI, LSE, HSI16 with HSIKERON = 1 or MSIK with MSIKERON = 1.
    #[inline(always)]
    #[must_use]
    pub fn lptim34sel(&mut self) -> LPTIM34SEL_W<8> {
        LPTIM34SEL_W::new(self)
    }
    ///Bits 10:11 - LPTIM1 kernel clock source selection These bits are used to select the LPTIM1 kernel clock source. Note: The LPTIM1 is functional in Stop 0, Stop 1 and Stop 2 modes only when the kernel clock is LSI, LSE, HSI16 with HSIKERON = 1 or MSIK with MSIKERON = 1.
    #[inline(always)]
    #[must_use]
    pub fn lptim1sel(&mut self) -> LPTIM1SEL_W<10> {
        LPTIM1SEL_W::new(self)
    }
    ///Bits 12:14 - ADC1, ADC4 and DAC1 kernel clock source selection These bits are used to select the ADC1, ADC4 and DAC1 kernel clock source. others: reserved Note: The ADC1, ADC4 and DAC1 are functional in Stop 0, Stop 1 and Stop 2 modes only when the kernel clock is HSI16 or MSIK (only ADC4 and DAC1 are functional in Stop 2 mode).
    #[inline(always)]
    #[must_use]
    pub fn adcdacsel(&mut self) -> ADCDACSEL_W<12> {
        ADCDACSEL_W::new(self)
    }
    ///Bit 15 - DAC1 sample and hold clock source selection This bit is used to select the DAC1 sample and hold clock source.
    #[inline(always)]
    #[must_use]
    pub fn dac1sel(&mut self) -> DAC1SEL_W<15> {
        DAC1SEL_W::new(self)
    }
    ///Bits 16:18 - ADF1 kernel clock source selection These bits are used to select the ADF1 kernel clock source. others: reserved Note: The ADF1 is functional in Stop 0, Stop 1 and Stop 2 modes only when the kernel clock is AUDIOCLK or MSIK.
    #[inline(always)]
    #[must_use]
    pub fn adf1sel(&mut self) -> ADF1SEL_W<16> {
        ADF1SEL_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RCC peripherals independent clock configuration register 3
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rcc_ccipr3](index.html) module
pub struct RCC_CCIPR3_SPEC;
impl crate::RegisterSpec for RCC_CCIPR3_SPEC {
    type Ux = u32;
}
///`read()` method returns [rcc_ccipr3::R](R) reader structure
impl crate::Readable for RCC_CCIPR3_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rcc_ccipr3::W](W) writer structure
impl crate::Writable for RCC_CCIPR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RCC_CCIPR3 to value 0
impl crate::Resettable for RCC_CCIPR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
