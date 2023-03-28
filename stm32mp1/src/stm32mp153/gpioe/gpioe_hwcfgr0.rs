///Register `GPIOE_HWCFGR0` reader
pub struct R(crate::R<GPIOE_HWCFGR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIOE_HWCFGR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIOE_HWCFGR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIOE_HWCFGR0_SPEC>) -> Self {
        R(reader)
    }
}
///Field `OR_RES` reader - OR_RES
pub type OR_RES_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:15 - OR_RES
    #[inline(always)]
    pub fn or_res(&self) -> OR_RES_R {
        OR_RES_R::new((self.bits & 0xffff) as u16)
    }
}
///GPIO hardware configuration register 0
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [gpioe_hwcfgr0](index.html) module
pub struct GPIOE_HWCFGR0_SPEC;
impl crate::RegisterSpec for GPIOE_HWCFGR0_SPEC {
    type Ux = u32;
}
///`read()` method returns [gpioe_hwcfgr0::R](R) reader structure
impl crate::Readable for GPIOE_HWCFGR0_SPEC {
    type Reader = R;
}
///`reset()` method sets GPIOE_HWCFGR0 to value 0
impl crate::Resettable for GPIOE_HWCFGR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
