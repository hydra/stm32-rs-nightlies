///Register `CR` reader
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CR` writer
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `HSION` reader - Internal high-speed clock enable
pub type HSION_R = crate::BitReader<HSION_A>;
///Internal high-speed clock enable
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSION_A {
    ///0: Clock Off
    Off = 0,
    ///1: Clock On
    On = 1,
}
impl From<HSION_A> for bool {
    #[inline(always)]
    fn from(variant: HSION_A) -> Self {
        variant as u8 != 0
    }
}
impl HSION_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> HSION_A {
        match self.bits {
            false => HSION_A::Off,
            true => HSION_A::On,
        }
    }
    ///Checks if the value of the field is `Off`
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == HSION_A::Off
    }
    ///Checks if the value of the field is `On`
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == HSION_A::On
    }
}
///Field `HSION` writer - Internal high-speed clock enable
pub type HSION_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, HSION_A, O>;
impl<'a, const O: u8> HSION_W<'a, O> {
    ///Clock Off
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(HSION_A::Off)
    }
    ///Clock On
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(HSION_A::On)
    }
}
///Field `HSIKERON` reader - High Speed Internal clock enable in Stop mode
pub use HSION_R as HSIKERON_R;
///Field `HSIKERON` writer - High Speed Internal clock enable in Stop mode
pub use HSION_W as HSIKERON_W;
///Field `HSIRDY` reader - HSI clock ready flag
pub type HSIRDY_R = crate::BitReader<HSIRDYR_A>;
///HSI clock ready flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSIRDYR_A {
    ///0: Clock not ready
    NotReady = 0,
    ///1: Clock ready
    Ready = 1,
}
impl From<HSIRDYR_A> for bool {
    #[inline(always)]
    fn from(variant: HSIRDYR_A) -> Self {
        variant as u8 != 0
    }
}
impl HSIRDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> HSIRDYR_A {
        match self.bits {
            false => HSIRDYR_A::NotReady,
            true => HSIRDYR_A::Ready,
        }
    }
    ///Checks if the value of the field is `NotReady`
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == HSIRDYR_A::NotReady
    }
    ///Checks if the value of the field is `Ready`
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == HSIRDYR_A::Ready
    }
}
///Field `HSIRDY` writer - HSI clock ready flag
pub type HSIRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, HSIRDYR_A, O>;
impl<'a, const O: u8> HSIRDY_W<'a, O> {
    ///Clock not ready
    #[inline(always)]
    pub fn not_ready(self) -> &'a mut W {
        self.variant(HSIRDYR_A::NotReady)
    }
    ///Clock ready
    #[inline(always)]
    pub fn ready(self) -> &'a mut W {
        self.variant(HSIRDYR_A::Ready)
    }
}
///Field `HSIDIV` reader - HSI clock divider
pub type HSIDIV_R = crate::FieldReader<u8, HSIDIV_A>;
///HSI clock divider
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HSIDIV_A {
    ///0: No division
    Div1 = 0,
    ///1: Division by 2
    Div2 = 1,
    ///2: Division by 4
    Div4 = 2,
    ///3: Division by 8
    Div8 = 3,
}
impl From<HSIDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: HSIDIV_A) -> Self {
        variant as _
    }
}
impl HSIDIV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> HSIDIV_A {
        match self.bits {
            0 => HSIDIV_A::Div1,
            1 => HSIDIV_A::Div2,
            2 => HSIDIV_A::Div4,
            3 => HSIDIV_A::Div8,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Div1`
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == HSIDIV_A::Div1
    }
    ///Checks if the value of the field is `Div2`
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == HSIDIV_A::Div2
    }
    ///Checks if the value of the field is `Div4`
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == HSIDIV_A::Div4
    }
    ///Checks if the value of the field is `Div8`
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == HSIDIV_A::Div8
    }
}
///Field `HSIDIV` writer - HSI clock divider
pub type HSIDIV_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CR_SPEC, u8, HSIDIV_A, 2, O>;
impl<'a, const O: u8> HSIDIV_W<'a, O> {
    ///No division
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(HSIDIV_A::Div1)
    }
    ///Division by 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(HSIDIV_A::Div2)
    }
    ///Division by 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(HSIDIV_A::Div4)
    }
    ///Division by 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(HSIDIV_A::Div8)
    }
}
///Field `HSIDIVF` reader - HSI divider flag
pub type HSIDIVF_R = crate::BitReader<HSIDIVFR_A>;
///HSI divider flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSIDIVFR_A {
    ///0: New HSIDIV ratio has not yet propagated to hsi_ck
    NotPropagated = 0,
    ///1: HSIDIV ratio has propagated to hsi_ck
    Propagated = 1,
}
impl From<HSIDIVFR_A> for bool {
    #[inline(always)]
    fn from(variant: HSIDIVFR_A) -> Self {
        variant as u8 != 0
    }
}
impl HSIDIVF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> HSIDIVFR_A {
        match self.bits {
            false => HSIDIVFR_A::NotPropagated,
            true => HSIDIVFR_A::Propagated,
        }
    }
    ///Checks if the value of the field is `NotPropagated`
    #[inline(always)]
    pub fn is_not_propagated(&self) -> bool {
        *self == HSIDIVFR_A::NotPropagated
    }
    ///Checks if the value of the field is `Propagated`
    #[inline(always)]
    pub fn is_propagated(&self) -> bool {
        *self == HSIDIVFR_A::Propagated
    }
}
///Field `HSIDIVF` writer - HSI divider flag
pub type HSIDIVF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, HSIDIVFR_A, O>;
impl<'a, const O: u8> HSIDIVF_W<'a, O> {
    ///New HSIDIV ratio has not yet propagated to hsi_ck
    #[inline(always)]
    pub fn not_propagated(self) -> &'a mut W {
        self.variant(HSIDIVFR_A::NotPropagated)
    }
    ///HSIDIV ratio has propagated to hsi_ck
    #[inline(always)]
    pub fn propagated(self) -> &'a mut W {
        self.variant(HSIDIVFR_A::Propagated)
    }
}
///Field `CSION` reader - CSI clock enable
pub use HSION_R as CSION_R;
///Field `CSIKERON` reader - CSI clock enable in Stop mode
pub use HSION_R as CSIKERON_R;
///Field `HSI48ON` reader - RC48 clock enable
pub use HSION_R as HSI48ON_R;
///Field `HSEON` reader - HSE clock enable
pub use HSION_R as HSEON_R;
///Field `CSION` writer - CSI clock enable
pub use HSION_W as CSION_W;
///Field `CSIKERON` writer - CSI clock enable in Stop mode
pub use HSION_W as CSIKERON_W;
///Field `HSI48ON` writer - RC48 clock enable
pub use HSION_W as HSI48ON_W;
///Field `HSEON` writer - HSE clock enable
pub use HSION_W as HSEON_W;
///Field `CSIRDY` reader - CSI clock ready flag
pub use HSIRDY_R as CSIRDY_R;
///Field `HSI48RDY` reader - RC48 clock ready flag
pub use HSIRDY_R as HSI48RDY_R;
///Field `D1CKRDY` reader - D1 domain clocks ready flag
pub use HSIRDY_R as D1CKRDY_R;
///Field `D2CKRDY` reader - D2 domain clocks ready flag
pub use HSIRDY_R as D2CKRDY_R;
///Field `HSERDY` reader - HSE clock ready flag
pub use HSIRDY_R as HSERDY_R;
///Field `CSIRDY` writer - CSI clock ready flag
pub use HSIRDY_W as CSIRDY_W;
///Field `HSI48RDY` writer - RC48 clock ready flag
pub use HSIRDY_W as HSI48RDY_W;
///Field `D1CKRDY` writer - D1 domain clocks ready flag
pub use HSIRDY_W as D1CKRDY_W;
///Field `D2CKRDY` writer - D2 domain clocks ready flag
pub use HSIRDY_W as D2CKRDY_W;
///Field `HSERDY` writer - HSE clock ready flag
pub use HSIRDY_W as HSERDY_W;
///Field `HSEBYP` reader - HSE clock bypass
pub type HSEBYP_R = crate::BitReader<HSEBYP_A>;
///HSE clock bypass
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSEBYP_A {
    ///0: HSE crystal oscillator not bypassed
    NotBypassed = 0,
    ///1: HSE crystal oscillator bypassed with external clock
    Bypassed = 1,
}
impl From<HSEBYP_A> for bool {
    #[inline(always)]
    fn from(variant: HSEBYP_A) -> Self {
        variant as u8 != 0
    }
}
impl HSEBYP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> HSEBYP_A {
        match self.bits {
            false => HSEBYP_A::NotBypassed,
            true => HSEBYP_A::Bypassed,
        }
    }
    ///Checks if the value of the field is `NotBypassed`
    #[inline(always)]
    pub fn is_not_bypassed(&self) -> bool {
        *self == HSEBYP_A::NotBypassed
    }
    ///Checks if the value of the field is `Bypassed`
    #[inline(always)]
    pub fn is_bypassed(&self) -> bool {
        *self == HSEBYP_A::Bypassed
    }
}
///Field `HSEBYP` writer - HSE clock bypass
pub type HSEBYP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, HSEBYP_A, O>;
impl<'a, const O: u8> HSEBYP_W<'a, O> {
    ///HSE crystal oscillator not bypassed
    #[inline(always)]
    pub fn not_bypassed(self) -> &'a mut W {
        self.variant(HSEBYP_A::NotBypassed)
    }
    ///HSE crystal oscillator bypassed with external clock
    #[inline(always)]
    pub fn bypassed(self) -> &'a mut W {
        self.variant(HSEBYP_A::Bypassed)
    }
}
///Field `HSECSSON` reader - HSE Clock Security System enable
pub use HSION_R as HSECSSON_R;
///Field `PLL1ON` reader - PLL1 enable
pub use HSION_R as PLL1ON_R;
///Field `PLL2ON` reader - PLL2 enable
pub use HSION_R as PLL2ON_R;
///Field `PLL3ON` reader - PLL3 enable
pub use HSION_R as PLL3ON_R;
///Field `HSECSSON` writer - HSE Clock Security System enable
pub use HSION_W as HSECSSON_W;
///Field `PLL1ON` writer - PLL1 enable
pub use HSION_W as PLL1ON_W;
///Field `PLL2ON` writer - PLL2 enable
pub use HSION_W as PLL2ON_W;
///Field `PLL3ON` writer - PLL3 enable
pub use HSION_W as PLL3ON_W;
///Field `PLL1RDY` reader - PLL1 clock ready flag
pub use HSIRDY_R as PLL1RDY_R;
///Field `PLL2RDY` reader - PLL2 clock ready flag
pub use HSIRDY_R as PLL2RDY_R;
///Field `PLL3RDY` reader - PLL3 clock ready flag
pub use HSIRDY_R as PLL3RDY_R;
///Field `PLL1RDY` writer - PLL1 clock ready flag
pub use HSIRDY_W as PLL1RDY_W;
///Field `PLL2RDY` writer - PLL2 clock ready flag
pub use HSIRDY_W as PLL2RDY_W;
///Field `PLL3RDY` writer - PLL3 clock ready flag
pub use HSIRDY_W as PLL3RDY_W;
impl R {
    ///Bit 0 - Internal high-speed clock enable
    #[inline(always)]
    pub fn hsion(&self) -> HSION_R {
        HSION_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - High Speed Internal clock enable in Stop mode
    #[inline(always)]
    pub fn hsikeron(&self) -> HSIKERON_R {
        HSIKERON_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - HSI clock ready flag
    #[inline(always)]
    pub fn hsirdy(&self) -> HSIRDY_R {
        HSIRDY_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:4 - HSI clock divider
    #[inline(always)]
    pub fn hsidiv(&self) -> HSIDIV_R {
        HSIDIV_R::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bit 5 - HSI divider flag
    #[inline(always)]
    pub fn hsidivf(&self) -> HSIDIVF_R {
        HSIDIVF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 7 - CSI clock enable
    #[inline(always)]
    pub fn csion(&self) -> CSION_R {
        CSION_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - CSI clock ready flag
    #[inline(always)]
    pub fn csirdy(&self) -> CSIRDY_R {
        CSIRDY_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - CSI clock enable in Stop mode
    #[inline(always)]
    pub fn csikeron(&self) -> CSIKERON_R {
        CSIKERON_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 12 - RC48 clock enable
    #[inline(always)]
    pub fn hsi48on(&self) -> HSI48ON_R {
        HSI48ON_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - RC48 clock ready flag
    #[inline(always)]
    pub fn hsi48rdy(&self) -> HSI48RDY_R {
        HSI48RDY_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - D1 domain clocks ready flag
    #[inline(always)]
    pub fn d1ckrdy(&self) -> D1CKRDY_R {
        D1CKRDY_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - D2 domain clocks ready flag
    #[inline(always)]
    pub fn d2ckrdy(&self) -> D2CKRDY_R {
        D2CKRDY_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - HSE clock enable
    #[inline(always)]
    pub fn hseon(&self) -> HSEON_R {
        HSEON_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - HSE clock ready flag
    #[inline(always)]
    pub fn hserdy(&self) -> HSERDY_R {
        HSERDY_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - HSE clock bypass
    #[inline(always)]
    pub fn hsebyp(&self) -> HSEBYP_R {
        HSEBYP_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - HSE Clock Security System enable
    #[inline(always)]
    pub fn hsecsson(&self) -> HSECSSON_R {
        HSECSSON_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 24 - PLL1 enable
    #[inline(always)]
    pub fn pll1on(&self) -> PLL1ON_R {
        PLL1ON_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - PLL1 clock ready flag
    #[inline(always)]
    pub fn pll1rdy(&self) -> PLL1RDY_R {
        PLL1RDY_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - PLL2 enable
    #[inline(always)]
    pub fn pll2on(&self) -> PLL2ON_R {
        PLL2ON_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - PLL2 clock ready flag
    #[inline(always)]
    pub fn pll2rdy(&self) -> PLL2RDY_R {
        PLL2RDY_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - PLL3 enable
    #[inline(always)]
    pub fn pll3on(&self) -> PLL3ON_R {
        PLL3ON_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - PLL3 clock ready flag
    #[inline(always)]
    pub fn pll3rdy(&self) -> PLL3RDY_R {
        PLL3RDY_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Internal high-speed clock enable
    #[inline(always)]
    #[must_use]
    pub fn hsion(&mut self) -> HSION_W<0> {
        HSION_W::new(self)
    }
    ///Bit 1 - High Speed Internal clock enable in Stop mode
    #[inline(always)]
    #[must_use]
    pub fn hsikeron(&mut self) -> HSIKERON_W<1> {
        HSIKERON_W::new(self)
    }
    ///Bit 2 - HSI clock ready flag
    #[inline(always)]
    #[must_use]
    pub fn hsirdy(&mut self) -> HSIRDY_W<2> {
        HSIRDY_W::new(self)
    }
    ///Bits 3:4 - HSI clock divider
    #[inline(always)]
    #[must_use]
    pub fn hsidiv(&mut self) -> HSIDIV_W<3> {
        HSIDIV_W::new(self)
    }
    ///Bit 5 - HSI divider flag
    #[inline(always)]
    #[must_use]
    pub fn hsidivf(&mut self) -> HSIDIVF_W<5> {
        HSIDIVF_W::new(self)
    }
    ///Bit 7 - CSI clock enable
    #[inline(always)]
    #[must_use]
    pub fn csion(&mut self) -> CSION_W<7> {
        CSION_W::new(self)
    }
    ///Bit 8 - CSI clock ready flag
    #[inline(always)]
    #[must_use]
    pub fn csirdy(&mut self) -> CSIRDY_W<8> {
        CSIRDY_W::new(self)
    }
    ///Bit 9 - CSI clock enable in Stop mode
    #[inline(always)]
    #[must_use]
    pub fn csikeron(&mut self) -> CSIKERON_W<9> {
        CSIKERON_W::new(self)
    }
    ///Bit 12 - RC48 clock enable
    #[inline(always)]
    #[must_use]
    pub fn hsi48on(&mut self) -> HSI48ON_W<12> {
        HSI48ON_W::new(self)
    }
    ///Bit 13 - RC48 clock ready flag
    #[inline(always)]
    #[must_use]
    pub fn hsi48rdy(&mut self) -> HSI48RDY_W<13> {
        HSI48RDY_W::new(self)
    }
    ///Bit 14 - D1 domain clocks ready flag
    #[inline(always)]
    #[must_use]
    pub fn d1ckrdy(&mut self) -> D1CKRDY_W<14> {
        D1CKRDY_W::new(self)
    }
    ///Bit 15 - D2 domain clocks ready flag
    #[inline(always)]
    #[must_use]
    pub fn d2ckrdy(&mut self) -> D2CKRDY_W<15> {
        D2CKRDY_W::new(self)
    }
    ///Bit 16 - HSE clock enable
    #[inline(always)]
    #[must_use]
    pub fn hseon(&mut self) -> HSEON_W<16> {
        HSEON_W::new(self)
    }
    ///Bit 17 - HSE clock ready flag
    #[inline(always)]
    #[must_use]
    pub fn hserdy(&mut self) -> HSERDY_W<17> {
        HSERDY_W::new(self)
    }
    ///Bit 18 - HSE clock bypass
    #[inline(always)]
    #[must_use]
    pub fn hsebyp(&mut self) -> HSEBYP_W<18> {
        HSEBYP_W::new(self)
    }
    ///Bit 19 - HSE Clock Security System enable
    #[inline(always)]
    #[must_use]
    pub fn hsecsson(&mut self) -> HSECSSON_W<19> {
        HSECSSON_W::new(self)
    }
    ///Bit 24 - PLL1 enable
    #[inline(always)]
    #[must_use]
    pub fn pll1on(&mut self) -> PLL1ON_W<24> {
        PLL1ON_W::new(self)
    }
    ///Bit 25 - PLL1 clock ready flag
    #[inline(always)]
    #[must_use]
    pub fn pll1rdy(&mut self) -> PLL1RDY_W<25> {
        PLL1RDY_W::new(self)
    }
    ///Bit 26 - PLL2 enable
    #[inline(always)]
    #[must_use]
    pub fn pll2on(&mut self) -> PLL2ON_W<26> {
        PLL2ON_W::new(self)
    }
    ///Bit 27 - PLL2 clock ready flag
    #[inline(always)]
    #[must_use]
    pub fn pll2rdy(&mut self) -> PLL2RDY_W<27> {
        PLL2RDY_W::new(self)
    }
    ///Bit 28 - PLL3 enable
    #[inline(always)]
    #[must_use]
    pub fn pll3on(&mut self) -> PLL3ON_W<28> {
        PLL3ON_W::new(self)
    }
    ///Bit 29 - PLL3 clock ready flag
    #[inline(always)]
    #[must_use]
    pub fn pll3rdy(&mut self) -> PLL3RDY_W<29> {
        PLL3RDY_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///clock control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr](index.html) module
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
///`read()` method returns [cr::R](R) reader structure
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cr::W](W) writer structure
impl crate::Writable for CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CR to value 0x83
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0x83;
}
