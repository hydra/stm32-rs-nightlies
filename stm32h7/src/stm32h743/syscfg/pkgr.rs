///Register `PKGR` reader
pub struct R(crate::R<PKGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PKGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PKGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PKGR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `PKG` reader - Package
pub type PKG_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:3 - Package
    #[inline(always)]
    pub fn pkg(&self) -> PKG_R {
        PKG_R::new((self.bits & 0x0f) as u8)
    }
}
///SYSCFG package register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pkgr](index.html) module
pub struct PKGR_SPEC;
impl crate::RegisterSpec for PKGR_SPEC {
    type Ux = u32;
}
///`read()` method returns [pkgr::R](R) reader structure
impl crate::Readable for PKGR_SPEC {
    type Reader = R;
}
///`reset()` method sets PKGR to value 0
impl crate::Resettable for PKGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
