///Register `GPIOH_HWCFGR5` reader
pub struct R(crate::R<GPIOH_HWCFGR5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIOH_HWCFGR5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIOH_HWCFGR5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIOH_HWCFGR5_SPEC>) -> Self {
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
///For information about available fields see [gpioh_hwcfgr5](index.html) module
pub struct GPIOH_HWCFGR5_SPEC;
impl crate::RegisterSpec for GPIOH_HWCFGR5_SPEC {
    type Ux = u32;
}
///`read()` method returns [gpioh_hwcfgr5::R](R) reader structure
impl crate::Readable for GPIOH_HWCFGR5_SPEC {
    type Reader = R;
}
///`reset()` method sets GPIOH_HWCFGR5 to value 0
impl crate::Resettable for GPIOH_HWCFGR5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
