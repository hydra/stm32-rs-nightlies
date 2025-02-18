///Register `EXTI_HWCFGR8` reader
pub struct R(crate::R<EXTI_HWCFGR8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTI_HWCFGR8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTI_HWCFGR8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTI_HWCFGR8_SPEC>) -> Self {
        R(reader)
    }
}
///EXTI hardware configuration register 8
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [exti_hwcfgr8](index.html) module
pub struct EXTI_HWCFGR8_SPEC;
impl crate::RegisterSpec for EXTI_HWCFGR8_SPEC {
    type Ux = u32;
}
///`read()` method returns [exti_hwcfgr8::R](R) reader structure
impl crate::Readable for EXTI_HWCFGR8_SPEC {
    type Reader = R;
}
///`reset()` method sets EXTI_HWCFGR8 to value 0
impl crate::Resettable for EXTI_HWCFGR8_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
