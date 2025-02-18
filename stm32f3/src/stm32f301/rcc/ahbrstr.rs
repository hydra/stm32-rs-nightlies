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
///Field `IOPARST` reader - I/O port A reset
pub type IOPARST_R = crate::BitReader<IOPARST_A>;
///I/O port A reset
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IOPARST_A {
    ///1: Reset the selected module
    Reset = 1,
}
impl From<IOPARST_A> for bool {
    #[inline(always)]
    fn from(variant: IOPARST_A) -> Self {
        variant as u8 != 0
    }
}
impl IOPARST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<IOPARST_A> {
        match self.bits {
            true => Some(IOPARST_A::Reset),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Reset`
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == IOPARST_A::Reset
    }
}
///Field `IOPARST` writer - I/O port A reset
pub type IOPARST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBRSTR_SPEC, IOPARST_A, O>;
impl<'a, const O: u8> IOPARST_W<'a, O> {
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(IOPARST_A::Reset)
    }
}
///Field `IOPBRST` reader - I/O port B reset
pub use IOPARST_R as IOPBRST_R;
///Field `IOPCRST` reader - I/O port C reset
pub use IOPARST_R as IOPCRST_R;
///Field `IOPDRST` reader - I/O port D reset
pub use IOPARST_R as IOPDRST_R;
///Field `IOPFRST` reader - I/O port F reset
pub use IOPARST_R as IOPFRST_R;
///Field `TSCRST` reader - Touch sensing controller reset
pub use IOPARST_R as TSCRST_R;
///Field `ADC1RST` reader - ADC1 reset
pub use IOPARST_R as ADC1RST_R;
///Field `IOPBRST` writer - I/O port B reset
pub use IOPARST_W as IOPBRST_W;
///Field `IOPCRST` writer - I/O port C reset
pub use IOPARST_W as IOPCRST_W;
///Field `IOPDRST` writer - I/O port D reset
pub use IOPARST_W as IOPDRST_W;
///Field `IOPFRST` writer - I/O port F reset
pub use IOPARST_W as IOPFRST_W;
///Field `TSCRST` writer - Touch sensing controller reset
pub use IOPARST_W as TSCRST_W;
///Field `ADC1RST` writer - ADC1 reset
pub use IOPARST_W as ADC1RST_W;
impl R {
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
    ///Bit 22 - I/O port F reset
    #[inline(always)]
    pub fn iopfrst(&self) -> IOPFRST_R {
        IOPFRST_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 24 - Touch sensing controller reset
    #[inline(always)]
    pub fn tscrst(&self) -> TSCRST_R {
        TSCRST_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 28 - ADC1 reset
    #[inline(always)]
    pub fn adc1rst(&self) -> ADC1RST_R {
        ADC1RST_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
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
    ///Bit 22 - I/O port F reset
    #[inline(always)]
    #[must_use]
    pub fn iopfrst(&mut self) -> IOPFRST_W<22> {
        IOPFRST_W::new(self)
    }
    ///Bit 24 - Touch sensing controller reset
    #[inline(always)]
    #[must_use]
    pub fn tscrst(&mut self) -> TSCRST_W<24> {
        TSCRST_W::new(self)
    }
    ///Bit 28 - ADC1 reset
    #[inline(always)]
    #[must_use]
    pub fn adc1rst(&mut self) -> ADC1RST_W<28> {
        ADC1RST_W::new(self)
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
