///Register `HASH_HR%s` reader
pub struct R(crate::R<HASH_HR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HASH_HR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HASH_HR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HASH_HR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `H` reader - H0
pub type H_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - H0
    #[inline(always)]
    pub fn h(&self) -> H_R {
        H_R::new(self.bits)
    }
}
///HASH digest register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hash_hr](index.html) module
pub struct HASH_HR_SPEC;
impl crate::RegisterSpec for HASH_HR_SPEC {
    type Ux = u32;
}
///`read()` method returns [hash_hr::R](R) reader structure
impl crate::Readable for HASH_HR_SPEC {
    type Reader = R;
}
///`reset()` method sets HASH_HR%s to value 0
impl crate::Resettable for HASH_HR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
