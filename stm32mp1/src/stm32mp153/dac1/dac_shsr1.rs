///Register `DAC_SHSR1` reader
pub struct R(crate::R<DAC_SHSR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAC_SHSR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAC_SHSR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAC_SHSR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DAC_SHSR1` writer
pub struct W(crate::W<DAC_SHSR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAC_SHSR1_SPEC>;
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
impl From<crate::W<DAC_SHSR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAC_SHSR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TSAMPLE1` reader - TSAMPLE1
pub type TSAMPLE1_R = crate::FieldReader<u16, u16>;
///Field `TSAMPLE1` writer - TSAMPLE1
pub type TSAMPLE1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DAC_SHSR1_SPEC, u16, u16, 10, O>;
impl R {
    ///Bits 0:9 - TSAMPLE1
    #[inline(always)]
    pub fn tsample1(&self) -> TSAMPLE1_R {
        TSAMPLE1_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    ///Bits 0:9 - TSAMPLE1
    #[inline(always)]
    #[must_use]
    pub fn tsample1(&mut self) -> TSAMPLE1_W<0> {
        TSAMPLE1_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DAC channel 1 sample and hold sample time register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dac_shsr1](index.html) module
pub struct DAC_SHSR1_SPEC;
impl crate::RegisterSpec for DAC_SHSR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [dac_shsr1::R](R) reader structure
impl crate::Readable for DAC_SHSR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dac_shsr1::W](W) writer structure
impl crate::Writable for DAC_SHSR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DAC_SHSR1 to value 0
impl crate::Resettable for DAC_SHSR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
