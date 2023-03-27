///Register `RCC_AHB2SMENR1` reader
pub struct R(crate::R<RCC_AHB2SMENR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_AHB2SMENR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_AHB2SMENR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_AHB2SMENR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RCC_AHB2SMENR1` writer
pub struct W(crate::W<RCC_AHB2SMENR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_AHB2SMENR1_SPEC>;
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
impl From<crate::W<RCC_AHB2SMENR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_AHB2SMENR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `GPIOASMEN` reader - IO port A clocks enable during Sleep and Stop modes Set and cleared by software.
pub type GPIOASMEN_R = crate::BitReader<bool>;
///Field `GPIOASMEN` writer - IO port A clocks enable during Sleep and Stop modes Set and cleared by software.
pub type GPIOASMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB2SMENR1_SPEC, bool, O>;
///Field `GPIOBSMEN` reader - IO port B clocks enable during Sleep and Stop modes Set and cleared by software.
pub type GPIOBSMEN_R = crate::BitReader<bool>;
///Field `GPIOBSMEN` writer - IO port B clocks enable during Sleep and Stop modes Set and cleared by software.
pub type GPIOBSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB2SMENR1_SPEC, bool, O>;
///Field `GPIOCSMEN` reader - IO port C clocks enable during Sleep and Stop modes Set and cleared by software.
pub type GPIOCSMEN_R = crate::BitReader<bool>;
///Field `GPIOCSMEN` writer - IO port C clocks enable during Sleep and Stop modes Set and cleared by software.
pub type GPIOCSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB2SMENR1_SPEC, bool, O>;
///Field `GPIODSMEN` reader - IO port D clocks enable during Sleep and Stop modes Set and cleared by software.
pub type GPIODSMEN_R = crate::BitReader<bool>;
///Field `GPIODSMEN` writer - IO port D clocks enable during Sleep and Stop modes Set and cleared by software.
pub type GPIODSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB2SMENR1_SPEC, bool, O>;
///Field `GPIOESMEN` reader - IO port E clocks enable during Sleep and Stop modes Set and cleared by software.
pub type GPIOESMEN_R = crate::BitReader<bool>;
///Field `GPIOESMEN` writer - IO port E clocks enable during Sleep and Stop modes Set and cleared by software.
pub type GPIOESMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB2SMENR1_SPEC, bool, O>;
///Field `GPIOFSMEN` reader - IO port F clocks enable during Sleep and Stop modes Set and cleared by software.
pub type GPIOFSMEN_R = crate::BitReader<bool>;
///Field `GPIOFSMEN` writer - IO port F clocks enable during Sleep and Stop modes Set and cleared by software.
pub type GPIOFSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB2SMENR1_SPEC, bool, O>;
///Field `GPIOGSMEN` reader - IO port G clocks enable during Sleep and Stop modes Set and cleared by software.
pub type GPIOGSMEN_R = crate::BitReader<bool>;
///Field `GPIOGSMEN` writer - IO port G clocks enable during Sleep and Stop modes Set and cleared by software.
pub type GPIOGSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB2SMENR1_SPEC, bool, O>;
///Field `GPIOHSMEN` reader - IO port H clocks enable during Sleep and Stop modes Set and cleared by software.
pub type GPIOHSMEN_R = crate::BitReader<bool>;
///Field `GPIOHSMEN` writer - IO port H clocks enable during Sleep and Stop modes Set and cleared by software.
pub type GPIOHSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB2SMENR1_SPEC, bool, O>;
///Field `GPIOISMEN` reader - IO port I clocks enable during Sleep and Stop modes Set and cleared by software.
pub type GPIOISMEN_R = crate::BitReader<bool>;
///Field `GPIOISMEN` writer - IO port I clocks enable during Sleep and Stop modes Set and cleared by software.
pub type GPIOISMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB2SMENR1_SPEC, bool, O>;
///Field `ADC1SMEN` reader - ADC1 clocks enable during Sleep and Stop modes Set and cleared by software.
pub type ADC1SMEN_R = crate::BitReader<bool>;
///Field `ADC1SMEN` writer - ADC1 clocks enable during Sleep and Stop modes Set and cleared by software.
pub type ADC1SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB2SMENR1_SPEC, bool, O>;
///Field `DCMI_PSSISMEN` reader - DCMI and PSSI clocks enable during Sleep and Stop modes Set and cleared by software.
pub type DCMI_PSSISMEN_R = crate::BitReader<bool>;
///Field `DCMI_PSSISMEN` writer - DCMI and PSSI clocks enable during Sleep and Stop modes Set and cleared by software.
pub type DCMI_PSSISMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB2SMENR1_SPEC, bool, O>;
///Field `OTGSMEN` reader - OTG_FS clocks enable during Sleep and Stop modes Set and cleared by software.
pub type OTGSMEN_R = crate::BitReader<bool>;
///Field `OTGSMEN` writer - OTG_FS clocks enable during Sleep and Stop modes Set and cleared by software.
pub type OTGSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB2SMENR1_SPEC, bool, O>;
///Field `AESSMEN` reader - AES clock enable during Sleep and Stop modes Set and cleared by software
pub type AESSMEN_R = crate::BitReader<bool>;
///Field `AESSMEN` writer - AES clock enable during Sleep and Stop modes Set and cleared by software
pub type AESSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB2SMENR1_SPEC, bool, O>;
///Field `HASHSMEN` reader - HASH clock enable during Sleep and Stop modes Set and cleared by software
pub type HASHSMEN_R = crate::BitReader<bool>;
///Field `HASHSMEN` writer - HASH clock enable during Sleep and Stop modes Set and cleared by software
pub type HASHSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB2SMENR1_SPEC, bool, O>;
///Field `RNGSMEN` reader - Random number generator (RNG) clocks enable during Sleep and Stop modes Set and cleared by software.
pub type RNGSMEN_R = crate::BitReader<bool>;
///Field `RNGSMEN` writer - Random number generator (RNG) clocks enable during Sleep and Stop modes Set and cleared by software.
pub type RNGSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB2SMENR1_SPEC, bool, O>;
///Field `PKASMEN` reader - PKA clocks enable during Sleep and Stop modes Set and cleared by software.
pub type PKASMEN_R = crate::BitReader<bool>;
///Field `PKASMEN` writer - PKA clocks enable during Sleep and Stop modes Set and cleared by software.
pub type PKASMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB2SMENR1_SPEC, bool, O>;
///Field `SAESSMEN` reader - SAES accelerator clocks enable during Sleep and Stop modes Set and cleared by software.
pub type SAESSMEN_R = crate::BitReader<bool>;
///Field `SAESSMEN` writer - SAES accelerator clocks enable during Sleep and Stop modes Set and cleared by software.
pub type SAESSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB2SMENR1_SPEC, bool, O>;
///Field `OCTOSPIMSMEN` reader - OCTOSPIM clocks enable during Sleep and Stop modes Set and cleared by software.
pub type OCTOSPIMSMEN_R = crate::BitReader<bool>;
///Field `OCTOSPIMSMEN` writer - OCTOSPIM clocks enable during Sleep and Stop modes Set and cleared by software.
pub type OCTOSPIMSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB2SMENR1_SPEC, bool, O>;
///Field `OTFDEC1SMEN` reader - OTFDEC1 clocks enable during Sleep and Stop modes Set and cleared by software.
pub type OTFDEC1SMEN_R = crate::BitReader<bool>;
///Field `OTFDEC1SMEN` writer - OTFDEC1 clocks enable during Sleep and Stop modes Set and cleared by software.
pub type OTFDEC1SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB2SMENR1_SPEC, bool, O>;
///Field `OTFDEC2SMEN` reader - OTFDEC2 clocks enable during Sleep and Stop modes Set and cleared by software.
pub type OTFDEC2SMEN_R = crate::BitReader<bool>;
///Field `OTFDEC2SMEN` writer - OTFDEC2 clocks enable during Sleep and Stop modes Set and cleared by software.
pub type OTFDEC2SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB2SMENR1_SPEC, bool, O>;
///Field `SDMMC1SMEN` reader - SDMMC1 clocks enable during Sleep and Stop modes Set and cleared by software.
pub type SDMMC1SMEN_R = crate::BitReader<bool>;
///Field `SDMMC1SMEN` writer - SDMMC1 clocks enable during Sleep and Stop modes Set and cleared by software.
pub type SDMMC1SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB2SMENR1_SPEC, bool, O>;
///Field `SDMMC2SMEN` reader - SDMMC2 clocks enable during Sleep and Stop modes Set and cleared by software.
pub type SDMMC2SMEN_R = crate::BitReader<bool>;
///Field `SDMMC2SMEN` writer - SDMMC2 clocks enable during Sleep and Stop modes Set and cleared by software.
pub type SDMMC2SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB2SMENR1_SPEC, bool, O>;
///Field `SRAM2SMEN` reader - SRAM2 clocks enable during Sleep and Stop modes Set and cleared by software.
pub type SRAM2SMEN_R = crate::BitReader<bool>;
///Field `SRAM2SMEN` writer - SRAM2 clocks enable during Sleep and Stop modes Set and cleared by software.
pub type SRAM2SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB2SMENR1_SPEC, bool, O>;
///Field `SRAM3SMEN` reader - SRAM3 clocks enable during Sleep and Stop modes Set and cleared by software.
pub type SRAM3SMEN_R = crate::BitReader<bool>;
///Field `SRAM3SMEN` writer - SRAM3 clocks enable during Sleep and Stop modes Set and cleared by software.
pub type SRAM3SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB2SMENR1_SPEC, bool, O>;
impl R {
    ///Bit 0 - IO port A clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn gpioasmen(&self) -> GPIOASMEN_R {
        GPIOASMEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - IO port B clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn gpiobsmen(&self) -> GPIOBSMEN_R {
        GPIOBSMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - IO port C clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn gpiocsmen(&self) -> GPIOCSMEN_R {
        GPIOCSMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - IO port D clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn gpiodsmen(&self) -> GPIODSMEN_R {
        GPIODSMEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - IO port E clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn gpioesmen(&self) -> GPIOESMEN_R {
        GPIOESMEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - IO port F clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn gpiofsmen(&self) -> GPIOFSMEN_R {
        GPIOFSMEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - IO port G clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn gpiogsmen(&self) -> GPIOGSMEN_R {
        GPIOGSMEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - IO port H clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn gpiohsmen(&self) -> GPIOHSMEN_R {
        GPIOHSMEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - IO port I clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn gpioismen(&self) -> GPIOISMEN_R {
        GPIOISMEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 10 - ADC1 clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn adc1smen(&self) -> ADC1SMEN_R {
        ADC1SMEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 12 - DCMI and PSSI clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn dcmi_pssismen(&self) -> DCMI_PSSISMEN_R {
        DCMI_PSSISMEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - OTG_FS clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn otgsmen(&self) -> OTGSMEN_R {
        OTGSMEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - AES clock enable during Sleep and Stop modes Set and cleared by software
    #[inline(always)]
    pub fn aessmen(&self) -> AESSMEN_R {
        AESSMEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - HASH clock enable during Sleep and Stop modes Set and cleared by software
    #[inline(always)]
    pub fn hashsmen(&self) -> HASHSMEN_R {
        HASHSMEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Random number generator (RNG) clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn rngsmen(&self) -> RNGSMEN_R {
        RNGSMEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - PKA clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn pkasmen(&self) -> PKASMEN_R {
        PKASMEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - SAES accelerator clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn saessmen(&self) -> SAESSMEN_R {
        SAESSMEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - OCTOSPIM clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn octospimsmen(&self) -> OCTOSPIMSMEN_R {
        OCTOSPIMSMEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 23 - OTFDEC1 clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn otfdec1smen(&self) -> OTFDEC1SMEN_R {
        OTFDEC1SMEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - OTFDEC2 clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn otfdec2smen(&self) -> OTFDEC2SMEN_R {
        OTFDEC2SMEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 27 - SDMMC1 clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn sdmmc1smen(&self) -> SDMMC1SMEN_R {
        SDMMC1SMEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - SDMMC2 clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn sdmmc2smen(&self) -> SDMMC2SMEN_R {
        SDMMC2SMEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 30 - SRAM2 clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn sram2smen(&self) -> SRAM2SMEN_R {
        SRAM2SMEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - SRAM3 clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn sram3smen(&self) -> SRAM3SMEN_R {
        SRAM3SMEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - IO port A clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn gpioasmen(&mut self) -> GPIOASMEN_W<0> {
        GPIOASMEN_W::new(self)
    }
    ///Bit 1 - IO port B clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn gpiobsmen(&mut self) -> GPIOBSMEN_W<1> {
        GPIOBSMEN_W::new(self)
    }
    ///Bit 2 - IO port C clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn gpiocsmen(&mut self) -> GPIOCSMEN_W<2> {
        GPIOCSMEN_W::new(self)
    }
    ///Bit 3 - IO port D clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn gpiodsmen(&mut self) -> GPIODSMEN_W<3> {
        GPIODSMEN_W::new(self)
    }
    ///Bit 4 - IO port E clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn gpioesmen(&mut self) -> GPIOESMEN_W<4> {
        GPIOESMEN_W::new(self)
    }
    ///Bit 5 - IO port F clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn gpiofsmen(&mut self) -> GPIOFSMEN_W<5> {
        GPIOFSMEN_W::new(self)
    }
    ///Bit 6 - IO port G clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn gpiogsmen(&mut self) -> GPIOGSMEN_W<6> {
        GPIOGSMEN_W::new(self)
    }
    ///Bit 7 - IO port H clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn gpiohsmen(&mut self) -> GPIOHSMEN_W<7> {
        GPIOHSMEN_W::new(self)
    }
    ///Bit 8 - IO port I clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn gpioismen(&mut self) -> GPIOISMEN_W<8> {
        GPIOISMEN_W::new(self)
    }
    ///Bit 10 - ADC1 clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn adc1smen(&mut self) -> ADC1SMEN_W<10> {
        ADC1SMEN_W::new(self)
    }
    ///Bit 12 - DCMI and PSSI clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn dcmi_pssismen(&mut self) -> DCMI_PSSISMEN_W<12> {
        DCMI_PSSISMEN_W::new(self)
    }
    ///Bit 14 - OTG_FS clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn otgsmen(&mut self) -> OTGSMEN_W<14> {
        OTGSMEN_W::new(self)
    }
    ///Bit 16 - AES clock enable during Sleep and Stop modes Set and cleared by software
    #[inline(always)]
    #[must_use]
    pub fn aessmen(&mut self) -> AESSMEN_W<16> {
        AESSMEN_W::new(self)
    }
    ///Bit 17 - HASH clock enable during Sleep and Stop modes Set and cleared by software
    #[inline(always)]
    #[must_use]
    pub fn hashsmen(&mut self) -> HASHSMEN_W<17> {
        HASHSMEN_W::new(self)
    }
    ///Bit 18 - Random number generator (RNG) clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn rngsmen(&mut self) -> RNGSMEN_W<18> {
        RNGSMEN_W::new(self)
    }
    ///Bit 19 - PKA clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn pkasmen(&mut self) -> PKASMEN_W<19> {
        PKASMEN_W::new(self)
    }
    ///Bit 20 - SAES accelerator clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn saessmen(&mut self) -> SAESSMEN_W<20> {
        SAESSMEN_W::new(self)
    }
    ///Bit 21 - OCTOSPIM clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn octospimsmen(&mut self) -> OCTOSPIMSMEN_W<21> {
        OCTOSPIMSMEN_W::new(self)
    }
    ///Bit 23 - OTFDEC1 clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn otfdec1smen(&mut self) -> OTFDEC1SMEN_W<23> {
        OTFDEC1SMEN_W::new(self)
    }
    ///Bit 24 - OTFDEC2 clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn otfdec2smen(&mut self) -> OTFDEC2SMEN_W<24> {
        OTFDEC2SMEN_W::new(self)
    }
    ///Bit 27 - SDMMC1 clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn sdmmc1smen(&mut self) -> SDMMC1SMEN_W<27> {
        SDMMC1SMEN_W::new(self)
    }
    ///Bit 28 - SDMMC2 clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn sdmmc2smen(&mut self) -> SDMMC2SMEN_W<28> {
        SDMMC2SMEN_W::new(self)
    }
    ///Bit 30 - SRAM2 clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn sram2smen(&mut self) -> SRAM2SMEN_W<30> {
        SRAM2SMEN_W::new(self)
    }
    ///Bit 31 - SRAM3 clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn sram3smen(&mut self) -> SRAM3SMEN_W<31> {
        SRAM3SMEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RCC AHB2 peripheral clocks enable in Sleep and Stop modes register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rcc_ahb2smenr1](index.html) module
pub struct RCC_AHB2SMENR1_SPEC;
impl crate::RegisterSpec for RCC_AHB2SMENR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [rcc_ahb2smenr1::R](R) reader structure
impl crate::Readable for RCC_AHB2SMENR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rcc_ahb2smenr1::W](W) writer structure
impl crate::Writable for RCC_AHB2SMENR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RCC_AHB2SMENR1 to value 0xffff_ffff
impl crate::Resettable for RCC_AHB2SMENR1_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
