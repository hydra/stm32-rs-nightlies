///Register `HASH_VERR` reader
pub struct R(crate::R<HASH_VERR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HASH_VERR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HASH_VERR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HASH_VERR_SPEC>) -> Self {
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
///HASH Version Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hash_verr](index.html) module
pub struct HASH_VERR_SPEC;
impl crate::RegisterSpec for HASH_VERR_SPEC {
    type Ux = u32;
}
///`read()` method returns [hash_verr::R](R) reader structure
impl crate::Readable for HASH_VERR_SPEC {
    type Reader = R;
}
///`reset()` method sets HASH_VERR to value 0x23
impl crate::Resettable for HASH_VERR_SPEC {
    const RESET_VALUE: Self::Ux = 0x23;
}
