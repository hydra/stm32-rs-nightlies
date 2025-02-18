///Register `PLLSAI1CFGR` reader
pub struct R(crate::R<PLLSAI1CFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLLSAI1CFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLLSAI1CFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLLSAI1CFGR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PLLSAI1CFGR` writer
pub struct W(crate::W<PLLSAI1CFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLLSAI1CFGR_SPEC>;
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
impl From<crate::W<PLLSAI1CFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLLSAI1CFGR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PLLN` reader - SAIPLL multiplication factor for VCO
pub type PLLN_R = crate::FieldReader<u8, u8>;
///Field `PLLN` writer - SAIPLL multiplication factor for VCO
pub type PLLN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLLSAI1CFGR_SPEC, u8, u8, 7, O>;
///Field `PLLPEN` reader - SAIPLL PLLSAI1CLK output enable
pub type PLLPEN_R = crate::BitReader<bool>;
///Field `PLLPEN` writer - SAIPLL PLLSAI1CLK output enable
pub type PLLPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLLSAI1CFGR_SPEC, bool, O>;
///Field `PLLP` reader - SAI1PLL division factor P for PLLSAICLK (SAI1clock)
pub type PLLP_R = crate::FieldReader<u8, u8>;
///Field `PLLP` writer - SAI1PLL division factor P for PLLSAICLK (SAI1clock)
pub type PLLP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLLSAI1CFGR_SPEC, u8, u8, 5, O>;
///Field `PLLQEN` reader - SAIPLL PLLSAIUSBCLK output enable
pub type PLLQEN_R = crate::BitReader<bool>;
///Field `PLLQEN` writer - SAIPLL PLLSAIUSBCLK output enable
pub type PLLQEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLLSAI1CFGR_SPEC, bool, O>;
///Field `PLLQ` reader - SAIPLL division factor Q for PLLSAIUSBCLK (48 MHz clock)
pub type PLLQ_R = crate::FieldReader<u8, u8>;
///Field `PLLQ` writer - SAIPLL division factor Q for PLLSAIUSBCLK (48 MHz clock)
pub type PLLQ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLLSAI1CFGR_SPEC, u8, u8, 3, O>;
///Field `PLLREN` reader - PLLSAI PLLADC1CLK output enable
pub type PLLREN_R = crate::BitReader<bool>;
///Field `PLLREN` writer - PLLSAI PLLADC1CLK output enable
pub type PLLREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLLSAI1CFGR_SPEC, bool, O>;
///Field `PLLR` reader - PLLSAI division factor R for PLLADC1CLK (ADC clock)
pub type PLLR_R = crate::FieldReader<u8, u8>;
///Field `PLLR` writer - PLLSAI division factor R for PLLADC1CLK (ADC clock)
pub type PLLR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLLSAI1CFGR_SPEC, u8, u8, 3, O>;
impl R {
    ///Bits 8:14 - SAIPLL multiplication factor for VCO
    #[inline(always)]
    pub fn plln(&self) -> PLLN_R {
        PLLN_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    ///Bit 16 - SAIPLL PLLSAI1CLK output enable
    #[inline(always)]
    pub fn pllpen(&self) -> PLLPEN_R {
        PLLPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:21 - SAI1PLL division factor P for PLLSAICLK (SAI1clock)
    #[inline(always)]
    pub fn pllp(&self) -> PLLP_R {
        PLLP_R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    ///Bit 24 - SAIPLL PLLSAIUSBCLK output enable
    #[inline(always)]
    pub fn pllqen(&self) -> PLLQEN_R {
        PLLQEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bits 25:27 - SAIPLL division factor Q for PLLSAIUSBCLK (48 MHz clock)
    #[inline(always)]
    pub fn pllq(&self) -> PLLQ_R {
        PLLQ_R::new(((self.bits >> 25) & 7) as u8)
    }
    ///Bit 28 - PLLSAI PLLADC1CLK output enable
    #[inline(always)]
    pub fn pllren(&self) -> PLLREN_R {
        PLLREN_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bits 29:31 - PLLSAI division factor R for PLLADC1CLK (ADC clock)
    #[inline(always)]
    pub fn pllr(&self) -> PLLR_R {
        PLLR_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    ///Bits 8:14 - SAIPLL multiplication factor for VCO
    #[inline(always)]
    #[must_use]
    pub fn plln(&mut self) -> PLLN_W<8> {
        PLLN_W::new(self)
    }
    ///Bit 16 - SAIPLL PLLSAI1CLK output enable
    #[inline(always)]
    #[must_use]
    pub fn pllpen(&mut self) -> PLLPEN_W<16> {
        PLLPEN_W::new(self)
    }
    ///Bits 17:21 - SAI1PLL division factor P for PLLSAICLK (SAI1clock)
    #[inline(always)]
    #[must_use]
    pub fn pllp(&mut self) -> PLLP_W<17> {
        PLLP_W::new(self)
    }
    ///Bit 24 - SAIPLL PLLSAIUSBCLK output enable
    #[inline(always)]
    #[must_use]
    pub fn pllqen(&mut self) -> PLLQEN_W<24> {
        PLLQEN_W::new(self)
    }
    ///Bits 25:27 - SAIPLL division factor Q for PLLSAIUSBCLK (48 MHz clock)
    #[inline(always)]
    #[must_use]
    pub fn pllq(&mut self) -> PLLQ_W<25> {
        PLLQ_W::new(self)
    }
    ///Bit 28 - PLLSAI PLLADC1CLK output enable
    #[inline(always)]
    #[must_use]
    pub fn pllren(&mut self) -> PLLREN_W<28> {
        PLLREN_W::new(self)
    }
    ///Bits 29:31 - PLLSAI division factor R for PLLADC1CLK (ADC clock)
    #[inline(always)]
    #[must_use]
    pub fn pllr(&mut self) -> PLLR_W<29> {
        PLLR_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///PLLSAI1 configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pllsai1cfgr](index.html) module
pub struct PLLSAI1CFGR_SPEC;
impl crate::RegisterSpec for PLLSAI1CFGR_SPEC {
    type Ux = u32;
}
///`read()` method returns [pllsai1cfgr::R](R) reader structure
impl crate::Readable for PLLSAI1CFGR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [pllsai1cfgr::W](W) writer structure
impl crate::Writable for PLLSAI1CFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets PLLSAI1CFGR to value 0x2204_0100
impl crate::Resettable for PLLSAI1CFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0x2204_0100;
}
