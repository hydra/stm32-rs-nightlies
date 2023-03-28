///Register `ADC_LTR1` reader
pub struct R(crate::R<ADC_LTR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_LTR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_LTR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_LTR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ADC_LTR1` writer
pub struct W(crate::W<ADC_LTR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_LTR1_SPEC>;
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
impl From<crate::W<ADC_LTR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_LTR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `LTR1` reader - LTR1
pub type LTR1_R = crate::FieldReader<u32, u32>;
///Field `LTR1` writer - LTR1
pub type LTR1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADC_LTR1_SPEC, u32, u32, 26, O>;
impl R {
    ///Bits 0:25 - LTR1
    #[inline(always)]
    pub fn ltr1(&self) -> LTR1_R {
        LTR1_R::new(self.bits & 0x03ff_ffff)
    }
}
impl W {
    ///Bits 0:25 - LTR1
    #[inline(always)]
    #[must_use]
    pub fn ltr1(&mut self) -> LTR1_W<0> {
        LTR1_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///ADC watchdog threshold register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [adc_ltr1](index.html) module
pub struct ADC_LTR1_SPEC;
impl crate::RegisterSpec for ADC_LTR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [adc_ltr1::R](R) reader structure
impl crate::Readable for ADC_LTR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [adc_ltr1::W](W) writer structure
impl crate::Writable for ADC_LTR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ADC_LTR1 to value 0
impl crate::Resettable for ADC_LTR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
