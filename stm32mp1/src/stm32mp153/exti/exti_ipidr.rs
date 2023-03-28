///Register `EXTI_IPIDR` reader
pub struct R(crate::R<EXTI_IPIDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTI_IPIDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTI_IPIDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTI_IPIDR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `IPID` reader - IPID
pub type IPID_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - IPID
    #[inline(always)]
    pub fn ipid(&self) -> IPID_R {
        IPID_R::new(self.bits)
    }
}
///EXTI identification register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [exti_ipidr](index.html) module
pub struct EXTI_IPIDR_SPEC;
impl crate::RegisterSpec for EXTI_IPIDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [exti_ipidr::R](R) reader structure
impl crate::Readable for EXTI_IPIDR_SPEC {
    type Reader = R;
}
///`reset()` method sets EXTI_IPIDR to value 0x000e_0001
impl crate::Resettable for EXTI_IPIDR_SPEC {
    const RESET_VALUE: Self::Ux = 0x000e_0001;
}
