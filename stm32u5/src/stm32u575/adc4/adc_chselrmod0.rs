///Register `ADC_CHSELRMOD0` reader
pub struct R(crate::R<ADC_CHSELRMOD0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_CHSELRMOD0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_CHSELRMOD0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_CHSELRMOD0_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ADC_CHSELRMOD0` writer
pub struct W(crate::W<ADC_CHSELRMOD0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_CHSELRMOD0_SPEC>;
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
impl From<crate::W<ADC_CHSELRMOD0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_CHSELRMOD0_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CHSEL` reader - CHSEL
pub type CHSEL_R = crate::FieldReader<u32, u32>;
///Field `CHSEL` writer - CHSEL
pub type CHSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADC_CHSELRMOD0_SPEC, u32, u32, 24, O>;
impl R {
    ///Bits 0:23 - CHSEL
    #[inline(always)]
    pub fn chsel(&self) -> CHSEL_R {
        CHSEL_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    ///Bits 0:23 - CHSEL
    #[inline(always)]
    #[must_use]
    pub fn chsel(&mut self) -> CHSEL_W<0> {
        CHSEL_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///ADC channel selection register \[alternate\]
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [adc_chselrmod0](index.html) module
pub struct ADC_CHSELRMOD0_SPEC;
impl crate::RegisterSpec for ADC_CHSELRMOD0_SPEC {
    type Ux = u32;
}
///`read()` method returns [adc_chselrmod0::R](R) reader structure
impl crate::Readable for ADC_CHSELRMOD0_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [adc_chselrmod0::W](W) writer structure
impl crate::Writable for ADC_CHSELRMOD0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ADC_CHSELRMOD0 to value 0
impl crate::Resettable for ADC_CHSELRMOD0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
