///Register `RCC_AHB3RSTR` reader
pub struct R(crate::R<RCC_AHB3RSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_AHB3RSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_AHB3RSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_AHB3RSTR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RCC_AHB3RSTR` writer
pub struct W(crate::W<RCC_AHB3RSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_AHB3RSTR_SPEC>;
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
impl From<crate::W<RCC_AHB3RSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_AHB3RSTR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `LPGPIO1RST` reader - LPGPIO1 reset Set and cleared by software.
pub type LPGPIO1RST_R = crate::BitReader<bool>;
///Field `LPGPIO1RST` writer - LPGPIO1 reset Set and cleared by software.
pub type LPGPIO1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB3RSTR_SPEC, bool, O>;
///Field `ADC4RST` reader - ADC4 reset Set and cleared by software.
pub type ADC4RST_R = crate::BitReader<bool>;
///Field `ADC4RST` writer - ADC4 reset Set and cleared by software.
pub type ADC4RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB3RSTR_SPEC, bool, O>;
///Field `DAC1RST` reader - DAC1 reset Set and cleared by software.
pub type DAC1RST_R = crate::BitReader<bool>;
///Field `DAC1RST` writer - DAC1 reset Set and cleared by software.
pub type DAC1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB3RSTR_SPEC, bool, O>;
///Field `LPDMA1RST` reader - LPDMA1 reset Set and cleared by software.
pub type LPDMA1RST_R = crate::BitReader<bool>;
///Field `LPDMA1RST` writer - LPDMA1 reset Set and cleared by software.
pub type LPDMA1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB3RSTR_SPEC, bool, O>;
///Field `ADF1RST` reader - ADF1 reset Set and cleared by software.
pub type ADF1RST_R = crate::BitReader<bool>;
///Field `ADF1RST` writer - ADF1 reset Set and cleared by software.
pub type ADF1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB3RSTR_SPEC, bool, O>;
impl R {
    ///Bit 0 - LPGPIO1 reset Set and cleared by software.
    #[inline(always)]
    pub fn lpgpio1rst(&self) -> LPGPIO1RST_R {
        LPGPIO1RST_R::new((self.bits & 1) != 0)
    }
    ///Bit 5 - ADC4 reset Set and cleared by software.
    #[inline(always)]
    pub fn adc4rst(&self) -> ADC4RST_R {
        ADC4RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - DAC1 reset Set and cleared by software.
    #[inline(always)]
    pub fn dac1rst(&self) -> DAC1RST_R {
        DAC1RST_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 9 - LPDMA1 reset Set and cleared by software.
    #[inline(always)]
    pub fn lpdma1rst(&self) -> LPDMA1RST_R {
        LPDMA1RST_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - ADF1 reset Set and cleared by software.
    #[inline(always)]
    pub fn adf1rst(&self) -> ADF1RST_R {
        ADF1RST_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - LPGPIO1 reset Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn lpgpio1rst(&mut self) -> LPGPIO1RST_W<0> {
        LPGPIO1RST_W::new(self)
    }
    ///Bit 5 - ADC4 reset Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn adc4rst(&mut self) -> ADC4RST_W<5> {
        ADC4RST_W::new(self)
    }
    ///Bit 6 - DAC1 reset Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn dac1rst(&mut self) -> DAC1RST_W<6> {
        DAC1RST_W::new(self)
    }
    ///Bit 9 - LPDMA1 reset Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn lpdma1rst(&mut self) -> LPDMA1RST_W<9> {
        LPDMA1RST_W::new(self)
    }
    ///Bit 10 - ADF1 reset Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn adf1rst(&mut self) -> ADF1RST_W<10> {
        ADF1RST_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RCC AHB3 peripheral reset register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rcc_ahb3rstr](index.html) module
pub struct RCC_AHB3RSTR_SPEC;
impl crate::RegisterSpec for RCC_AHB3RSTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [rcc_ahb3rstr::R](R) reader structure
impl crate::Readable for RCC_AHB3RSTR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rcc_ahb3rstr::W](W) writer structure
impl crate::Writable for RCC_AHB3RSTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RCC_AHB3RSTR to value 0
impl crate::Resettable for RCC_AHB3RSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
