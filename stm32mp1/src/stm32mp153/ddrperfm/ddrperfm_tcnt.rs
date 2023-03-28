///Register `DDRPERFM_TCNT` reader
pub struct R(crate::R<DDRPERFM_TCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRPERFM_TCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRPERFM_TCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRPERFM_TCNT_SPEC>) -> Self {
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
///DDRPERFM time counter register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ddrperfm_tcnt](index.html) module
pub struct DDRPERFM_TCNT_SPEC;
impl crate::RegisterSpec for DDRPERFM_TCNT_SPEC {
    type Ux = u32;
}
///`read()` method returns [ddrperfm_tcnt::R](R) reader structure
impl crate::Readable for DDRPERFM_TCNT_SPEC {
    type Reader = R;
}
///`reset()` method sets DDRPERFM_TCNT to value 0
impl crate::Resettable for DDRPERFM_TCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
