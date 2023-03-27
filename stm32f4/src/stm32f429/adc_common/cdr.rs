///Register `CDR` reader
pub struct R(crate::R<CDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CDR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `DATA1` reader - 1st data item of a pair of regular conversions
pub type DATA1_R = crate::FieldReader<u16, u16>;
///Field `DATA2` reader - 2nd data item of a pair of regular conversions
pub type DATA2_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:15 - 1st data item of a pair of regular conversions
    #[inline(always)]
    pub fn data1(&self) -> DATA1_R {
        DATA1_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - 2nd data item of a pair of regular conversions
    #[inline(always)]
    pub fn data2(&self) -> DATA2_R {
        DATA2_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
///ADC common regular data register for dual and triple modes
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cdr](index.html) module
pub struct CDR_SPEC;
impl crate::RegisterSpec for CDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [cdr::R](R) reader structure
impl crate::Readable for CDR_SPEC {
    type Reader = R;
}
///`reset()` method sets CDR to value 0
impl crate::Resettable for CDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
