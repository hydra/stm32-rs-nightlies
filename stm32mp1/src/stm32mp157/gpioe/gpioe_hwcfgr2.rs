///Register `GPIOE_HWCFGR2` reader
pub struct R(crate::R<GPIOE_HWCFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIOE_HWCFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIOE_HWCFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIOE_HWCFGR2_SPEC>) -> Self {
        R(reader)
    }
}
///Field `AFRL_RES` reader - AFRL_RES
pub type AFRL_RES_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - AFRL_RES
    #[inline(always)]
    pub fn afrl_res(&self) -> AFRL_RES_R {
        AFRL_RES_R::new(self.bits)
    }
}
///GPIO hardware configuration register 2
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [gpioe_hwcfgr2](index.html) module
pub struct GPIOE_HWCFGR2_SPEC;
impl crate::RegisterSpec for GPIOE_HWCFGR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [gpioe_hwcfgr2::R](R) reader structure
impl crate::Readable for GPIOE_HWCFGR2_SPEC {
    type Reader = R;
}
///`reset()` method sets GPIOE_HWCFGR2 to value 0
impl crate::Resettable for GPIOE_HWCFGR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
