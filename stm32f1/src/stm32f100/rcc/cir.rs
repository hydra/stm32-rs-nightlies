///Register `CIR` reader
pub struct R(crate::R<CIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CIR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CIR` writer
pub struct W(crate::W<CIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CIR_SPEC>;
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
impl From<crate::W<CIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CIR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `LSIRDYF` reader - LSI Ready Interrupt flag
pub type LSIRDYF_R = crate::BitReader<LSIRDYFR_A>;
///LSI Ready Interrupt flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSIRDYFR_A {
    ///0: No clock ready interrupt
    NotInterrupted = 0,
    ///1: Clock ready interrupt
    Interrupted = 1,
}
impl From<LSIRDYFR_A> for bool {
    #[inline(always)]
    fn from(variant: LSIRDYFR_A) -> Self {
        variant as u8 != 0
    }
}
impl LSIRDYF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LSIRDYFR_A {
        match self.bits {
            false => LSIRDYFR_A::NotInterrupted,
            true => LSIRDYFR_A::Interrupted,
        }
    }
    ///Checks if the value of the field is `NotInterrupted`
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == LSIRDYFR_A::NotInterrupted
    }
    ///Checks if the value of the field is `Interrupted`
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == LSIRDYFR_A::Interrupted
    }
}
///Field `LSERDYF` reader - LSE Ready Interrupt flag
pub use LSIRDYF_R as LSERDYF_R;
///Field `HSIRDYF` reader - HSI Ready Interrupt flag
pub use LSIRDYF_R as HSIRDYF_R;
///Field `HSERDYF` reader - HSE Ready Interrupt flag
pub use LSIRDYF_R as HSERDYF_R;
///Field `PLLRDYF` reader - PLL Ready Interrupt flag
pub use LSIRDYF_R as PLLRDYF_R;
///Field `CSSF` reader - Clock Security System Interrupt flag
pub type CSSF_R = crate::BitReader<CSSFR_A>;
///Clock Security System Interrupt flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSSFR_A {
    ///0: No clock security interrupt caused by HSE clock failure
    NotInterrupted = 0,
    ///1: Clock security interrupt caused by HSE clock failure
    Interrupted = 1,
}
impl From<CSSFR_A> for bool {
    #[inline(always)]
    fn from(variant: CSSFR_A) -> Self {
        variant as u8 != 0
    }
}
impl CSSF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CSSFR_A {
        match self.bits {
            false => CSSFR_A::NotInterrupted,
            true => CSSFR_A::Interrupted,
        }
    }
    ///Checks if the value of the field is `NotInterrupted`
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == CSSFR_A::NotInterrupted
    }
    ///Checks if the value of the field is `Interrupted`
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == CSSFR_A::Interrupted
    }
}
///Field `LSIRDYIE` reader - LSI Ready Interrupt Enable
pub type LSIRDYIE_R = crate::BitReader<LSIRDYIE_A>;
///LSI Ready Interrupt Enable
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
///Field `LSIRDYIE` writer - LSI Ready Interrupt Enable
pub type LSIRDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIR_SPEC, LSIRDYIE_A, O>;
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
///Field `LSERDYIE` reader - LSE Ready Interrupt Enable
pub use LSIRDYIE_R as LSERDYIE_R;
///Field `HSIRDYIE` reader - HSI Ready Interrupt Enable
pub use LSIRDYIE_R as HSIRDYIE_R;
///Field `HSERDYIE` reader - HSE Ready Interrupt Enable
pub use LSIRDYIE_R as HSERDYIE_R;
///Field `PLLRDYIE` reader - PLL Ready Interrupt Enable
pub use LSIRDYIE_R as PLLRDYIE_R;
///Field `LSERDYIE` writer - LSE Ready Interrupt Enable
pub use LSIRDYIE_W as LSERDYIE_W;
///Field `HSIRDYIE` writer - HSI Ready Interrupt Enable
pub use LSIRDYIE_W as HSIRDYIE_W;
///Field `HSERDYIE` writer - HSE Ready Interrupt Enable
pub use LSIRDYIE_W as HSERDYIE_W;
///Field `PLLRDYIE` writer - PLL Ready Interrupt Enable
pub use LSIRDYIE_W as PLLRDYIE_W;
///LSI Ready Interrupt Clear
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSIRDYCW_AW {
    ///1: Clear interrupt flag
    Clear = 1,
}
impl From<LSIRDYCW_AW> for bool {
    #[inline(always)]
    fn from(variant: LSIRDYCW_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `LSIRDYC` writer - LSI Ready Interrupt Clear
pub type LSIRDYC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIR_SPEC, LSIRDYCW_AW, O>;
impl<'a, const O: u8> LSIRDYC_W<'a, O> {
    ///Clear interrupt flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(LSIRDYCW_AW::Clear)
    }
}
///Field `LSERDYC` writer - LSE Ready Interrupt Clear
pub use LSIRDYC_W as LSERDYC_W;
///Field `HSIRDYC` writer - HSI Ready Interrupt Clear
pub use LSIRDYC_W as HSIRDYC_W;
///Field `HSERDYC` writer - HSE Ready Interrupt Clear
pub use LSIRDYC_W as HSERDYC_W;
///Field `PLLRDYC` writer - PLL Ready Interrupt Clear
pub use LSIRDYC_W as PLLRDYC_W;
///Clock security system interrupt clear
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSSCW_AW {
    ///1: Clear CSSF flag
    Clear = 1,
}
impl From<CSSCW_AW> for bool {
    #[inline(always)]
    fn from(variant: CSSCW_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `CSSC` writer - Clock security system interrupt clear
pub type CSSC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIR_SPEC, CSSCW_AW, O>;
impl<'a, const O: u8> CSSC_W<'a, O> {
    ///Clear CSSF flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CSSCW_AW::Clear)
    }
}
impl R {
    ///Bit 0 - LSI Ready Interrupt flag
    #[inline(always)]
    pub fn lsirdyf(&self) -> LSIRDYF_R {
        LSIRDYF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LSE Ready Interrupt flag
    #[inline(always)]
    pub fn lserdyf(&self) -> LSERDYF_R {
        LSERDYF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - HSI Ready Interrupt flag
    #[inline(always)]
    pub fn hsirdyf(&self) -> HSIRDYF_R {
        HSIRDYF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - HSE Ready Interrupt flag
    #[inline(always)]
    pub fn hserdyf(&self) -> HSERDYF_R {
        HSERDYF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - PLL Ready Interrupt flag
    #[inline(always)]
    pub fn pllrdyf(&self) -> PLLRDYF_R {
        PLLRDYF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 7 - Clock Security System Interrupt flag
    #[inline(always)]
    pub fn cssf(&self) -> CSSF_R {
        CSSF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - LSI Ready Interrupt Enable
    #[inline(always)]
    pub fn lsirdyie(&self) -> LSIRDYIE_R {
        LSIRDYIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - LSE Ready Interrupt Enable
    #[inline(always)]
    pub fn lserdyie(&self) -> LSERDYIE_R {
        LSERDYIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - HSI Ready Interrupt Enable
    #[inline(always)]
    pub fn hsirdyie(&self) -> HSIRDYIE_R {
        HSIRDYIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - HSE Ready Interrupt Enable
    #[inline(always)]
    pub fn hserdyie(&self) -> HSERDYIE_R {
        HSERDYIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - PLL Ready Interrupt Enable
    #[inline(always)]
    pub fn pllrdyie(&self) -> PLLRDYIE_R {
        PLLRDYIE_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    ///Bit 8 - LSI Ready Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn lsirdyie(&mut self) -> LSIRDYIE_W<8> {
        LSIRDYIE_W::new(self)
    }
    ///Bit 9 - LSE Ready Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn lserdyie(&mut self) -> LSERDYIE_W<9> {
        LSERDYIE_W::new(self)
    }
    ///Bit 10 - HSI Ready Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn hsirdyie(&mut self) -> HSIRDYIE_W<10> {
        HSIRDYIE_W::new(self)
    }
    ///Bit 11 - HSE Ready Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn hserdyie(&mut self) -> HSERDYIE_W<11> {
        HSERDYIE_W::new(self)
    }
    ///Bit 12 - PLL Ready Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn pllrdyie(&mut self) -> PLLRDYIE_W<12> {
        PLLRDYIE_W::new(self)
    }
    ///Bit 16 - LSI Ready Interrupt Clear
    #[inline(always)]
    #[must_use]
    pub fn lsirdyc(&mut self) -> LSIRDYC_W<16> {
        LSIRDYC_W::new(self)
    }
    ///Bit 17 - LSE Ready Interrupt Clear
    #[inline(always)]
    #[must_use]
    pub fn lserdyc(&mut self) -> LSERDYC_W<17> {
        LSERDYC_W::new(self)
    }
    ///Bit 18 - HSI Ready Interrupt Clear
    #[inline(always)]
    #[must_use]
    pub fn hsirdyc(&mut self) -> HSIRDYC_W<18> {
        HSIRDYC_W::new(self)
    }
    ///Bit 19 - HSE Ready Interrupt Clear
    #[inline(always)]
    #[must_use]
    pub fn hserdyc(&mut self) -> HSERDYC_W<19> {
        HSERDYC_W::new(self)
    }
    ///Bit 20 - PLL Ready Interrupt Clear
    #[inline(always)]
    #[must_use]
    pub fn pllrdyc(&mut self) -> PLLRDYC_W<20> {
        PLLRDYC_W::new(self)
    }
    ///Bit 23 - Clock security system interrupt clear
    #[inline(always)]
    #[must_use]
    pub fn cssc(&mut self) -> CSSC_W<23> {
        CSSC_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Clock interrupt register (RCC_CIR)
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cir](index.html) module
pub struct CIR_SPEC;
impl crate::RegisterSpec for CIR_SPEC {
    type Ux = u32;
}
///`read()` method returns [cir::R](R) reader structure
impl crate::Readable for CIR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cir::W](W) writer structure
impl crate::Writable for CIR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CIR to value 0
impl crate::Resettable for CIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
