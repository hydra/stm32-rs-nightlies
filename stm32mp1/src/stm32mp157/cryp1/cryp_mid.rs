///Register `CRYP_MID` reader
pub struct R(crate::R<CRYP_MID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRYP_MID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRYP_MID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRYP_MID_SPEC>) -> Self {
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
///CRYP HW Magic ID
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cryp_mid](index.html) module
pub struct CRYP_MID_SPEC;
impl crate::RegisterSpec for CRYP_MID_SPEC {
    type Ux = u32;
}
///`read()` method returns [cryp_mid::R](R) reader structure
impl crate::Readable for CRYP_MID_SPEC {
    type Reader = R;
}
///`reset()` method sets CRYP_MID to value 0xa3c5_dd01
impl crate::Resettable for CRYP_MID_SPEC {
    const RESET_VALUE: Self::Ux = 0xa3c5_dd01;
}
