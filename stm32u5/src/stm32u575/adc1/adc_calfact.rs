///Register `ADC_CALFACT` reader
pub struct R(crate::R<ADC_CALFACT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_CALFACT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_CALFACT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_CALFACT_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ADC_CALFACT` writer
pub struct W(crate::W<ADC_CALFACT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_CALFACT_SPEC>;
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
impl From<crate::W<ADC_CALFACT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_CALFACT_SPEC>) -> Self {
        W(writer)
    }
}
///Field `I_APB_ADDR` reader - Delayed write access address This bitfield contains the address that is being written during delayed write accesses.
pub type I_APB_ADDR_R = crate::FieldReader<u8, u8>;
///Field `I_APB_DATA` reader - Delayed write access data This bitfield contains the data that are being written during delayed write accesses.
pub type I_APB_DATA_R = crate::FieldReader<u8, u8>;
///Field `VALIDITY` reader - Delayed write access status bit This bit indicates the communication status between the ADC digital and analog blocks.
pub type VALIDITY_R = crate::BitReader<bool>;
///Field `LATCH_COEF` reader - Calibration factor latch enable bit This bit latches the calibration factor in the CALFACT\[31:0\]
///bits.
pub type LATCH_COEF_R = crate::BitReader<bool>;
///Field `LATCH_COEF` writer - Calibration factor latch enable bit This bit latches the calibration factor in the CALFACT\[31:0\]
///bits.
pub type LATCH_COEF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_CALFACT_SPEC, bool, O>;
///Field `CAPTURE_COEF` reader - Calibration factor capture enable bit This bit enables the internal calibration factor capture.
pub type CAPTURE_COEF_R = crate::BitReader<bool>;
///Field `CAPTURE_COEF` writer - Calibration factor capture enable bit This bit enables the internal calibration factor capture.
pub type CAPTURE_COEF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_CALFACT_SPEC, bool, O>;
impl R {
    ///Bits 0:7 - Delayed write access address This bitfield contains the address that is being written during delayed write accesses.
    #[inline(always)]
    pub fn i_apb_addr(&self) -> I_APB_ADDR_R {
        I_APB_ADDR_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Delayed write access data This bitfield contains the data that are being written during delayed write accesses.
    #[inline(always)]
    pub fn i_apb_data(&self) -> I_APB_DATA_R {
        I_APB_DATA_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bit 16 - Delayed write access status bit This bit indicates the communication status between the ADC digital and analog blocks.
    #[inline(always)]
    pub fn validity(&self) -> VALIDITY_R {
        VALIDITY_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 24 - Calibration factor latch enable bit This bit latches the calibration factor in the CALFACT\[31:0\]
    ///bits.
    #[inline(always)]
    pub fn latch_coef(&self) -> LATCH_COEF_R {
        LATCH_COEF_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Calibration factor capture enable bit This bit enables the internal calibration factor capture.
    #[inline(always)]
    pub fn capture_coef(&self) -> CAPTURE_COEF_R {
        CAPTURE_COEF_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    ///Bit 24 - Calibration factor latch enable bit This bit latches the calibration factor in the CALFACT\[31:0\]
    ///bits.
    #[inline(always)]
    #[must_use]
    pub fn latch_coef(&mut self) -> LATCH_COEF_W<24> {
        LATCH_COEF_W::new(self)
    }
    ///Bit 25 - Calibration factor capture enable bit This bit enables the internal calibration factor capture.
    #[inline(always)]
    #[must_use]
    pub fn capture_coef(&mut self) -> CAPTURE_COEF_W<25> {
        CAPTURE_COEF_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///ADC user control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [adc_calfact](index.html) module
pub struct ADC_CALFACT_SPEC;
impl crate::RegisterSpec for ADC_CALFACT_SPEC {
    type Ux = u32;
}
///`read()` method returns [adc_calfact::R](R) reader structure
impl crate::Readable for ADC_CALFACT_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [adc_calfact::W](W) writer structure
impl crate::Writable for ADC_CALFACT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ADC_CALFACT to value 0
impl crate::Resettable for ADC_CALFACT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
