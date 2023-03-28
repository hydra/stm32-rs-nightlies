///Register `EXTI_HWCFGR5` reader
pub struct R(crate::R<EXTI_HWCFGR5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTI_HWCFGR5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTI_HWCFGR5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTI_HWCFGR5_SPEC>) -> Self {
        R(reader)
    }
}
///Field `CPUEVENT` reader - CPUEVENT
pub type CPUEVENT_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - CPUEVENT
    #[inline(always)]
    pub fn cpuevent(&self) -> CPUEVENT_R {
        CPUEVENT_R::new(self.bits)
    }
}
///EXTI hardware configuration register 5
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [exti_hwcfgr5](index.html) module
pub struct EXTI_HWCFGR5_SPEC;
impl crate::RegisterSpec for EXTI_HWCFGR5_SPEC {
    type Ux = u32;
}
///`read()` method returns [exti_hwcfgr5::R](R) reader structure
impl crate::Readable for EXTI_HWCFGR5_SPEC {
    type Reader = R;
}
///`reset()` method sets EXTI_HWCFGR5 to value 0x000e_ffff
impl crate::Resettable for EXTI_HWCFGR5_SPEC {
    const RESET_VALUE: Self::Ux = 0x000e_ffff;
}
