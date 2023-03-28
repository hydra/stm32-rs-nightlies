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
///Field `GPIOERST` reader - GPIOE block reset Set and reset by software.
pub type GPIOERST_R = crate::BitReader<bool>;
///Field `GPIOERST` writer - GPIOE block reset Set and reset by software.
pub type GPIOERST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2RSTR_SPEC, bool, O>;
///Field `GPIOFRST` reader - GPIOF block reset Set and reset by software.
pub type GPIOFRST_R = crate::BitReader<bool>;
///Field `GPIOFRST` writer - GPIOF block reset Set and reset by software.
pub type GPIOFRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2RSTR_SPEC, bool, O>;
///Field `GPIOGRST` reader - GPIOG block reset Set and reset by software.
pub type GPIOGRST_R = crate::BitReader<bool>;
///Field `GPIOGRST` writer - GPIOG block reset Set and reset by software.
pub type GPIOGRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2RSTR_SPEC, bool, O>;
///Field `GPIOHRST` reader - GPIOH block reset Set and reset by software.
pub type GPIOHRST_R = crate::BitReader<bool>;
///Field `GPIOHRST` writer - GPIOH block reset Set and reset by software.
pub type GPIOHRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2RSTR_SPEC, bool, O>;
///Field `GPIOIRST` reader - GPIOI block reset Set and reset by software.
pub type GPIOIRST_R = crate::BitReader<bool>;
///Field `GPIOIRST` writer - GPIOI block reset Set and reset by software.
pub type GPIOIRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2RSTR_SPEC, bool, O>;
///Field `ADC12RST` reader - ADC1 and 2 blocks reset Set and reset by software.
pub type ADC12RST_R = crate::BitReader<bool>;
///Field `ADC12RST` writer - ADC1 and 2 blocks reset Set and reset by software.
pub type ADC12RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2RSTR_SPEC, bool, O>;
///Field `DAC12RST` reader - DAC block reset Set and reset by software.
pub type DAC12RST_R = crate::BitReader<bool>;
///Field `DAC12RST` writer - DAC block reset Set and reset by software.
pub type DAC12RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2RSTR_SPEC, bool, O>;
///Field `DCMI_PSSIRST` reader - digital camera interface block reset (DCMI or PSSI depending which interface is active) Set and reset by software.
pub type DCMI_PSSIRST_R = crate::BitReader<bool>;
///Field `DCMI_PSSIRST` writer - digital camera interface block reset (DCMI or PSSI depending which interface is active) Set and reset by software.
pub type DCMI_PSSIRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2RSTR_SPEC, bool, O>;
///Field `AESRST` reader - AES block reset Set and reset by software.
pub type AESRST_R = crate::BitReader<bool>;
///Field `AESRST` writer - AES block reset Set and reset by software.
pub type AESRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2RSTR_SPEC, bool, O>;
///Field `HASHRST` reader - HASH block reset Set and reset by software.
pub type HASHRST_R = crate::BitReader<bool>;
///Field `HASHRST` writer - HASH block reset Set and reset by software.
pub type HASHRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2RSTR_SPEC, bool, O>;
///Field `RNGRST` reader - RNG block reset Set and reset by software.
pub type RNGRST_R = crate::BitReader<bool>;
///Field `RNGRST` writer - RNG block reset Set and reset by software.
pub type RNGRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2RSTR_SPEC, bool, O>;
///Field `PKARST` reader - PKA block reset Set and reset by software.
pub type PKARST_R = crate::BitReader<bool>;
///Field `PKARST` writer - PKA block reset Set and reset by software.
pub type PKARST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2RSTR_SPEC, bool, O>;
///Field `SAESRST` reader - SAES block reset Set and reset by software.
pub type SAESRST_R = crate::BitReader<bool>;
///Field `SAESRST` writer - SAES block reset Set and reset by software.
pub type SAESRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2RSTR_SPEC, bool, O>;
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
    ///Bit 4 - GPIOE block reset Set and reset by software.
    #[inline(always)]
    pub fn gpioerst(&self) -> GPIOERST_R {
        GPIOERST_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - GPIOF block reset Set and reset by software.
    #[inline(always)]
    pub fn gpiofrst(&self) -> GPIOFRST_R {
        GPIOFRST_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - GPIOG block reset Set and reset by software.
    #[inline(always)]
    pub fn gpiogrst(&self) -> GPIOGRST_R {
        GPIOGRST_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - GPIOH block reset Set and reset by software.
    #[inline(always)]
    pub fn gpiohrst(&self) -> GPIOHRST_R {
        GPIOHRST_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - GPIOI block reset Set and reset by software.
    #[inline(always)]
    pub fn gpioirst(&self) -> GPIOIRST_R {
        GPIOIRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 10 - ADC1 and 2 blocks reset Set and reset by software.
    #[inline(always)]
    pub fn adc12rst(&self) -> ADC12RST_R {
        ADC12RST_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - DAC block reset Set and reset by software.
    #[inline(always)]
    pub fn dac12rst(&self) -> DAC12RST_R {
        DAC12RST_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - digital camera interface block reset (DCMI or PSSI depending which interface is active) Set and reset by software.
    #[inline(always)]
    pub fn dcmi_pssirst(&self) -> DCMI_PSSIRST_R {
        DCMI_PSSIRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 16 - AES block reset Set and reset by software.
    #[inline(always)]
    pub fn aesrst(&self) -> AESRST_R {
        AESRST_R::new(((self.bits >> 16) & 1) != 0)
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
    ///Bit 19 - PKA block reset Set and reset by software.
    #[inline(always)]
    pub fn pkarst(&self) -> PKARST_R {
        PKARST_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - SAES block reset Set and reset by software.
    #[inline(always)]
    pub fn saesrst(&self) -> SAESRST_R {
        SAESRST_R::new(((self.bits >> 20) & 1) != 0)
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
    ///Bit 4 - GPIOE block reset Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn gpioerst(&mut self) -> GPIOERST_W<4> {
        GPIOERST_W::new(self)
    }
    ///Bit 5 - GPIOF block reset Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn gpiofrst(&mut self) -> GPIOFRST_W<5> {
        GPIOFRST_W::new(self)
    }
    ///Bit 6 - GPIOG block reset Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn gpiogrst(&mut self) -> GPIOGRST_W<6> {
        GPIOGRST_W::new(self)
    }
    ///Bit 7 - GPIOH block reset Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn gpiohrst(&mut self) -> GPIOHRST_W<7> {
        GPIOHRST_W::new(self)
    }
    ///Bit 8 - GPIOI block reset Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn gpioirst(&mut self) -> GPIOIRST_W<8> {
        GPIOIRST_W::new(self)
    }
    ///Bit 10 - ADC1 and 2 blocks reset Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn adc12rst(&mut self) -> ADC12RST_W<10> {
        ADC12RST_W::new(self)
    }
    ///Bit 11 - DAC block reset Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn dac12rst(&mut self) -> DAC12RST_W<11> {
        DAC12RST_W::new(self)
    }
    ///Bit 12 - digital camera interface block reset (DCMI or PSSI depending which interface is active) Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn dcmi_pssirst(&mut self) -> DCMI_PSSIRST_W<12> {
        DCMI_PSSIRST_W::new(self)
    }
    ///Bit 16 - AES block reset Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn aesrst(&mut self) -> AESRST_W<16> {
        AESRST_W::new(self)
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
    ///Bit 19 - PKA block reset Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn pkarst(&mut self) -> PKARST_W<19> {
        PKARST_W::new(self)
    }
    ///Bit 20 - SAES block reset Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn saesrst(&mut self) -> SAESRST_W<20> {
        SAESRST_W::new(self)
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
