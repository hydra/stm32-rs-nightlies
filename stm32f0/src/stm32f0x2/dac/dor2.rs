///Register `DOR2` reader
pub struct R(crate::R<DOR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOR2_SPEC>) -> Self {
        R(reader)
    }
}
///Field `DACC2DOR` reader - DAC channel2 data output
pub type DACC2DOR_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:11 - DAC channel2 data output
    #[inline(always)]
    pub fn dacc2dor(&self) -> DACC2DOR_R {
        DACC2DOR_R::new((self.bits & 0x0fff) as u16)
    }
}
///DAC channel2 data output register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dor2](index.html) module
pub struct DOR2_SPEC;
impl crate::RegisterSpec for DOR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [dor2::R](R) reader structure
impl crate::Readable for DOR2_SPEC {
    type Reader = R;
}
///`reset()` method sets DOR2 to value 0
impl crate::Resettable for DOR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
