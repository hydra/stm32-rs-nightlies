///Register `EXTI_HWCFGR2` reader
pub struct R(crate::R<EXTI_HWCFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTI_HWCFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTI_HWCFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTI_HWCFGR2_SPEC>) -> Self {
        R(reader)
    }
}
///Field `EVENT_TRG` reader - EVENT_TRG
pub type EVENT_TRG_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - EVENT_TRG
    #[inline(always)]
    pub fn event_trg(&self) -> EVENT_TRG_R {
        EVENT_TRG_R::new(self.bits)
    }
}
///EXTI hardware configuration register 2
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [exti_hwcfgr2](index.html) module
pub struct EXTI_HWCFGR2_SPEC;
impl crate::RegisterSpec for EXTI_HWCFGR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [exti_hwcfgr2::R](R) reader structure
impl crate::Readable for EXTI_HWCFGR2_SPEC {
    type Reader = R;
}
///`reset()` method sets EXTI_HWCFGR2 to value 0x0001_ffff
impl crate::Resettable for EXTI_HWCFGR2_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_ffff;
}
