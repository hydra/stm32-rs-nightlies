///Register `ADC_JDR2` reader
pub struct R(crate::R<ADC_JDR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_JDR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_JDR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_JDR2_SPEC>) -> Self {
        R(reader)
    }
}
///Field `JDATA` reader - JDATA
pub type JDATA_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - JDATA
    #[inline(always)]
    pub fn jdata(&self) -> JDATA_R {
        JDATA_R::new(self.bits)
    }
}
///ADC injected data register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [adc_jdr2](index.html) module
pub struct ADC_JDR2_SPEC;
impl crate::RegisterSpec for ADC_JDR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [adc_jdr2::R](R) reader structure
impl crate::Readable for ADC_JDR2_SPEC {
    type Reader = R;
}
///`reset()` method sets ADC_JDR2 to value 0
impl crate::Resettable for ADC_JDR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
