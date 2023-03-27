///Register `PLLCFGR` reader
pub struct R(crate::R<PLLCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLLCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLLCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLLCFGR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PLLCFGR` writer
pub struct W(crate::W<PLLCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLLCFGR_SPEC>;
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
impl From<crate::W<PLLCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLLCFGR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PLLSRC` reader - Main PLL, PLLSAI1 and PLLSAI2 entry clock source
pub type PLLSRC_R = crate::FieldReader<u8, u8>;
///Field `PLLSRC` writer - Main PLL, PLLSAI1 and PLLSAI2 entry clock source
pub type PLLSRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLLCFGR_SPEC, u8, u8, 2, O>;
///Field `PLLM` reader - Division factor for the main PLL and audio PLL (PLLSAI1 and PLLSAI2) input clock
pub type PLLM_R = crate::FieldReader<u8, u8>;
///Field `PLLM` writer - Division factor for the main PLL and audio PLL (PLLSAI1 and PLLSAI2) input clock
pub type PLLM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLLCFGR_SPEC, u8, u8, 4, O>;
///Field `PLLN` reader - Main PLL multiplication factor for VCO
pub type PLLN_R = crate::FieldReader<u8, u8>;
///Field `PLLN` writer - Main PLL multiplication factor for VCO
pub type PLLN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLLCFGR_SPEC, u8, u8, 7, O>;
///Field `PLLPEN` reader - Main PLL PLLSAI3CLK output enable
pub type PLLPEN_R = crate::BitReader<bool>;
///Field `PLLPEN` writer - Main PLL PLLSAI3CLK output enable
pub type PLLPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLLCFGR_SPEC, bool, O>;
///Field `PLLP` reader - Main PLL division factor for PLLSAI3CLK (SAI1 and SAI2 clock)
pub type PLLP_R = crate::BitReader<bool>;
///Field `PLLP` writer - Main PLL division factor for PLLSAI3CLK (SAI1 and SAI2 clock)
pub type PLLP_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLLCFGR_SPEC, bool, O>;
///Field `PLLQEN` reader - Main PLL PLLUSB1CLK output enable
pub type PLLQEN_R = crate::BitReader<bool>;
///Field `PLLQEN` writer - Main PLL PLLUSB1CLK output enable
pub type PLLQEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLLCFGR_SPEC, bool, O>;
///Field `PLLQ` reader - Main PLL division factor for PLLUSB1CLK(48 MHz clock)
pub type PLLQ_R = crate::FieldReader<u8, u8>;
///Field `PLLQ` writer - Main PLL division factor for PLLUSB1CLK(48 MHz clock)
pub type PLLQ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLLCFGR_SPEC, u8, u8, 2, O>;
///Field `PLLREN` reader - Main PLL PLLCLK output enable
pub type PLLREN_R = crate::BitReader<bool>;
///Field `PLLREN` writer - Main PLL PLLCLK output enable
pub type PLLREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLLCFGR_SPEC, bool, O>;
///Field `PLLR` reader - Main PLL division factor for PLLCLK (system clock)
pub type PLLR_R = crate::FieldReader<u8, u8>;
///Field `PLLR` writer - Main PLL division factor for PLLCLK (system clock)
pub type PLLR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLLCFGR_SPEC, u8, u8, 2, O>;
///Field `PLLPDIV` reader - Main PLL division factor for PLLSAI2CLK
pub type PLLPDIV_R = crate::FieldReader<u8, u8>;
///Field `PLLPDIV` writer - Main PLL division factor for PLLSAI2CLK
pub type PLLPDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLLCFGR_SPEC, u8, u8, 5, O>;
impl R {
    ///Bits 0:1 - Main PLL, PLLSAI1 and PLLSAI2 entry clock source
    #[inline(always)]
    pub fn pllsrc(&self) -> PLLSRC_R {
        PLLSRC_R::new((self.bits & 3) as u8)
    }
    ///Bits 4:7 - Division factor for the main PLL and audio PLL (PLLSAI1 and PLLSAI2) input clock
    #[inline(always)]
    pub fn pllm(&self) -> PLLM_R {
        PLLM_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:14 - Main PLL multiplication factor for VCO
    #[inline(always)]
    pub fn plln(&self) -> PLLN_R {
        PLLN_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    ///Bit 16 - Main PLL PLLSAI3CLK output enable
    #[inline(always)]
    pub fn pllpen(&self) -> PLLPEN_R {
        PLLPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Main PLL division factor for PLLSAI3CLK (SAI1 and SAI2 clock)
    #[inline(always)]
    pub fn pllp(&self) -> PLLP_R {
        PLLP_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 20 - Main PLL PLLUSB1CLK output enable
    #[inline(always)]
    pub fn pllqen(&self) -> PLLQEN_R {
        PLLQEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bits 21:22 - Main PLL division factor for PLLUSB1CLK(48 MHz clock)
    #[inline(always)]
    pub fn pllq(&self) -> PLLQ_R {
        PLLQ_R::new(((self.bits >> 21) & 3) as u8)
    }
    ///Bit 24 - Main PLL PLLCLK output enable
    #[inline(always)]
    pub fn pllren(&self) -> PLLREN_R {
        PLLREN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bits 25:26 - Main PLL division factor for PLLCLK (system clock)
    #[inline(always)]
    pub fn pllr(&self) -> PLLR_R {
        PLLR_R::new(((self.bits >> 25) & 3) as u8)
    }
    ///Bits 27:31 - Main PLL division factor for PLLSAI2CLK
    #[inline(always)]
    pub fn pllpdiv(&self) -> PLLPDIV_R {
        PLLPDIV_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    ///Bits 0:1 - Main PLL, PLLSAI1 and PLLSAI2 entry clock source
    #[inline(always)]
    #[must_use]
    pub fn pllsrc(&mut self) -> PLLSRC_W<0> {
        PLLSRC_W::new(self)
    }
    ///Bits 4:7 - Division factor for the main PLL and audio PLL (PLLSAI1 and PLLSAI2) input clock
    #[inline(always)]
    #[must_use]
    pub fn pllm(&mut self) -> PLLM_W<4> {
        PLLM_W::new(self)
    }
    ///Bits 8:14 - Main PLL multiplication factor for VCO
    #[inline(always)]
    #[must_use]
    pub fn plln(&mut self) -> PLLN_W<8> {
        PLLN_W::new(self)
    }
    ///Bit 16 - Main PLL PLLSAI3CLK output enable
    #[inline(always)]
    #[must_use]
    pub fn pllpen(&mut self) -> PLLPEN_W<16> {
        PLLPEN_W::new(self)
    }
    ///Bit 17 - Main PLL division factor for PLLSAI3CLK (SAI1 and SAI2 clock)
    #[inline(always)]
    #[must_use]
    pub fn pllp(&mut self) -> PLLP_W<17> {
        PLLP_W::new(self)
    }
    ///Bit 20 - Main PLL PLLUSB1CLK output enable
    #[inline(always)]
    #[must_use]
    pub fn pllqen(&mut self) -> PLLQEN_W<20> {
        PLLQEN_W::new(self)
    }
    ///Bits 21:22 - Main PLL division factor for PLLUSB1CLK(48 MHz clock)
    #[inline(always)]
    #[must_use]
    pub fn pllq(&mut self) -> PLLQ_W<21> {
        PLLQ_W::new(self)
    }
    ///Bit 24 - Main PLL PLLCLK output enable
    #[inline(always)]
    #[must_use]
    pub fn pllren(&mut self) -> PLLREN_W<24> {
        PLLREN_W::new(self)
    }
    ///Bits 25:26 - Main PLL division factor for PLLCLK (system clock)
    #[inline(always)]
    #[must_use]
    pub fn pllr(&mut self) -> PLLR_W<25> {
        PLLR_W::new(self)
    }
    ///Bits 27:31 - Main PLL division factor for PLLSAI2CLK
    #[inline(always)]
    #[must_use]
    pub fn pllpdiv(&mut self) -> PLLPDIV_W<27> {
        PLLPDIV_W::new(self)
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
///For information about available fields see [pllcfgr](index.html) module
pub struct PLLCFGR_SPEC;
impl crate::RegisterSpec for PLLCFGR_SPEC {
    type Ux = u32;
}
///`read()` method returns [pllcfgr::R](R) reader structure
impl crate::Readable for PLLCFGR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [pllcfgr::W](W) writer structure
impl crate::Writable for PLLCFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets PLLCFGR to value 0x1000
impl crate::Resettable for PLLCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0x1000;
}
