///Register `RCC_AHB2RSTR1` reader
pub struct R(crate::R<RCC_AHB2RSTR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_AHB2RSTR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_AHB2RSTR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_AHB2RSTR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RCC_AHB2RSTR1` writer
pub struct W(crate::W<RCC_AHB2RSTR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_AHB2RSTR1_SPEC>;
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
impl From<crate::W<RCC_AHB2RSTR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_AHB2RSTR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `GPIOARST` reader - IO port A reset Set and cleared by software.
pub type GPIOARST_R = crate::BitReader<bool>;
///Field `GPIOARST` writer - IO port A reset Set and cleared by software.
pub type GPIOARST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB2RSTR1_SPEC, bool, O>;
///Field `GPIOBRST` reader - IO port B reset Set and cleared by software.
pub type GPIOBRST_R = crate::BitReader<bool>;
///Field `GPIOBRST` writer - IO port B reset Set and cleared by software.
pub type GPIOBRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB2RSTR1_SPEC, bool, O>;
///Field `GPIOCRST` reader - IO port C reset Set and cleared by software.
pub type GPIOCRST_R = crate::BitReader<bool>;
///Field `GPIOCRST` writer - IO port C reset Set and cleared by software.
pub type GPIOCRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB2RSTR1_SPEC, bool, O>;
///Field `GPIODRST` reader - IO port D reset Set and cleared by software.
pub type GPIODRST_R = crate::BitReader<bool>;
///Field `GPIODRST` writer - IO port D reset Set and cleared by software.
pub type GPIODRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB2RSTR1_SPEC, bool, O>;
///Field `GPIOERST` reader - IO port E reset Set and cleared by software.
pub type GPIOERST_R = crate::BitReader<bool>;
///Field `GPIOERST` writer - IO port E reset Set and cleared by software.
pub type GPIOERST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB2RSTR1_SPEC, bool, O>;
///Field `GPIOFRST` reader - IO port F reset Set and cleared by software.
pub type GPIOFRST_R = crate::BitReader<bool>;
///Field `GPIOFRST` writer - IO port F reset Set and cleared by software.
pub type GPIOFRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB2RSTR1_SPEC, bool, O>;
///Field `GPIOGRST` reader - IO port G reset Set and cleared by software.
pub type GPIOGRST_R = crate::BitReader<bool>;
///Field `GPIOGRST` writer - IO port G reset Set and cleared by software.
pub type GPIOGRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB2RSTR1_SPEC, bool, O>;
///Field `GPIOHRST` reader - IO port H reset Set and cleared by software.
pub type GPIOHRST_R = crate::BitReader<bool>;
///Field `GPIOHRST` writer - IO port H reset Set and cleared by software.
pub type GPIOHRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB2RSTR1_SPEC, bool, O>;
///Field `GPIOIRST` reader - IO port I reset Set and cleared by software.
pub type GPIOIRST_R = crate::BitReader<bool>;
///Field `GPIOIRST` writer - IO port I reset Set and cleared by software.
pub type GPIOIRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB2RSTR1_SPEC, bool, O>;
///Field `ADC1RST` reader - ADC1 reset Set and cleared by software.
pub type ADC1RST_R = crate::BitReader<bool>;
///Field `ADC1RST` writer - ADC1 reset Set and cleared by software.
pub type ADC1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB2RSTR1_SPEC, bool, O>;
///Field `DCMI_PSSIRST` reader - DCMI and PSSI reset Set and cleared by software.
pub type DCMI_PSSIRST_R = crate::BitReader<bool>;
///Field `DCMI_PSSIRST` writer - DCMI and PSSI reset Set and cleared by software.
pub type DCMI_PSSIRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB2RSTR1_SPEC, bool, O>;
///Field `OTGRST` reader - OTG_FS reset Set and cleared by software.
pub type OTGRST_R = crate::BitReader<bool>;
///Field `OTGRST` writer - OTG_FS reset Set and cleared by software.
pub type OTGRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB2RSTR1_SPEC, bool, O>;
///Field `AESRST` reader - AES hardware accelerator reset Set and cleared by software.
pub type AESRST_R = crate::BitReader<bool>;
///Field `AESRST` writer - AES hardware accelerator reset Set and cleared by software.
pub type AESRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB2RSTR1_SPEC, bool, O>;
///Field `HASHRST` reader - Hash reset Set and cleared by software.
pub type HASHRST_R = crate::BitReader<bool>;
///Field `HASHRST` writer - Hash reset Set and cleared by software.
pub type HASHRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB2RSTR1_SPEC, bool, O>;
///Field `RNGRST` reader - Random number generator reset Set and cleared by software.
pub type RNGRST_R = crate::BitReader<bool>;
///Field `RNGRST` writer - Random number generator reset Set and cleared by software.
pub type RNGRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB2RSTR1_SPEC, bool, O>;
///Field `PKARST` reader - PKA reset Set and cleared by software.
pub type PKARST_R = crate::BitReader<bool>;
///Field `PKARST` writer - PKA reset Set and cleared by software.
pub type PKARST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB2RSTR1_SPEC, bool, O>;
///Field `SAESRST` reader - SAES hardware accelerator reset Set and cleared by software.
pub type SAESRST_R = crate::BitReader<bool>;
///Field `SAESRST` writer - SAES hardware accelerator reset Set and cleared by software.
pub type SAESRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB2RSTR1_SPEC, bool, O>;
///Field `OCTOSPIMRST` reader - OCTOSPIM reset Set and cleared by software.
pub type OCTOSPIMRST_R = crate::BitReader<bool>;
///Field `OCTOSPIMRST` writer - OCTOSPIM reset Set and cleared by software.
pub type OCTOSPIMRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB2RSTR1_SPEC, bool, O>;
///Field `OTFDEC1RST` reader - OTFDEC1 reset Set and cleared by software.
pub type OTFDEC1RST_R = crate::BitReader<bool>;
///Field `OTFDEC1RST` writer - OTFDEC1 reset Set and cleared by software.
pub type OTFDEC1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB2RSTR1_SPEC, bool, O>;
///Field `OTFDEC2RST` reader - OTFDEC2 reset Set and cleared by software.
pub type OTFDEC2RST_R = crate::BitReader<bool>;
///Field `OTFDEC2RST` writer - OTFDEC2 reset Set and cleared by software.
pub type OTFDEC2RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB2RSTR1_SPEC, bool, O>;
///Field `SDMMC1RST` reader - SDMMC1 reset Set and cleared by software.
pub type SDMMC1RST_R = crate::BitReader<bool>;
///Field `SDMMC1RST` writer - SDMMC1 reset Set and cleared by software.
pub type SDMMC1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB2RSTR1_SPEC, bool, O>;
///Field `SDMMC2RST` reader - SDMMC2 reset Set and cleared by software.
pub type SDMMC2RST_R = crate::BitReader<bool>;
///Field `SDMMC2RST` writer - SDMMC2 reset Set and cleared by software.
pub type SDMMC2RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB2RSTR1_SPEC, bool, O>;
impl R {
    ///Bit 0 - IO port A reset Set and cleared by software.
    #[inline(always)]
    pub fn gpioarst(&self) -> GPIOARST_R {
        GPIOARST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - IO port B reset Set and cleared by software.
    #[inline(always)]
    pub fn gpiobrst(&self) -> GPIOBRST_R {
        GPIOBRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - IO port C reset Set and cleared by software.
    #[inline(always)]
    pub fn gpiocrst(&self) -> GPIOCRST_R {
        GPIOCRST_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - IO port D reset Set and cleared by software.
    #[inline(always)]
    pub fn gpiodrst(&self) -> GPIODRST_R {
        GPIODRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - IO port E reset Set and cleared by software.
    #[inline(always)]
    pub fn gpioerst(&self) -> GPIOERST_R {
        GPIOERST_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - IO port F reset Set and cleared by software.
    #[inline(always)]
    pub fn gpiofrst(&self) -> GPIOFRST_R {
        GPIOFRST_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - IO port G reset Set and cleared by software.
    #[inline(always)]
    pub fn gpiogrst(&self) -> GPIOGRST_R {
        GPIOGRST_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - IO port H reset Set and cleared by software.
    #[inline(always)]
    pub fn gpiohrst(&self) -> GPIOHRST_R {
        GPIOHRST_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - IO port I reset Set and cleared by software.
    #[inline(always)]
    pub fn gpioirst(&self) -> GPIOIRST_R {
        GPIOIRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 10 - ADC1 reset Set and cleared by software.
    #[inline(always)]
    pub fn adc1rst(&self) -> ADC1RST_R {
        ADC1RST_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 12 - DCMI and PSSI reset Set and cleared by software.
    #[inline(always)]
    pub fn dcmi_pssirst(&self) -> DCMI_PSSIRST_R {
        DCMI_PSSIRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - OTG_FS reset Set and cleared by software.
    #[inline(always)]
    pub fn otgrst(&self) -> OTGRST_R {
        OTGRST_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - AES hardware accelerator reset Set and cleared by software.
    #[inline(always)]
    pub fn aesrst(&self) -> AESRST_R {
        AESRST_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Hash reset Set and cleared by software.
    #[inline(always)]
    pub fn hashrst(&self) -> HASHRST_R {
        HASHRST_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Random number generator reset Set and cleared by software.
    #[inline(always)]
    pub fn rngrst(&self) -> RNGRST_R {
        RNGRST_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - PKA reset Set and cleared by software.
    #[inline(always)]
    pub fn pkarst(&self) -> PKARST_R {
        PKARST_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - SAES hardware accelerator reset Set and cleared by software.
    #[inline(always)]
    pub fn saesrst(&self) -> SAESRST_R {
        SAESRST_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - OCTOSPIM reset Set and cleared by software.
    #[inline(always)]
    pub fn octospimrst(&self) -> OCTOSPIMRST_R {
        OCTOSPIMRST_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 23 - OTFDEC1 reset Set and cleared by software.
    #[inline(always)]
    pub fn otfdec1rst(&self) -> OTFDEC1RST_R {
        OTFDEC1RST_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - OTFDEC2 reset Set and cleared by software.
    #[inline(always)]
    pub fn otfdec2rst(&self) -> OTFDEC2RST_R {
        OTFDEC2RST_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 27 - SDMMC1 reset Set and cleared by software.
    #[inline(always)]
    pub fn sdmmc1rst(&self) -> SDMMC1RST_R {
        SDMMC1RST_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - SDMMC2 reset Set and cleared by software.
    #[inline(always)]
    pub fn sdmmc2rst(&self) -> SDMMC2RST_R {
        SDMMC2RST_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - IO port A reset Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn gpioarst(&mut self) -> GPIOARST_W<0> {
        GPIOARST_W::new(self)
    }
    ///Bit 1 - IO port B reset Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn gpiobrst(&mut self) -> GPIOBRST_W<1> {
        GPIOBRST_W::new(self)
    }
    ///Bit 2 - IO port C reset Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn gpiocrst(&mut self) -> GPIOCRST_W<2> {
        GPIOCRST_W::new(self)
    }
    ///Bit 3 - IO port D reset Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn gpiodrst(&mut self) -> GPIODRST_W<3> {
        GPIODRST_W::new(self)
    }
    ///Bit 4 - IO port E reset Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn gpioerst(&mut self) -> GPIOERST_W<4> {
        GPIOERST_W::new(self)
    }
    ///Bit 5 - IO port F reset Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn gpiofrst(&mut self) -> GPIOFRST_W<5> {
        GPIOFRST_W::new(self)
    }
    ///Bit 6 - IO port G reset Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn gpiogrst(&mut self) -> GPIOGRST_W<6> {
        GPIOGRST_W::new(self)
    }
    ///Bit 7 - IO port H reset Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn gpiohrst(&mut self) -> GPIOHRST_W<7> {
        GPIOHRST_W::new(self)
    }
    ///Bit 8 - IO port I reset Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn gpioirst(&mut self) -> GPIOIRST_W<8> {
        GPIOIRST_W::new(self)
    }
    ///Bit 10 - ADC1 reset Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn adc1rst(&mut self) -> ADC1RST_W<10> {
        ADC1RST_W::new(self)
    }
    ///Bit 12 - DCMI and PSSI reset Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn dcmi_pssirst(&mut self) -> DCMI_PSSIRST_W<12> {
        DCMI_PSSIRST_W::new(self)
    }
    ///Bit 14 - OTG_FS reset Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn otgrst(&mut self) -> OTGRST_W<14> {
        OTGRST_W::new(self)
    }
    ///Bit 16 - AES hardware accelerator reset Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn aesrst(&mut self) -> AESRST_W<16> {
        AESRST_W::new(self)
    }
    ///Bit 17 - Hash reset Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn hashrst(&mut self) -> HASHRST_W<17> {
        HASHRST_W::new(self)
    }
    ///Bit 18 - Random number generator reset Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn rngrst(&mut self) -> RNGRST_W<18> {
        RNGRST_W::new(self)
    }
    ///Bit 19 - PKA reset Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn pkarst(&mut self) -> PKARST_W<19> {
        PKARST_W::new(self)
    }
    ///Bit 20 - SAES hardware accelerator reset Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn saesrst(&mut self) -> SAESRST_W<20> {
        SAESRST_W::new(self)
    }
    ///Bit 21 - OCTOSPIM reset Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn octospimrst(&mut self) -> OCTOSPIMRST_W<21> {
        OCTOSPIMRST_W::new(self)
    }
    ///Bit 23 - OTFDEC1 reset Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn otfdec1rst(&mut self) -> OTFDEC1RST_W<23> {
        OTFDEC1RST_W::new(self)
    }
    ///Bit 24 - OTFDEC2 reset Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn otfdec2rst(&mut self) -> OTFDEC2RST_W<24> {
        OTFDEC2RST_W::new(self)
    }
    ///Bit 27 - SDMMC1 reset Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn sdmmc1rst(&mut self) -> SDMMC1RST_W<27> {
        SDMMC1RST_W::new(self)
    }
    ///Bit 28 - SDMMC2 reset Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn sdmmc2rst(&mut self) -> SDMMC2RST_W<28> {
        SDMMC2RST_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RCC AHB2 peripheral reset register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rcc_ahb2rstr1](index.html) module
pub struct RCC_AHB2RSTR1_SPEC;
impl crate::RegisterSpec for RCC_AHB2RSTR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [rcc_ahb2rstr1::R](R) reader structure
impl crate::Readable for RCC_AHB2RSTR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rcc_ahb2rstr1::W](W) writer structure
impl crate::Writable for RCC_AHB2RSTR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RCC_AHB2RSTR1 to value 0
impl crate::Resettable for RCC_AHB2RSTR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
