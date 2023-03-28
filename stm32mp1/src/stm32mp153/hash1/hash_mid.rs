///Register `HASH_MID` reader
pub struct R(crate::R<HASH_MID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HASH_MID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HASH_MID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HASH_MID_SPEC>) -> Self {
        R(reader)
    }
}
///Field `MID` reader - MID
pub type MID_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - MID
    #[inline(always)]
    pub fn mid(&self) -> MID_R {
        MID_R::new(self.bits)
    }
}
///HASH Hardware Magic ID
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hash_mid](index.html) module
pub struct HASH_MID_SPEC;
impl crate::RegisterSpec for HASH_MID_SPEC {
    type Ux = u32;
}
///`read()` method returns [hash_mid::R](R) reader structure
impl crate::Readable for HASH_MID_SPEC {
    type Reader = R;
}
///`reset()` method sets HASH_MID to value 0xa3c5_dd01
impl crate::Resettable for HASH_MID_SPEC {
    const RESET_VALUE: Self::Ux = 0xa3c5_dd01;
}
