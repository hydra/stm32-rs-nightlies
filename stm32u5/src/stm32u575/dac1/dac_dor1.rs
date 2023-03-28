///Register `DAC_DOR1` reader
pub struct R(crate::R<DAC_DOR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAC_DOR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAC_DOR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAC_DOR1_SPEC>) -> Self {
        R(reader)
    }
}
///Field `DACC1DOR` reader - DAC channel1 data output
pub type DACC1DOR_R = crate::FieldReader<u16, u16>;
///Field `DACC1DORB` reader - DAC channel1 data output
pub type DACC1DORB_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:11 - DAC channel1 data output
    #[inline(always)]
    pub fn dacc1dor(&self) -> DACC1DOR_R {
        DACC1DOR_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 16:27 - DAC channel1 data output
    #[inline(always)]
    pub fn dacc1dorb(&self) -> DACC1DORB_R {
        DACC1DORB_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
///DAC channel1 data output register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dac_dor1](index.html) module
pub struct DAC_DOR1_SPEC;
impl crate::RegisterSpec for DAC_DOR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [dac_dor1::R](R) reader structure
impl crate::Readable for DAC_DOR1_SPEC {
    type Reader = R;
}
///`reset()` method sets DAC_DOR1 to value 0
impl crate::Resettable for DAC_DOR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
