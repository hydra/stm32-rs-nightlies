///Register `OR` reader
pub struct R(crate::R<OR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `OR` writer
pub struct W(crate::W<OR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OR_SPEC>;
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
impl From<crate::W<OR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TIM8_ETR_ADC2_RMP` reader - TIM8_ETR_ADC2 remapping capability
pub type TIM8_ETR_ADC2_RMP_R = crate::FieldReader<u8, u8>;
///Field `TIM8_ETR_ADC2_RMP` writer - TIM8_ETR_ADC2 remapping capability
pub type TIM8_ETR_ADC2_RMP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OR_SPEC, u8, u8, 2, O>;
///Field `TIM8_ETR_ADC3_RMP` reader - TIM8_ETR_ADC3 remapping capability
pub type TIM8_ETR_ADC3_RMP_R = crate::FieldReader<u8, u8>;
///Field `TIM8_ETR_ADC3_RMP` writer - TIM8_ETR_ADC3 remapping capability
pub type TIM8_ETR_ADC3_RMP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OR_SPEC, u8, u8, 2, O>;
impl R {
    ///Bits 0:1 - TIM8_ETR_ADC2 remapping capability
    #[inline(always)]
    pub fn tim8_etr_adc2_rmp(&self) -> TIM8_ETR_ADC2_RMP_R {
        TIM8_ETR_ADC2_RMP_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - TIM8_ETR_ADC3 remapping capability
    #[inline(always)]
    pub fn tim8_etr_adc3_rmp(&self) -> TIM8_ETR_ADC3_RMP_R {
        TIM8_ETR_ADC3_RMP_R::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    ///Bits 0:1 - TIM8_ETR_ADC2 remapping capability
    #[inline(always)]
    #[must_use]
    pub fn tim8_etr_adc2_rmp(&mut self) -> TIM8_ETR_ADC2_RMP_W<0> {
        TIM8_ETR_ADC2_RMP_W::new(self)
    }
    ///Bits 2:3 - TIM8_ETR_ADC3 remapping capability
    #[inline(always)]
    #[must_use]
    pub fn tim8_etr_adc3_rmp(&mut self) -> TIM8_ETR_ADC3_RMP_W<2> {
        TIM8_ETR_ADC3_RMP_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///option registers
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [or](index.html) module
pub struct OR_SPEC;
impl crate::RegisterSpec for OR_SPEC {
    type Ux = u32;
}
///`read()` method returns [or::R](R) reader structure
impl crate::Readable for OR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [or::W](W) writer structure
impl crate::Writable for OR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets OR to value 0
impl crate::Resettable for OR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
