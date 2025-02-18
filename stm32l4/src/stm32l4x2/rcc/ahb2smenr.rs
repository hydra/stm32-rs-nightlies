///Register `AHB2SMENR` reader
pub struct R(crate::R<AHB2SMENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB2SMENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB2SMENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB2SMENR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `AHB2SMENR` writer
pub struct W(crate::W<AHB2SMENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB2SMENR_SPEC>;
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
impl From<crate::W<AHB2SMENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB2SMENR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `GPIOASMEN` reader - IO port A clocks enable during Sleep and Stop modes
pub type GPIOASMEN_R = crate::BitReader<bool>;
///Field `GPIOASMEN` writer - IO port A clocks enable during Sleep and Stop modes
pub type GPIOASMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2SMENR_SPEC, bool, O>;
///Field `GPIOBSMEN` reader - IO port B clocks enable during Sleep and Stop modes
pub type GPIOBSMEN_R = crate::BitReader<bool>;
///Field `GPIOBSMEN` writer - IO port B clocks enable during Sleep and Stop modes
pub type GPIOBSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2SMENR_SPEC, bool, O>;
///Field `GPIOCSMEN` reader - IO port C clocks enable during Sleep and Stop modes
pub type GPIOCSMEN_R = crate::BitReader<bool>;
///Field `GPIOCSMEN` writer - IO port C clocks enable during Sleep and Stop modes
pub type GPIOCSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2SMENR_SPEC, bool, O>;
///Field `GPIODSMEN` reader - IO port D clocks enable during Sleep and Stop modes
pub type GPIODSMEN_R = crate::BitReader<bool>;
///Field `GPIODSMEN` writer - IO port D clocks enable during Sleep and Stop modes
pub type GPIODSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2SMENR_SPEC, bool, O>;
///Field `GPIOESMEN` reader - IO port E clocks enable during Sleep and Stop modes
pub type GPIOESMEN_R = crate::BitReader<bool>;
///Field `GPIOESMEN` writer - IO port E clocks enable during Sleep and Stop modes
pub type GPIOESMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2SMENR_SPEC, bool, O>;
///Field `GPIOHSMEN` reader - IO port H clocks enable during Sleep and Stop modes
pub type GPIOHSMEN_R = crate::BitReader<bool>;
///Field `GPIOHSMEN` writer - IO port H clocks enable during Sleep and Stop modes
pub type GPIOHSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2SMENR_SPEC, bool, O>;
///Field `SRAM2SMEN` reader - SRAM2 interface clocks enable during Sleep and Stop modes
pub type SRAM2SMEN_R = crate::BitReader<bool>;
///Field `SRAM2SMEN` writer - SRAM2 interface clocks enable during Sleep and Stop modes
pub type SRAM2SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2SMENR_SPEC, bool, O>;
///Field `ADCFSSMEN` reader - ADC clocks enable during Sleep and Stop modes
pub type ADCFSSMEN_R = crate::BitReader<bool>;
///Field `ADCFSSMEN` writer - ADC clocks enable during Sleep and Stop modes
pub type ADCFSSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2SMENR_SPEC, bool, O>;
///Field `AESSMEN` reader - AES accelerator clocks enable during Sleep and Stop modes
pub type AESSMEN_R = crate::BitReader<bool>;
///Field `AESSMEN` writer - AES accelerator clocks enable during Sleep and Stop modes
pub type AESSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2SMENR_SPEC, bool, O>;
///Field `RNGSMEN` reader - Random Number Generator clocks enable during Sleep and Stop modes
pub type RNGSMEN_R = crate::BitReader<bool>;
///Field `RNGSMEN` writer - Random Number Generator clocks enable during Sleep and Stop modes
pub type RNGSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2SMENR_SPEC, bool, O>;
impl R {
    ///Bit 0 - IO port A clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpioasmen(&self) -> GPIOASMEN_R {
        GPIOASMEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - IO port B clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpiobsmen(&self) -> GPIOBSMEN_R {
        GPIOBSMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - IO port C clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpiocsmen(&self) -> GPIOCSMEN_R {
        GPIOCSMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - IO port D clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpiodsmen(&self) -> GPIODSMEN_R {
        GPIODSMEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - IO port E clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpioesmen(&self) -> GPIOESMEN_R {
        GPIOESMEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 7 - IO port H clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpiohsmen(&self) -> GPIOHSMEN_R {
        GPIOHSMEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 9 - SRAM2 interface clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn sram2smen(&self) -> SRAM2SMEN_R {
        SRAM2SMEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 13 - ADC clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn adcfssmen(&self) -> ADCFSSMEN_R {
        ADCFSSMEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 16 - AES accelerator clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn aessmen(&self) -> AESSMEN_R {
        AESSMEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 18 - Random Number Generator clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn rngsmen(&self) -> RNGSMEN_R {
        RNGSMEN_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - IO port A clocks enable during Sleep and Stop modes
    #[inline(always)]
    #[must_use]
    pub fn gpioasmen(&mut self) -> GPIOASMEN_W<0> {
        GPIOASMEN_W::new(self)
    }
    ///Bit 1 - IO port B clocks enable during Sleep and Stop modes
    #[inline(always)]
    #[must_use]
    pub fn gpiobsmen(&mut self) -> GPIOBSMEN_W<1> {
        GPIOBSMEN_W::new(self)
    }
    ///Bit 2 - IO port C clocks enable during Sleep and Stop modes
    #[inline(always)]
    #[must_use]
    pub fn gpiocsmen(&mut self) -> GPIOCSMEN_W<2> {
        GPIOCSMEN_W::new(self)
    }
    ///Bit 3 - IO port D clocks enable during Sleep and Stop modes
    #[inline(always)]
    #[must_use]
    pub fn gpiodsmen(&mut self) -> GPIODSMEN_W<3> {
        GPIODSMEN_W::new(self)
    }
    ///Bit 4 - IO port E clocks enable during Sleep and Stop modes
    #[inline(always)]
    #[must_use]
    pub fn gpioesmen(&mut self) -> GPIOESMEN_W<4> {
        GPIOESMEN_W::new(self)
    }
    ///Bit 7 - IO port H clocks enable during Sleep and Stop modes
    #[inline(always)]
    #[must_use]
    pub fn gpiohsmen(&mut self) -> GPIOHSMEN_W<7> {
        GPIOHSMEN_W::new(self)
    }
    ///Bit 9 - SRAM2 interface clocks enable during Sleep and Stop modes
    #[inline(always)]
    #[must_use]
    pub fn sram2smen(&mut self) -> SRAM2SMEN_W<9> {
        SRAM2SMEN_W::new(self)
    }
    ///Bit 13 - ADC clocks enable during Sleep and Stop modes
    #[inline(always)]
    #[must_use]
    pub fn adcfssmen(&mut self) -> ADCFSSMEN_W<13> {
        ADCFSSMEN_W::new(self)
    }
    ///Bit 16 - AES accelerator clocks enable during Sleep and Stop modes
    #[inline(always)]
    #[must_use]
    pub fn aessmen(&mut self) -> AESSMEN_W<16> {
        AESSMEN_W::new(self)
    }
    ///Bit 18 - Random Number Generator clocks enable during Sleep and Stop modes
    #[inline(always)]
    #[must_use]
    pub fn rngsmen(&mut self) -> RNGSMEN_W<18> {
        RNGSMEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///AHB2 peripheral clocks enable in Sleep and Stop modes register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ahb2smenr](index.html) module
pub struct AHB2SMENR_SPEC;
impl crate::RegisterSpec for AHB2SMENR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ahb2smenr::R](R) reader structure
impl crate::Readable for AHB2SMENR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ahb2smenr::W](W) writer structure
impl crate::Writable for AHB2SMENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets AHB2SMENR to value 0x0005_32ff
impl crate::Resettable for AHB2SMENR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0005_32ff;
}
