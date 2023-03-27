///Register `PLLSYSCFGR` reader
pub struct R(crate::R<PLLSYSCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLLSYSCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLLSYSCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLLSYSCFGR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PLLSYSCFGR` writer
pub struct W(crate::W<PLLSYSCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLLSYSCFGR_SPEC>;
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
impl From<crate::W<PLLSYSCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLLSYSCFGR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PLLSRC` reader - PLL input clock source
pub type PLLSRC_R = crate::FieldReader<u8, u8>;
///Field `PLLSRC` writer - PLL input clock source
pub type PLLSRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLLSYSCFGR_SPEC, u8, u8, 2, O>;
///Field `PLLM` reader - Division factor M of the PLL input clock divider
pub type PLLM_R = crate::FieldReader<u8, u8>;
///Field `PLLM` writer - Division factor M of the PLL input clock divider
pub type PLLM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLLSYSCFGR_SPEC, u8, u8, 3, O>;
///Field `PLLN` reader - PLL frequency multiplication factor N
pub type PLLN_R = crate::FieldReader<u8, u8>;
///Field `PLLN` writer - PLL frequency multiplication factor N
pub type PLLN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLLSYSCFGR_SPEC, u8, u8, 7, O>;
///Field `PLLPEN` reader - PLLPCLK clock output enable
pub type PLLPEN_R = crate::BitReader<bool>;
///Field `PLLPEN` writer - PLLPCLK clock output enable
pub type PLLPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLLSYSCFGR_SPEC, bool, O>;
///Field `PLLP` reader - PLL VCO division factor P for PLLPCLK clock output
pub type PLLP_R = crate::FieldReader<u8, u8>;
///Field `PLLP` writer - PLL VCO division factor P for PLLPCLK clock output
pub type PLLP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLLSYSCFGR_SPEC, u8, u8, 5, O>;
///Field `PLLQEN` reader - PLLQCLK clock output enable
pub type PLLQEN_R = crate::BitReader<bool>;
///Field `PLLQEN` writer - PLLQCLK clock output enable
pub type PLLQEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLLSYSCFGR_SPEC, bool, O>;
///Field `PLLQ` reader - PLL VCO division factor Q for PLLQCLK clock output
pub type PLLQ_R = crate::FieldReader<u8, u8>;
///Field `PLLQ` writer - PLL VCO division factor Q for PLLQCLK clock output
pub type PLLQ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLLSYSCFGR_SPEC, u8, u8, 3, O>;
///Field `PLLREN` reader - PLLRCLK clock output enable
pub type PLLREN_R = crate::BitReader<bool>;
///Field `PLLREN` writer - PLLRCLK clock output enable
pub type PLLREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLLSYSCFGR_SPEC, bool, O>;
///Field `PLLR` reader - PLL VCO division factor R for PLLRCLK clock output
pub type PLLR_R = crate::FieldReader<u8, u8>;
///Field `PLLR` writer - PLL VCO division factor R for PLLRCLK clock output
pub type PLLR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLLSYSCFGR_SPEC, u8, u8, 3, O>;
impl R {
    ///Bits 0:1 - PLL input clock source
    #[inline(always)]
    pub fn pllsrc(&self) -> PLLSRC_R {
        PLLSRC_R::new((self.bits & 3) as u8)
    }
    ///Bits 4:6 - Division factor M of the PLL input clock divider
    #[inline(always)]
    pub fn pllm(&self) -> PLLM_R {
        PLLM_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 8:14 - PLL frequency multiplication factor N
    #[inline(always)]
    pub fn plln(&self) -> PLLN_R {
        PLLN_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    ///Bit 16 - PLLPCLK clock output enable
    #[inline(always)]
    pub fn pllpen(&self) -> PLLPEN_R {
        PLLPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:21 - PLL VCO division factor P for PLLPCLK clock output
    #[inline(always)]
    pub fn pllp(&self) -> PLLP_R {
        PLLP_R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    ///Bit 24 - PLLQCLK clock output enable
    #[inline(always)]
    pub fn pllqen(&self) -> PLLQEN_R {
        PLLQEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bits 25:27 - PLL VCO division factor Q for PLLQCLK clock output
    #[inline(always)]
    pub fn pllq(&self) -> PLLQ_R {
        PLLQ_R::new(((self.bits >> 25) & 7) as u8)
    }
    ///Bit 28 - PLLRCLK clock output enable
    #[inline(always)]
    pub fn pllren(&self) -> PLLREN_R {
        PLLREN_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bits 29:31 - PLL VCO division factor R for PLLRCLK clock output
    #[inline(always)]
    pub fn pllr(&self) -> PLLR_R {
        PLLR_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    ///Bits 0:1 - PLL input clock source
    #[inline(always)]
    #[must_use]
    pub fn pllsrc(&mut self) -> PLLSRC_W<0> {
        PLLSRC_W::new(self)
    }
    ///Bits 4:6 - Division factor M of the PLL input clock divider
    #[inline(always)]
    #[must_use]
    pub fn pllm(&mut self) -> PLLM_W<4> {
        PLLM_W::new(self)
    }
    ///Bits 8:14 - PLL frequency multiplication factor N
    #[inline(always)]
    #[must_use]
    pub fn plln(&mut self) -> PLLN_W<8> {
        PLLN_W::new(self)
    }
    ///Bit 16 - PLLPCLK clock output enable
    #[inline(always)]
    #[must_use]
    pub fn pllpen(&mut self) -> PLLPEN_W<16> {
        PLLPEN_W::new(self)
    }
    ///Bits 17:21 - PLL VCO division factor P for PLLPCLK clock output
    #[inline(always)]
    #[must_use]
    pub fn pllp(&mut self) -> PLLP_W<17> {
        PLLP_W::new(self)
    }
    ///Bit 24 - PLLQCLK clock output enable
    #[inline(always)]
    #[must_use]
    pub fn pllqen(&mut self) -> PLLQEN_W<24> {
        PLLQEN_W::new(self)
    }
    ///Bits 25:27 - PLL VCO division factor Q for PLLQCLK clock output
    #[inline(always)]
    #[must_use]
    pub fn pllq(&mut self) -> PLLQ_W<25> {
        PLLQ_W::new(self)
    }
    ///Bit 28 - PLLRCLK clock output enable
    #[inline(always)]
    #[must_use]
    pub fn pllren(&mut self) -> PLLREN_W<28> {
        PLLREN_W::new(self)
    }
    ///Bits 29:31 - PLL VCO division factor R for PLLRCLK clock output
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
///PLL configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pllsyscfgr](index.html) module
pub struct PLLSYSCFGR_SPEC;
impl crate::RegisterSpec for PLLSYSCFGR_SPEC {
    type Ux = u32;
}
///`read()` method returns [pllsyscfgr::R](R) reader structure
impl crate::Readable for PLLSYSCFGR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [pllsyscfgr::W](W) writer structure
impl crate::Writable for PLLSYSCFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets PLLSYSCFGR to value 0x1000
impl crate::Resettable for PLLSYSCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0x1000;
}
