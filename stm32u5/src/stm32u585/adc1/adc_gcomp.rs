///Register `ADC_GCOMP` reader
pub struct R(crate::R<ADC_GCOMP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_GCOMP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_GCOMP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_GCOMP_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ADC_GCOMP` writer
pub struct W(crate::W<ADC_GCOMP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_GCOMP_SPEC>;
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
impl From<crate::W<ADC_GCOMP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_GCOMP_SPEC>) -> Self {
        W(writer)
    }
}
///Field `GCOMPCOEFF` reader - Gain compensation coefficient These bits are set and cleared by software to program the gain compensation coefficient. ... ... The coefficient is divided by 4096 to get the gain factor ranging from 0 to 3.999756. Note: This gain compensation is only applied when GCOMP bit of ADCx_CFGR2 register is 1.
pub type GCOMPCOEFF_R = crate::FieldReader<u16, u16>;
///Field `GCOMPCOEFF` writer - Gain compensation coefficient These bits are set and cleared by software to program the gain compensation coefficient. ... ... The coefficient is divided by 4096 to get the gain factor ranging from 0 to 3.999756. Note: This gain compensation is only applied when GCOMP bit of ADCx_CFGR2 register is 1.
pub type GCOMPCOEFF_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADC_GCOMP_SPEC, u16, u16, 14, O>;
///Field `GCOMP` reader - Gain compensation mode This bit is set and cleared by software to enable the gain compensation mode. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type GCOMP_R = crate::BitReader<bool>;
///Field `GCOMP` writer - Gain compensation mode This bit is set and cleared by software to enable the gain compensation mode. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type GCOMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_GCOMP_SPEC, bool, O>;
impl R {
    ///Bits 0:13 - Gain compensation coefficient These bits are set and cleared by software to program the gain compensation coefficient. ... ... The coefficient is divided by 4096 to get the gain factor ranging from 0 to 3.999756. Note: This gain compensation is only applied when GCOMP bit of ADCx_CFGR2 register is 1.
    #[inline(always)]
    pub fn gcompcoeff(&self) -> GCOMPCOEFF_R {
        GCOMPCOEFF_R::new((self.bits & 0x3fff) as u16)
    }
    ///Bit 31 - Gain compensation mode This bit is set and cleared by software to enable the gain compensation mode. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn gcomp(&self) -> GCOMP_R {
        GCOMP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 0:13 - Gain compensation coefficient These bits are set and cleared by software to program the gain compensation coefficient. ... ... The coefficient is divided by 4096 to get the gain factor ranging from 0 to 3.999756. Note: This gain compensation is only applied when GCOMP bit of ADCx_CFGR2 register is 1.
    #[inline(always)]
    #[must_use]
    pub fn gcompcoeff(&mut self) -> GCOMPCOEFF_W<0> {
        GCOMPCOEFF_W::new(self)
    }
    ///Bit 31 - Gain compensation mode This bit is set and cleared by software to enable the gain compensation mode. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    #[must_use]
    pub fn gcomp(&mut self) -> GCOMP_W<31> {
        GCOMP_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///ADC gain compensation register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [adc_gcomp](index.html) module
pub struct ADC_GCOMP_SPEC;
impl crate::RegisterSpec for ADC_GCOMP_SPEC {
    type Ux = u32;
}
///`read()` method returns [adc_gcomp::R](R) reader structure
impl crate::Readable for ADC_GCOMP_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [adc_gcomp::W](W) writer structure
impl crate::Writable for ADC_GCOMP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ADC_GCOMP to value 0
impl crate::Resettable for ADC_GCOMP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
