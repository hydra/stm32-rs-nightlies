///Register `GPIOD_HWCFGR6` reader
pub struct R(crate::R<GPIOD_HWCFGR6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIOD_HWCFGR6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIOD_HWCFGR6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIOD_HWCFGR6_SPEC>) -> Self {
        R(reader)
    }
}
///Field `MODER_RES` reader - MODER_RES
pub type MODER_RES_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - MODER_RES
    #[inline(always)]
    pub fn moder_res(&self) -> MODER_RES_R {
        MODER_RES_R::new(self.bits)
    }
}
///GPIO hardware configuration register 6
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [gpiod_hwcfgr6](index.html) module
pub struct GPIOD_HWCFGR6_SPEC;
impl crate::RegisterSpec for GPIOD_HWCFGR6_SPEC {
    type Ux = u32;
}
///`read()` method returns [gpiod_hwcfgr6::R](R) reader structure
impl crate::Readable for GPIOD_HWCFGR6_SPEC {
    type Reader = R;
}
///`reset()` method sets GPIOD_HWCFGR6 to value 0xffff_ffff
impl crate::Resettable for GPIOD_HWCFGR6_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
