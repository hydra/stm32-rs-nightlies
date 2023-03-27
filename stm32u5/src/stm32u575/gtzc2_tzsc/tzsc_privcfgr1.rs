///Register `TZSC_PRIVCFGR1` reader
pub struct R(crate::R<TZSC_PRIVCFGR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZSC_PRIVCFGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZSC_PRIVCFGR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZSC_PRIVCFGR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TZSC_PRIVCFGR1` writer
pub struct W(crate::W<TZSC_PRIVCFGR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TZSC_PRIVCFGR1_SPEC>;
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
impl From<crate::W<TZSC_PRIVCFGR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TZSC_PRIVCFGR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SPI3PRIV` reader - privileged access mode for SPI3
pub type SPI3PRIV_R = crate::BitReader<bool>;
///Field `SPI3PRIV` writer - privileged access mode for SPI3
pub type SPI3PRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_PRIVCFGR1_SPEC, bool, O>;
///Field `LPUART1PRIV` reader - privileged access mode for LPUART1
pub type LPUART1PRIV_R = crate::BitReader<bool>;
///Field `LPUART1PRIV` writer - privileged access mode for LPUART1
pub type LPUART1PRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_PRIVCFGR1_SPEC, bool, O>;
///Field `I2C3PRIV` reader - privileged access mode for I2C3
pub type I2C3PRIV_R = crate::BitReader<bool>;
///Field `I2C3PRIV` writer - privileged access mode for I2C3
pub type I2C3PRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_PRIVCFGR1_SPEC, bool, O>;
///Field `LPTIM1PRIV` reader - privileged access mode for LPTIM1
pub type LPTIM1PRIV_R = crate::BitReader<bool>;
///Field `LPTIM1PRIV` writer - privileged access mode for LPTIM1
pub type LPTIM1PRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_PRIVCFGR1_SPEC, bool, O>;
///Field `LPTIM3PRIV` reader - privileged access mode for LPTIM3
pub type LPTIM3PRIV_R = crate::BitReader<bool>;
///Field `LPTIM3PRIV` writer - privileged access mode for LPTIM3
pub type LPTIM3PRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_PRIVCFGR1_SPEC, bool, O>;
///Field `LPTIM4PRIV` reader - privileged access mode for LPTIM4
pub type LPTIM4PRIV_R = crate::BitReader<bool>;
///Field `LPTIM4PRIV` writer - privileged access mode for LPTIM4
pub type LPTIM4PRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_PRIVCFGR1_SPEC, bool, O>;
///Field `OPAMPPRIV` reader - privileged access mode for OPAMP
pub type OPAMPPRIV_R = crate::BitReader<bool>;
///Field `OPAMPPRIV` writer - privileged access mode for OPAMP
pub type OPAMPPRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_PRIVCFGR1_SPEC, bool, O>;
///Field `COMPPRIV` reader - privileged access mode for COMP
pub type COMPPRIV_R = crate::BitReader<bool>;
///Field `COMPPRIV` writer - privileged access mode for COMP
pub type COMPPRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_PRIVCFGR1_SPEC, bool, O>;
///Field `ADC4PRIV` reader - privileged access mode for ADC4
pub type ADC4PRIV_R = crate::BitReader<bool>;
///Field `ADC4PRIV` writer - privileged access mode for ADC4
pub type ADC4PRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_PRIVCFGR1_SPEC, bool, O>;
///Field `VREFBUFPRIV` reader - privileged access mode for VREFBUF
pub type VREFBUFPRIV_R = crate::BitReader<bool>;
///Field `VREFBUFPRIV` writer - privileged access mode for VREFBUF
pub type VREFBUFPRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_PRIVCFGR1_SPEC, bool, O>;
///Field `DAC1PRIV` reader - privileged access mode for DAC1
pub type DAC1PRIV_R = crate::BitReader<bool>;
///Field `DAC1PRIV` writer - privileged access mode for DAC1
pub type DAC1PRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_PRIVCFGR1_SPEC, bool, O>;
///Field `ADF1PRIV` reader - privileged access mode for ADF1
pub type ADF1PRIV_R = crate::BitReader<bool>;
///Field `ADF1PRIV` writer - privileged access mode for ADF1
pub type ADF1PRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_PRIVCFGR1_SPEC, bool, O>;
impl R {
    ///Bit 0 - privileged access mode for SPI3
    #[inline(always)]
    pub fn spi3priv(&self) -> SPI3PRIV_R {
        SPI3PRIV_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - privileged access mode for LPUART1
    #[inline(always)]
    pub fn lpuart1priv(&self) -> LPUART1PRIV_R {
        LPUART1PRIV_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - privileged access mode for I2C3
    #[inline(always)]
    pub fn i2c3priv(&self) -> I2C3PRIV_R {
        I2C3PRIV_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - privileged access mode for LPTIM1
    #[inline(always)]
    pub fn lptim1priv(&self) -> LPTIM1PRIV_R {
        LPTIM1PRIV_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - privileged access mode for LPTIM3
    #[inline(always)]
    pub fn lptim3priv(&self) -> LPTIM3PRIV_R {
        LPTIM3PRIV_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - privileged access mode for LPTIM4
    #[inline(always)]
    pub fn lptim4priv(&self) -> LPTIM4PRIV_R {
        LPTIM4PRIV_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - privileged access mode for OPAMP
    #[inline(always)]
    pub fn opamppriv(&self) -> OPAMPPRIV_R {
        OPAMPPRIV_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - privileged access mode for COMP
    #[inline(always)]
    pub fn comppriv(&self) -> COMPPRIV_R {
        COMPPRIV_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - privileged access mode for ADC4
    #[inline(always)]
    pub fn adc4priv(&self) -> ADC4PRIV_R {
        ADC4PRIV_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - privileged access mode for VREFBUF
    #[inline(always)]
    pub fn vrefbufpriv(&self) -> VREFBUFPRIV_R {
        VREFBUFPRIV_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - privileged access mode for DAC1
    #[inline(always)]
    pub fn dac1priv(&self) -> DAC1PRIV_R {
        DAC1PRIV_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - privileged access mode for ADF1
    #[inline(always)]
    pub fn adf1priv(&self) -> ADF1PRIV_R {
        ADF1PRIV_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - privileged access mode for SPI3
    #[inline(always)]
    #[must_use]
    pub fn spi3priv(&mut self) -> SPI3PRIV_W<0> {
        SPI3PRIV_W::new(self)
    }
    ///Bit 1 - privileged access mode for LPUART1
    #[inline(always)]
    #[must_use]
    pub fn lpuart1priv(&mut self) -> LPUART1PRIV_W<1> {
        LPUART1PRIV_W::new(self)
    }
    ///Bit 2 - privileged access mode for I2C3
    #[inline(always)]
    #[must_use]
    pub fn i2c3priv(&mut self) -> I2C3PRIV_W<2> {
        I2C3PRIV_W::new(self)
    }
    ///Bit 3 - privileged access mode for LPTIM1
    #[inline(always)]
    #[must_use]
    pub fn lptim1priv(&mut self) -> LPTIM1PRIV_W<3> {
        LPTIM1PRIV_W::new(self)
    }
    ///Bit 4 - privileged access mode for LPTIM3
    #[inline(always)]
    #[must_use]
    pub fn lptim3priv(&mut self) -> LPTIM3PRIV_W<4> {
        LPTIM3PRIV_W::new(self)
    }
    ///Bit 5 - privileged access mode for LPTIM4
    #[inline(always)]
    #[must_use]
    pub fn lptim4priv(&mut self) -> LPTIM4PRIV_W<5> {
        LPTIM4PRIV_W::new(self)
    }
    ///Bit 6 - privileged access mode for OPAMP
    #[inline(always)]
    #[must_use]
    pub fn opamppriv(&mut self) -> OPAMPPRIV_W<6> {
        OPAMPPRIV_W::new(self)
    }
    ///Bit 7 - privileged access mode for COMP
    #[inline(always)]
    #[must_use]
    pub fn comppriv(&mut self) -> COMPPRIV_W<7> {
        COMPPRIV_W::new(self)
    }
    ///Bit 8 - privileged access mode for ADC4
    #[inline(always)]
    #[must_use]
    pub fn adc4priv(&mut self) -> ADC4PRIV_W<8> {
        ADC4PRIV_W::new(self)
    }
    ///Bit 9 - privileged access mode for VREFBUF
    #[inline(always)]
    #[must_use]
    pub fn vrefbufpriv(&mut self) -> VREFBUFPRIV_W<9> {
        VREFBUFPRIV_W::new(self)
    }
    ///Bit 11 - privileged access mode for DAC1
    #[inline(always)]
    #[must_use]
    pub fn dac1priv(&mut self) -> DAC1PRIV_W<11> {
        DAC1PRIV_W::new(self)
    }
    ///Bit 12 - privileged access mode for ADF1
    #[inline(always)]
    #[must_use]
    pub fn adf1priv(&mut self) -> ADF1PRIV_W<12> {
        ADF1PRIV_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TZSC privilege configuration register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tzsc_privcfgr1](index.html) module
pub struct TZSC_PRIVCFGR1_SPEC;
impl crate::RegisterSpec for TZSC_PRIVCFGR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [tzsc_privcfgr1::R](R) reader structure
impl crate::Readable for TZSC_PRIVCFGR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [tzsc_privcfgr1::W](W) writer structure
impl crate::Writable for TZSC_PRIVCFGR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets TZSC_PRIVCFGR1 to value 0
impl crate::Resettable for TZSC_PRIVCFGR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
