///Register `DAC_DOR2` reader
pub struct R(crate::R<DAC_DOR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAC_DOR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAC_DOR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAC_DOR2_SPEC>) -> Self {
        R(reader)
    }
}
///Field `DACC2DOR` reader - DACC2DOR
pub type DACC2DOR_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:11 - DACC2DOR
    #[inline(always)]
    pub fn dacc2dor(&self) -> DACC2DOR_R {
        DACC2DOR_R::new((self.bits & 0x0fff) as u16)
    }
}
///This register is available only on dual-channel DACs. Refer to Section29.3: DAC implementation.
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dac_dor2](index.html) module
pub struct DAC_DOR2_SPEC;
impl crate::RegisterSpec for DAC_DOR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [dac_dor2::R](R) reader structure
impl crate::Readable for DAC_DOR2_SPEC {
    type Reader = R;
}
///`reset()` method sets DAC_DOR2 to value 0
impl crate::Resettable for DAC_DOR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
