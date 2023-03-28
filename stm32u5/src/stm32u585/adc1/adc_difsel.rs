///Register `ADC_DIFSEL` reader
pub struct R(crate::R<ADC_DIFSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_DIFSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_DIFSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_DIFSEL_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ADC_DIFSEL` writer
pub struct W(crate::W<ADC_DIFSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_DIFSEL_SPEC>;
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
impl From<crate::W<ADC_DIFSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_DIFSEL_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DIFSEL` reader - Differential mode for channels 19 to 0 These bits are set and cleared by software. They allow selecting if a channel is configured as single ended or differential mode. DIFSEL\[i\]
///= 0: ADC analog input channel-i is configured in single ended mode DIFSEL\[i\]
///= 1: ADC analog input channel-i is configured in differential mode Note: The software is allowed to write these bits only when the ADC is disabled (ADCAL = 0, JADSTART = 0, JADSTP = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).
pub type DIFSEL_R = crate::FieldReader<u32, u32>;
///Field `DIFSEL` writer - Differential mode for channels 19 to 0 These bits are set and cleared by software. They allow selecting if a channel is configured as single ended or differential mode. DIFSEL\[i\]
///= 0: ADC analog input channel-i is configured in single ended mode DIFSEL\[i\]
///= 1: ADC analog input channel-i is configured in differential mode Note: The software is allowed to write these bits only when the ADC is disabled (ADCAL = 0, JADSTART = 0, JADSTP = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).
pub type DIFSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADC_DIFSEL_SPEC, u32, u32, 20, O>;
impl R {
    ///Bits 0:19 - Differential mode for channels 19 to 0 These bits are set and cleared by software. They allow selecting if a channel is configured as single ended or differential mode. DIFSEL\[i\]
    ///= 0: ADC analog input channel-i is configured in single ended mode DIFSEL\[i\]
    ///= 1: ADC analog input channel-i is configured in differential mode Note: The software is allowed to write these bits only when the ADC is disabled (ADCAL = 0, JADSTART = 0, JADSTP = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).
    #[inline(always)]
    pub fn difsel(&self) -> DIFSEL_R {
        DIFSEL_R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    ///Bits 0:19 - Differential mode for channels 19 to 0 These bits are set and cleared by software. They allow selecting if a channel is configured as single ended or differential mode. DIFSEL\[i\]
    ///= 0: ADC analog input channel-i is configured in single ended mode DIFSEL\[i\]
    ///= 1: ADC analog input channel-i is configured in differential mode Note: The software is allowed to write these bits only when the ADC is disabled (ADCAL = 0, JADSTART = 0, JADSTP = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).
    #[inline(always)]
    #[must_use]
    pub fn difsel(&mut self) -> DIFSEL_W<0> {
        DIFSEL_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///ADC differential mode selection register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [adc_difsel](index.html) module
pub struct ADC_DIFSEL_SPEC;
impl crate::RegisterSpec for ADC_DIFSEL_SPEC {
    type Ux = u32;
}
///`read()` method returns [adc_difsel::R](R) reader structure
impl crate::Readable for ADC_DIFSEL_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [adc_difsel::W](W) writer structure
impl crate::Writable for ADC_DIFSEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ADC_DIFSEL to value 0
impl crate::Resettable for ADC_DIFSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
