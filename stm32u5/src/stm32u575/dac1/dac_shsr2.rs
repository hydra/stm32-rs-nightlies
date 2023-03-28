///Register `DAC_SHSR2` reader
pub struct R(crate::R<DAC_SHSR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAC_SHSR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAC_SHSR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAC_SHSR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DAC_SHSR2` writer
pub struct W(crate::W<DAC_SHSR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAC_SHSR2_SPEC>;
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
impl From<crate::W<DAC_SHSR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAC_SHSR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TSAMPLE2` reader - DAC Channel 2 sample Time (only valid in sample and hold mode)
pub type TSAMPLE2_R = crate::FieldReader<u16, u16>;
///Field `TSAMPLE2` writer - DAC Channel 2 sample Time (only valid in sample and hold mode)
pub type TSAMPLE2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DAC_SHSR2_SPEC, u16, u16, 10, O>;
impl R {
    ///Bits 0:9 - DAC Channel 2 sample Time (only valid in sample and hold mode)
    #[inline(always)]
    pub fn tsample2(&self) -> TSAMPLE2_R {
        TSAMPLE2_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    ///Bits 0:9 - DAC Channel 2 sample Time (only valid in sample and hold mode)
    #[inline(always)]
    #[must_use]
    pub fn tsample2(&mut self) -> TSAMPLE2_W<0> {
        TSAMPLE2_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DAC channel2 sample and hold sample time register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dac_shsr2](index.html) module
pub struct DAC_SHSR2_SPEC;
impl crate::RegisterSpec for DAC_SHSR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [dac_shsr2::R](R) reader structure
impl crate::Readable for DAC_SHSR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dac_shsr2::W](W) writer structure
impl crate::Writable for DAC_SHSR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DAC_SHSR2 to value 0
impl crate::Resettable for DAC_SHSR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
