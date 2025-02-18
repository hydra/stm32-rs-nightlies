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
///Field `GPIOAEN` reader - IO port A clock enable
pub type GPIOAEN_R = crate::BitReader<GPIOAEN_A>;
///IO port A clock enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOAEN_A {
    ///0: The selected clock is disabled
    Disabled = 0,
    ///1: The selected clock is enabled
    Enabled = 1,
}
impl From<GPIOAEN_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOAEN_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIOAEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> GPIOAEN_A {
        match self.bits {
            false => GPIOAEN_A::Disabled,
            true => GPIOAEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIOAEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIOAEN_A::Enabled
    }
}
///Field `GPIOAEN` writer - IO port A clock enable
pub type GPIOAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2ENR_SPEC, GPIOAEN_A, O>;
impl<'a, const O: u8> GPIOAEN_W<'a, O> {
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIOAEN_A::Disabled)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIOAEN_A::Enabled)
    }
}
///Field `GPIOBEN` reader - IO port B clock enable
pub use GPIOAEN_R as GPIOBEN_R;
///Field `GPIOCEN` reader - IO port C clock enable
pub use GPIOAEN_R as GPIOCEN_R;
///Field `GPIODEN` reader - IO port D clock enable
pub use GPIOAEN_R as GPIODEN_R;
///Field `GPIOEEN` reader - IO port E clock enable
pub use GPIOAEN_R as GPIOEEN_R;
///Field `GPIOFEN` reader - IO port F clock enable
pub use GPIOAEN_R as GPIOFEN_R;
///Field `GPIOGEN` reader - IO port G clock enable
pub use GPIOAEN_R as GPIOGEN_R;
///Field `ADC12EN` reader - ADC clock enable
pub use GPIOAEN_R as ADC12EN_R;
///Field `ADC345EN` reader - DCMI clock enable
pub use GPIOAEN_R as ADC345EN_R;
///Field `DAC1EN` reader - AES accelerator clock enable
pub use GPIOAEN_R as DAC1EN_R;
///Field `DAC2EN` reader - HASH clock enable
pub use GPIOAEN_R as DAC2EN_R;
///Field `DAC3EN` reader - Random Number Generator clock enable
pub use GPIOAEN_R as DAC3EN_R;
///Field `DAC4EN` reader - DAC4 clock enable
pub use GPIOAEN_R as DAC4EN_R;
///Field `AESEN` reader - AES clock enable
pub use GPIOAEN_R as AESEN_R;
///Field `RNGEN` reader - Random Number Generator clock enable
pub use GPIOAEN_R as RNGEN_R;
///Field `GPIOBEN` writer - IO port B clock enable
pub use GPIOAEN_W as GPIOBEN_W;
///Field `GPIOCEN` writer - IO port C clock enable
pub use GPIOAEN_W as GPIOCEN_W;
///Field `GPIODEN` writer - IO port D clock enable
pub use GPIOAEN_W as GPIODEN_W;
///Field `GPIOEEN` writer - IO port E clock enable
pub use GPIOAEN_W as GPIOEEN_W;
///Field `GPIOFEN` writer - IO port F clock enable
pub use GPIOAEN_W as GPIOFEN_W;
///Field `GPIOGEN` writer - IO port G clock enable
pub use GPIOAEN_W as GPIOGEN_W;
///Field `ADC12EN` writer - ADC clock enable
pub use GPIOAEN_W as ADC12EN_W;
///Field `ADC345EN` writer - DCMI clock enable
pub use GPIOAEN_W as ADC345EN_W;
///Field `DAC1EN` writer - AES accelerator clock enable
pub use GPIOAEN_W as DAC1EN_W;
///Field `DAC2EN` writer - HASH clock enable
pub use GPIOAEN_W as DAC2EN_W;
///Field `DAC3EN` writer - Random Number Generator clock enable
pub use GPIOAEN_W as DAC3EN_W;
///Field `DAC4EN` writer - DAC4 clock enable
pub use GPIOAEN_W as DAC4EN_W;
///Field `AESEN` writer - AES clock enable
pub use GPIOAEN_W as AESEN_W;
///Field `RNGEN` writer - Random Number Generator clock enable
pub use GPIOAEN_W as RNGEN_W;
impl R {
    ///Bit 0 - IO port A clock enable
    #[inline(always)]
    pub fn gpioaen(&self) -> GPIOAEN_R {
        GPIOAEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - IO port B clock enable
    #[inline(always)]
    pub fn gpioben(&self) -> GPIOBEN_R {
        GPIOBEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - IO port C clock enable
    #[inline(always)]
    pub fn gpiocen(&self) -> GPIOCEN_R {
        GPIOCEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - IO port D clock enable
    #[inline(always)]
    pub fn gpioden(&self) -> GPIODEN_R {
        GPIODEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - IO port E clock enable
    #[inline(always)]
    pub fn gpioeen(&self) -> GPIOEEN_R {
        GPIOEEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - IO port F clock enable
    #[inline(always)]
    pub fn gpiofen(&self) -> GPIOFEN_R {
        GPIOFEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - IO port G clock enable
    #[inline(always)]
    pub fn gpiogen(&self) -> GPIOGEN_R {
        GPIOGEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 13 - ADC clock enable
    #[inline(always)]
    pub fn adc12en(&self) -> ADC12EN_R {
        ADC12EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - DCMI clock enable
    #[inline(always)]
    pub fn adc345en(&self) -> ADC345EN_R {
        ADC345EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - AES accelerator clock enable
    #[inline(always)]
    pub fn dac1en(&self) -> DAC1EN_R {
        DAC1EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - HASH clock enable
    #[inline(always)]
    pub fn dac2en(&self) -> DAC2EN_R {
        DAC2EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Random Number Generator clock enable
    #[inline(always)]
    pub fn dac3en(&self) -> DAC3EN_R {
        DAC3EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - DAC4 clock enable
    #[inline(always)]
    pub fn dac4en(&self) -> DAC4EN_R {
        DAC4EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 24 - AES clock enable
    #[inline(always)]
    pub fn aesen(&self) -> AESEN_R {
        AESEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 26 - Random Number Generator clock enable
    #[inline(always)]
    pub fn rngen(&self) -> RNGEN_R {
        RNGEN_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - IO port A clock enable
    #[inline(always)]
    #[must_use]
    pub fn gpioaen(&mut self) -> GPIOAEN_W<0> {
        GPIOAEN_W::new(self)
    }
    ///Bit 1 - IO port B clock enable
    #[inline(always)]
    #[must_use]
    pub fn gpioben(&mut self) -> GPIOBEN_W<1> {
        GPIOBEN_W::new(self)
    }
    ///Bit 2 - IO port C clock enable
    #[inline(always)]
    #[must_use]
    pub fn gpiocen(&mut self) -> GPIOCEN_W<2> {
        GPIOCEN_W::new(self)
    }
    ///Bit 3 - IO port D clock enable
    #[inline(always)]
    #[must_use]
    pub fn gpioden(&mut self) -> GPIODEN_W<3> {
        GPIODEN_W::new(self)
    }
    ///Bit 4 - IO port E clock enable
    #[inline(always)]
    #[must_use]
    pub fn gpioeen(&mut self) -> GPIOEEN_W<4> {
        GPIOEEN_W::new(self)
    }
    ///Bit 5 - IO port F clock enable
    #[inline(always)]
    #[must_use]
    pub fn gpiofen(&mut self) -> GPIOFEN_W<5> {
        GPIOFEN_W::new(self)
    }
    ///Bit 6 - IO port G clock enable
    #[inline(always)]
    #[must_use]
    pub fn gpiogen(&mut self) -> GPIOGEN_W<6> {
        GPIOGEN_W::new(self)
    }
    ///Bit 13 - ADC clock enable
    #[inline(always)]
    #[must_use]
    pub fn adc12en(&mut self) -> ADC12EN_W<13> {
        ADC12EN_W::new(self)
    }
    ///Bit 14 - DCMI clock enable
    #[inline(always)]
    #[must_use]
    pub fn adc345en(&mut self) -> ADC345EN_W<14> {
        ADC345EN_W::new(self)
    }
    ///Bit 16 - AES accelerator clock enable
    #[inline(always)]
    #[must_use]
    pub fn dac1en(&mut self) -> DAC1EN_W<16> {
        DAC1EN_W::new(self)
    }
    ///Bit 17 - HASH clock enable
    #[inline(always)]
    #[must_use]
    pub fn dac2en(&mut self) -> DAC2EN_W<17> {
        DAC2EN_W::new(self)
    }
    ///Bit 18 - Random Number Generator clock enable
    #[inline(always)]
    #[must_use]
    pub fn dac3en(&mut self) -> DAC3EN_W<18> {
        DAC3EN_W::new(self)
    }
    ///Bit 19 - DAC4 clock enable
    #[inline(always)]
    #[must_use]
    pub fn dac4en(&mut self) -> DAC4EN_W<19> {
        DAC4EN_W::new(self)
    }
    ///Bit 24 - AES clock enable
    #[inline(always)]
    #[must_use]
    pub fn aesen(&mut self) -> AESEN_W<24> {
        AESEN_W::new(self)
    }
    ///Bit 26 - Random Number Generator clock enable
    #[inline(always)]
    #[must_use]
    pub fn rngen(&mut self) -> RNGEN_W<26> {
        RNGEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///AHB2 peripheral clock enable register
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
///`reset()` method sets AHB2ENR to value 0
impl crate::Resettable for AHB2ENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
