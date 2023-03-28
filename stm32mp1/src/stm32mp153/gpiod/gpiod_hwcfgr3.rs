///Register `GPIOD_HWCFGR3` reader
pub struct R(crate::R<GPIOD_HWCFGR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIOD_HWCFGR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIOD_HWCFGR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIOD_HWCFGR3_SPEC>) -> Self {
        R(reader)
    }
}
///Field `ODR_RES` reader - ODR_RES
pub type ODR_RES_R = crate::FieldReader<u16, u16>;
///Field `OTYPER_RES` reader - OTYPER_RES
pub type OTYPER_RES_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:15 - ODR_RES
    #[inline(always)]
    pub fn odr_res(&self) -> ODR_RES_R {
        ODR_RES_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - OTYPER_RES
    #[inline(always)]
    pub fn otyper_res(&self) -> OTYPER_RES_R {
        OTYPER_RES_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
///GPIO hardware configuration register 3
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [gpiod_hwcfgr3](index.html) module
pub struct GPIOD_HWCFGR3_SPEC;
impl crate::RegisterSpec for GPIOD_HWCFGR3_SPEC {
    type Ux = u32;
}
///`read()` method returns [gpiod_hwcfgr3::R](R) reader structure
impl crate::Readable for GPIOD_HWCFGR3_SPEC {
    type Reader = R;
}
///`reset()` method sets GPIOD_HWCFGR3 to value 0
impl crate::Resettable for GPIOD_HWCFGR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
