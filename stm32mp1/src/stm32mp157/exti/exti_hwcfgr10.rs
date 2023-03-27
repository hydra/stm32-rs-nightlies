///Register `EXTI_HWCFGR10` reader
pub struct R(crate::R<EXTI_HWCFGR10_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTI_HWCFGR10_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTI_HWCFGR10_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTI_HWCFGR10_SPEC>) -> Self {
        R(reader)
    }
}
///EXTI hardware configuration register 10
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [exti_hwcfgr10](index.html) module
pub struct EXTI_HWCFGR10_SPEC;
impl crate::RegisterSpec for EXTI_HWCFGR10_SPEC {
    type Ux = u32;
}
///`read()` method returns [exti_hwcfgr10::R](R) reader structure
impl crate::Readable for EXTI_HWCFGR10_SPEC {
    type Reader = R;
}
///`reset()` method sets EXTI_HWCFGR10 to value 0
impl crate::Resettable for EXTI_HWCFGR10_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
