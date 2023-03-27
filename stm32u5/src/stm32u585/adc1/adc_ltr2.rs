///Register `ADC_LTR2` reader
pub struct R(crate::R<ADC_LTR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_LTR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_LTR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_LTR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ADC_LTR2` writer
pub struct W(crate::W<ADC_LTR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_LTR2_SPEC>;
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
impl From<crate::W<ADC_LTR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_LTR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `LTR2` reader - Analog watchdog 2 lower threshold These bits are written by software to define the lower threshold for the analog watchdog 2. Refer to AWD2CH, AWD3CH, AWD_HTRy, AWD_LTRy, AWDy).
pub type LTR2_R = crate::FieldReader<u32, u32>;
///Field `LTR2` writer - Analog watchdog 2 lower threshold These bits are written by software to define the lower threshold for the analog watchdog 2. Refer to AWD2CH, AWD3CH, AWD_HTRy, AWD_LTRy, AWDy).
pub type LTR2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADC_LTR2_SPEC, u32, u32, 25, O>;
impl R {
    ///Bits 0:24 - Analog watchdog 2 lower threshold These bits are written by software to define the lower threshold for the analog watchdog 2. Refer to AWD2CH, AWD3CH, AWD_HTRy, AWD_LTRy, AWDy).
    #[inline(always)]
    pub fn ltr2(&self) -> LTR2_R {
        LTR2_R::new(self.bits & 0x01ff_ffff)
    }
}
impl W {
    ///Bits 0:24 - Analog watchdog 2 lower threshold These bits are written by software to define the lower threshold for the analog watchdog 2. Refer to AWD2CH, AWD3CH, AWD_HTRy, AWD_LTRy, AWDy).
    #[inline(always)]
    #[must_use]
    pub fn ltr2(&mut self) -> LTR2_W<0> {
        LTR2_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///ADC watchdog lower threshold register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [adc_ltr2](index.html) module
pub struct ADC_LTR2_SPEC;
impl crate::RegisterSpec for ADC_LTR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [adc_ltr2::R](R) reader structure
impl crate::Readable for ADC_LTR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [adc_ltr2::W](W) writer structure
impl crate::Writable for ADC_LTR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ADC_LTR2 to value 0
impl crate::Resettable for ADC_LTR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
