///Register `AHB2ENR` reader
pub struct R(crate::R<AHB2ENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB2ENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB2ENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB2ENR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `AHB2ENR` writer
pub struct W(crate::W<AHB2ENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB2ENR_SPEC>;
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
impl From<crate::W<AHB2ENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB2ENR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `GPIOAEN` reader - GPIOA clock enable Set and reset by software.
pub type GPIOAEN_R = crate::BitReader<bool>;
///Field `GPIOAEN` writer - GPIOA clock enable Set and reset by software.
pub type GPIOAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2ENR_SPEC, bool, O>;
///Field `GPIOBEN` reader - GPIOB clock enable Set and reset by software.
pub type GPIOBEN_R = crate::BitReader<bool>;
///Field `GPIOBEN` writer - GPIOB clock enable Set and reset by software.
pub type GPIOBEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2ENR_SPEC, bool, O>;
///Field `GPIOCEN` reader - GPIOC clock enable Set and reset by software.
pub type GPIOCEN_R = crate::BitReader<bool>;
///Field `GPIOCEN` writer - GPIOC clock enable Set and reset by software.
pub type GPIOCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2ENR_SPEC, bool, O>;
///Field `GPIODEN` reader - GPIOD clock enable Set and reset by software.
pub type GPIODEN_R = crate::BitReader<bool>;
///Field `GPIODEN` writer - GPIOD clock enable Set and reset by software.
pub type GPIODEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2ENR_SPEC, bool, O>;
///Field `GPIOEEN` reader - GPIOE clock enable Set and reset by software.
pub type GPIOEEN_R = crate::BitReader<bool>;
///Field `GPIOEEN` writer - GPIOE clock enable Set and reset by software.
pub type GPIOEEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2ENR_SPEC, bool, O>;
///Field `GPIOFEN` reader - GPIOF clock enable Set and reset by software.
pub type GPIOFEN_R = crate::BitReader<bool>;
///Field `GPIOFEN` writer - GPIOF clock enable Set and reset by software.
pub type GPIOFEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2ENR_SPEC, bool, O>;
///Field `GPIOGEN` reader - GPIOG clock enable Set and reset by software.
pub type GPIOGEN_R = crate::BitReader<bool>;
///Field `GPIOGEN` writer - GPIOG clock enable Set and reset by software.
pub type GPIOGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2ENR_SPEC, bool, O>;
///Field `GPIOHEN` reader - GPIOH clock enable Set and reset by software.
pub type GPIOHEN_R = crate::BitReader<bool>;
///Field `GPIOHEN` writer - GPIOH clock enable Set and reset by software.
pub type GPIOHEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2ENR_SPEC, bool, O>;
///Field `GPIOIEN` reader - GPIOI clock enable Set and reset by software.
pub type GPIOIEN_R = crate::BitReader<bool>;
///Field `GPIOIEN` writer - GPIOI clock enable Set and reset by software.
pub type GPIOIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2ENR_SPEC, bool, O>;
///Field `ADC12EN` reader - ADC1 and 2 peripherals clock enabled Set and reset by software.
pub type ADC12EN_R = crate::BitReader<bool>;
///Field `ADC12EN` writer - ADC1 and 2 peripherals clock enabled Set and reset by software.
pub type ADC12EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2ENR_SPEC, bool, O>;
///Field `DAC12EN` reader - DAC clock enable Set and reset by software.
pub type DAC12EN_R = crate::BitReader<bool>;
///Field `DAC12EN` writer - DAC clock enable Set and reset by software.
pub type DAC12EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2ENR_SPEC, bool, O>;
///Field `DCMI_PSSIEN` reader - digital camera interface clock enable (DCMI or PSSI depending which interface is active) Set and reset by software.
pub type DCMI_PSSIEN_R = crate::BitReader<bool>;
///Field `DCMI_PSSIEN` writer - digital camera interface clock enable (DCMI or PSSI depending which interface is active) Set and reset by software.
pub type DCMI_PSSIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2ENR_SPEC, bool, O>;
///Field `AESEN` reader - AES clock enable Set and reset by software.
pub type AESEN_R = crate::BitReader<bool>;
///Field `AESEN` writer - AES clock enable Set and reset by software.
pub type AESEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2ENR_SPEC, bool, O>;
///Field `HASHEN` reader - HASH clock enable Set and reset by software.
pub type HASHEN_R = crate::BitReader<bool>;
///Field `HASHEN` writer - HASH clock enable Set and reset by software.
pub type HASHEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2ENR_SPEC, bool, O>;
///Field `RNGEN` reader - RNG clock enable Set and reset by software.
pub type RNGEN_R = crate::BitReader<bool>;
///Field `RNGEN` writer - RNG clock enable Set and reset by software.
pub type RNGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2ENR_SPEC, bool, O>;
///Field `PKAEN` reader - PKA clock enable Set and reset by software.
pub type PKAEN_R = crate::BitReader<bool>;
///Field `PKAEN` writer - PKA clock enable Set and reset by software.
pub type PKAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2ENR_SPEC, bool, O>;
///Field `SAESEN` reader - SAES clock enable Set and reset by software.
pub type SAESEN_R = crate::BitReader<bool>;
///Field `SAESEN` writer - SAES clock enable Set and reset by software.
pub type SAESEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2ENR_SPEC, bool, O>;
///Field `SRAM3EN` reader - SRAM3 clock enable Set and reset by software.
pub type SRAM3EN_R = crate::BitReader<bool>;
///Field `SRAM3EN` writer - SRAM3 clock enable Set and reset by software.
pub type SRAM3EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2ENR_SPEC, bool, O>;
///Field `SRAM2EN` reader - SRAM2 clock enable Set and reset by software.
pub type SRAM2EN_R = crate::BitReader<bool>;
///Field `SRAM2EN` writer - SRAM2 clock enable Set and reset by software.
pub type SRAM2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2ENR_SPEC, bool, O>;
impl R {
    ///Bit 0 - GPIOA clock enable Set and reset by software.
    #[inline(always)]
    pub fn gpioaen(&self) -> GPIOAEN_R {
        GPIOAEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - GPIOB clock enable Set and reset by software.
    #[inline(always)]
    pub fn gpioben(&self) -> GPIOBEN_R {
        GPIOBEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - GPIOC clock enable Set and reset by software.
    #[inline(always)]
    pub fn gpiocen(&self) -> GPIOCEN_R {
        GPIOCEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - GPIOD clock enable Set and reset by software.
    #[inline(always)]
    pub fn gpioden(&self) -> GPIODEN_R {
        GPIODEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - GPIOE clock enable Set and reset by software.
    #[inline(always)]
    pub fn gpioeen(&self) -> GPIOEEN_R {
        GPIOEEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - GPIOF clock enable Set and reset by software.
    #[inline(always)]
    pub fn gpiofen(&self) -> GPIOFEN_R {
        GPIOFEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - GPIOG clock enable Set and reset by software.
    #[inline(always)]
    pub fn gpiogen(&self) -> GPIOGEN_R {
        GPIOGEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - GPIOH clock enable Set and reset by software.
    #[inline(always)]
    pub fn gpiohen(&self) -> GPIOHEN_R {
        GPIOHEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - GPIOI clock enable Set and reset by software.
    #[inline(always)]
    pub fn gpioien(&self) -> GPIOIEN_R {
        GPIOIEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 10 - ADC1 and 2 peripherals clock enabled Set and reset by software.
    #[inline(always)]
    pub fn adc12en(&self) -> ADC12EN_R {
        ADC12EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - DAC clock enable Set and reset by software.
    #[inline(always)]
    pub fn dac12en(&self) -> DAC12EN_R {
        DAC12EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - digital camera interface clock enable (DCMI or PSSI depending which interface is active) Set and reset by software.
    #[inline(always)]
    pub fn dcmi_pssien(&self) -> DCMI_PSSIEN_R {
        DCMI_PSSIEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 16 - AES clock enable Set and reset by software.
    #[inline(always)]
    pub fn aesen(&self) -> AESEN_R {
        AESEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - HASH clock enable Set and reset by software.
    #[inline(always)]
    pub fn hashen(&self) -> HASHEN_R {
        HASHEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - RNG clock enable Set and reset by software.
    #[inline(always)]
    pub fn rngen(&self) -> RNGEN_R {
        RNGEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - PKA clock enable Set and reset by software.
    #[inline(always)]
    pub fn pkaen(&self) -> PKAEN_R {
        PKAEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - SAES clock enable Set and reset by software.
    #[inline(always)]
    pub fn saesen(&self) -> SAESEN_R {
        SAESEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 30 - SRAM3 clock enable Set and reset by software.
    #[inline(always)]
    pub fn sram3en(&self) -> SRAM3EN_R {
        SRAM3EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - SRAM2 clock enable Set and reset by software.
    #[inline(always)]
    pub fn sram2en(&self) -> SRAM2EN_R {
        SRAM2EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - GPIOA clock enable Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn gpioaen(&mut self) -> GPIOAEN_W<0> {
        GPIOAEN_W::new(self)
    }
    ///Bit 1 - GPIOB clock enable Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn gpioben(&mut self) -> GPIOBEN_W<1> {
        GPIOBEN_W::new(self)
    }
    ///Bit 2 - GPIOC clock enable Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn gpiocen(&mut self) -> GPIOCEN_W<2> {
        GPIOCEN_W::new(self)
    }
    ///Bit 3 - GPIOD clock enable Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn gpioden(&mut self) -> GPIODEN_W<3> {
        GPIODEN_W::new(self)
    }
    ///Bit 4 - GPIOE clock enable Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn gpioeen(&mut self) -> GPIOEEN_W<4> {
        GPIOEEN_W::new(self)
    }
    ///Bit 5 - GPIOF clock enable Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn gpiofen(&mut self) -> GPIOFEN_W<5> {
        GPIOFEN_W::new(self)
    }
    ///Bit 6 - GPIOG clock enable Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn gpiogen(&mut self) -> GPIOGEN_W<6> {
        GPIOGEN_W::new(self)
    }
    ///Bit 7 - GPIOH clock enable Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn gpiohen(&mut self) -> GPIOHEN_W<7> {
        GPIOHEN_W::new(self)
    }
    ///Bit 8 - GPIOI clock enable Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn gpioien(&mut self) -> GPIOIEN_W<8> {
        GPIOIEN_W::new(self)
    }
    ///Bit 10 - ADC1 and 2 peripherals clock enabled Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn adc12en(&mut self) -> ADC12EN_W<10> {
        ADC12EN_W::new(self)
    }
    ///Bit 11 - DAC clock enable Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn dac12en(&mut self) -> DAC12EN_W<11> {
        DAC12EN_W::new(self)
    }
    ///Bit 12 - digital camera interface clock enable (DCMI or PSSI depending which interface is active) Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn dcmi_pssien(&mut self) -> DCMI_PSSIEN_W<12> {
        DCMI_PSSIEN_W::new(self)
    }
    ///Bit 16 - AES clock enable Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn aesen(&mut self) -> AESEN_W<16> {
        AESEN_W::new(self)
    }
    ///Bit 17 - HASH clock enable Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn hashen(&mut self) -> HASHEN_W<17> {
        HASHEN_W::new(self)
    }
    ///Bit 18 - RNG clock enable Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn rngen(&mut self) -> RNGEN_W<18> {
        RNGEN_W::new(self)
    }
    ///Bit 19 - PKA clock enable Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn pkaen(&mut self) -> PKAEN_W<19> {
        PKAEN_W::new(self)
    }
    ///Bit 20 - SAES clock enable Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn saesen(&mut self) -> SAESEN_W<20> {
        SAESEN_W::new(self)
    }
    ///Bit 30 - SRAM3 clock enable Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn sram3en(&mut self) -> SRAM3EN_W<30> {
        SRAM3EN_W::new(self)
    }
    ///Bit 31 - SRAM2 clock enable Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn sram2en(&mut self) -> SRAM2EN_W<31> {
        SRAM2EN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RCC AHB2 peripheral clock register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ahb2enr](index.html) module
pub struct AHB2ENR_SPEC;
impl crate::RegisterSpec for AHB2ENR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ahb2enr::R](R) reader structure
impl crate::Readable for AHB2ENR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ahb2enr::W](W) writer structure
impl crate::Writable for AHB2ENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets AHB2ENR to value 0xc000_0000
impl crate::Resettable for AHB2ENR_SPEC {
    const RESET_VALUE: Self::Ux = 0xc000_0000;
}
