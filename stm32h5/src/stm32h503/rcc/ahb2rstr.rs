///Register `AHB2RSTR` reader
pub struct R(crate::R<AHB2RSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB2RSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB2RSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB2RSTR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `AHB2RSTR` writer
pub struct W(crate::W<AHB2RSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB2RSTR_SPEC>;
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
impl From<crate::W<AHB2RSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB2RSTR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `GPIOARST` reader - GPIOA block reset Set and reset by software.
pub type GPIOARST_R = crate::BitReader<bool>;
///Field `GPIOARST` writer - GPIOA block reset Set and reset by software.
pub type GPIOARST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2RSTR_SPEC, bool, O>;
///Field `GPIOBRST` reader - GPIOB block reset Set and reset by software.
pub type GPIOBRST_R = crate::BitReader<bool>;
///Field `GPIOBRST` writer - GPIOB block reset Set and reset by software.
pub type GPIOBRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2RSTR_SPEC, bool, O>;
///Field `GPIOCRST` reader - GPIOC block reset Set and reset by software.
pub type GPIOCRST_R = crate::BitReader<bool>;
///Field `GPIOCRST` writer - GPIOC block reset Set and reset by software.
pub type GPIOCRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2RSTR_SPEC, bool, O>;
///Field `GPIODRST` reader - GPIOD block reset Set and reset by software.
pub type GPIODRST_R = crate::BitReader<bool>;
///Field `GPIODRST` writer - GPIOD block reset Set and reset by software.
pub type GPIODRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2RSTR_SPEC, bool, O>;
///Field `GPIOHRST` reader - GPIOH block reset Set and reset by software.
pub type GPIOHRST_R = crate::BitReader<bool>;
///Field `GPIOHRST` writer - GPIOH block reset Set and reset by software.
pub type GPIOHRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2RSTR_SPEC, bool, O>;
///Field `ADC1RST` reader - ADC1 block reset Set and reset by software.
pub type ADC1RST_R = crate::BitReader<bool>;
///Field `ADC1RST` writer - ADC1 block reset Set and reset by software.
pub type ADC1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2RSTR_SPEC, bool, O>;
///Field `DAC12RST` reader - DAC block reset Set and reset by software.
pub type DAC12RST_R = crate::BitReader<bool>;
///Field `DAC12RST` writer - DAC block reset Set and reset by software.
pub type DAC12RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2RSTR_SPEC, bool, O>;
///Field `HASHRST` reader - HASH block reset Set and reset by software.
pub type HASHRST_R = crate::BitReader<bool>;
///Field `HASHRST` writer - HASH block reset Set and reset by software.
pub type HASHRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2RSTR_SPEC, bool, O>;
///Field `RNGRST` reader - RNG block reset Set and reset by software.
pub type RNGRST_R = crate::BitReader<bool>;
///Field `RNGRST` writer - RNG block reset Set and reset by software.
pub type RNGRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2RSTR_SPEC, bool, O>;
impl R {
    ///Bit 0 - GPIOA block reset Set and reset by software.
    #[inline(always)]
    pub fn gpioarst(&self) -> GPIOARST_R {
        GPIOARST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - GPIOB block reset Set and reset by software.
    #[inline(always)]
    pub fn gpiobrst(&self) -> GPIOBRST_R {
        GPIOBRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - GPIOC block reset Set and reset by software.
    #[inline(always)]
    pub fn gpiocrst(&self) -> GPIOCRST_R {
        GPIOCRST_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - GPIOD block reset Set and reset by software.
    #[inline(always)]
    pub fn gpiodrst(&self) -> GPIODRST_R {
        GPIODRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 7 - GPIOH block reset Set and reset by software.
    #[inline(always)]
    pub fn gpiohrst(&self) -> GPIOHRST_R {
        GPIOHRST_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 10 - ADC1 block reset Set and reset by software.
    #[inline(always)]
    pub fn adc1rst(&self) -> ADC1RST_R {
        ADC1RST_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - DAC block reset Set and reset by software.
    #[inline(always)]
    pub fn dac12rst(&self) -> DAC12RST_R {
        DAC12RST_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 17 - HASH block reset Set and reset by software.
    #[inline(always)]
    pub fn hashrst(&self) -> HASHRST_R {
        HASHRST_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - RNG block reset Set and reset by software.
    #[inline(always)]
    pub fn rngrst(&self) -> RNGRST_R {
        RNGRST_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - GPIOA block reset Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn gpioarst(&mut self) -> GPIOARST_W<0> {
        GPIOARST_W::new(self)
    }
    ///Bit 1 - GPIOB block reset Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn gpiobrst(&mut self) -> GPIOBRST_W<1> {
        GPIOBRST_W::new(self)
    }
    ///Bit 2 - GPIOC block reset Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn gpiocrst(&mut self) -> GPIOCRST_W<2> {
        GPIOCRST_W::new(self)
    }
    ///Bit 3 - GPIOD block reset Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn gpiodrst(&mut self) -> GPIODRST_W<3> {
        GPIODRST_W::new(self)
    }
    ///Bit 7 - GPIOH block reset Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn gpiohrst(&mut self) -> GPIOHRST_W<7> {
        GPIOHRST_W::new(self)
    }
    ///Bit 10 - ADC1 block reset Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn adc1rst(&mut self) -> ADC1RST_W<10> {
        ADC1RST_W::new(self)
    }
    ///Bit 11 - DAC block reset Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn dac12rst(&mut self) -> DAC12RST_W<11> {
        DAC12RST_W::new(self)
    }
    ///Bit 17 - HASH block reset Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn hashrst(&mut self) -> HASHRST_W<17> {
        HASHRST_W::new(self)
    }
    ///Bit 18 - RNG block reset Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn rngrst(&mut self) -> RNGRST_W<18> {
        RNGRST_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RCC AHB2 peripheral reset register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ahb2rstr](index.html) module
pub struct AHB2RSTR_SPEC;
impl crate::RegisterSpec for AHB2RSTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ahb2rstr::R](R) reader structure
impl crate::Readable for AHB2RSTR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ahb2rstr::W](W) writer structure
impl crate::Writable for AHB2RSTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets AHB2RSTR to value 0
impl crate::Resettable for AHB2RSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
