///Register `ADC_OFR3` reader
pub struct R(crate::R<ADC_OFR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_OFR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_OFR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_OFR3_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ADC_OFR3` writer
pub struct W(crate::W<ADC_OFR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_OFR3_SPEC>;
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
impl From<crate::W<ADC_OFR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_OFR3_SPEC>) -> Self {
        W(writer)
    }
}
///Field `OFFSET3` reader - OFFSET3
pub type OFFSET3_R = crate::FieldReader<u32, u32>;
///Field `OFFSET3` writer - OFFSET3
pub type OFFSET3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADC_OFR3_SPEC, u32, u32, 26, O>;
///Field `OFFSET3_CH` reader - OFFSET3_CH
pub type OFFSET3_CH_R = crate::FieldReader<u8, u8>;
///Field `OFFSET3_CH` writer - OFFSET3_CH
pub type OFFSET3_CH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADC_OFR3_SPEC, u8, u8, 5, O>;
///Field `SSATE` reader - SSATE
pub type SSATE_R = crate::BitReader<bool>;
///Field `SSATE` writer - SSATE
pub type SSATE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_OFR3_SPEC, bool, O>;
impl R {
    ///Bits 0:25 - OFFSET3
    #[inline(always)]
    pub fn offset3(&self) -> OFFSET3_R {
        OFFSET3_R::new(self.bits & 0x03ff_ffff)
    }
    ///Bits 26:30 - OFFSET3_CH
    #[inline(always)]
    pub fn offset3_ch(&self) -> OFFSET3_CH_R {
        OFFSET3_CH_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    ///Bit 31 - SSATE
    #[inline(always)]
    pub fn ssate(&self) -> SSATE_R {
        SSATE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 0:25 - OFFSET3
    #[inline(always)]
    #[must_use]
    pub fn offset3(&mut self) -> OFFSET3_W<0> {
        OFFSET3_W::new(self)
    }
    ///Bits 26:30 - OFFSET3_CH
    #[inline(always)]
    #[must_use]
    pub fn offset3_ch(&mut self) -> OFFSET3_CH_W<26> {
        OFFSET3_CH_W::new(self)
    }
    ///Bit 31 - SSATE
    #[inline(always)]
    #[must_use]
    pub fn ssate(&mut self) -> SSATE_W<31> {
        SSATE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///ADC offset register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [adc_ofr3](index.html) module
pub struct ADC_OFR3_SPEC;
impl crate::RegisterSpec for ADC_OFR3_SPEC {
    type Ux = u32;
}
///`read()` method returns [adc_ofr3::R](R) reader structure
impl crate::Readable for ADC_OFR3_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [adc_ofr3::W](W) writer structure
impl crate::Writable for ADC_OFR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ADC_OFR3 to value 0
impl crate::Resettable for ADC_OFR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
