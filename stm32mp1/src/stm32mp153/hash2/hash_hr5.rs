///Register `HASH_HR5` reader
pub struct R(crate::R<HASH_HR5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HASH_HR5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HASH_HR5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HASH_HR5_SPEC>) -> Self {
        R(reader)
    }
}
///Field `H5` reader - H5
pub type H5_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - H5
    #[inline(always)]
    pub fn h5(&self) -> H5_R {
        H5_R::new(self.bits)
    }
}
///HASH digest register 5
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hash_hr5](index.html) module
pub struct HASH_HR5_SPEC;
impl crate::RegisterSpec for HASH_HR5_SPEC {
    type Ux = u32;
}
///`read()` method returns [hash_hr5::R](R) reader structure
impl crate::Readable for HASH_HR5_SPEC {
    type Reader = R;
}
///`reset()` method sets HASH_HR5 to value 0
impl crate::Resettable for HASH_HR5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
