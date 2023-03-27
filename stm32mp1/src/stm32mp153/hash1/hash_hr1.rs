///Register `HASH_HR1` reader
pub struct R(crate::R<HASH_HR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HASH_HR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HASH_HR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HASH_HR1_SPEC>) -> Self {
        R(reader)
    }
}
///Field `H1` reader - H1
pub type H1_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - H1
    #[inline(always)]
    pub fn h1(&self) -> H1_R {
        H1_R::new(self.bits)
    }
}
///HASH digest register 1
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hash_hr1](index.html) module
pub struct HASH_HR1_SPEC;
impl crate::RegisterSpec for HASH_HR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [hash_hr1::R](R) reader structure
impl crate::Readable for HASH_HR1_SPEC {
    type Reader = R;
}
///`reset()` method sets HASH_HR1 to value 0
impl crate::Resettable for HASH_HR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
