///Register `AHBRSTR` reader
pub struct R(crate::R<AHBRSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHBRSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHBRSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHBRSTR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `AHBRSTR` writer
pub struct W(crate::W<AHBRSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHBRSTR_SPEC>;
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
impl From<crate::W<AHBRSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHBRSTR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `FMCRST` reader - FMC reset
pub type FMCRST_R = crate::BitReader<FMCRST_A>;
///FMC reset
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FMCRST_A {
    ///1: Reset the selected module
    Reset = 1,
}
impl From<FMCRST_A> for bool {
    #[inline(always)]
    fn from(variant: FMCRST_A) -> Self {
        variant as u8 != 0
    }
}
impl FMCRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<FMCRST_A> {
        match self.bits {
            true => Some(FMCRST_A::Reset),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Reset`
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == FMCRST_A::Reset
    }
}
///Field `FMCRST` writer - FMC reset
pub type FMCRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBRSTR_SPEC, FMCRST_A, O>;
impl<'a, const O: u8> FMCRST_W<'a, O> {
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(FMCRST_A::Reset)
    }
}
///Field `IOPHRST` reader - IO port H reset
pub use FMCRST_R as IOPHRST_R;
///Field `IOPARST` reader - I/O port A reset
pub use FMCRST_R as IOPARST_R;
///Field `IOPBRST` reader - I/O port B reset
pub use FMCRST_R as IOPBRST_R;
///Field `IOPCRST` reader - I/O port C reset
pub use FMCRST_R as IOPCRST_R;
///Field `IOPDRST` reader - I/O port D reset
pub use FMCRST_R as IOPDRST_R;
///Field `IOPERST` reader - I/O port E reset
pub use FMCRST_R as IOPERST_R;
///Field `IOPFRST` reader - I/O port F reset
pub use FMCRST_R as IOPFRST_R;
///Field `IOPGRST` reader - IO port G reset
pub use FMCRST_R as IOPGRST_R;
///Field `TSCRST` reader - Touch sensing controller reset
pub use FMCRST_R as TSCRST_R;
///Field `ADC12RST` reader - ADC1 and ADC2 reset
pub use FMCRST_R as ADC12RST_R;
///Field `IOPHRST` writer - IO port H reset
pub use FMCRST_W as IOPHRST_W;
///Field `IOPARST` writer - I/O port A reset
pub use FMCRST_W as IOPARST_W;
///Field `IOPBRST` writer - I/O port B reset
pub use FMCRST_W as IOPBRST_W;
///Field `IOPCRST` writer - I/O port C reset
pub use FMCRST_W as IOPCRST_W;
///Field `IOPDRST` writer - I/O port D reset
pub use FMCRST_W as IOPDRST_W;
///Field `IOPERST` writer - I/O port E reset
pub use FMCRST_W as IOPERST_W;
///Field `IOPFRST` writer - I/O port F reset
pub use FMCRST_W as IOPFRST_W;
///Field `IOPGRST` writer - IO port G reset
pub use FMCRST_W as IOPGRST_W;
///Field `TSCRST` writer - Touch sensing controller reset
pub use FMCRST_W as TSCRST_W;
///Field `ADC12RST` writer - ADC1 and ADC2 reset
pub use FMCRST_W as ADC12RST_W;
impl R {
    ///Bit 5 - FMC reset
    #[inline(always)]
    pub fn fmcrst(&self) -> FMCRST_R {
        FMCRST_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 16 - IO port H reset
    #[inline(always)]
    pub fn iophrst(&self) -> IOPHRST_R {
        IOPHRST_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - I/O port A reset
    #[inline(always)]
    pub fn ioparst(&self) -> IOPARST_R {
        IOPARST_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - I/O port B reset
    #[inline(always)]
    pub fn iopbrst(&self) -> IOPBRST_R {
        IOPBRST_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - I/O port C reset
    #[inline(always)]
    pub fn iopcrst(&self) -> IOPCRST_R {
        IOPCRST_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - I/O port D reset
    #[inline(always)]
    pub fn iopdrst(&self) -> IOPDRST_R {
        IOPDRST_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - I/O port E reset
    #[inline(always)]
    pub fn ioperst(&self) -> IOPERST_R {
        IOPERST_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - I/O port F reset
    #[inline(always)]
    pub fn iopfrst(&self) -> IOPFRST_R {
        IOPFRST_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - IO port G reset
    #[inline(always)]
    pub fn iopgrst(&self) -> IOPGRST_R {
        IOPGRST_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Touch sensing controller reset
    #[inline(always)]
    pub fn tscrst(&self) -> TSCRST_R {
        TSCRST_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 28 - ADC1 and ADC2 reset
    #[inline(always)]
    pub fn adc12rst(&self) -> ADC12RST_R {
        ADC12RST_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    ///Bit 5 - FMC reset
    #[inline(always)]
    #[must_use]
    pub fn fmcrst(&mut self) -> FMCRST_W<5> {
        FMCRST_W::new(self)
    }
    ///Bit 16 - IO port H reset
    #[inline(always)]
    #[must_use]
    pub fn iophrst(&mut self) -> IOPHRST_W<16> {
        IOPHRST_W::new(self)
    }
    ///Bit 17 - I/O port A reset
    #[inline(always)]
    #[must_use]
    pub fn ioparst(&mut self) -> IOPARST_W<17> {
        IOPARST_W::new(self)
    }
    ///Bit 18 - I/O port B reset
    #[inline(always)]
    #[must_use]
    pub fn iopbrst(&mut self) -> IOPBRST_W<18> {
        IOPBRST_W::new(self)
    }
    ///Bit 19 - I/O port C reset
    #[inline(always)]
    #[must_use]
    pub fn iopcrst(&mut self) -> IOPCRST_W<19> {
        IOPCRST_W::new(self)
    }
    ///Bit 20 - I/O port D reset
    #[inline(always)]
    #[must_use]
    pub fn iopdrst(&mut self) -> IOPDRST_W<20> {
        IOPDRST_W::new(self)
    }
    ///Bit 21 - I/O port E reset
    #[inline(always)]
    #[must_use]
    pub fn ioperst(&mut self) -> IOPERST_W<21> {
        IOPERST_W::new(self)
    }
    ///Bit 22 - I/O port F reset
    #[inline(always)]
    #[must_use]
    pub fn iopfrst(&mut self) -> IOPFRST_W<22> {
        IOPFRST_W::new(self)
    }
    ///Bit 23 - IO port G reset
    #[inline(always)]
    #[must_use]
    pub fn iopgrst(&mut self) -> IOPGRST_W<23> {
        IOPGRST_W::new(self)
    }
    ///Bit 24 - Touch sensing controller reset
    #[inline(always)]
    #[must_use]
    pub fn tscrst(&mut self) -> TSCRST_W<24> {
        TSCRST_W::new(self)
    }
    ///Bit 28 - ADC1 and ADC2 reset
    #[inline(always)]
    #[must_use]
    pub fn adc12rst(&mut self) -> ADC12RST_W<28> {
        ADC12RST_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///AHB peripheral reset register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ahbrstr](index.html) module
pub struct AHBRSTR_SPEC;
impl crate::RegisterSpec for AHBRSTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ahbrstr::R](R) reader structure
impl crate::Readable for AHBRSTR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ahbrstr::W](W) writer structure
impl crate::Writable for AHBRSTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets AHBRSTR to value 0
impl crate::Resettable for AHBRSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
