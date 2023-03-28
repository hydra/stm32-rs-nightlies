///Register `RCC_MP_CIFR` reader
pub struct R(crate::R<RCC_MP_CIFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_MP_CIFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_MP_CIFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_MP_CIFR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RCC_MP_CIFR` writer
pub struct W(crate::W<RCC_MP_CIFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_MP_CIFR_SPEC>;
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
impl From<crate::W<RCC_MP_CIFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_MP_CIFR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `LSIRDYF` reader - LSIRDYF
pub type LSIRDYF_R = crate::BitReader<bool>;
///Field `LSIRDYF` writer - LSIRDYF
pub type LSIRDYF_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_CIFR_SPEC, bool, O>;
///Field `LSERDYF` reader - LSERDYF
pub type LSERDYF_R = crate::BitReader<bool>;
///Field `LSERDYF` writer - LSERDYF
pub type LSERDYF_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_CIFR_SPEC, bool, O>;
///Field `HSIRDYF` reader - HSIRDYF
pub type HSIRDYF_R = crate::BitReader<bool>;
///Field `HSIRDYF` writer - HSIRDYF
pub type HSIRDYF_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_CIFR_SPEC, bool, O>;
///Field `HSERDYF` reader - HSERDYF
pub type HSERDYF_R = crate::BitReader<bool>;
///Field `HSERDYF` writer - HSERDYF
pub type HSERDYF_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_CIFR_SPEC, bool, O>;
///Field `CSIRDYF` reader - CSIRDYF
pub type CSIRDYF_R = crate::BitReader<bool>;
///Field `CSIRDYF` writer - CSIRDYF
pub type CSIRDYF_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_CIFR_SPEC, bool, O>;
///Field `PLL1DYF` reader - PLL1DYF
pub type PLL1DYF_R = crate::BitReader<bool>;
///Field `PLL1DYF` writer - PLL1DYF
pub type PLL1DYF_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_CIFR_SPEC, bool, O>;
///Field `PLL2DYF` reader - PLL2DYF
pub type PLL2DYF_R = crate::BitReader<bool>;
///Field `PLL2DYF` writer - PLL2DYF
pub type PLL2DYF_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_CIFR_SPEC, bool, O>;
///Field `PLL3DYF` reader - PLL3DYF
pub type PLL3DYF_R = crate::BitReader<bool>;
///Field `PLL3DYF` writer - PLL3DYF
pub type PLL3DYF_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_CIFR_SPEC, bool, O>;
///Field `PLL4DYF` reader - PLL4DYF
pub type PLL4DYF_R = crate::BitReader<bool>;
///Field `PLL4DYF` writer - PLL4DYF
pub type PLL4DYF_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_CIFR_SPEC, bool, O>;
///Field `LSECSSF` reader - LSECSSF
pub type LSECSSF_R = crate::BitReader<bool>;
///Field `LSECSSF` writer - LSECSSF
pub type LSECSSF_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_CIFR_SPEC, bool, O>;
///Field `WKUPF` reader - WKUPF
pub type WKUPF_R = crate::BitReader<bool>;
///Field `WKUPF` writer - WKUPF
pub type WKUPF_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_CIFR_SPEC, bool, O>;
impl R {
    ///Bit 0 - LSIRDYF
    #[inline(always)]
    pub fn lsirdyf(&self) -> LSIRDYF_R {
        LSIRDYF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LSERDYF
    #[inline(always)]
    pub fn lserdyf(&self) -> LSERDYF_R {
        LSERDYF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - HSIRDYF
    #[inline(always)]
    pub fn hsirdyf(&self) -> HSIRDYF_R {
        HSIRDYF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - HSERDYF
    #[inline(always)]
    pub fn hserdyf(&self) -> HSERDYF_R {
        HSERDYF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - CSIRDYF
    #[inline(always)]
    pub fn csirdyf(&self) -> CSIRDYF_R {
        CSIRDYF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - PLL1DYF
    #[inline(always)]
    pub fn pll1dyf(&self) -> PLL1DYF_R {
        PLL1DYF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - PLL2DYF
    #[inline(always)]
    pub fn pll2dyf(&self) -> PLL2DYF_R {
        PLL2DYF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - PLL3DYF
    #[inline(always)]
    pub fn pll3dyf(&self) -> PLL3DYF_R {
        PLL3DYF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - PLL4DYF
    #[inline(always)]
    pub fn pll4dyf(&self) -> PLL4DYF_R {
        PLL4DYF_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 16 - LSECSSF
    #[inline(always)]
    pub fn lsecssf(&self) -> LSECSSF_R {
        LSECSSF_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 20 - WKUPF
    #[inline(always)]
    pub fn wkupf(&self) -> WKUPF_R {
        WKUPF_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - LSIRDYF
    #[inline(always)]
    #[must_use]
    pub fn lsirdyf(&mut self) -> LSIRDYF_W<0> {
        LSIRDYF_W::new(self)
    }
    ///Bit 1 - LSERDYF
    #[inline(always)]
    #[must_use]
    pub fn lserdyf(&mut self) -> LSERDYF_W<1> {
        LSERDYF_W::new(self)
    }
    ///Bit 2 - HSIRDYF
    #[inline(always)]
    #[must_use]
    pub fn hsirdyf(&mut self) -> HSIRDYF_W<2> {
        HSIRDYF_W::new(self)
    }
    ///Bit 3 - HSERDYF
    #[inline(always)]
    #[must_use]
    pub fn hserdyf(&mut self) -> HSERDYF_W<3> {
        HSERDYF_W::new(self)
    }
    ///Bit 4 - CSIRDYF
    #[inline(always)]
    #[must_use]
    pub fn csirdyf(&mut self) -> CSIRDYF_W<4> {
        CSIRDYF_W::new(self)
    }
    ///Bit 8 - PLL1DYF
    #[inline(always)]
    #[must_use]
    pub fn pll1dyf(&mut self) -> PLL1DYF_W<8> {
        PLL1DYF_W::new(self)
    }
    ///Bit 9 - PLL2DYF
    #[inline(always)]
    #[must_use]
    pub fn pll2dyf(&mut self) -> PLL2DYF_W<9> {
        PLL2DYF_W::new(self)
    }
    ///Bit 10 - PLL3DYF
    #[inline(always)]
    #[must_use]
    pub fn pll3dyf(&mut self) -> PLL3DYF_W<10> {
        PLL3DYF_W::new(self)
    }
    ///Bit 11 - PLL4DYF
    #[inline(always)]
    #[must_use]
    pub fn pll4dyf(&mut self) -> PLL4DYF_W<11> {
        PLL4DYF_W::new(self)
    }
    ///Bit 16 - LSECSSF
    #[inline(always)]
    #[must_use]
    pub fn lsecssf(&mut self) -> LSECSSF_W<16> {
        LSECSSF_W::new(self)
    }
    ///Bit 20 - WKUPF
    #[inline(always)]
    #[must_use]
    pub fn wkupf(&mut self) -> WKUPF_W<20> {
        WKUPF_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///This register shall be used by the MPU in order to read and clear the interrupt flags.Writing has no effect, writing will clear the corresponding flag.Refer to Section10.5: RCC interrupts for more details. If TZEN = , this register can only be modified in secure mode.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rcc_mp_cifr](index.html) module
pub struct RCC_MP_CIFR_SPEC;
impl crate::RegisterSpec for RCC_MP_CIFR_SPEC {
    type Ux = u32;
}
///`read()` method returns [rcc_mp_cifr::R](R) reader structure
impl crate::Readable for RCC_MP_CIFR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rcc_mp_cifr::W](W) writer structure
impl crate::Writable for RCC_MP_CIFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RCC_MP_CIFR to value 0
impl crate::Resettable for RCC_MP_CIFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
