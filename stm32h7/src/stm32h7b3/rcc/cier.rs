///Register `CIER` reader
pub struct R(crate::R<CIER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CIER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CIER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CIER_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CIER` writer
pub struct W(crate::W<CIER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CIER_SPEC>;
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
impl From<crate::W<CIER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CIER_SPEC>) -> Self {
        W(writer)
    }
}
///Field `LSIRDYIE` reader - LSI ready interrupt enable Set and reset by software to enable/disable interrupt caused by the LSI oscillator stabilization.
pub type LSIRDYIE_R = crate::BitReader<LSIRDYIE_A>;
///LSI ready interrupt enable Set and reset by software to enable/disable interrupt caused by the LSI oscillator stabilization.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSIRDYIE_A {
    ///0: Interrupt disabled
    Disabled = 0,
    ///1: Interrupt enabled
    Enabled = 1,
}
impl From<LSIRDYIE_A> for bool {
    #[inline(always)]
    fn from(variant: LSIRDYIE_A) -> Self {
        variant as u8 != 0
    }
}
impl LSIRDYIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LSIRDYIE_A {
        match self.bits {
            false => LSIRDYIE_A::Disabled,
            true => LSIRDYIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LSIRDYIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LSIRDYIE_A::Enabled
    }
}
///Field `LSIRDYIE` writer - LSI ready interrupt enable Set and reset by software to enable/disable interrupt caused by the LSI oscillator stabilization.
pub type LSIRDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIER_SPEC, LSIRDYIE_A, O>;
impl<'a, const O: u8> LSIRDYIE_W<'a, O> {
    ///Interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LSIRDYIE_A::Disabled)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LSIRDYIE_A::Enabled)
    }
}
///Field `LSERDYIE` reader - LSE ready interrupt enable Set and reset by software to enable/disable interrupt caused by the LSE oscillator stabilization.
pub use LSIRDYIE_R as LSERDYIE_R;
///Field `HSIRDYIE` reader - HSI ready interrupt enable Set and reset by software to enable/disable interrupt caused by the HSI oscillator stabilization.
pub use LSIRDYIE_R as HSIRDYIE_R;
///Field `HSERDYIE` reader - HSE ready interrupt enable Set and reset by software to enable/disable interrupt caused by the HSE oscillator stabilization.
pub use LSIRDYIE_R as HSERDYIE_R;
///Field `CSIRDYIE` reader - CSI ready interrupt enable Set and reset by software to enable/disable interrupt caused by the CSI oscillator stabilization.
pub use LSIRDYIE_R as CSIRDYIE_R;
///Field `HSI48RDYIE` reader - HSI48 ready interrupt enable Set and reset by software to enable/disable interrupt caused by the HSI48 oscillator stabilization.
pub use LSIRDYIE_R as HSI48RDYIE_R;
///Field `PLL1RDYIE` reader - PLL1 ready interrupt enable Set and reset by software to enable/disable interrupt caused by PLL1 lock.
pub use LSIRDYIE_R as PLL1RDYIE_R;
///Field `PLL2RDYIE` reader - PLL2 ready interrupt enable Set and reset by software to enable/disable interrupt caused by PLL2 lock.
pub use LSIRDYIE_R as PLL2RDYIE_R;
///Field `PLL3RDYIE` reader - PLL3 ready interrupt enable Set and reset by software to enable/disable interrupt caused by PLL3 lock.
pub use LSIRDYIE_R as PLL3RDYIE_R;
///Field `LSECSSIE` reader - LSE clock security system interrupt enable Set and reset by software to enable/disable interrupt caused by the clock security system (CSS) on external 32 kHz oscillator.
pub use LSIRDYIE_R as LSECSSIE_R;
///Field `LSERDYIE` writer - LSE ready interrupt enable Set and reset by software to enable/disable interrupt caused by the LSE oscillator stabilization.
pub use LSIRDYIE_W as LSERDYIE_W;
///Field `HSIRDYIE` writer - HSI ready interrupt enable Set and reset by software to enable/disable interrupt caused by the HSI oscillator stabilization.
pub use LSIRDYIE_W as HSIRDYIE_W;
///Field `HSERDYIE` writer - HSE ready interrupt enable Set and reset by software to enable/disable interrupt caused by the HSE oscillator stabilization.
pub use LSIRDYIE_W as HSERDYIE_W;
///Field `CSIRDYIE` writer - CSI ready interrupt enable Set and reset by software to enable/disable interrupt caused by the CSI oscillator stabilization.
pub use LSIRDYIE_W as CSIRDYIE_W;
///Field `HSI48RDYIE` writer - HSI48 ready interrupt enable Set and reset by software to enable/disable interrupt caused by the HSI48 oscillator stabilization.
pub use LSIRDYIE_W as HSI48RDYIE_W;
///Field `PLL1RDYIE` writer - PLL1 ready interrupt enable Set and reset by software to enable/disable interrupt caused by PLL1 lock.
pub use LSIRDYIE_W as PLL1RDYIE_W;
///Field `PLL2RDYIE` writer - PLL2 ready interrupt enable Set and reset by software to enable/disable interrupt caused by PLL2 lock.
pub use LSIRDYIE_W as PLL2RDYIE_W;
///Field `PLL3RDYIE` writer - PLL3 ready interrupt enable Set and reset by software to enable/disable interrupt caused by PLL3 lock.
pub use LSIRDYIE_W as PLL3RDYIE_W;
///Field `LSECSSIE` writer - LSE clock security system interrupt enable Set and reset by software to enable/disable interrupt caused by the clock security system (CSS) on external 32 kHz oscillator.
pub use LSIRDYIE_W as LSECSSIE_W;
impl R {
    ///Bit 0 - LSI ready interrupt enable Set and reset by software to enable/disable interrupt caused by the LSI oscillator stabilization.
    #[inline(always)]
    pub fn lsirdyie(&self) -> LSIRDYIE_R {
        LSIRDYIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LSE ready interrupt enable Set and reset by software to enable/disable interrupt caused by the LSE oscillator stabilization.
    #[inline(always)]
    pub fn lserdyie(&self) -> LSERDYIE_R {
        LSERDYIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - HSI ready interrupt enable Set and reset by software to enable/disable interrupt caused by the HSI oscillator stabilization.
    #[inline(always)]
    pub fn hsirdyie(&self) -> HSIRDYIE_R {
        HSIRDYIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - HSE ready interrupt enable Set and reset by software to enable/disable interrupt caused by the HSE oscillator stabilization.
    #[inline(always)]
    pub fn hserdyie(&self) -> HSERDYIE_R {
        HSERDYIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - CSI ready interrupt enable Set and reset by software to enable/disable interrupt caused by the CSI oscillator stabilization.
    #[inline(always)]
    pub fn csirdyie(&self) -> CSIRDYIE_R {
        CSIRDYIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - HSI48 ready interrupt enable Set and reset by software to enable/disable interrupt caused by the HSI48 oscillator stabilization.
    #[inline(always)]
    pub fn hsi48rdyie(&self) -> HSI48RDYIE_R {
        HSI48RDYIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - PLL1 ready interrupt enable Set and reset by software to enable/disable interrupt caused by PLL1 lock.
    #[inline(always)]
    pub fn pll1rdyie(&self) -> PLL1RDYIE_R {
        PLL1RDYIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - PLL2 ready interrupt enable Set and reset by software to enable/disable interrupt caused by PLL2 lock.
    #[inline(always)]
    pub fn pll2rdyie(&self) -> PLL2RDYIE_R {
        PLL2RDYIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - PLL3 ready interrupt enable Set and reset by software to enable/disable interrupt caused by PLL3 lock.
    #[inline(always)]
    pub fn pll3rdyie(&self) -> PLL3RDYIE_R {
        PLL3RDYIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - LSE clock security system interrupt enable Set and reset by software to enable/disable interrupt caused by the clock security system (CSS) on external 32 kHz oscillator.
    #[inline(always)]
    pub fn lsecssie(&self) -> LSECSSIE_R {
        LSECSSIE_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - LSI ready interrupt enable Set and reset by software to enable/disable interrupt caused by the LSI oscillator stabilization.
    #[inline(always)]
    #[must_use]
    pub fn lsirdyie(&mut self) -> LSIRDYIE_W<0> {
        LSIRDYIE_W::new(self)
    }
    ///Bit 1 - LSE ready interrupt enable Set and reset by software to enable/disable interrupt caused by the LSE oscillator stabilization.
    #[inline(always)]
    #[must_use]
    pub fn lserdyie(&mut self) -> LSERDYIE_W<1> {
        LSERDYIE_W::new(self)
    }
    ///Bit 2 - HSI ready interrupt enable Set and reset by software to enable/disable interrupt caused by the HSI oscillator stabilization.
    #[inline(always)]
    #[must_use]
    pub fn hsirdyie(&mut self) -> HSIRDYIE_W<2> {
        HSIRDYIE_W::new(self)
    }
    ///Bit 3 - HSE ready interrupt enable Set and reset by software to enable/disable interrupt caused by the HSE oscillator stabilization.
    #[inline(always)]
    #[must_use]
    pub fn hserdyie(&mut self) -> HSERDYIE_W<3> {
        HSERDYIE_W::new(self)
    }
    ///Bit 4 - CSI ready interrupt enable Set and reset by software to enable/disable interrupt caused by the CSI oscillator stabilization.
    #[inline(always)]
    #[must_use]
    pub fn csirdyie(&mut self) -> CSIRDYIE_W<4> {
        CSIRDYIE_W::new(self)
    }
    ///Bit 5 - HSI48 ready interrupt enable Set and reset by software to enable/disable interrupt caused by the HSI48 oscillator stabilization.
    #[inline(always)]
    #[must_use]
    pub fn hsi48rdyie(&mut self) -> HSI48RDYIE_W<5> {
        HSI48RDYIE_W::new(self)
    }
    ///Bit 6 - PLL1 ready interrupt enable Set and reset by software to enable/disable interrupt caused by PLL1 lock.
    #[inline(always)]
    #[must_use]
    pub fn pll1rdyie(&mut self) -> PLL1RDYIE_W<6> {
        PLL1RDYIE_W::new(self)
    }
    ///Bit 7 - PLL2 ready interrupt enable Set and reset by software to enable/disable interrupt caused by PLL2 lock.
    #[inline(always)]
    #[must_use]
    pub fn pll2rdyie(&mut self) -> PLL2RDYIE_W<7> {
        PLL2RDYIE_W::new(self)
    }
    ///Bit 8 - PLL3 ready interrupt enable Set and reset by software to enable/disable interrupt caused by PLL3 lock.
    #[inline(always)]
    #[must_use]
    pub fn pll3rdyie(&mut self) -> PLL3RDYIE_W<8> {
        PLL3RDYIE_W::new(self)
    }
    ///Bit 9 - LSE clock security system interrupt enable Set and reset by software to enable/disable interrupt caused by the clock security system (CSS) on external 32 kHz oscillator.
    #[inline(always)]
    #[must_use]
    pub fn lsecssie(&mut self) -> LSECSSIE_W<9> {
        LSECSSIE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cier](index.html) module
pub struct CIER_SPEC;
impl crate::RegisterSpec for CIER_SPEC {
    type Ux = u32;
}
///`read()` method returns [cier::R](R) reader structure
impl crate::Readable for CIER_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cier::W](W) writer structure
impl crate::Writable for CIER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CIER to value 0
impl crate::Resettable for CIER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
