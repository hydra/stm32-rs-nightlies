///Register `HASH_HR2` reader
pub struct R(crate::R<HASH_HR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HASH_HR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HASH_HR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HASH_HR2_SPEC>) -> Self {
        R(reader)
    }
}
///Field `H2` reader - H2
pub type H2_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - H2
    #[inline(always)]
    pub fn h2(&self) -> H2_R {
        H2_R::new(self.bits)
    }
}
///HASH digest register 2
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hash_hr2](index.html) module
pub struct HASH_HR2_SPEC;
impl crate::RegisterSpec for HASH_HR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [hash_hr2::R](R) reader structure
impl crate::Readable for HASH_HR2_SPEC {
    type Reader = R;
}
///`reset()` method sets HASH_HR2 to value 0
impl crate::Resettable for HASH_HR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
