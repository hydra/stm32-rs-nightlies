///Register `SIDR` reader
pub struct R(crate::R<SIDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIDR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `SID` reader - Size Identification SID\[31:8\]: fixed code that characterizes the ADC_SIDR register. This field is always read at 0xA3C5DD. SID\[7:0\]: read-only numeric field that returns the address offset (in Kbytes) of the identification registers from the IP base address:
pub type SID_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - Size Identification SID\[31:8\]: fixed code that characterizes the ADC_SIDR register. This field is always read at 0xA3C5DD. SID\[7:0\]: read-only numeric field that returns the address offset (in Kbytes) of the identification registers from the IP base address:
    #[inline(always)]
    pub fn sid(&self) -> SID_R {
        SID_R::new(self.bits)
    }
}
///ADC size identification register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sidr](index.html) module
pub struct SIDR_SPEC;
impl crate::RegisterSpec for SIDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [sidr::R](R) reader structure
impl crate::Readable for SIDR_SPEC {
    type Reader = R;
}
///`reset()` method sets SIDR to value 0xa3c5_dd01
impl crate::Resettable for SIDR_SPEC {
    const RESET_VALUE: Self::Ux = 0xa3c5_dd01;
}
