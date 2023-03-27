///Register `IER1` reader
pub struct R(crate::R<IER1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IER1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IER1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IER1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `IER1` writer
pub struct W(crate::W<IER1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IER1_SPEC>;
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
impl From<crate::W<IER1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IER1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SPI3IE` reader - illegal access interrupt enable for SPI3
pub type SPI3IE_R = crate::BitReader<bool>;
///Field `SPI3IE` writer - illegal access interrupt enable for SPI3
pub type SPI3IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
///Field `LPUART1IE` reader - illegal access interrupt enable for LPUART1
pub type LPUART1IE_R = crate::BitReader<bool>;
///Field `LPUART1IE` writer - illegal access interrupt enable for LPUART1
pub type LPUART1IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
///Field `I2C3IE` reader - illegal access interrupt enable for I2C3
pub type I2C3IE_R = crate::BitReader<bool>;
///Field `I2C3IE` writer - illegal access interrupt enable for I2C3
pub type I2C3IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
///Field `LPTIM1IE` reader - illegal access interrupt enable for LPTIM1
pub type LPTIM1IE_R = crate::BitReader<bool>;
///Field `LPTIM1IE` writer - illegal access interrupt enable for LPTIM1
pub type LPTIM1IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
///Field `LPTIM3IE` reader - illegal access interrupt enable for LPTIM3
pub type LPTIM3IE_R = crate::BitReader<bool>;
///Field `LPTIM3IE` writer - illegal access interrupt enable for LPTIM3
pub type LPTIM3IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
///Field `LPTIM4IE` reader - illegal access interrupt enable for LPTIM4
pub type LPTIM4IE_R = crate::BitReader<bool>;
///Field `LPTIM4IE` writer - illegal access interrupt enable for LPTIM4
pub type LPTIM4IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
///Field `OPAMPIE` reader - illegal access interrupt enable for OPAMP
pub type OPAMPIE_R = crate::BitReader<bool>;
///Field `OPAMPIE` writer - illegal access interrupt enable for OPAMP
pub type OPAMPIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
///Field `COMPIE` reader - illegal access interrupt enable for COMP
pub type COMPIE_R = crate::BitReader<bool>;
///Field `COMPIE` writer - illegal access interrupt enable for COMP
pub type COMPIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
///Field `ADC4IE` reader - illegal access interrupt enable for ADC4
pub type ADC4IE_R = crate::BitReader<bool>;
///Field `ADC4IE` writer - illegal access interrupt enable for ADC4
pub type ADC4IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
///Field `VREFBUFIE` reader - illegal access interrupt enable for VREFBUF
pub type VREFBUFIE_R = crate::BitReader<bool>;
///Field `VREFBUFIE` writer - illegal access interrupt enable for VREFBUF
pub type VREFBUFIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
///Field `DAC1IE` reader - illegal access interrupt enable for DAC1
pub type DAC1IE_R = crate::BitReader<bool>;
///Field `DAC1IE` writer - illegal access interrupt enable for DAC1
pub type DAC1IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
///Field `ADF1IE` reader - illegal access interrupt enable for ADF1
pub type ADF1IE_R = crate::BitReader<bool>;
///Field `ADF1IE` writer - illegal access interrupt enable for ADF1
pub type ADF1IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
impl R {
    ///Bit 0 - illegal access interrupt enable for SPI3
    #[inline(always)]
    pub fn spi3ie(&self) -> SPI3IE_R {
        SPI3IE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - illegal access interrupt enable for LPUART1
    #[inline(always)]
    pub fn lpuart1ie(&self) -> LPUART1IE_R {
        LPUART1IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - illegal access interrupt enable for I2C3
    #[inline(always)]
    pub fn i2c3ie(&self) -> I2C3IE_R {
        I2C3IE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - illegal access interrupt enable for LPTIM1
    #[inline(always)]
    pub fn lptim1ie(&self) -> LPTIM1IE_R {
        LPTIM1IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - illegal access interrupt enable for LPTIM3
    #[inline(always)]
    pub fn lptim3ie(&self) -> LPTIM3IE_R {
        LPTIM3IE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - illegal access interrupt enable for LPTIM4
    #[inline(always)]
    pub fn lptim4ie(&self) -> LPTIM4IE_R {
        LPTIM4IE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - illegal access interrupt enable for OPAMP
    #[inline(always)]
    pub fn opampie(&self) -> OPAMPIE_R {
        OPAMPIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - illegal access interrupt enable for COMP
    #[inline(always)]
    pub fn compie(&self) -> COMPIE_R {
        COMPIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - illegal access interrupt enable for ADC4
    #[inline(always)]
    pub fn adc4ie(&self) -> ADC4IE_R {
        ADC4IE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - illegal access interrupt enable for VREFBUF
    #[inline(always)]
    pub fn vrefbufie(&self) -> VREFBUFIE_R {
        VREFBUFIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - illegal access interrupt enable for DAC1
    #[inline(always)]
    pub fn dac1ie(&self) -> DAC1IE_R {
        DAC1IE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - illegal access interrupt enable for ADF1
    #[inline(always)]
    pub fn adf1ie(&self) -> ADF1IE_R {
        ADF1IE_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - illegal access interrupt enable for SPI3
    #[inline(always)]
    #[must_use]
    pub fn spi3ie(&mut self) -> SPI3IE_W<0> {
        SPI3IE_W::new(self)
    }
    ///Bit 1 - illegal access interrupt enable for LPUART1
    #[inline(always)]
    #[must_use]
    pub fn lpuart1ie(&mut self) -> LPUART1IE_W<1> {
        LPUART1IE_W::new(self)
    }
    ///Bit 2 - illegal access interrupt enable for I2C3
    #[inline(always)]
    #[must_use]
    pub fn i2c3ie(&mut self) -> I2C3IE_W<2> {
        I2C3IE_W::new(self)
    }
    ///Bit 3 - illegal access interrupt enable for LPTIM1
    #[inline(always)]
    #[must_use]
    pub fn lptim1ie(&mut self) -> LPTIM1IE_W<3> {
        LPTIM1IE_W::new(self)
    }
    ///Bit 4 - illegal access interrupt enable for LPTIM3
    #[inline(always)]
    #[must_use]
    pub fn lptim3ie(&mut self) -> LPTIM3IE_W<4> {
        LPTIM3IE_W::new(self)
    }
    ///Bit 5 - illegal access interrupt enable for LPTIM4
    #[inline(always)]
    #[must_use]
    pub fn lptim4ie(&mut self) -> LPTIM4IE_W<5> {
        LPTIM4IE_W::new(self)
    }
    ///Bit 6 - illegal access interrupt enable for OPAMP
    #[inline(always)]
    #[must_use]
    pub fn opampie(&mut self) -> OPAMPIE_W<6> {
        OPAMPIE_W::new(self)
    }
    ///Bit 7 - illegal access interrupt enable for COMP
    #[inline(always)]
    #[must_use]
    pub fn compie(&mut self) -> COMPIE_W<7> {
        COMPIE_W::new(self)
    }
    ///Bit 8 - illegal access interrupt enable for ADC4
    #[inline(always)]
    #[must_use]
    pub fn adc4ie(&mut self) -> ADC4IE_W<8> {
        ADC4IE_W::new(self)
    }
    ///Bit 9 - illegal access interrupt enable for VREFBUF
    #[inline(always)]
    #[must_use]
    pub fn vrefbufie(&mut self) -> VREFBUFIE_W<9> {
        VREFBUFIE_W::new(self)
    }
    ///Bit 11 - illegal access interrupt enable for DAC1
    #[inline(always)]
    #[must_use]
    pub fn dac1ie(&mut self) -> DAC1IE_W<11> {
        DAC1IE_W::new(self)
    }
    ///Bit 12 - illegal access interrupt enable for ADF1
    #[inline(always)]
    #[must_use]
    pub fn adf1ie(&mut self) -> ADF1IE_W<12> {
        ADF1IE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TZIC interrupt enable register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ier1](index.html) module
pub struct IER1_SPEC;
impl crate::RegisterSpec for IER1_SPEC {
    type Ux = u32;
}
///`read()` method returns [ier1::R](R) reader structure
impl crate::Readable for IER1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ier1::W](W) writer structure
impl crate::Writable for IER1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets IER1 to value 0
impl crate::Resettable for IER1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
