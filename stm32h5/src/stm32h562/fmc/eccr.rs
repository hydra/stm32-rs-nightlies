///Register `ECCR` reader
pub struct R(crate::R<ECCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ECCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ECCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ECCR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `ECC` reader - ECC result This field contains the value computed by the ECC computation logic. Table 99 describes the contents of these bitfields.
pub type ECC_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - ECC result This field contains the value computed by the ECC computation logic. Table 99 describes the contents of these bitfields.
    #[inline(always)]
    pub fn ecc(&self) -> ECC_R {
        ECC_R::new(self.bits)
    }
}
///ECC result registers
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [eccr](index.html) module
pub struct ECCR_SPEC;
impl crate::RegisterSpec for ECCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [eccr::R](R) reader structure
impl crate::Readable for ECCR_SPEC {
    type Reader = R;
}
///`reset()` method sets ECCR to value 0
impl crate::Resettable for ECCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
