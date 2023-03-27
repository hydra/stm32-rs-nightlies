///Register `DAC_DHR12L2` reader
pub struct R(crate::R<DAC_DHR12L2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAC_DHR12L2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAC_DHR12L2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAC_DHR12L2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DAC_DHR12L2` writer
pub struct W(crate::W<DAC_DHR12L2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAC_DHR12L2_SPEC>;
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
impl From<crate::W<DAC_DHR12L2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAC_DHR12L2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DACC2DHR` reader - DAC channel2 12-bit left-aligned data
pub type DACC2DHR_R = crate::FieldReader<u16, u16>;
///Field `DACC2DHR` writer - DAC channel2 12-bit left-aligned data
pub type DACC2DHR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DAC_DHR12L2_SPEC, u16, u16, 12, O>;
///Field `DACC2DHRB` reader - DAC channel2 12-bit left-aligned data B
pub type DACC2DHRB_R = crate::FieldReader<u16, u16>;
///Field `DACC2DHRB` writer - DAC channel2 12-bit left-aligned data B
pub type DACC2DHRB_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DAC_DHR12L2_SPEC, u16, u16, 12, O>;
impl R {
    ///Bits 4:15 - DAC channel2 12-bit left-aligned data
    #[inline(always)]
    pub fn dacc2dhr(&self) -> DACC2DHR_R {
        DACC2DHR_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    ///Bits 20:31 - DAC channel2 12-bit left-aligned data B
    #[inline(always)]
    pub fn dacc2dhrb(&self) -> DACC2DHRB_R {
        DACC2DHRB_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    ///Bits 4:15 - DAC channel2 12-bit left-aligned data
    #[inline(always)]
    #[must_use]
    pub fn dacc2dhr(&mut self) -> DACC2DHR_W<4> {
        DACC2DHR_W::new(self)
    }
    ///Bits 20:31 - DAC channel2 12-bit left-aligned data B
    #[inline(always)]
    #[must_use]
    pub fn dacc2dhrb(&mut self) -> DACC2DHRB_W<20> {
        DACC2DHRB_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DAC channel2 12-bit left aligned data holding register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dac_dhr12l2](index.html) module
pub struct DAC_DHR12L2_SPEC;
impl crate::RegisterSpec for DAC_DHR12L2_SPEC {
    type Ux = u32;
}
///`read()` method returns [dac_dhr12l2::R](R) reader structure
impl crate::Readable for DAC_DHR12L2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dac_dhr12l2::W](W) writer structure
impl crate::Writable for DAC_DHR12L2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DAC_DHR12L2 to value 0
impl crate::Resettable for DAC_DHR12L2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
