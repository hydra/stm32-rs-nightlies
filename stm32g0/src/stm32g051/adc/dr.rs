///Register `DR` reader
pub struct R(crate::R<DR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `DATA` reader - Converted data These bits are read-only. They contain the conversion result from the last converted channel. The data are left- or right-aligned as shown in OVSE = 0) on pageÂ 389. Just after a calibration is complete, DATA\[6:0\]
///contains the calibration factor.
pub type DATA_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:15 - Converted data These bits are read-only. They contain the conversion result from the last converted channel. The data are left- or right-aligned as shown in OVSE = 0) on pageÂ 389. Just after a calibration is complete, DATA\[6:0\]
    ///contains the calibration factor.
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff) as u16)
    }
}
///ADC data register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dr](index.html) module
pub struct DR_SPEC;
impl crate::RegisterSpec for DR_SPEC {
    type Ux = u32;
}
///`read()` method returns [dr::R](R) reader structure
impl crate::Readable for DR_SPEC {
    type Reader = R;
}
///`reset()` method sets DR to value 0
impl crate::Resettable for DR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
