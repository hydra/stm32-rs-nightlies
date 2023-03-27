///Register `RCC_CICR` writer
pub struct W(crate::W<RCC_CICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_CICR_SPEC>;
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
impl From<crate::W<RCC_CICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_CICR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `LSIRDYC` writer - LSI ready interrupt clear Writing this bit to 1 clears the LSIRDYF flag. Writing 0 has no effect.
pub type LSIRDYC_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_CICR_SPEC, bool, O>;
///Field `LSERDYC` writer - LSE ready interrupt clear Writing this bit to 1 clears the LSERDYF flag. Writing 0 has no effect.
pub type LSERDYC_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_CICR_SPEC, bool, O>;
///Field `MSISRDYC` writer - MSIS ready interrupt clear Writing this bit to 1 clears the MSISRDYF flag. Writing 0 has no effect.
pub type MSISRDYC_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_CICR_SPEC, bool, O>;
///Field `HSIRDYC` writer - HSI16 ready interrupt clear Writing this bit to 1 clears the HSIRDYF flag. Writing 0 has no effect.
pub type HSIRDYC_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_CICR_SPEC, bool, O>;
///Field `HSERDYC` writer - HSE ready interrupt clear Writing this bit to 1 clears the HSERDYF flag. Writing 0 has no effect.
pub type HSERDYC_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_CICR_SPEC, bool, O>;
///Field `HSI48RDYC` writer - HSI48 ready interrupt clear Writing this bit to 1 clears the HSI48RDYF flag. Writing 0 has no effect.
pub type HSI48RDYC_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_CICR_SPEC, bool, O>;
///Field `PLL1RDYC` writer - PLL1 ready interrupt clear Writing this bit to 1 clears the PLL1RDYF flag. Writing 0 has no effect.
pub type PLL1RDYC_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_CICR_SPEC, bool, O>;
///Field `PLL2RDYC` writer - PLL2 ready interrupt clear Writing this bit to 1 clears the PLL2RDYF flag. Writing 0 has no effect.
pub type PLL2RDYC_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_CICR_SPEC, bool, O>;
///Field `PLL3RDYC` writer - PLL3 ready interrupt clear Writing this bit to 1 clears the PLL3RDYF flag. Writing 0 has no effect.
pub type PLL3RDYC_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_CICR_SPEC, bool, O>;
///Field `CSSC` writer - Clock security system interrupt clear Writing this bit to 1 clears the CSSF flag. Writing 0 has no effect.
pub type CSSC_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_CICR_SPEC, bool, O>;
///Field `MSIKRDYC` writer - MSIK oscillator ready interrupt clear Writing this bit to 1 clears the MSIKRDYF flag. Writing 0 has no effect.
pub type MSIKRDYC_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_CICR_SPEC, bool, O>;
///Field `SHSIRDYC` writer - SHSI oscillator ready interrupt clear Writing this bit to 1 clears the SHSIRDYF flag. Writing 0 has no effect.
pub type SHSIRDYC_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_CICR_SPEC, bool, O>;
impl W {
    ///Bit 0 - LSI ready interrupt clear Writing this bit to 1 clears the LSIRDYF flag. Writing 0 has no effect.
    #[inline(always)]
    #[must_use]
    pub fn lsirdyc(&mut self) -> LSIRDYC_W<0> {
        LSIRDYC_W::new(self)
    }
    ///Bit 1 - LSE ready interrupt clear Writing this bit to 1 clears the LSERDYF flag. Writing 0 has no effect.
    #[inline(always)]
    #[must_use]
    pub fn lserdyc(&mut self) -> LSERDYC_W<1> {
        LSERDYC_W::new(self)
    }
    ///Bit 2 - MSIS ready interrupt clear Writing this bit to 1 clears the MSISRDYF flag. Writing 0 has no effect.
    #[inline(always)]
    #[must_use]
    pub fn msisrdyc(&mut self) -> MSISRDYC_W<2> {
        MSISRDYC_W::new(self)
    }
    ///Bit 3 - HSI16 ready interrupt clear Writing this bit to 1 clears the HSIRDYF flag. Writing 0 has no effect.
    #[inline(always)]
    #[must_use]
    pub fn hsirdyc(&mut self) -> HSIRDYC_W<3> {
        HSIRDYC_W::new(self)
    }
    ///Bit 4 - HSE ready interrupt clear Writing this bit to 1 clears the HSERDYF flag. Writing 0 has no effect.
    #[inline(always)]
    #[must_use]
    pub fn hserdyc(&mut self) -> HSERDYC_W<4> {
        HSERDYC_W::new(self)
    }
    ///Bit 5 - HSI48 ready interrupt clear Writing this bit to 1 clears the HSI48RDYF flag. Writing 0 has no effect.
    #[inline(always)]
    #[must_use]
    pub fn hsi48rdyc(&mut self) -> HSI48RDYC_W<5> {
        HSI48RDYC_W::new(self)
    }
    ///Bit 6 - PLL1 ready interrupt clear Writing this bit to 1 clears the PLL1RDYF flag. Writing 0 has no effect.
    #[inline(always)]
    #[must_use]
    pub fn pll1rdyc(&mut self) -> PLL1RDYC_W<6> {
        PLL1RDYC_W::new(self)
    }
    ///Bit 7 - PLL2 ready interrupt clear Writing this bit to 1 clears the PLL2RDYF flag. Writing 0 has no effect.
    #[inline(always)]
    #[must_use]
    pub fn pll2rdyc(&mut self) -> PLL2RDYC_W<7> {
        PLL2RDYC_W::new(self)
    }
    ///Bit 8 - PLL3 ready interrupt clear Writing this bit to 1 clears the PLL3RDYF flag. Writing 0 has no effect.
    #[inline(always)]
    #[must_use]
    pub fn pll3rdyc(&mut self) -> PLL3RDYC_W<8> {
        PLL3RDYC_W::new(self)
    }
    ///Bit 10 - Clock security system interrupt clear Writing this bit to 1 clears the CSSF flag. Writing 0 has no effect.
    #[inline(always)]
    #[must_use]
    pub fn cssc(&mut self) -> CSSC_W<10> {
        CSSC_W::new(self)
    }
    ///Bit 11 - MSIK oscillator ready interrupt clear Writing this bit to 1 clears the MSIKRDYF flag. Writing 0 has no effect.
    #[inline(always)]
    #[must_use]
    pub fn msikrdyc(&mut self) -> MSIKRDYC_W<11> {
        MSIKRDYC_W::new(self)
    }
    ///Bit 12 - SHSI oscillator ready interrupt clear Writing this bit to 1 clears the SHSIRDYF flag. Writing 0 has no effect.
    #[inline(always)]
    #[must_use]
    pub fn shsirdyc(&mut self) -> SHSIRDYC_W<12> {
        SHSIRDYC_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RCC clock interrupt clear register
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rcc_cicr](index.html) module
pub struct RCC_CICR_SPEC;
impl crate::RegisterSpec for RCC_CICR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [rcc_cicr::W](W) writer structure
impl crate::Writable for RCC_CICR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RCC_CICR to value 0
impl crate::Resettable for RCC_CICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
