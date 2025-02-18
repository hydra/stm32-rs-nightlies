///Register `HDP_IPIDR` reader
pub struct R(crate::R<HDP_IPIDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HDP_IPIDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HDP_IPIDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HDP_IPIDR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `ID` reader - ID
pub type ID_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - ID
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(self.bits)
    }
}
///HDP IP identification register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hdp_ipidr](index.html) module
pub struct HDP_IPIDR_SPEC;
impl crate::RegisterSpec for HDP_IPIDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [hdp_ipidr::R](R) reader structure
impl crate::Readable for HDP_IPIDR_SPEC {
    type Reader = R;
}
///`reset()` method sets HDP_IPIDR to value 0x0003_0002
impl crate::Resettable for HDP_IPIDR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0003_0002;
}
