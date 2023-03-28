///Register `GPIOI_HWCFGR1` reader
pub struct R(crate::R<GPIOI_HWCFGR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIOI_HWCFGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIOI_HWCFGR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIOI_HWCFGR1_SPEC>) -> Self {
        R(reader)
    }
}
///Field `AFRH_RES` reader - AFRH_RES
pub type AFRH_RES_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - AFRH_RES
    #[inline(always)]
    pub fn afrh_res(&self) -> AFRH_RES_R {
        AFRH_RES_R::new(self.bits)
    }
}
///GPIO hardware configuration register 1
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [gpioi_hwcfgr1](index.html) module
pub struct GPIOI_HWCFGR1_SPEC;
impl crate::RegisterSpec for GPIOI_HWCFGR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [gpioi_hwcfgr1::R](R) reader structure
impl crate::Readable for GPIOI_HWCFGR1_SPEC {
    type Reader = R;
}
///`reset()` method sets GPIOI_HWCFGR1 to value 0
impl crate::Resettable for GPIOI_HWCFGR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
