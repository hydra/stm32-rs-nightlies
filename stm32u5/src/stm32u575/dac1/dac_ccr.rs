///Register `DAC_CCR` reader
pub struct R(crate::R<DAC_CCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAC_CCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAC_CCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAC_CCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DAC_CCR` writer
pub struct W(crate::W<DAC_CCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAC_CCR_SPEC>;
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
impl From<crate::W<DAC_CCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAC_CCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `OTRIM1` reader - DAC Channel 1 offset trimming value
pub type OTRIM1_R = crate::FieldReader<u8, u8>;
///Field `OTRIM1` writer - DAC Channel 1 offset trimming value
pub type OTRIM1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DAC_CCR_SPEC, u8, u8, 5, O>;
///Field `OTRIM2` reader - DAC Channel 2 offset trimming value
pub type OTRIM2_R = crate::FieldReader<u8, u8>;
///Field `OTRIM2` writer - DAC Channel 2 offset trimming value
pub type OTRIM2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DAC_CCR_SPEC, u8, u8, 5, O>;
impl R {
    ///Bits 0:4 - DAC Channel 1 offset trimming value
    #[inline(always)]
    pub fn otrim1(&self) -> OTRIM1_R {
        OTRIM1_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 16:20 - DAC Channel 2 offset trimming value
    #[inline(always)]
    pub fn otrim2(&self) -> OTRIM2_R {
        OTRIM2_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    ///Bits 0:4 - DAC Channel 1 offset trimming value
    #[inline(always)]
    #[must_use]
    pub fn otrim1(&mut self) -> OTRIM1_W<0> {
        OTRIM1_W::new(self)
    }
    ///Bits 16:20 - DAC Channel 2 offset trimming value
    #[inline(always)]
    #[must_use]
    pub fn otrim2(&mut self) -> OTRIM2_W<16> {
        OTRIM2_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DAC calibration control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dac_ccr](index.html) module
pub struct DAC_CCR_SPEC;
impl crate::RegisterSpec for DAC_CCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [dac_ccr::R](R) reader structure
impl crate::Readable for DAC_CCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dac_ccr::W](W) writer structure
impl crate::Writable for DAC_CCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DAC_CCR to value 0
impl crate::Resettable for DAC_CCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
