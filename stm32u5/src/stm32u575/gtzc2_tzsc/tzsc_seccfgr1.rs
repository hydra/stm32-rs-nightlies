///Register `TZSC_SECCFGR1` reader
pub struct R(crate::R<TZSC_SECCFGR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZSC_SECCFGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZSC_SECCFGR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZSC_SECCFGR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TZSC_SECCFGR1` writer
pub struct W(crate::W<TZSC_SECCFGR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TZSC_SECCFGR1_SPEC>;
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
impl From<crate::W<TZSC_SECCFGR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TZSC_SECCFGR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SPI3SEC` reader - secure access mode for SPI3
pub type SPI3SEC_R = crate::BitReader<bool>;
///Field `SPI3SEC` writer - secure access mode for SPI3
pub type SPI3SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_SECCFGR1_SPEC, bool, O>;
///Field `LPUART1SEC` reader - secure access mode for LPUART1
pub type LPUART1SEC_R = crate::BitReader<bool>;
///Field `LPUART1SEC` writer - secure access mode for LPUART1
pub type LPUART1SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_SECCFGR1_SPEC, bool, O>;
///Field `I2C3SEC` reader - secure access mode for I2C3
pub type I2C3SEC_R = crate::BitReader<bool>;
///Field `I2C3SEC` writer - secure access mode for I2C3
pub type I2C3SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_SECCFGR1_SPEC, bool, O>;
///Field `LPTIM1SEC` reader - secure access mode for LPTIM1
pub type LPTIM1SEC_R = crate::BitReader<bool>;
///Field `LPTIM1SEC` writer - secure access mode for LPTIM1
pub type LPTIM1SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_SECCFGR1_SPEC, bool, O>;
///Field `LPTIM3SEC` reader - secure access mode for LPTIM3
pub type LPTIM3SEC_R = crate::BitReader<bool>;
///Field `LPTIM3SEC` writer - secure access mode for LPTIM3
pub type LPTIM3SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_SECCFGR1_SPEC, bool, O>;
///Field `LPTIM4SEC` reader - secure access mode for LPTIM4
pub type LPTIM4SEC_R = crate::BitReader<bool>;
///Field `LPTIM4SEC` writer - secure access mode for LPTIM4
pub type LPTIM4SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_SECCFGR1_SPEC, bool, O>;
///Field `OPAMPSEC` reader - secure access mode for OPAMP
pub type OPAMPSEC_R = crate::BitReader<bool>;
///Field `OPAMPSEC` writer - secure access mode for OPAMP
pub type OPAMPSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_SECCFGR1_SPEC, bool, O>;
///Field `COMPSEC` reader - secure access mode for COMP
pub type COMPSEC_R = crate::BitReader<bool>;
///Field `COMPSEC` writer - secure access mode for COMP
pub type COMPSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_SECCFGR1_SPEC, bool, O>;
///Field `ADC4SEC` reader - secure access mode for ADC4
pub type ADC4SEC_R = crate::BitReader<bool>;
///Field `ADC4SEC` writer - secure access mode for ADC4
pub type ADC4SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_SECCFGR1_SPEC, bool, O>;
///Field `VREFBUFSEC` reader - secure access mode for VREFBUF
pub type VREFBUFSEC_R = crate::BitReader<bool>;
///Field `VREFBUFSEC` writer - secure access mode for VREFBUF
pub type VREFBUFSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_SECCFGR1_SPEC, bool, O>;
///Field `DAC1SEC` reader - secure access mode for DAC1
pub type DAC1SEC_R = crate::BitReader<bool>;
///Field `DAC1SEC` writer - secure access mode for DAC1
pub type DAC1SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_SECCFGR1_SPEC, bool, O>;
///Field `ADF1SEC` reader - secure access mode for ADF1
pub type ADF1SEC_R = crate::BitReader<bool>;
///Field `ADF1SEC` writer - secure access mode for ADF1
pub type ADF1SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_SECCFGR1_SPEC, bool, O>;
impl R {
    ///Bit 0 - secure access mode for SPI3
    #[inline(always)]
    pub fn spi3sec(&self) -> SPI3SEC_R {
        SPI3SEC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - secure access mode for LPUART1
    #[inline(always)]
    pub fn lpuart1sec(&self) -> LPUART1SEC_R {
        LPUART1SEC_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - secure access mode for I2C3
    #[inline(always)]
    pub fn i2c3sec(&self) -> I2C3SEC_R {
        I2C3SEC_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - secure access mode for LPTIM1
    #[inline(always)]
    pub fn lptim1sec(&self) -> LPTIM1SEC_R {
        LPTIM1SEC_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - secure access mode for LPTIM3
    #[inline(always)]
    pub fn lptim3sec(&self) -> LPTIM3SEC_R {
        LPTIM3SEC_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - secure access mode for LPTIM4
    #[inline(always)]
    pub fn lptim4sec(&self) -> LPTIM4SEC_R {
        LPTIM4SEC_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - secure access mode for OPAMP
    #[inline(always)]
    pub fn opampsec(&self) -> OPAMPSEC_R {
        OPAMPSEC_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - secure access mode for COMP
    #[inline(always)]
    pub fn compsec(&self) -> COMPSEC_R {
        COMPSEC_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - secure access mode for ADC4
    #[inline(always)]
    pub fn adc4sec(&self) -> ADC4SEC_R {
        ADC4SEC_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - secure access mode for VREFBUF
    #[inline(always)]
    pub fn vrefbufsec(&self) -> VREFBUFSEC_R {
        VREFBUFSEC_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - secure access mode for DAC1
    #[inline(always)]
    pub fn dac1sec(&self) -> DAC1SEC_R {
        DAC1SEC_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - secure access mode for ADF1
    #[inline(always)]
    pub fn adf1sec(&self) -> ADF1SEC_R {
        ADF1SEC_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - secure access mode for SPI3
    #[inline(always)]
    #[must_use]
    pub fn spi3sec(&mut self) -> SPI3SEC_W<0> {
        SPI3SEC_W::new(self)
    }
    ///Bit 1 - secure access mode for LPUART1
    #[inline(always)]
    #[must_use]
    pub fn lpuart1sec(&mut self) -> LPUART1SEC_W<1> {
        LPUART1SEC_W::new(self)
    }
    ///Bit 2 - secure access mode for I2C3
    #[inline(always)]
    #[must_use]
    pub fn i2c3sec(&mut self) -> I2C3SEC_W<2> {
        I2C3SEC_W::new(self)
    }
    ///Bit 3 - secure access mode for LPTIM1
    #[inline(always)]
    #[must_use]
    pub fn lptim1sec(&mut self) -> LPTIM1SEC_W<3> {
        LPTIM1SEC_W::new(self)
    }
    ///Bit 4 - secure access mode for LPTIM3
    #[inline(always)]
    #[must_use]
    pub fn lptim3sec(&mut self) -> LPTIM3SEC_W<4> {
        LPTIM3SEC_W::new(self)
    }
    ///Bit 5 - secure access mode for LPTIM4
    #[inline(always)]
    #[must_use]
    pub fn lptim4sec(&mut self) -> LPTIM4SEC_W<5> {
        LPTIM4SEC_W::new(self)
    }
    ///Bit 6 - secure access mode for OPAMP
    #[inline(always)]
    #[must_use]
    pub fn opampsec(&mut self) -> OPAMPSEC_W<6> {
        OPAMPSEC_W::new(self)
    }
    ///Bit 7 - secure access mode for COMP
    #[inline(always)]
    #[must_use]
    pub fn compsec(&mut self) -> COMPSEC_W<7> {
        COMPSEC_W::new(self)
    }
    ///Bit 8 - secure access mode for ADC4
    #[inline(always)]
    #[must_use]
    pub fn adc4sec(&mut self) -> ADC4SEC_W<8> {
        ADC4SEC_W::new(self)
    }
    ///Bit 9 - secure access mode for VREFBUF
    #[inline(always)]
    #[must_use]
    pub fn vrefbufsec(&mut self) -> VREFBUFSEC_W<9> {
        VREFBUFSEC_W::new(self)
    }
    ///Bit 11 - secure access mode for DAC1
    #[inline(always)]
    #[must_use]
    pub fn dac1sec(&mut self) -> DAC1SEC_W<11> {
        DAC1SEC_W::new(self)
    }
    ///Bit 12 - secure access mode for ADF1
    #[inline(always)]
    #[must_use]
    pub fn adf1sec(&mut self) -> ADF1SEC_W<12> {
        ADF1SEC_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TZSC secure configuration register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tzsc_seccfgr1](index.html) module
pub struct TZSC_SECCFGR1_SPEC;
impl crate::RegisterSpec for TZSC_SECCFGR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [tzsc_seccfgr1::R](R) reader structure
impl crate::Readable for TZSC_SECCFGR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [tzsc_seccfgr1::W](W) writer structure
impl crate::Writable for TZSC_SECCFGR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets TZSC_SECCFGR1 to value 0
impl crate::Resettable for TZSC_SECCFGR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
