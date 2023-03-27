///Register `ADC_CALFACT2` reader
pub struct R(crate::R<ADC_CALFACT2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_CALFACT2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_CALFACT2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_CALFACT2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ADC_CALFACT2` writer
pub struct W(crate::W<ADC_CALFACT2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_CALFACT2_SPEC>;
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
impl From<crate::W<ADC_CALFACT2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_CALFACT2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CALFACT` reader - Linearity or offset calibration factor These bits can be written either by hardware or by software. They contain the 32-bit offset or linearity calibration factor. When CAPTURE_COEF is set to 1, the calibration factor of the analog block is read back and stored in CALFACT\[31:0\], indexed by CALINDEX\[3:0\]
///bits. When LATCH_COEF is set to 1, the calibration factor of the analog block is updated with the value programmed in CALFACT\[31:0\], indexed by CALINDEX\[3:0\]
///bits. To read all calibration factors, perform nine accesses to the ADC_CALFACT2 register. To write all calibration factors, perform eight accesses to the ADC_CALFACT2 register. Note: The software is allowed to write these bits only when ADEN = 1, ADSTART = 0 and JADSTART = 0 (ADC is enabled and no calibration is ongoing and no conversion is ongoing).
pub type CALFACT_R = crate::FieldReader<u32, u32>;
///Field `CALFACT` writer - Linearity or offset calibration factor These bits can be written either by hardware or by software. They contain the 32-bit offset or linearity calibration factor. When CAPTURE_COEF is set to 1, the calibration factor of the analog block is read back and stored in CALFACT\[31:0\], indexed by CALINDEX\[3:0\]
///bits. When LATCH_COEF is set to 1, the calibration factor of the analog block is updated with the value programmed in CALFACT\[31:0\], indexed by CALINDEX\[3:0\]
///bits. To read all calibration factors, perform nine accesses to the ADC_CALFACT2 register. To write all calibration factors, perform eight accesses to the ADC_CALFACT2 register. Note: The software is allowed to write these bits only when ADEN = 1, ADSTART = 0 and JADSTART = 0 (ADC is enabled and no calibration is ongoing and no conversion is ongoing).
pub type CALFACT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADC_CALFACT2_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - Linearity or offset calibration factor These bits can be written either by hardware or by software. They contain the 32-bit offset or linearity calibration factor. When CAPTURE_COEF is set to 1, the calibration factor of the analog block is read back and stored in CALFACT\[31:0\], indexed by CALINDEX\[3:0\]
    ///bits. When LATCH_COEF is set to 1, the calibration factor of the analog block is updated with the value programmed in CALFACT\[31:0\], indexed by CALINDEX\[3:0\]
    ///bits. To read all calibration factors, perform nine accesses to the ADC_CALFACT2 register. To write all calibration factors, perform eight accesses to the ADC_CALFACT2 register. Note: The software is allowed to write these bits only when ADEN = 1, ADSTART = 0 and JADSTART = 0 (ADC is enabled and no calibration is ongoing and no conversion is ongoing).
    #[inline(always)]
    pub fn calfact(&self) -> CALFACT_R {
        CALFACT_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Linearity or offset calibration factor These bits can be written either by hardware or by software. They contain the 32-bit offset or linearity calibration factor. When CAPTURE_COEF is set to 1, the calibration factor of the analog block is read back and stored in CALFACT\[31:0\], indexed by CALINDEX\[3:0\]
    ///bits. When LATCH_COEF is set to 1, the calibration factor of the analog block is updated with the value programmed in CALFACT\[31:0\], indexed by CALINDEX\[3:0\]
    ///bits. To read all calibration factors, perform nine accesses to the ADC_CALFACT2 register. To write all calibration factors, perform eight accesses to the ADC_CALFACT2 register. Note: The software is allowed to write these bits only when ADEN = 1, ADSTART = 0 and JADSTART = 0 (ADC is enabled and no calibration is ongoing and no conversion is ongoing).
    #[inline(always)]
    #[must_use]
    pub fn calfact(&mut self) -> CALFACT_W<0> {
        CALFACT_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///ADC calibration factor register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [adc_calfact2](index.html) module
pub struct ADC_CALFACT2_SPEC;
impl crate::RegisterSpec for ADC_CALFACT2_SPEC {
    type Ux = u32;
}
///`read()` method returns [adc_calfact2::R](R) reader structure
impl crate::Readable for ADC_CALFACT2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [adc_calfact2::W](W) writer structure
impl crate::Writable for ADC_CALFACT2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ADC_CALFACT2 to value 0
impl crate::Resettable for ADC_CALFACT2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
