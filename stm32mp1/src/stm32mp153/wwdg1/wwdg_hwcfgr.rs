///Register `WWDG_HWCFGR` reader
pub struct R(crate::R<WWDG_HWCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WWDG_HWCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WWDG_HWCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WWDG_HWCFGR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `PREDIV` reader - PREDIV
pub type PREDIV_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:15 - PREDIV
    #[inline(always)]
    pub fn prediv(&self) -> PREDIV_R {
        PREDIV_R::new((self.bits & 0xffff) as u16)
    }
}
///WWDG hardware configuration register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [wwdg_hwcfgr](index.html) module
pub struct WWDG_HWCFGR_SPEC;
impl crate::RegisterSpec for WWDG_HWCFGR_SPEC {
    type Ux = u32;
}
///`read()` method returns [wwdg_hwcfgr::R](R) reader structure
impl crate::Readable for WWDG_HWCFGR_SPEC {
    type Reader = R;
}
///`reset()` method sets WWDG_HWCFGR to value 0x0fff
impl crate::Resettable for WWDG_HWCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0fff;
}
