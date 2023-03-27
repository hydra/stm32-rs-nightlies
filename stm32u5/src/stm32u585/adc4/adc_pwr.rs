///Register `ADC_PWR` reader
pub struct R(crate::R<ADC_PWR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_PWR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_PWR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_PWR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ADC_PWR` writer
pub struct W(crate::W<ADC_PWR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_PWR_SPEC>;
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
impl From<crate::W<ADC_PWR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_PWR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `AUTOFF` reader - AUTOFF
pub type AUTOFF_R = crate::BitReader<bool>;
///Field `AUTOFF` writer - AUTOFF
pub type AUTOFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_PWR_SPEC, bool, O>;
///Field `DPD` reader - DPD
pub type DPD_R = crate::BitReader<bool>;
///Field `DPD` writer - DPD
pub type DPD_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_PWR_SPEC, bool, O>;
///Field `VREFPROT` reader - VREFPROT
pub type VREFPROT_R = crate::BitReader<bool>;
///Field `VREFPROT` writer - VREFPROT
pub type VREFPROT_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_PWR_SPEC, bool, O>;
///Field `VREFSECSMP` reader - VREFSECSMP
pub type VREFSECSMP_R = crate::BitReader<bool>;
///Field `VREFSECSMP` writer - VREFSECSMP
pub type VREFSECSMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_PWR_SPEC, bool, O>;
impl R {
    ///Bit 0 - AUTOFF
    #[inline(always)]
    pub fn autoff(&self) -> AUTOFF_R {
        AUTOFF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DPD
    #[inline(always)]
    pub fn dpd(&self) -> DPD_R {
        DPD_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - VREFPROT
    #[inline(always)]
    pub fn vrefprot(&self) -> VREFPROT_R {
        VREFPROT_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - VREFSECSMP
    #[inline(always)]
    pub fn vrefsecsmp(&self) -> VREFSECSMP_R {
        VREFSECSMP_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - AUTOFF
    #[inline(always)]
    #[must_use]
    pub fn autoff(&mut self) -> AUTOFF_W<0> {
        AUTOFF_W::new(self)
    }
    ///Bit 1 - DPD
    #[inline(always)]
    #[must_use]
    pub fn dpd(&mut self) -> DPD_W<1> {
        DPD_W::new(self)
    }
    ///Bit 2 - VREFPROT
    #[inline(always)]
    #[must_use]
    pub fn vrefprot(&mut self) -> VREFPROT_W<2> {
        VREFPROT_W::new(self)
    }
    ///Bit 3 - VREFSECSMP
    #[inline(always)]
    #[must_use]
    pub fn vrefsecsmp(&mut self) -> VREFSECSMP_W<3> {
        VREFSECSMP_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///ADC data register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [adc_pwr](index.html) module
pub struct ADC_PWR_SPEC;
impl crate::RegisterSpec for ADC_PWR_SPEC {
    type Ux = u32;
}
///`read()` method returns [adc_pwr::R](R) reader structure
impl crate::Readable for ADC_PWR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [adc_pwr::W](W) writer structure
impl crate::Writable for ADC_PWR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ADC_PWR to value 0
impl crate::Resettable for ADC_PWR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
