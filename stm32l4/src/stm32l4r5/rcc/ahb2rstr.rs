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
///Field `GPIOARST` reader - IO port A reset
pub type GPIOARST_R = crate::BitReader<bool>;
///Field `GPIOARST` writer - IO port A reset
pub type GPIOARST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2RSTR_SPEC, bool, O>;
///Field `GPIOBRST` reader - IO port B reset
pub type GPIOBRST_R = crate::BitReader<bool>;
///Field `GPIOBRST` writer - IO port B reset
pub type GPIOBRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2RSTR_SPEC, bool, O>;
///Field `GPIOCRST` reader - IO port C reset
pub type GPIOCRST_R = crate::BitReader<bool>;
///Field `GPIOCRST` writer - IO port C reset
pub type GPIOCRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2RSTR_SPEC, bool, O>;
///Field `GPIODRST` reader - IO port D reset
pub type GPIODRST_R = crate::BitReader<bool>;
///Field `GPIODRST` writer - IO port D reset
pub type GPIODRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2RSTR_SPEC, bool, O>;
///Field `GPIOERST` reader - IO port E reset
pub type GPIOERST_R = crate::BitReader<bool>;
///Field `GPIOERST` writer - IO port E reset
pub type GPIOERST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2RSTR_SPEC, bool, O>;
///Field `GPIOFRST` reader - IO port F reset
pub type GPIOFRST_R = crate::BitReader<bool>;
///Field `GPIOFRST` writer - IO port F reset
pub type GPIOFRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2RSTR_SPEC, bool, O>;
///Field `GPIOGRST` reader - IO port G reset
pub type GPIOGRST_R = crate::BitReader<bool>;
///Field `GPIOGRST` writer - IO port G reset
pub type GPIOGRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2RSTR_SPEC, bool, O>;
///Field `GPIOHRST` reader - IO port H reset
pub type GPIOHRST_R = crate::BitReader<bool>;
///Field `GPIOHRST` writer - IO port H reset
pub type GPIOHRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2RSTR_SPEC, bool, O>;
///Field `GPIOIRST` reader - IO port I reset
pub type GPIOIRST_R = crate::BitReader<bool>;
///Field `GPIOIRST` writer - IO port I reset
pub type GPIOIRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2RSTR_SPEC, bool, O>;
///Field `OTGFSRST` reader - USB OTG FS reset
pub type OTGFSRST_R = crate::BitReader<bool>;
///Field `OTGFSRST` writer - USB OTG FS reset
pub type OTGFSRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2RSTR_SPEC, bool, O>;
///Field `ADCRST` reader - ADC reset
pub type ADCRST_R = crate::BitReader<bool>;
///Field `ADCRST` writer - ADC reset
pub type ADCRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2RSTR_SPEC, bool, O>;
///Field `DCMIRST` reader - Digital Camera Interface reset
pub type DCMIRST_R = crate::BitReader<bool>;
///Field `DCMIRST` writer - Digital Camera Interface reset
pub type DCMIRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2RSTR_SPEC, bool, O>;
///Field `AESRST` reader - AES hardware accelerator reset
pub type AESRST_R = crate::BitReader<bool>;
///Field `AESRST` writer - AES hardware accelerator reset
pub type AESRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2RSTR_SPEC, bool, O>;
///Field `HASHRST` reader - Hash reset
pub type HASHRST_R = crate::BitReader<bool>;
///Field `HASHRST` writer - Hash reset
pub type HASHRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2RSTR_SPEC, bool, O>;
///Field `RNGRST` reader - Random number generator reset
pub type RNGRST_R = crate::BitReader<bool>;
///Field `RNGRST` writer - Random number generator reset
pub type RNGRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2RSTR_SPEC, bool, O>;
///Field `OSPIMRST` reader - OCTOSPI IO manager reset
pub type OSPIMRST_R = crate::BitReader<bool>;
///Field `OSPIMRST` writer - OCTOSPI IO manager reset
pub type OSPIMRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2RSTR_SPEC, bool, O>;
///Field `SDMMC1RST` reader - SDMMC1 reset
pub type SDMMC1RST_R = crate::BitReader<bool>;
///Field `SDMMC1RST` writer - SDMMC1 reset
pub type SDMMC1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2RSTR_SPEC, bool, O>;
impl R {
    ///Bit 0 - IO port A reset
    #[inline(always)]
    pub fn gpioarst(&self) -> GPIOARST_R {
        GPIOARST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - IO port B reset
    #[inline(always)]
    pub fn gpiobrst(&self) -> GPIOBRST_R {
        GPIOBRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - IO port C reset
    #[inline(always)]
    pub fn gpiocrst(&self) -> GPIOCRST_R {
        GPIOCRST_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - IO port D reset
    #[inline(always)]
    pub fn gpiodrst(&self) -> GPIODRST_R {
        GPIODRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - IO port E reset
    #[inline(always)]
    pub fn gpioerst(&self) -> GPIOERST_R {
        GPIOERST_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - IO port F reset
    #[inline(always)]
    pub fn gpiofrst(&self) -> GPIOFRST_R {
        GPIOFRST_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - IO port G reset
    #[inline(always)]
    pub fn gpiogrst(&self) -> GPIOGRST_R {
        GPIOGRST_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - IO port H reset
    #[inline(always)]
    pub fn gpiohrst(&self) -> GPIOHRST_R {
        GPIOHRST_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - IO port I reset
    #[inline(always)]
    pub fn gpioirst(&self) -> GPIOIRST_R {
        GPIOIRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 12 - USB OTG FS reset
    #[inline(always)]
    pub fn otgfsrst(&self) -> OTGFSRST_R {
        OTGFSRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - ADC reset
    #[inline(always)]
    pub fn adcrst(&self) -> ADCRST_R {
        ADCRST_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Digital Camera Interface reset
    #[inline(always)]
    pub fn dcmirst(&self) -> DCMIRST_R {
        DCMIRST_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - AES hardware accelerator reset
    #[inline(always)]
    pub fn aesrst(&self) -> AESRST_R {
        AESRST_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Hash reset
    #[inline(always)]
    pub fn hashrst(&self) -> HASHRST_R {
        HASHRST_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Random number generator reset
    #[inline(always)]
    pub fn rngrst(&self) -> RNGRST_R {
        RNGRST_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 20 - OCTOSPI IO manager reset
    #[inline(always)]
    pub fn ospimrst(&self) -> OSPIMRST_R {
        OSPIMRST_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 22 - SDMMC1 reset
    #[inline(always)]
    pub fn sdmmc1rst(&self) -> SDMMC1RST_R {
        SDMMC1RST_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - IO port A reset
    #[inline(always)]
    #[must_use]
    pub fn gpioarst(&mut self) -> GPIOARST_W<0> {
        GPIOARST_W::new(self)
    }
    ///Bit 1 - IO port B reset
    #[inline(always)]
    #[must_use]
    pub fn gpiobrst(&mut self) -> GPIOBRST_W<1> {
        GPIOBRST_W::new(self)
    }
    ///Bit 2 - IO port C reset
    #[inline(always)]
    #[must_use]
    pub fn gpiocrst(&mut self) -> GPIOCRST_W<2> {
        GPIOCRST_W::new(self)
    }
    ///Bit 3 - IO port D reset
    #[inline(always)]
    #[must_use]
    pub fn gpiodrst(&mut self) -> GPIODRST_W<3> {
        GPIODRST_W::new(self)
    }
    ///Bit 4 - IO port E reset
    #[inline(always)]
    #[must_use]
    pub fn gpioerst(&mut self) -> GPIOERST_W<4> {
        GPIOERST_W::new(self)
    }
    ///Bit 5 - IO port F reset
    #[inline(always)]
    #[must_use]
    pub fn gpiofrst(&mut self) -> GPIOFRST_W<5> {
        GPIOFRST_W::new(self)
    }
    ///Bit 6 - IO port G reset
    #[inline(always)]
    #[must_use]
    pub fn gpiogrst(&mut self) -> GPIOGRST_W<6> {
        GPIOGRST_W::new(self)
    }
    ///Bit 7 - IO port H reset
    #[inline(always)]
    #[must_use]
    pub fn gpiohrst(&mut self) -> GPIOHRST_W<7> {
        GPIOHRST_W::new(self)
    }
    ///Bit 8 - IO port I reset
    #[inline(always)]
    #[must_use]
    pub fn gpioirst(&mut self) -> GPIOIRST_W<8> {
        GPIOIRST_W::new(self)
    }
    ///Bit 12 - USB OTG FS reset
    #[inline(always)]
    #[must_use]
    pub fn otgfsrst(&mut self) -> OTGFSRST_W<12> {
        OTGFSRST_W::new(self)
    }
    ///Bit 13 - ADC reset
    #[inline(always)]
    #[must_use]
    pub fn adcrst(&mut self) -> ADCRST_W<13> {
        ADCRST_W::new(self)
    }
    ///Bit 14 - Digital Camera Interface reset
    #[inline(always)]
    #[must_use]
    pub fn dcmirst(&mut self) -> DCMIRST_W<14> {
        DCMIRST_W::new(self)
    }
    ///Bit 16 - AES hardware accelerator reset
    #[inline(always)]
    #[must_use]
    pub fn aesrst(&mut self) -> AESRST_W<16> {
        AESRST_W::new(self)
    }
    ///Bit 17 - Hash reset
    #[inline(always)]
    #[must_use]
    pub fn hashrst(&mut self) -> HASHRST_W<17> {
        HASHRST_W::new(self)
    }
    ///Bit 18 - Random number generator reset
    #[inline(always)]
    #[must_use]
    pub fn rngrst(&mut self) -> RNGRST_W<18> {
        RNGRST_W::new(self)
    }
    ///Bit 20 - OCTOSPI IO manager reset
    #[inline(always)]
    #[must_use]
    pub fn ospimrst(&mut self) -> OSPIMRST_W<20> {
        OSPIMRST_W::new(self)
    }
    ///Bit 22 - SDMMC1 reset
    #[inline(always)]
    #[must_use]
    pub fn sdmmc1rst(&mut self) -> SDMMC1RST_W<22> {
        SDMMC1RST_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///AHB2 peripheral reset register
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
