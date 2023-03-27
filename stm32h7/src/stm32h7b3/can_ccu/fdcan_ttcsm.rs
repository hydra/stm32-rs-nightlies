///Register `FDCAN_TTCSM` reader
pub struct R(crate::R<FDCAN_TTCSM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_TTCSM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_TTCSM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_TTCSM_SPEC>) -> Self {
        R(reader)
    }
}
///Field `CSM` reader - Cycle Sync Mark
pub type CSM_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:15 - Cycle Sync Mark
    #[inline(always)]
    pub fn csm(&self) -> CSM_R {
        CSM_R::new((self.bits & 0xffff) as u16)
    }
}
///FDCAN TT Cycle Sync Mark Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fdcan_ttcsm](index.html) module
pub struct FDCAN_TTCSM_SPEC;
impl crate::RegisterSpec for FDCAN_TTCSM_SPEC {
    type Ux = u32;
}
///`read()` method returns [fdcan_ttcsm::R](R) reader structure
impl crate::Readable for FDCAN_TTCSM_SPEC {
    type Reader = R;
}
///`reset()` method sets FDCAN_TTCSM to value 0
impl crate::Resettable for FDCAN_TTCSM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
