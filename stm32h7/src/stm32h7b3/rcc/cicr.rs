///Register `CICR` reader
pub struct R(crate::R<CICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CICR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CICR` writer
pub struct W(crate::W<CICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CICR_SPEC>;
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
impl From<crate::W<CICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CICR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `LSIRDYC` reader - LSI ready interrupt clear Set by software to clear LSIRDYF. Reset by hardware when clear done.
pub type LSIRDYC_R = crate::BitReader<LSIRDYC_A>;
///LSI ready interrupt clear Set by software to clear LSIRDYF. Reset by hardware when clear done.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSIRDYC_A {
    ///1: Clear interrupt flag
    Clear = 1,
}
impl From<LSIRDYC_A> for bool {
    #[inline(always)]
    fn from(variant: LSIRDYC_A) -> Self {
        variant as u8 != 0
    }
}
impl LSIRDYC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<LSIRDYC_A> {
        match self.bits {
            true => Some(LSIRDYC_A::Clear),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Clear`
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == LSIRDYC_A::Clear
    }
}
///Field `LSIRDYC` writer - LSI ready interrupt clear Set by software to clear LSIRDYF. Reset by hardware when clear done.
pub type LSIRDYC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CICR_SPEC, LSIRDYC_A, O>;
impl<'a, const O: u8> LSIRDYC_W<'a, O> {
    ///Clear interrupt flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(LSIRDYC_A::Clear)
    }
}
///Field `LSERDYC` reader - LSE ready interrupt clear Set by software to clear LSERDYF. Reset by hardware when clear done.
pub use LSIRDYC_R as LSERDYC_R;
///Field `HSIRDYC` reader - HSI ready interrupt clear Set by software to clear HSIRDYF. Reset by hardware when clear done.
pub use LSIRDYC_R as HSIRDYC_R;
///Field `HSERDYC` reader - HSE ready interrupt clear Set by software to clear HSERDYF. Reset by hardware when clear done.
pub use LSIRDYC_R as HSERDYC_R;
///Field `CSIRDYC` reader - CSI ready interrupt clear Set by software to clear CSIRDYF. Reset by hardware when clear done.
pub use LSIRDYC_R as CSIRDYC_R;
///Field `HSI48RDYC` reader - HSI48 ready interrupt clear Set by software to clear HSI48RDYF. Reset by hardware when clear done.
pub use LSIRDYC_R as HSI48RDYC_R;
///Field `PLL1RDYC` reader - PLL1 ready interrupt clear Set by software to clear PLL1RDYF. Reset by hardware when clear done.
pub use LSIRDYC_R as PLL1RDYC_R;
///Field `PLL2RDYC` reader - PLL2 ready interrupt clear Set by software to clear PLL2RDYF. Reset by hardware when clear done.
pub use LSIRDYC_R as PLL2RDYC_R;
///Field `PLL3RDYC` reader - PLL3 ready interrupt clear Set by software to clear PLL3RDYF. Reset by hardware when clear done.
pub use LSIRDYC_R as PLL3RDYC_R;
///Field `LSECSSC` reader - LSE clock security system interrupt clear Set by software to clear LSECSSF. Reset by hardware when clear done.
pub use LSIRDYC_R as LSECSSC_R;
///Field `HSECSSC` reader - HSE clock security system interrupt clear Set by software to clear HSECSSF. Reset by hardware when clear done.
pub use LSIRDYC_R as HSECSSC_R;
///Field `LSERDYC` writer - LSE ready interrupt clear Set by software to clear LSERDYF. Reset by hardware when clear done.
pub use LSIRDYC_W as LSERDYC_W;
///Field `HSIRDYC` writer - HSI ready interrupt clear Set by software to clear HSIRDYF. Reset by hardware when clear done.
pub use LSIRDYC_W as HSIRDYC_W;
///Field `HSERDYC` writer - HSE ready interrupt clear Set by software to clear HSERDYF. Reset by hardware when clear done.
pub use LSIRDYC_W as HSERDYC_W;
///Field `CSIRDYC` writer - CSI ready interrupt clear Set by software to clear CSIRDYF. Reset by hardware when clear done.
pub use LSIRDYC_W as CSIRDYC_W;
///Field `HSI48RDYC` writer - HSI48 ready interrupt clear Set by software to clear HSI48RDYF. Reset by hardware when clear done.
pub use LSIRDYC_W as HSI48RDYC_W;
///Field `PLL1RDYC` writer - PLL1 ready interrupt clear Set by software to clear PLL1RDYF. Reset by hardware when clear done.
pub use LSIRDYC_W as PLL1RDYC_W;
///Field `PLL2RDYC` writer - PLL2 ready interrupt clear Set by software to clear PLL2RDYF. Reset by hardware when clear done.
pub use LSIRDYC_W as PLL2RDYC_W;
///Field `PLL3RDYC` writer - PLL3 ready interrupt clear Set by software to clear PLL3RDYF. Reset by hardware when clear done.
pub use LSIRDYC_W as PLL3RDYC_W;
///Field `LSECSSC` writer - LSE clock security system interrupt clear Set by software to clear LSECSSF. Reset by hardware when clear done.
pub use LSIRDYC_W as LSECSSC_W;
///Field `HSECSSC` writer - HSE clock security system interrupt clear Set by software to clear HSECSSF. Reset by hardware when clear done.
pub use LSIRDYC_W as HSECSSC_W;
impl R {
    ///Bit 0 - LSI ready interrupt clear Set by software to clear LSIRDYF. Reset by hardware when clear done.
    #[inline(always)]
    pub fn lsirdyc(&self) -> LSIRDYC_R {
        LSIRDYC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LSE ready interrupt clear Set by software to clear LSERDYF. Reset by hardware when clear done.
    #[inline(always)]
    pub fn lserdyc(&self) -> LSERDYC_R {
        LSERDYC_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - HSI ready interrupt clear Set by software to clear HSIRDYF. Reset by hardware when clear done.
    #[inline(always)]
    pub fn hsirdyc(&self) -> HSIRDYC_R {
        HSIRDYC_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - HSE ready interrupt clear Set by software to clear HSERDYF. Reset by hardware when clear done.
    #[inline(always)]
    pub fn hserdyc(&self) -> HSERDYC_R {
        HSERDYC_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - CSI ready interrupt clear Set by software to clear CSIRDYF. Reset by hardware when clear done.
    #[inline(always)]
    pub fn csirdyc(&self) -> CSIRDYC_R {
        CSIRDYC_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - HSI48 ready interrupt clear Set by software to clear HSI48RDYF. Reset by hardware when clear done.
    #[inline(always)]
    pub fn hsi48rdyc(&self) -> HSI48RDYC_R {
        HSI48RDYC_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - PLL1 ready interrupt clear Set by software to clear PLL1RDYF. Reset by hardware when clear done.
    #[inline(always)]
    pub fn pll1rdyc(&self) -> PLL1RDYC_R {
        PLL1RDYC_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - PLL2 ready interrupt clear Set by software to clear PLL2RDYF. Reset by hardware when clear done.
    #[inline(always)]
    pub fn pll2rdyc(&self) -> PLL2RDYC_R {
        PLL2RDYC_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - PLL3 ready interrupt clear Set by software to clear PLL3RDYF. Reset by hardware when clear done.
    #[inline(always)]
    pub fn pll3rdyc(&self) -> PLL3RDYC_R {
        PLL3RDYC_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - LSE clock security system interrupt clear Set by software to clear LSECSSF. Reset by hardware when clear done.
    #[inline(always)]
    pub fn lsecssc(&self) -> LSECSSC_R {
        LSECSSC_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - HSE clock security system interrupt clear Set by software to clear HSECSSF. Reset by hardware when clear done.
    #[inline(always)]
    pub fn hsecssc(&self) -> HSECSSC_R {
        HSECSSC_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - LSI ready interrupt clear Set by software to clear LSIRDYF. Reset by hardware when clear done.
    #[inline(always)]
    #[must_use]
    pub fn lsirdyc(&mut self) -> LSIRDYC_W<0> {
        LSIRDYC_W::new(self)
    }
    ///Bit 1 - LSE ready interrupt clear Set by software to clear LSERDYF. Reset by hardware when clear done.
    #[inline(always)]
    #[must_use]
    pub fn lserdyc(&mut self) -> LSERDYC_W<1> {
        LSERDYC_W::new(self)
    }
    ///Bit 2 - HSI ready interrupt clear Set by software to clear HSIRDYF. Reset by hardware when clear done.
    #[inline(always)]
    #[must_use]
    pub fn hsirdyc(&mut self) -> HSIRDYC_W<2> {
        HSIRDYC_W::new(self)
    }
    ///Bit 3 - HSE ready interrupt clear Set by software to clear HSERDYF. Reset by hardware when clear done.
    #[inline(always)]
    #[must_use]
    pub fn hserdyc(&mut self) -> HSERDYC_W<3> {
        HSERDYC_W::new(self)
    }
    ///Bit 4 - CSI ready interrupt clear Set by software to clear CSIRDYF. Reset by hardware when clear done.
    #[inline(always)]
    #[must_use]
    pub fn csirdyc(&mut self) -> CSIRDYC_W<4> {
        CSIRDYC_W::new(self)
    }
    ///Bit 5 - HSI48 ready interrupt clear Set by software to clear HSI48RDYF. Reset by hardware when clear done.
    #[inline(always)]
    #[must_use]
    pub fn hsi48rdyc(&mut self) -> HSI48RDYC_W<5> {
        HSI48RDYC_W::new(self)
    }
    ///Bit 6 - PLL1 ready interrupt clear Set by software to clear PLL1RDYF. Reset by hardware when clear done.
    #[inline(always)]
    #[must_use]
    pub fn pll1rdyc(&mut self) -> PLL1RDYC_W<6> {
        PLL1RDYC_W::new(self)
    }
    ///Bit 7 - PLL2 ready interrupt clear Set by software to clear PLL2RDYF. Reset by hardware when clear done.
    #[inline(always)]
    #[must_use]
    pub fn pll2rdyc(&mut self) -> PLL2RDYC_W<7> {
        PLL2RDYC_W::new(self)
    }
    ///Bit 8 - PLL3 ready interrupt clear Set by software to clear PLL3RDYF. Reset by hardware when clear done.
    #[inline(always)]
    #[must_use]
    pub fn pll3rdyc(&mut self) -> PLL3RDYC_W<8> {
        PLL3RDYC_W::new(self)
    }
    ///Bit 9 - LSE clock security system interrupt clear Set by software to clear LSECSSF. Reset by hardware when clear done.
    #[inline(always)]
    #[must_use]
    pub fn lsecssc(&mut self) -> LSECSSC_W<9> {
        LSECSSC_W::new(self)
    }
    ///Bit 10 - HSE clock security system interrupt clear Set by software to clear HSECSSF. Reset by hardware when clear done.
    #[inline(always)]
    #[must_use]
    pub fn hsecssc(&mut self) -> HSECSSC_W<10> {
        HSECSSC_W::new(self)
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
///For information about available fields see [cicr](index.html) module
pub struct CICR_SPEC;
impl crate::RegisterSpec for CICR_SPEC {
    type Ux = u32;
}
///`read()` method returns [cicr::R](R) reader structure
impl crate::Readable for CICR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cicr::W](W) writer structure
impl crate::Writable for CICR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CICR to value 0
impl crate::Resettable for CICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
