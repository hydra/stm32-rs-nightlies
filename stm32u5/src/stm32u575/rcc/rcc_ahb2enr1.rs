///Register `RCC_AHB2ENR1` reader
pub struct R(crate::R<RCC_AHB2ENR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_AHB2ENR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_AHB2ENR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_AHB2ENR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RCC_AHB2ENR1` writer
pub struct W(crate::W<RCC_AHB2ENR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_AHB2ENR1_SPEC>;
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
impl From<crate::W<RCC_AHB2ENR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_AHB2ENR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `GPIOAEN` reader - IO port A clock enable Set and cleared by software.
pub type GPIOAEN_R = crate::BitReader<bool>;
///Field `GPIOAEN` writer - IO port A clock enable Set and cleared by software.
pub type GPIOAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB2ENR1_SPEC, bool, O>;
///Field `GPIOBEN` reader - IO port B clock enable Set and cleared by software.
pub type GPIOBEN_R = crate::BitReader<bool>;
///Field `GPIOBEN` writer - IO port B clock enable Set and cleared by software.
pub type GPIOBEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB2ENR1_SPEC, bool, O>;
///Field `GPIOCEN` reader - IO port C clock enable Set and cleared by software.
pub type GPIOCEN_R = crate::BitReader<bool>;
///Field `GPIOCEN` writer - IO port C clock enable Set and cleared by software.
pub type GPIOCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB2ENR1_SPEC, bool, O>;
///Field `GPIODEN` reader - IO port D clock enable Set and cleared by software.
pub type GPIODEN_R = crate::BitReader<bool>;
///Field `GPIODEN` writer - IO port D clock enable Set and cleared by software.
pub type GPIODEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB2ENR1_SPEC, bool, O>;
///Field `GPIOEEN` reader - IO port E clock enable Set and cleared by software.
pub type GPIOEEN_R = crate::BitReader<bool>;
///Field `GPIOEEN` writer - IO port E clock enable Set and cleared by software.
pub type GPIOEEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB2ENR1_SPEC, bool, O>;
///Field `GPIOFEN` reader - IO port F clock enable Set and cleared by software.
pub type GPIOFEN_R = crate::BitReader<bool>;
///Field `GPIOFEN` writer - IO port F clock enable Set and cleared by software.
pub type GPIOFEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB2ENR1_SPEC, bool, O>;
///Field `GPIOGEN` reader - IO port G clock enable Set and cleared by software.
pub type GPIOGEN_R = crate::BitReader<bool>;
///Field `GPIOGEN` writer - IO port G clock enable Set and cleared by software.
pub type GPIOGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB2ENR1_SPEC, bool, O>;
///Field `GPIOHEN` reader - IO port H clock enable Set and cleared by software.
pub type GPIOHEN_R = crate::BitReader<bool>;
///Field `GPIOHEN` writer - IO port H clock enable Set and cleared by software.
pub type GPIOHEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB2ENR1_SPEC, bool, O>;
///Field `GPIOIEN` reader - IO port I clock enable Set and cleared by software.
pub type GPIOIEN_R = crate::BitReader<bool>;
///Field `GPIOIEN` writer - IO port I clock enable Set and cleared by software.
pub type GPIOIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB2ENR1_SPEC, bool, O>;
///Field `ADC1EN` reader - ADC1 clock enable Set and cleared by software.
pub type ADC1EN_R = crate::BitReader<bool>;
///Field `ADC1EN` writer - ADC1 clock enable Set and cleared by software.
pub type ADC1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB2ENR1_SPEC, bool, O>;
///Field `DCMI_PSSIEN` reader - DCMI and PSSI clock enable Set and cleared by software.
pub type DCMI_PSSIEN_R = crate::BitReader<bool>;
///Field `DCMI_PSSIEN` writer - DCMI and PSSI clock enable Set and cleared by software.
pub type DCMI_PSSIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB2ENR1_SPEC, bool, O>;
///Field `OTGEN` reader - OTG_FS clock enable Set and cleared by software.
pub type OTGEN_R = crate::BitReader<bool>;
///Field `OTGEN` writer - OTG_FS clock enable Set and cleared by software.
pub type OTGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB2ENR1_SPEC, bool, O>;
///Field `AESEN` reader - AES clock enable Set and cleared by software.
pub type AESEN_R = crate::BitReader<bool>;
///Field `AESEN` writer - AES clock enable Set and cleared by software.
pub type AESEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB2ENR1_SPEC, bool, O>;
///Field `HASHEN` reader - HASH clock enable Set and cleared by software
pub type HASHEN_R = crate::BitReader<bool>;
///Field `HASHEN` writer - HASH clock enable Set and cleared by software
pub type HASHEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB2ENR1_SPEC, bool, O>;
///Field `RNGEN` reader - RNG clock enable Set and cleared by software.
pub type RNGEN_R = crate::BitReader<bool>;
///Field `RNGEN` writer - RNG clock enable Set and cleared by software.
pub type RNGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB2ENR1_SPEC, bool, O>;
///Field `PKAEN` reader - PKA clock enable Set and cleared by software.
pub type PKAEN_R = crate::BitReader<bool>;
///Field `PKAEN` writer - PKA clock enable Set and cleared by software.
pub type PKAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB2ENR1_SPEC, bool, O>;
///Field `SAESEN` reader - SAES clock enable Set and cleared by software.
pub type SAESEN_R = crate::BitReader<bool>;
///Field `SAESEN` writer - SAES clock enable Set and cleared by software.
pub type SAESEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB2ENR1_SPEC, bool, O>;
///Field `OCTOSPIMEN` reader - OCTOSPIM clock enable Set and cleared by software.
pub type OCTOSPIMEN_R = crate::BitReader<bool>;
///Field `OCTOSPIMEN` writer - OCTOSPIM clock enable Set and cleared by software.
pub type OCTOSPIMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB2ENR1_SPEC, bool, O>;
///Field `OTFDEC1EN` reader - OTFDEC1 clock enable Set and cleared by software.
pub type OTFDEC1EN_R = crate::BitReader<bool>;
///Field `OTFDEC1EN` writer - OTFDEC1 clock enable Set and cleared by software.
pub type OTFDEC1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB2ENR1_SPEC, bool, O>;
///Field `OTFDEC2EN` reader - OTFDEC2 clock enable Set and cleared by software.
pub type OTFDEC2EN_R = crate::BitReader<bool>;
///Field `OTFDEC2EN` writer - OTFDEC2 clock enable Set and cleared by software.
pub type OTFDEC2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB2ENR1_SPEC, bool, O>;
///Field `SDMMC1EN` reader - SDMMC1 clock enable Set and cleared by software.
pub type SDMMC1EN_R = crate::BitReader<bool>;
///Field `SDMMC1EN` writer - SDMMC1 clock enable Set and cleared by software.
pub type SDMMC1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB2ENR1_SPEC, bool, O>;
///Field `SDMMC2EN` reader - SDMMC2 clock enable Set and cleared by software.
pub type SDMMC2EN_R = crate::BitReader<bool>;
///Field `SDMMC2EN` writer - SDMMC2 clock enable Set and cleared by software.
pub type SDMMC2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB2ENR1_SPEC, bool, O>;
///Field `SRAM2EN` reader - SRAM2 clock enable Set and reset by software.
pub type SRAM2EN_R = crate::BitReader<bool>;
///Field `SRAM2EN` writer - SRAM2 clock enable Set and reset by software.
pub type SRAM2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB2ENR1_SPEC, bool, O>;
///Field `SRAM3EN` reader - SRAM3 clock enable Set and reset by software.
pub type SRAM3EN_R = crate::BitReader<bool>;
///Field `SRAM3EN` writer - SRAM3 clock enable Set and reset by software.
pub type SRAM3EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB2ENR1_SPEC, bool, O>;
impl R {
    ///Bit 0 - IO port A clock enable Set and cleared by software.
    #[inline(always)]
    pub fn gpioaen(&self) -> GPIOAEN_R {
        GPIOAEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - IO port B clock enable Set and cleared by software.
    #[inline(always)]
    pub fn gpioben(&self) -> GPIOBEN_R {
        GPIOBEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - IO port C clock enable Set and cleared by software.
    #[inline(always)]
    pub fn gpiocen(&self) -> GPIOCEN_R {
        GPIOCEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - IO port D clock enable Set and cleared by software.
    #[inline(always)]
    pub fn gpioden(&self) -> GPIODEN_R {
        GPIODEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - IO port E clock enable Set and cleared by software.
    #[inline(always)]
    pub fn gpioeen(&self) -> GPIOEEN_R {
        GPIOEEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - IO port F clock enable Set and cleared by software.
    #[inline(always)]
    pub fn gpiofen(&self) -> GPIOFEN_R {
        GPIOFEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - IO port G clock enable Set and cleared by software.
    #[inline(always)]
    pub fn gpiogen(&self) -> GPIOGEN_R {
        GPIOGEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - IO port H clock enable Set and cleared by software.
    #[inline(always)]
    pub fn gpiohen(&self) -> GPIOHEN_R {
        GPIOHEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - IO port I clock enable Set and cleared by software.
    #[inline(always)]
    pub fn gpioien(&self) -> GPIOIEN_R {
        GPIOIEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 10 - ADC1 clock enable Set and cleared by software.
    #[inline(always)]
    pub fn adc1en(&self) -> ADC1EN_R {
        ADC1EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 12 - DCMI and PSSI clock enable Set and cleared by software.
    #[inline(always)]
    pub fn dcmi_pssien(&self) -> DCMI_PSSIEN_R {
        DCMI_PSSIEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - OTG_FS clock enable Set and cleared by software.
    #[inline(always)]
    pub fn otgen(&self) -> OTGEN_R {
        OTGEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - AES clock enable Set and cleared by software.
    #[inline(always)]
    pub fn aesen(&self) -> AESEN_R {
        AESEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - HASH clock enable Set and cleared by software
    #[inline(always)]
    pub fn hashen(&self) -> HASHEN_R {
        HASHEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - RNG clock enable Set and cleared by software.
    #[inline(always)]
    pub fn rngen(&self) -> RNGEN_R {
        RNGEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - PKA clock enable Set and cleared by software.
    #[inline(always)]
    pub fn pkaen(&self) -> PKAEN_R {
        PKAEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - SAES clock enable Set and cleared by software.
    #[inline(always)]
    pub fn saesen(&self) -> SAESEN_R {
        SAESEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - OCTOSPIM clock enable Set and cleared by software.
    #[inline(always)]
    pub fn octospimen(&self) -> OCTOSPIMEN_R {
        OCTOSPIMEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 23 - OTFDEC1 clock enable Set and cleared by software.
    #[inline(always)]
    pub fn otfdec1en(&self) -> OTFDEC1EN_R {
        OTFDEC1EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - OTFDEC2 clock enable Set and cleared by software.
    #[inline(always)]
    pub fn otfdec2en(&self) -> OTFDEC2EN_R {
        OTFDEC2EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 27 - SDMMC1 clock enable Set and cleared by software.
    #[inline(always)]
    pub fn sdmmc1en(&self) -> SDMMC1EN_R {
        SDMMC1EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - SDMMC2 clock enable Set and cleared by software.
    #[inline(always)]
    pub fn sdmmc2en(&self) -> SDMMC2EN_R {
        SDMMC2EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 30 - SRAM2 clock enable Set and reset by software.
    #[inline(always)]
    pub fn sram2en(&self) -> SRAM2EN_R {
        SRAM2EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - SRAM3 clock enable Set and reset by software.
    #[inline(always)]
    pub fn sram3en(&self) -> SRAM3EN_R {
        SRAM3EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - IO port A clock enable Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn gpioaen(&mut self) -> GPIOAEN_W<0> {
        GPIOAEN_W::new(self)
    }
    ///Bit 1 - IO port B clock enable Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn gpioben(&mut self) -> GPIOBEN_W<1> {
        GPIOBEN_W::new(self)
    }
    ///Bit 2 - IO port C clock enable Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn gpiocen(&mut self) -> GPIOCEN_W<2> {
        GPIOCEN_W::new(self)
    }
    ///Bit 3 - IO port D clock enable Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn gpioden(&mut self) -> GPIODEN_W<3> {
        GPIODEN_W::new(self)
    }
    ///Bit 4 - IO port E clock enable Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn gpioeen(&mut self) -> GPIOEEN_W<4> {
        GPIOEEN_W::new(self)
    }
    ///Bit 5 - IO port F clock enable Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn gpiofen(&mut self) -> GPIOFEN_W<5> {
        GPIOFEN_W::new(self)
    }
    ///Bit 6 - IO port G clock enable Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn gpiogen(&mut self) -> GPIOGEN_W<6> {
        GPIOGEN_W::new(self)
    }
    ///Bit 7 - IO port H clock enable Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn gpiohen(&mut self) -> GPIOHEN_W<7> {
        GPIOHEN_W::new(self)
    }
    ///Bit 8 - IO port I clock enable Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn gpioien(&mut self) -> GPIOIEN_W<8> {
        GPIOIEN_W::new(self)
    }
    ///Bit 10 - ADC1 clock enable Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn adc1en(&mut self) -> ADC1EN_W<10> {
        ADC1EN_W::new(self)
    }
    ///Bit 12 - DCMI and PSSI clock enable Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn dcmi_pssien(&mut self) -> DCMI_PSSIEN_W<12> {
        DCMI_PSSIEN_W::new(self)
    }
    ///Bit 14 - OTG_FS clock enable Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn otgen(&mut self) -> OTGEN_W<14> {
        OTGEN_W::new(self)
    }
    ///Bit 16 - AES clock enable Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn aesen(&mut self) -> AESEN_W<16> {
        AESEN_W::new(self)
    }
    ///Bit 17 - HASH clock enable Set and cleared by software
    #[inline(always)]
    #[must_use]
    pub fn hashen(&mut self) -> HASHEN_W<17> {
        HASHEN_W::new(self)
    }
    ///Bit 18 - RNG clock enable Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn rngen(&mut self) -> RNGEN_W<18> {
        RNGEN_W::new(self)
    }
    ///Bit 19 - PKA clock enable Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn pkaen(&mut self) -> PKAEN_W<19> {
        PKAEN_W::new(self)
    }
    ///Bit 20 - SAES clock enable Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn saesen(&mut self) -> SAESEN_W<20> {
        SAESEN_W::new(self)
    }
    ///Bit 21 - OCTOSPIM clock enable Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn octospimen(&mut self) -> OCTOSPIMEN_W<21> {
        OCTOSPIMEN_W::new(self)
    }
    ///Bit 23 - OTFDEC1 clock enable Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn otfdec1en(&mut self) -> OTFDEC1EN_W<23> {
        OTFDEC1EN_W::new(self)
    }
    ///Bit 24 - OTFDEC2 clock enable Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn otfdec2en(&mut self) -> OTFDEC2EN_W<24> {
        OTFDEC2EN_W::new(self)
    }
    ///Bit 27 - SDMMC1 clock enable Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn sdmmc1en(&mut self) -> SDMMC1EN_W<27> {
        SDMMC1EN_W::new(self)
    }
    ///Bit 28 - SDMMC2 clock enable Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn sdmmc2en(&mut self) -> SDMMC2EN_W<28> {
        SDMMC2EN_W::new(self)
    }
    ///Bit 30 - SRAM2 clock enable Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn sram2en(&mut self) -> SRAM2EN_W<30> {
        SRAM2EN_W::new(self)
    }
    ///Bit 31 - SRAM3 clock enable Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn sram3en(&mut self) -> SRAM3EN_W<31> {
        SRAM3EN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RCC AHB2 peripheral clock enable register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rcc_ahb2enr1](index.html) module
pub struct RCC_AHB2ENR1_SPEC;
impl crate::RegisterSpec for RCC_AHB2ENR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [rcc_ahb2enr1::R](R) reader structure
impl crate::Readable for RCC_AHB2ENR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rcc_ahb2enr1::W](W) writer structure
impl crate::Writable for RCC_AHB2ENR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RCC_AHB2ENR1 to value 0xc000_0000
impl crate::Resettable for RCC_AHB2ENR1_SPEC {
    const RESET_VALUE: Self::Ux = 0xc000_0000;
}
