///Register `HASH_HR4` reader
pub struct R(crate::R<HASH_HR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HASH_HR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HASH_HR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HASH_HR4_SPEC>) -> Self {
        R(reader)
    }
}
///Field `H4` reader - H4
pub type H4_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - H4
    #[inline(always)]
    pub fn h4(&self) -> H4_R {
        H4_R::new(self.bits)
    }
}
///HASH digest register 4
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hash_hr4](index.html) module
pub struct HASH_HR4_SPEC;
impl crate::RegisterSpec for HASH_HR4_SPEC {
    type Ux = u32;
}
///`read()` method returns [hash_hr4::R](R) reader structure
impl crate::Readable for HASH_HR4_SPEC {
    type Reader = R;
}
///`reset()` method sets HASH_HR4 to value 0
impl crate::Resettable for HASH_HR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
