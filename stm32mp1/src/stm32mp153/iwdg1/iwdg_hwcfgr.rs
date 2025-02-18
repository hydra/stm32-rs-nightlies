///Register `IWDG_HWCFGR` reader
pub struct R(crate::R<IWDG_HWCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IWDG_HWCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IWDG_HWCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IWDG_HWCFGR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `WINDOW` reader - WINDOW
pub type WINDOW_R = crate::FieldReader<u8, u8>;
///Field `PR_DEFAULT` reader - PR_DEFAULT
pub type PR_DEFAULT_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:3 - WINDOW
    #[inline(always)]
    pub fn window(&self) -> WINDOW_R {
        WINDOW_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - PR_DEFAULT
    #[inline(always)]
    pub fn pr_default(&self) -> PR_DEFAULT_R {
        PR_DEFAULT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
///IWDG hardware configuration register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [iwdg_hwcfgr](index.html) module
pub struct IWDG_HWCFGR_SPEC;
impl crate::RegisterSpec for IWDG_HWCFGR_SPEC {
    type Ux = u32;
}
///`read()` method returns [iwdg_hwcfgr::R](R) reader structure
impl crate::Readable for IWDG_HWCFGR_SPEC {
    type Reader = R;
}
///`reset()` method sets IWDG_HWCFGR to value 0x71
impl crate::Resettable for IWDG_HWCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0x71;
}
