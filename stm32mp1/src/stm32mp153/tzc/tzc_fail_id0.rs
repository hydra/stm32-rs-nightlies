///Register `TZC_FAIL_ID0` reader
pub struct R(crate::R<TZC_FAIL_ID0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZC_FAIL_ID0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZC_FAIL_ID0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZC_FAIL_ID0_SPEC>) -> Self {
        R(reader)
    }
}
///Field `ID` reader - ID
pub type ID_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:10 - ID
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new((self.bits & 0x07ff) as u16)
    }
}
///Contains the master AXI ARID or AWID of the first access that failed a region permission check in the associated filter unit. This occurs even if the ACTION register is set to not drive the interrupt signal. AXI ID mapping is described in Table4: NSAID definition table (TBD).
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tzc_fail_id0](index.html) module
pub struct TZC_FAIL_ID0_SPEC;
impl crate::RegisterSpec for TZC_FAIL_ID0_SPEC {
    type Ux = u32;
}
///`read()` method returns [tzc_fail_id0::R](R) reader structure
impl crate::Readable for TZC_FAIL_ID0_SPEC {
    type Reader = R;
}
///`reset()` method sets TZC_FAIL_ID0 to value 0
impl crate::Resettable for TZC_FAIL_ID0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
