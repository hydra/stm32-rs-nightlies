///Register `DDRPERFM_CNT0` reader
pub struct R(crate::R<DDRPERFM_CNT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRPERFM_CNT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRPERFM_CNT0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRPERFM_CNT0_SPEC>) -> Self {
        R(reader)
    }
}
///Field `CNT` reader - CNT
pub type CNT_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - CNT
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new(self.bits)
    }
}
///DDRPERFM event counter 0 register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ddrperfm_cnt0](index.html) module
pub struct DDRPERFM_CNT0_SPEC;
impl crate::RegisterSpec for DDRPERFM_CNT0_SPEC {
    type Ux = u32;
}
///`read()` method returns [ddrperfm_cnt0::R](R) reader structure
impl crate::Readable for DDRPERFM_CNT0_SPEC {
    type Reader = R;
}
///`reset()` method sets DDRPERFM_CNT0 to value 0
impl crate::Resettable for DDRPERFM_CNT0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
