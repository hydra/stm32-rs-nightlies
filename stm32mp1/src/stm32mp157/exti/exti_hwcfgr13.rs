///Register `EXTI_HWCFGR13` reader
pub struct R(crate::R<EXTI_HWCFGR13_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTI_HWCFGR13_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTI_HWCFGR13_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTI_HWCFGR13_SPEC>) -> Self {
        R(reader)
    }
}
///Field `TZ` reader - TZ
pub type TZ_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - TZ
    #[inline(always)]
    pub fn tz(&self) -> TZ_R {
        TZ_R::new(self.bits)
    }
}
///EXTI hardware configuration register 13
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [exti_hwcfgr13](index.html) module
pub struct EXTI_HWCFGR13_SPEC;
impl crate::RegisterSpec for EXTI_HWCFGR13_SPEC {
    type Ux = u32;
}
///`read()` method returns [exti_hwcfgr13::R](R) reader structure
impl crate::Readable for EXTI_HWCFGR13_SPEC {
    type Reader = R;
}
///`reset()` method sets EXTI_HWCFGR13 to value 0x050e_ffff
impl crate::Resettable for EXTI_HWCFGR13_SPEC {
    const RESET_VALUE: Self::Ux = 0x050e_ffff;
}
