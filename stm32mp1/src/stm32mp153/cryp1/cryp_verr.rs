///Register `CRYP_VERR` reader
pub struct R(crate::R<CRYP_VERR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRYP_VERR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRYP_VERR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRYP_VERR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `VER` reader - VER
pub type VER_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:7 - VER
    #[inline(always)]
    pub fn ver(&self) -> VER_R {
        VER_R::new((self.bits & 0xff) as u8)
    }
}
///CRYP HW Version Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cryp_verr](index.html) module
pub struct CRYP_VERR_SPEC;
impl crate::RegisterSpec for CRYP_VERR_SPEC {
    type Ux = u32;
}
///`read()` method returns [cryp_verr::R](R) reader structure
impl crate::Readable for CRYP_VERR_SPEC {
    type Reader = R;
}
///`reset()` method sets CRYP_VERR to value 0x22
impl crate::Resettable for CRYP_VERR_SPEC {
    const RESET_VALUE: Self::Ux = 0x22;
}
