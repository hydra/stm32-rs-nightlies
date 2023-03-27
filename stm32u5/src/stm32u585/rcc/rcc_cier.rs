///Register `RCC_CIER` reader
pub struct R(crate::R<RCC_CIER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_CIER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_CIER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_CIER_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RCC_CIER` writer
pub struct W(crate::W<RCC_CIER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_CIER_SPEC>;
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
impl From<crate::W<RCC_CIER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_CIER_SPEC>) -> Self {
        W(writer)
    }
}
///Field `LSIRDYIE` reader - LSI ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSI oscillator stabilization.
pub type LSIRDYIE_R = crate::BitReader<bool>;
///Field `LSIRDYIE` writer - LSI ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSI oscillator stabilization.
pub type LSIRDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_CIER_SPEC, bool, O>;
///Field `LSERDYIE` reader - LSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSE oscillator stabilization.
pub type LSERDYIE_R = crate::BitReader<bool>;
///Field `LSERDYIE` writer - LSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSE oscillator stabilization.
pub type LSERDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_CIER_SPEC, bool, O>;
///Field `MSISRDYIE` reader - MSIS ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the MSIS oscillator stabilization.
pub type MSISRDYIE_R = crate::BitReader<bool>;
///Field `MSISRDYIE` writer - MSIS ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the MSIS oscillator stabilization.
pub type MSISRDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_CIER_SPEC, bool, O>;
///Field `HSIRDYIE` reader - HSI16 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSI16 oscillator stabilization.
pub type HSIRDYIE_R = crate::BitReader<bool>;
///Field `HSIRDYIE` writer - HSI16 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSI16 oscillator stabilization.
pub type HSIRDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_CIER_SPEC, bool, O>;
///Field `HSERDYIE` reader - HSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSE oscillator stabilization.
pub type HSERDYIE_R = crate::BitReader<bool>;
///Field `HSERDYIE` writer - HSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSE oscillator stabilization.
pub type HSERDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_CIER_SPEC, bool, O>;
///Field `HSI48RDYIE` reader - HSI48 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSI48 oscillator stabilization.
pub type HSI48RDYIE_R = crate::BitReader<bool>;
///Field `HSI48RDYIE` writer - HSI48 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSI48 oscillator stabilization.
pub type HSI48RDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_CIER_SPEC, bool, O>;
///Field `PLL1RDYIE` reader - PLL ready interrupt enable Set and cleared by software to enable/disable interrupt caused by PLL1 lock.
pub type PLL1RDYIE_R = crate::BitReader<bool>;
///Field `PLL1RDYIE` writer - PLL ready interrupt enable Set and cleared by software to enable/disable interrupt caused by PLL1 lock.
pub type PLL1RDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_CIER_SPEC, bool, O>;
///Field `PLL2RDYIE` reader - PLL2 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by PLL2 lock.
pub type PLL2RDYIE_R = crate::BitReader<bool>;
///Field `PLL2RDYIE` writer - PLL2 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by PLL2 lock.
pub type PLL2RDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_CIER_SPEC, bool, O>;
///Field `PLL3RDYIE` reader - PLL3 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by PLL3 lock.
pub type PLL3RDYIE_R = crate::BitReader<bool>;
///Field `PLL3RDYIE` writer - PLL3 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by PLL3 lock.
pub type PLL3RDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_CIER_SPEC, bool, O>;
///Field `MSIKRDYIE` reader - MSIK ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the MSIK oscillator stabilization.
pub type MSIKRDYIE_R = crate::BitReader<bool>;
///Field `MSIKRDYIE` writer - MSIK ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the MSIK oscillator stabilization.
pub type MSIKRDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_CIER_SPEC, bool, O>;
///Field `SHSIRDYIE` reader - SHSI ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the SHSI oscillator stabilization.
pub type SHSIRDYIE_R = crate::BitReader<bool>;
///Field `SHSIRDYIE` writer - SHSI ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the SHSI oscillator stabilization.
pub type SHSIRDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_CIER_SPEC, bool, O>;
impl R {
    ///Bit 0 - LSI ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSI oscillator stabilization.
    #[inline(always)]
    pub fn lsirdyie(&self) -> LSIRDYIE_R {
        LSIRDYIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSE oscillator stabilization.
    #[inline(always)]
    pub fn lserdyie(&self) -> LSERDYIE_R {
        LSERDYIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - MSIS ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the MSIS oscillator stabilization.
    #[inline(always)]
    pub fn msisrdyie(&self) -> MSISRDYIE_R {
        MSISRDYIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - HSI16 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSI16 oscillator stabilization.
    #[inline(always)]
    pub fn hsirdyie(&self) -> HSIRDYIE_R {
        HSIRDYIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - HSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSE oscillator stabilization.
    #[inline(always)]
    pub fn hserdyie(&self) -> HSERDYIE_R {
        HSERDYIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - HSI48 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSI48 oscillator stabilization.
    #[inline(always)]
    pub fn hsi48rdyie(&self) -> HSI48RDYIE_R {
        HSI48RDYIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - PLL ready interrupt enable Set and cleared by software to enable/disable interrupt caused by PLL1 lock.
    #[inline(always)]
    pub fn pll1rdyie(&self) -> PLL1RDYIE_R {
        PLL1RDYIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - PLL2 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by PLL2 lock.
    #[inline(always)]
    pub fn pll2rdyie(&self) -> PLL2RDYIE_R {
        PLL2RDYIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - PLL3 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by PLL3 lock.
    #[inline(always)]
    pub fn pll3rdyie(&self) -> PLL3RDYIE_R {
        PLL3RDYIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 11 - MSIK ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the MSIK oscillator stabilization.
    #[inline(always)]
    pub fn msikrdyie(&self) -> MSIKRDYIE_R {
        MSIKRDYIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - SHSI ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the SHSI oscillator stabilization.
    #[inline(always)]
    pub fn shsirdyie(&self) -> SHSIRDYIE_R {
        SHSIRDYIE_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - LSI ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSI oscillator stabilization.
    #[inline(always)]
    #[must_use]
    pub fn lsirdyie(&mut self) -> LSIRDYIE_W<0> {
        LSIRDYIE_W::new(self)
    }
    ///Bit 1 - LSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSE oscillator stabilization.
    #[inline(always)]
    #[must_use]
    pub fn lserdyie(&mut self) -> LSERDYIE_W<1> {
        LSERDYIE_W::new(self)
    }
    ///Bit 2 - MSIS ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the MSIS oscillator stabilization.
    #[inline(always)]
    #[must_use]
    pub fn msisrdyie(&mut self) -> MSISRDYIE_W<2> {
        MSISRDYIE_W::new(self)
    }
    ///Bit 3 - HSI16 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSI16 oscillator stabilization.
    #[inline(always)]
    #[must_use]
    pub fn hsirdyie(&mut self) -> HSIRDYIE_W<3> {
        HSIRDYIE_W::new(self)
    }
    ///Bit 4 - HSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSE oscillator stabilization.
    #[inline(always)]
    #[must_use]
    pub fn hserdyie(&mut self) -> HSERDYIE_W<4> {
        HSERDYIE_W::new(self)
    }
    ///Bit 5 - HSI48 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSI48 oscillator stabilization.
    #[inline(always)]
    #[must_use]
    pub fn hsi48rdyie(&mut self) -> HSI48RDYIE_W<5> {
        HSI48RDYIE_W::new(self)
    }
    ///Bit 6 - PLL ready interrupt enable Set and cleared by software to enable/disable interrupt caused by PLL1 lock.
    #[inline(always)]
    #[must_use]
    pub fn pll1rdyie(&mut self) -> PLL1RDYIE_W<6> {
        PLL1RDYIE_W::new(self)
    }
    ///Bit 7 - PLL2 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by PLL2 lock.
    #[inline(always)]
    #[must_use]
    pub fn pll2rdyie(&mut self) -> PLL2RDYIE_W<7> {
        PLL2RDYIE_W::new(self)
    }
    ///Bit 8 - PLL3 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by PLL3 lock.
    #[inline(always)]
    #[must_use]
    pub fn pll3rdyie(&mut self) -> PLL3RDYIE_W<8> {
        PLL3RDYIE_W::new(self)
    }
    ///Bit 11 - MSIK ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the MSIK oscillator stabilization.
    #[inline(always)]
    #[must_use]
    pub fn msikrdyie(&mut self) -> MSIKRDYIE_W<11> {
        MSIKRDYIE_W::new(self)
    }
    ///Bit 12 - SHSI ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the SHSI oscillator stabilization.
    #[inline(always)]
    #[must_use]
    pub fn shsirdyie(&mut self) -> SHSIRDYIE_W<12> {
        SHSIRDYIE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RCC clock interrupt enable register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rcc_cier](index.html) module
pub struct RCC_CIER_SPEC;
impl crate::RegisterSpec for RCC_CIER_SPEC {
    type Ux = u32;
}
///`read()` method returns [rcc_cier::R](R) reader structure
impl crate::Readable for RCC_CIER_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rcc_cier::W](W) writer structure
impl crate::Writable for RCC_CIER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RCC_CIER to value 0
impl crate::Resettable for RCC_CIER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
