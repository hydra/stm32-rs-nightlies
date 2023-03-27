///Register `BSEC_HWCFGR` reader
pub struct R(crate::R<BSEC_HWCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BSEC_HWCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BSEC_HWCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BSEC_HWCFGR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `SIZE` reader - SIZE
pub type SIZE_R = crate::FieldReader<u8, u8>;
///Field `ECC_USE` reader - ECC_USE
pub type ECC_USE_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:3 - SIZE
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - ECC_USE
    #[inline(always)]
    pub fn ecc_use(&self) -> ECC_USE_R {
        ECC_USE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
///BSEC hardware configuration register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [bsec_hwcfgr](index.html) module
pub struct BSEC_HWCFGR_SPEC;
impl crate::RegisterSpec for BSEC_HWCFGR_SPEC {
    type Ux = u32;
}
///`read()` method returns [bsec_hwcfgr::R](R) reader structure
impl crate::Readable for BSEC_HWCFGR_SPEC {
    type Reader = R;
}
///`reset()` method sets BSEC_HWCFGR to value 0x14
impl crate::Resettable for BSEC_HWCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0x14;
}
