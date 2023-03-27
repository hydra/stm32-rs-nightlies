///Register `ADC_AWD1TR` reader
pub struct R(crate::R<ADC_AWD1TR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_AWD1TR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_AWD1TR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_AWD1TR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ADC_AWD1TR` writer
pub struct W(crate::W<ADC_AWD1TR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_AWD1TR_SPEC>;
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
impl From<crate::W<ADC_AWD1TR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_AWD1TR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `LT1` reader - LT1
pub type LT1_R = crate::FieldReader<u16, u16>;
///Field `LT1` writer - LT1
pub type LT1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADC_AWD1TR_SPEC, u16, u16, 12, O>;
///Field `HT1` reader - HT1
pub type HT1_R = crate::FieldReader<u16, u16>;
///Field `HT1` writer - HT1
pub type HT1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADC_AWD1TR_SPEC, u16, u16, 12, O>;
impl R {
    ///Bits 0:11 - LT1
    #[inline(always)]
    pub fn lt1(&self) -> LT1_R {
        LT1_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 16:27 - HT1
    #[inline(always)]
    pub fn ht1(&self) -> HT1_R {
        HT1_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    ///Bits 0:11 - LT1
    #[inline(always)]
    #[must_use]
    pub fn lt1(&mut self) -> LT1_W<0> {
        LT1_W::new(self)
    }
    ///Bits 16:27 - HT1
    #[inline(always)]
    #[must_use]
    pub fn ht1(&mut self) -> HT1_W<16> {
        HT1_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///ADC watchdog threshold register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [adc_awd1tr](index.html) module
pub struct ADC_AWD1TR_SPEC;
impl crate::RegisterSpec for ADC_AWD1TR_SPEC {
    type Ux = u32;
}
///`read()` method returns [adc_awd1tr::R](R) reader structure
impl crate::Readable for ADC_AWD1TR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [adc_awd1tr::W](W) writer structure
impl crate::Writable for ADC_AWD1TR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ADC_AWD1TR to value 0x0fff_0000
impl crate::Resettable for ADC_AWD1TR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0fff_0000;
}
