///Register `ADC_CCR` reader
pub struct R(crate::R<ADC_CCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_CCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_CCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_CCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ADC_CCR` writer
pub struct W(crate::W<ADC_CCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_CCR_SPEC>;
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
impl From<crate::W<ADC_CCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_CCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PRESC` reader - PRESC
pub type PRESC_R = crate::FieldReader<u8, u8>;
///Field `PRESC` writer - PRESC
pub type PRESC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADC_CCR_SPEC, u8, u8, 4, O>;
///Field `VREFEN` reader - VREFEN
pub type VREFEN_R = crate::BitReader<bool>;
///Field `VREFEN` writer - VREFEN
pub type VREFEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_CCR_SPEC, bool, O>;
///Field `VSENSESEL` reader - VSENSESEL
pub type VSENSESEL_R = crate::BitReader<bool>;
///Field `VSENSESEL` writer - VSENSESEL
pub type VSENSESEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_CCR_SPEC, bool, O>;
///Field `VBATEN` reader - VBATEN
pub type VBATEN_R = crate::BitReader<bool>;
///Field `VBATEN` writer - VBATEN
pub type VBATEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_CCR_SPEC, bool, O>;
impl R {
    ///Bits 18:21 - PRESC
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    ///Bit 22 - VREFEN
    #[inline(always)]
    pub fn vrefen(&self) -> VREFEN_R {
        VREFEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - VSENSESEL
    #[inline(always)]
    pub fn vsensesel(&self) -> VSENSESEL_R {
        VSENSESEL_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - VBATEN
    #[inline(always)]
    pub fn vbaten(&self) -> VBATEN_R {
        VBATEN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    ///Bits 18:21 - PRESC
    #[inline(always)]
    #[must_use]
    pub fn presc(&mut self) -> PRESC_W<18> {
        PRESC_W::new(self)
    }
    ///Bit 22 - VREFEN
    #[inline(always)]
    #[must_use]
    pub fn vrefen(&mut self) -> VREFEN_W<22> {
        VREFEN_W::new(self)
    }
    ///Bit 23 - VSENSESEL
    #[inline(always)]
    #[must_use]
    pub fn vsensesel(&mut self) -> VSENSESEL_W<23> {
        VSENSESEL_W::new(self)
    }
    ///Bit 24 - VBATEN
    #[inline(always)]
    #[must_use]
    pub fn vbaten(&mut self) -> VBATEN_W<24> {
        VBATEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///ADC common configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [adc_ccr](index.html) module
pub struct ADC_CCR_SPEC;
impl crate::RegisterSpec for ADC_CCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [adc_ccr::R](R) reader structure
impl crate::Readable for ADC_CCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [adc_ccr::W](W) writer structure
impl crate::Writable for ADC_CCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ADC_CCR to value 0
impl crate::Resettable for ADC_CCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
