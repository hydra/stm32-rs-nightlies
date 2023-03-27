///Register `ADC_HTR1` reader
pub struct R(crate::R<ADC_HTR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_HTR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_HTR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_HTR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ADC_HTR1` writer
pub struct W(crate::W<ADC_HTR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_HTR1_SPEC>;
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
impl From<crate::W<ADC_HTR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_HTR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `HTR1` reader - Analog watchdog 1 higher threshold These bits are written by software to define the higher threshold for the analog watchdog 1. Refer to AWD2CH, AWD3CH, AWD_HTRy, AWD_LTRy, AWDy).
pub type HTR1_R = crate::FieldReader<u32, u32>;
///Field `HTR1` writer - Analog watchdog 1 higher threshold These bits are written by software to define the higher threshold for the analog watchdog 1. Refer to AWD2CH, AWD3CH, AWD_HTRy, AWD_LTRy, AWDy).
pub type HTR1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADC_HTR1_SPEC, u32, u32, 25, O>;
///Field `AWDFILT1` reader - Analog watchdog filtering parameter This bit is set and cleared by software. ... Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type AWDFILT1_R = crate::FieldReader<u8, u8>;
///Field `AWDFILT1` writer - Analog watchdog filtering parameter This bit is set and cleared by software. ... Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type AWDFILT1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADC_HTR1_SPEC, u8, u8, 3, O>;
impl R {
    ///Bits 0:24 - Analog watchdog 1 higher threshold These bits are written by software to define the higher threshold for the analog watchdog 1. Refer to AWD2CH, AWD3CH, AWD_HTRy, AWD_LTRy, AWDy).
    #[inline(always)]
    pub fn htr1(&self) -> HTR1_R {
        HTR1_R::new(self.bits & 0x01ff_ffff)
    }
    ///Bits 29:31 - Analog watchdog filtering parameter This bit is set and cleared by software. ... Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awdfilt1(&self) -> AWDFILT1_R {
        AWDFILT1_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    ///Bits 0:24 - Analog watchdog 1 higher threshold These bits are written by software to define the higher threshold for the analog watchdog 1. Refer to AWD2CH, AWD3CH, AWD_HTRy, AWD_LTRy, AWDy).
    #[inline(always)]
    #[must_use]
    pub fn htr1(&mut self) -> HTR1_W<0> {
        HTR1_W::new(self)
    }
    ///Bits 29:31 - Analog watchdog filtering parameter This bit is set and cleared by software. ... Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    #[must_use]
    pub fn awdfilt1(&mut self) -> AWDFILT1_W<29> {
        AWDFILT1_W::new(self)
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
///For information about available fields see [adc_htr1](index.html) module
pub struct ADC_HTR1_SPEC;
impl crate::RegisterSpec for ADC_HTR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [adc_htr1::R](R) reader structure
impl crate::Readable for ADC_HTR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [adc_htr1::W](W) writer structure
impl crate::Writable for ADC_HTR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ADC_HTR1 to value 0x01ff_ffff
impl crate::Resettable for ADC_HTR1_SPEC {
    const RESET_VALUE: Self::Ux = 0x01ff_ffff;
}
