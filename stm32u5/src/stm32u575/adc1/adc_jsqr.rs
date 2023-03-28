///Register `ADC_JSQR` reader
pub struct R(crate::R<ADC_JSQR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_JSQR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_JSQR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_JSQR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ADC_JSQR` writer
pub struct W(crate::W<ADC_JSQR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_JSQR_SPEC>;
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
impl From<crate::W<ADC_JSQR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_JSQR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `JL` reader - Injected channel sequence length These bits are written by software to define the total number of conversions in the injected channel conversion sequence. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing.
pub type JL_R = crate::FieldReader<u8, u8>;
///Field `JL` writer - Injected channel sequence length These bits are written by software to define the total number of conversions in the injected channel conversion sequence. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing.
pub type JL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADC_JSQR_SPEC, u8, u8, 2, O>;
///Field `JEXTSEL` reader - External trigger selection for injected group These bits select the external event used to trigger the start of conversion of an injected group: ... Refer to the ADC external trigger for injected channels in internal signals for details on trigger mapping. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing.
pub type JEXTSEL_R = crate::FieldReader<u8, u8>;
///Field `JEXTSEL` writer - External trigger selection for injected group These bits select the external event used to trigger the start of conversion of an injected group: ... Refer to the ADC external trigger for injected channels in internal signals for details on trigger mapping. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing.
pub type JEXTSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADC_JSQR_SPEC, u8, u8, 5, O>;
///Field `JEXTEN` reader - External trigger enable and polarity selection for injected channels These bits are set and cleared by software to select the external trigger polarity and enable the trigger of an injected group. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing.
pub type JEXTEN_R = crate::FieldReader<u8, u8>;
///Field `JEXTEN` writer - External trigger enable and polarity selection for injected channels These bits are set and cleared by software to select the external trigger polarity and enable the trigger of an injected group. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing.
pub type JEXTEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADC_JSQR_SPEC, u8, u8, 2, O>;
///Field `JSQ1` reader - 1st conversion in the injected sequence These bits are written by software with the channel number (0..19) assigned as the 1st in the injected conversion sequence. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing.
pub type JSQ1_R = crate::FieldReader<u8, u8>;
///Field `JSQ1` writer - 1st conversion in the injected sequence These bits are written by software with the channel number (0..19) assigned as the 1st in the injected conversion sequence. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing.
pub type JSQ1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADC_JSQR_SPEC, u8, u8, 5, O>;
///Field `JSQ2` reader - 2nd conversion in the injected sequence These bits are written by software with the channel number (0..19) assigned as the 2nd in the injected conversion sequence. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing.
pub type JSQ2_R = crate::FieldReader<u8, u8>;
///Field `JSQ2` writer - 2nd conversion in the injected sequence These bits are written by software with the channel number (0..19) assigned as the 2nd in the injected conversion sequence. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing.
pub type JSQ2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADC_JSQR_SPEC, u8, u8, 5, O>;
///Field `JSQ3` reader - 3rd conversion in the injected sequence These bits are written by software with the channel number (0..19) assigned as the 3rd in the injected conversion sequence. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing.
pub type JSQ3_R = crate::FieldReader<u8, u8>;
///Field `JSQ3` writer - 3rd conversion in the injected sequence These bits are written by software with the channel number (0..19) assigned as the 3rd in the injected conversion sequence. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing.
pub type JSQ3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADC_JSQR_SPEC, u8, u8, 5, O>;
///Field `JSQ4` reader - 4th conversion in the injected sequence These bits are written by software with the channel number (0..19) assigned as the 4th in the injected conversion sequence. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing.
pub type JSQ4_R = crate::FieldReader<u8, u8>;
///Field `JSQ4` writer - 4th conversion in the injected sequence These bits are written by software with the channel number (0..19) assigned as the 4th in the injected conversion sequence. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing.
pub type JSQ4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADC_JSQR_SPEC, u8, u8, 5, O>;
impl R {
    ///Bits 0:1 - Injected channel sequence length These bits are written by software to define the total number of conversions in the injected channel conversion sequence. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing.
    #[inline(always)]
    pub fn jl(&self) -> JL_R {
        JL_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:6 - External trigger selection for injected group These bits select the external event used to trigger the start of conversion of an injected group: ... Refer to the ADC external trigger for injected channels in internal signals for details on trigger mapping. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing.
    #[inline(always)]
    pub fn jextsel(&self) -> JEXTSEL_R {
        JEXTSEL_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    ///Bits 7:8 - External trigger enable and polarity selection for injected channels These bits are set and cleared by software to select the external trigger polarity and enable the trigger of an injected group. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing.
    #[inline(always)]
    pub fn jexten(&self) -> JEXTEN_R {
        JEXTEN_R::new(((self.bits >> 7) & 3) as u8)
    }
    ///Bits 9:13 - 1st conversion in the injected sequence These bits are written by software with the channel number (0..19) assigned as the 1st in the injected conversion sequence. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing.
    #[inline(always)]
    pub fn jsq1(&self) -> JSQ1_R {
        JSQ1_R::new(((self.bits >> 9) & 0x1f) as u8)
    }
    ///Bits 15:19 - 2nd conversion in the injected sequence These bits are written by software with the channel number (0..19) assigned as the 2nd in the injected conversion sequence. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing.
    #[inline(always)]
    pub fn jsq2(&self) -> JSQ2_R {
        JSQ2_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    ///Bits 21:25 - 3rd conversion in the injected sequence These bits are written by software with the channel number (0..19) assigned as the 3rd in the injected conversion sequence. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing.
    #[inline(always)]
    pub fn jsq3(&self) -> JSQ3_R {
        JSQ3_R::new(((self.bits >> 21) & 0x1f) as u8)
    }
    ///Bits 27:31 - 4th conversion in the injected sequence These bits are written by software with the channel number (0..19) assigned as the 4th in the injected conversion sequence. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing.
    #[inline(always)]
    pub fn jsq4(&self) -> JSQ4_R {
        JSQ4_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    ///Bits 0:1 - Injected channel sequence length These bits are written by software to define the total number of conversions in the injected channel conversion sequence. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing.
    #[inline(always)]
    #[must_use]
    pub fn jl(&mut self) -> JL_W<0> {
        JL_W::new(self)
    }
    ///Bits 2:6 - External trigger selection for injected group These bits select the external event used to trigger the start of conversion of an injected group: ... Refer to the ADC external trigger for injected channels in internal signals for details on trigger mapping. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing.
    #[inline(always)]
    #[must_use]
    pub fn jextsel(&mut self) -> JEXTSEL_W<2> {
        JEXTSEL_W::new(self)
    }
    ///Bits 7:8 - External trigger enable and polarity selection for injected channels These bits are set and cleared by software to select the external trigger polarity and enable the trigger of an injected group. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing.
    #[inline(always)]
    #[must_use]
    pub fn jexten(&mut self) -> JEXTEN_W<7> {
        JEXTEN_W::new(self)
    }
    ///Bits 9:13 - 1st conversion in the injected sequence These bits are written by software with the channel number (0..19) assigned as the 1st in the injected conversion sequence. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing.
    #[inline(always)]
    #[must_use]
    pub fn jsq1(&mut self) -> JSQ1_W<9> {
        JSQ1_W::new(self)
    }
    ///Bits 15:19 - 2nd conversion in the injected sequence These bits are written by software with the channel number (0..19) assigned as the 2nd in the injected conversion sequence. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing.
    #[inline(always)]
    #[must_use]
    pub fn jsq2(&mut self) -> JSQ2_W<15> {
        JSQ2_W::new(self)
    }
    ///Bits 21:25 - 3rd conversion in the injected sequence These bits are written by software with the channel number (0..19) assigned as the 3rd in the injected conversion sequence. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing.
    #[inline(always)]
    #[must_use]
    pub fn jsq3(&mut self) -> JSQ3_W<21> {
        JSQ3_W::new(self)
    }
    ///Bits 27:31 - 4th conversion in the injected sequence These bits are written by software with the channel number (0..19) assigned as the 4th in the injected conversion sequence. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing.
    #[inline(always)]
    #[must_use]
    pub fn jsq4(&mut self) -> JSQ4_W<27> {
        JSQ4_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///ADC injected sequence register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [adc_jsqr](index.html) module
pub struct ADC_JSQR_SPEC;
impl crate::RegisterSpec for ADC_JSQR_SPEC {
    type Ux = u32;
}
///`read()` method returns [adc_jsqr::R](R) reader structure
impl crate::Readable for ADC_JSQR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [adc_jsqr::W](W) writer structure
impl crate::Writable for ADC_JSQR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ADC_JSQR to value 0
impl crate::Resettable for ADC_JSQR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
