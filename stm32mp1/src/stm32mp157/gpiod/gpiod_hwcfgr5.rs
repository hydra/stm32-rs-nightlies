///Register `GPIOD_HWCFGR5` reader
pub struct R(crate::R<GPIOD_HWCFGR5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIOD_HWCFGR5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIOD_HWCFGR5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIOD_HWCFGR5_SPEC>) -> Self {
        R(reader)
    }
}
///Field `PUPDR_RES` reader - PUPDR_RES
pub type PUPDR_RES_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - PUPDR_RES
    #[inline(always)]
    pub fn pupdr_res(&self) -> PUPDR_RES_R {
        PUPDR_RES_R::new(self.bits)
    }
}
///GPIO hardware configuration register 5
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [gpiod_hwcfgr5](index.html) module
pub struct GPIOD_HWCFGR5_SPEC;
impl crate::RegisterSpec for GPIOD_HWCFGR5_SPEC {
    type Ux = u32;
}
///`read()` method returns [gpiod_hwcfgr5::R](R) reader structure
impl crate::Readable for GPIOD_HWCFGR5_SPEC {
    type Reader = R;
}
///`reset()` method sets GPIOD_HWCFGR5 to value 0
impl crate::Resettable for GPIOD_HWCFGR5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
