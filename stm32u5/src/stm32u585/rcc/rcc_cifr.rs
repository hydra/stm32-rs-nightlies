///Register `RCC_CIFR` reader
pub struct R(crate::R<RCC_CIFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_CIFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_CIFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_CIFR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `LSIRDYF` reader - LSI ready interrupt flag Set by hardware when the LSI clock becomes stable and LSIRDYIE is set. Cleared by software setting the LSIRDYC bit.
pub type LSIRDYF_R = crate::BitReader<bool>;
///Field `LSERDYF` reader - LSE ready interrupt flag Set by hardware when the LSE clock becomes stable and LSERDYIE is set. Cleared by software setting the LSERDYC bit.
pub type LSERDYF_R = crate::BitReader<bool>;
///Field `MSISRDYF` reader - MSIS ready interrupt flag Set by hardware when the MSIS clock becomes stable and MSISRDYIE is set. Cleared by software setting the MSISRDYC bit.
pub type MSISRDYF_R = crate::BitReader<bool>;
///Field `HSIRDYF` reader - HSI16 ready interrupt flag Set by hardware when the HSI16 clock becomes stable and HSIRDYIE is set in a response to setting the HSION (see RCC_CR). When HSION is not set but the HSI16 oscillator is enabled by the peripheral through a clock request, this bit is not set and no interrupt is generated. Cleared by software setting the HSIRDYC bit.
pub type HSIRDYF_R = crate::BitReader<bool>;
///Field `HSERDYF` reader - HSE ready interrupt flag Set by hardware when the HSE clock becomes stable and HSERDYIE is set. Cleared by software setting the HSERDYC bit.
pub type HSERDYF_R = crate::BitReader<bool>;
///Field `HSI48RDYF` reader - HSI48 ready interrupt flag Set by hardware when the HSI48 clock becomes stable and HSI48RDYIE is set. Cleared by software setting the HSI48RDYC bit.
pub type HSI48RDYF_R = crate::BitReader<bool>;
///Field `PLL1RDYF` reader - PLL1 ready interrupt flag Set by hardware when the PLL1 locks and PLL1RDYIE is set. Cleared by software setting the PLL1RDYC bit.
pub type PLL1RDYF_R = crate::BitReader<bool>;
///Field `PLL2RDYF` reader - PLL2 ready interrupt flag Set by hardware when the PLL2 locks and PLL2RDYIE is set. Cleared by software setting the PLL2RDYC bit.
pub type PLL2RDYF_R = crate::BitReader<bool>;
///Field `PLL3RDYF` reader - PLL3 ready interrupt flag Set by hardware when the PLL3 locks and PLL3RDYIE is set. Cleared by software setting the PLL3RDYC bit.
pub type PLL3RDYF_R = crate::BitReader<bool>;
///Field `CSSF` reader - Clock security system interrupt flag Set by hardware when a failure is detected in the HSE oscillator. Cleared by software setting the CSSC bit.
pub type CSSF_R = crate::BitReader<bool>;
///Field `MSIKRDYF` reader - MSIK ready interrupt flag Set by hardware when the MSIK clock becomes stable and MSIKRDYIE is set. Cleared by software setting the MSIKRDYC bit.
pub type MSIKRDYF_R = crate::BitReader<bool>;
///Field `SHSIRDYF` reader - SHSI ready interrupt flag Set by hardware when the SHSI clock becomes stable and SHSIRDYIE is set. Cleared by software setting the SHSIRDYC bit.
pub type SHSIRDYF_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - LSI ready interrupt flag Set by hardware when the LSI clock becomes stable and LSIRDYIE is set. Cleared by software setting the LSIRDYC bit.
    #[inline(always)]
    pub fn lsirdyf(&self) -> LSIRDYF_R {
        LSIRDYF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LSE ready interrupt flag Set by hardware when the LSE clock becomes stable and LSERDYIE is set. Cleared by software setting the LSERDYC bit.
    #[inline(always)]
    pub fn lserdyf(&self) -> LSERDYF_R {
        LSERDYF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - MSIS ready interrupt flag Set by hardware when the MSIS clock becomes stable and MSISRDYIE is set. Cleared by software setting the MSISRDYC bit.
    #[inline(always)]
    pub fn msisrdyf(&self) -> MSISRDYF_R {
        MSISRDYF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - HSI16 ready interrupt flag Set by hardware when the HSI16 clock becomes stable and HSIRDYIE is set in a response to setting the HSION (see RCC_CR). When HSION is not set but the HSI16 oscillator is enabled by the peripheral through a clock request, this bit is not set and no interrupt is generated. Cleared by software setting the HSIRDYC bit.
    #[inline(always)]
    pub fn hsirdyf(&self) -> HSIRDYF_R {
        HSIRDYF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - HSE ready interrupt flag Set by hardware when the HSE clock becomes stable and HSERDYIE is set. Cleared by software setting the HSERDYC bit.
    #[inline(always)]
    pub fn hserdyf(&self) -> HSERDYF_R {
        HSERDYF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - HSI48 ready interrupt flag Set by hardware when the HSI48 clock becomes stable and HSI48RDYIE is set. Cleared by software setting the HSI48RDYC bit.
    #[inline(always)]
    pub fn hsi48rdyf(&self) -> HSI48RDYF_R {
        HSI48RDYF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - PLL1 ready interrupt flag Set by hardware when the PLL1 locks and PLL1RDYIE is set. Cleared by software setting the PLL1RDYC bit.
    #[inline(always)]
    pub fn pll1rdyf(&self) -> PLL1RDYF_R {
        PLL1RDYF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - PLL2 ready interrupt flag Set by hardware when the PLL2 locks and PLL2RDYIE is set. Cleared by software setting the PLL2RDYC bit.
    #[inline(always)]
    pub fn pll2rdyf(&self) -> PLL2RDYF_R {
        PLL2RDYF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - PLL3 ready interrupt flag Set by hardware when the PLL3 locks and PLL3RDYIE is set. Cleared by software setting the PLL3RDYC bit.
    #[inline(always)]
    pub fn pll3rdyf(&self) -> PLL3RDYF_R {
        PLL3RDYF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 10 - Clock security system interrupt flag Set by hardware when a failure is detected in the HSE oscillator. Cleared by software setting the CSSC bit.
    #[inline(always)]
    pub fn cssf(&self) -> CSSF_R {
        CSSF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - MSIK ready interrupt flag Set by hardware when the MSIK clock becomes stable and MSIKRDYIE is set. Cleared by software setting the MSIKRDYC bit.
    #[inline(always)]
    pub fn msikrdyf(&self) -> MSIKRDYF_R {
        MSIKRDYF_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - SHSI ready interrupt flag Set by hardware when the SHSI clock becomes stable and SHSIRDYIE is set. Cleared by software setting the SHSIRDYC bit.
    #[inline(always)]
    pub fn shsirdyf(&self) -> SHSIRDYF_R {
        SHSIRDYF_R::new(((self.bits >> 12) & 1) != 0)
    }
}
///RCC clock interrupt flag register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rcc_cifr](index.html) module
pub struct RCC_CIFR_SPEC;
impl crate::RegisterSpec for RCC_CIFR_SPEC {
    type Ux = u32;
}
///`read()` method returns [rcc_cifr::R](R) reader structure
impl crate::Readable for RCC_CIFR_SPEC {
    type Reader = R;
}
///`reset()` method sets RCC_CIFR to value 0
impl crate::Resettable for RCC_CIFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
