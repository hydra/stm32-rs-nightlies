///Register `AHB2RSTR` reader
pub struct R(crate::R<AHB2RSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB2RSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB2RSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB2RSTR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `AHB2RSTR` writer
pub struct W(crate::W<AHB2RSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB2RSTR_SPEC>;
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
impl From<crate::W<AHB2RSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB2RSTR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `GPIOARST` reader - IO port A reset
pub type GPIOARST_R = crate::BitReader<GPIOARST_A>;
///IO port A reset
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOARST_A {
    ///1: Reset the selected module
    Reset = 1,
}
impl From<GPIOARST_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOARST_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIOARST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<GPIOARST_A> {
        match self.bits {
            true => Some(GPIOARST_A::Reset),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Reset`
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == GPIOARST_A::Reset
    }
}
///Field `GPIOARST` writer - IO port A reset
pub type GPIOARST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2RSTR_SPEC, GPIOARST_A, O>;
impl<'a, const O: u8> GPIOARST_W<'a, O> {
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(GPIOARST_A::Reset)
    }
}
///Field `GPIOBRST` reader - IO port B reset
pub use GPIOARST_R as GPIOBRST_R;
///Field `GPIOCRST` reader - IO port C reset
pub use GPIOARST_R as GPIOCRST_R;
///Field `GPIODRST` reader - IO port D reset
pub use GPIOARST_R as GPIODRST_R;
///Field `GPIOERST` reader - IO port E reset
pub use GPIOARST_R as GPIOERST_R;
///Field `GPIOFRST` reader - IO port F reset
pub use GPIOARST_R as GPIOFRST_R;
///Field `GPIOGRST` reader - IO port G reset
pub use GPIOARST_R as GPIOGRST_R;
///Field `ADC12RST` reader - ADC reset
pub use GPIOARST_R as ADC12RST_R;
///Field `ADC345RST` reader - SAR ADC345 interface reset
pub use GPIOARST_R as ADC345RST_R;
///Field `DAC1RST` reader - DAC1 interface reset
pub use GPIOARST_R as DAC1RST_R;
///Field `DAC2RST` reader - DAC2 interface reset
pub use GPIOARST_R as DAC2RST_R;
///Field `DAC3RST` reader - DAC3 interface reset
pub use GPIOARST_R as DAC3RST_R;
///Field `DAC4RST` reader - DAC4 interface reset
pub use GPIOARST_R as DAC4RST_R;
///Field `AESRST` reader - Cryptography module reset
pub use GPIOARST_R as AESRST_R;
///Field `RNGRST` reader - Random Number Generator module reset
pub use GPIOARST_R as RNGRST_R;
///Field `GPIOBRST` writer - IO port B reset
pub use GPIOARST_W as GPIOBRST_W;
///Field `GPIOCRST` writer - IO port C reset
pub use GPIOARST_W as GPIOCRST_W;
///Field `GPIODRST` writer - IO port D reset
pub use GPIOARST_W as GPIODRST_W;
///Field `GPIOERST` writer - IO port E reset
pub use GPIOARST_W as GPIOERST_W;
///Field `GPIOFRST` writer - IO port F reset
pub use GPIOARST_W as GPIOFRST_W;
///Field `GPIOGRST` writer - IO port G reset
pub use GPIOARST_W as GPIOGRST_W;
///Field `ADC12RST` writer - ADC reset
pub use GPIOARST_W as ADC12RST_W;
///Field `ADC345RST` writer - SAR ADC345 interface reset
pub use GPIOARST_W as ADC345RST_W;
///Field `DAC1RST` writer - DAC1 interface reset
pub use GPIOARST_W as DAC1RST_W;
///Field `DAC2RST` writer - DAC2 interface reset
pub use GPIOARST_W as DAC2RST_W;
///Field `DAC3RST` writer - DAC3 interface reset
pub use GPIOARST_W as DAC3RST_W;
///Field `DAC4RST` writer - DAC4 interface reset
pub use GPIOARST_W as DAC4RST_W;
///Field `AESRST` writer - Cryptography module reset
pub use GPIOARST_W as AESRST_W;
///Field `RNGRST` writer - Random Number Generator module reset
pub use GPIOARST_W as RNGRST_W;
impl R {
    ///Bit 0 - IO port A reset
    #[inline(always)]
    pub fn gpioarst(&self) -> GPIOARST_R {
        GPIOARST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - IO port B reset
    #[inline(always)]
    pub fn gpiobrst(&self) -> GPIOBRST_R {
        GPIOBRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - IO port C reset
    #[inline(always)]
    pub fn gpiocrst(&self) -> GPIOCRST_R {
        GPIOCRST_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - IO port D reset
    #[inline(always)]
    pub fn gpiodrst(&self) -> GPIODRST_R {
        GPIODRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - IO port E reset
    #[inline(always)]
    pub fn gpioerst(&self) -> GPIOERST_R {
        GPIOERST_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - IO port F reset
    #[inline(always)]
    pub fn gpiofrst(&self) -> GPIOFRST_R {
        GPIOFRST_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - IO port G reset
    #[inline(always)]
    pub fn gpiogrst(&self) -> GPIOGRST_R {
        GPIOGRST_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 13 - ADC reset
    #[inline(always)]
    pub fn adc12rst(&self) -> ADC12RST_R {
        ADC12RST_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - SAR ADC345 interface reset
    #[inline(always)]
    pub fn adc345rst(&self) -> ADC345RST_R {
        ADC345RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - DAC1 interface reset
    #[inline(always)]
    pub fn dac1rst(&self) -> DAC1RST_R {
        DAC1RST_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - DAC2 interface reset
    #[inline(always)]
    pub fn dac2rst(&self) -> DAC2RST_R {
        DAC2RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - DAC3 interface reset
    #[inline(always)]
    pub fn dac3rst(&self) -> DAC3RST_R {
        DAC3RST_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - DAC4 interface reset
    #[inline(always)]
    pub fn dac4rst(&self) -> DAC4RST_R {
        DAC4RST_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 24 - Cryptography module reset
    #[inline(always)]
    pub fn aesrst(&self) -> AESRST_R {
        AESRST_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 26 - Random Number Generator module reset
    #[inline(always)]
    pub fn rngrst(&self) -> RNGRST_R {
        RNGRST_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - IO port A reset
    #[inline(always)]
    #[must_use]
    pub fn gpioarst(&mut self) -> GPIOARST_W<0> {
        GPIOARST_W::new(self)
    }
    ///Bit 1 - IO port B reset
    #[inline(always)]
    #[must_use]
    pub fn gpiobrst(&mut self) -> GPIOBRST_W<1> {
        GPIOBRST_W::new(self)
    }
    ///Bit 2 - IO port C reset
    #[inline(always)]
    #[must_use]
    pub fn gpiocrst(&mut self) -> GPIOCRST_W<2> {
        GPIOCRST_W::new(self)
    }
    ///Bit 3 - IO port D reset
    #[inline(always)]
    #[must_use]
    pub fn gpiodrst(&mut self) -> GPIODRST_W<3> {
        GPIODRST_W::new(self)
    }
    ///Bit 4 - IO port E reset
    #[inline(always)]
    #[must_use]
    pub fn gpioerst(&mut self) -> GPIOERST_W<4> {
        GPIOERST_W::new(self)
    }
    ///Bit 5 - IO port F reset
    #[inline(always)]
    #[must_use]
    pub fn gpiofrst(&mut self) -> GPIOFRST_W<5> {
        GPIOFRST_W::new(self)
    }
    ///Bit 6 - IO port G reset
    #[inline(always)]
    #[must_use]
    pub fn gpiogrst(&mut self) -> GPIOGRST_W<6> {
        GPIOGRST_W::new(self)
    }
    ///Bit 13 - ADC reset
    #[inline(always)]
    #[must_use]
    pub fn adc12rst(&mut self) -> ADC12RST_W<13> {
        ADC12RST_W::new(self)
    }
    ///Bit 14 - SAR ADC345 interface reset
    #[inline(always)]
    #[must_use]
    pub fn adc345rst(&mut self) -> ADC345RST_W<14> {
        ADC345RST_W::new(self)
    }
    ///Bit 16 - DAC1 interface reset
    #[inline(always)]
    #[must_use]
    pub fn dac1rst(&mut self) -> DAC1RST_W<16> {
        DAC1RST_W::new(self)
    }
    ///Bit 17 - DAC2 interface reset
    #[inline(always)]
    #[must_use]
    pub fn dac2rst(&mut self) -> DAC2RST_W<17> {
        DAC2RST_W::new(self)
    }
    ///Bit 18 - DAC3 interface reset
    #[inline(always)]
    #[must_use]
    pub fn dac3rst(&mut self) -> DAC3RST_W<18> {
        DAC3RST_W::new(self)
    }
    ///Bit 19 - DAC4 interface reset
    #[inline(always)]
    #[must_use]
    pub fn dac4rst(&mut self) -> DAC4RST_W<19> {
        DAC4RST_W::new(self)
    }
    ///Bit 24 - Cryptography module reset
    #[inline(always)]
    #[must_use]
    pub fn aesrst(&mut self) -> AESRST_W<24> {
        AESRST_W::new(self)
    }
    ///Bit 26 - Random Number Generator module reset
    #[inline(always)]
    #[must_use]
    pub fn rngrst(&mut self) -> RNGRST_W<26> {
        RNGRST_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///AHB2 peripheral reset register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ahb2rstr](index.html) module
pub struct AHB2RSTR_SPEC;
impl crate::RegisterSpec for AHB2RSTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ahb2rstr::R](R) reader structure
impl crate::Readable for AHB2RSTR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ahb2rstr::W](W) writer structure
impl crate::Writable for AHB2RSTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets AHB2RSTR to value 0
impl crate::Resettable for AHB2RSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
