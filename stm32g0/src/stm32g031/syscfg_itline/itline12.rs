///Register `ITLINE12` reader
pub struct R(crate::R<ITLINE12_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ITLINE12_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ITLINE12_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ITLINE12_SPEC>) -> Self {
        R(reader)
    }
}
///Field `ADC` reader - ADC
pub type ADC_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - ADC
    #[inline(always)]
    pub fn adc(&self) -> ADC_R {
        ADC_R::new((self.bits & 1) != 0)
    }
}
///interrupt line 12 status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [itline12](index.html) module
pub struct ITLINE12_SPEC;
impl crate::RegisterSpec for ITLINE12_SPEC {
    type Ux = u32;
}
///`read()` method returns [itline12::R](R) reader structure
impl crate::Readable for ITLINE12_SPEC {
    type Reader = R;
}
///`reset()` method sets ITLINE12 to value 0
impl crate::Resettable for ITLINE12_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
