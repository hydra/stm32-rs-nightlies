///Register `HR3` reader
pub struct R(crate::R<HR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HR3_SPEC>) -> Self {
        R(reader)
    }
}
///Field `H3` reader - H3
pub type H3_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - H3
    #[inline(always)]
    pub fn h3(&self) -> H3_R {
        H3_R::new(self.bits)
    }
}
///read-only
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hr3](index.html) module
pub struct HR3_SPEC;
impl crate::RegisterSpec for HR3_SPEC {
    type Ux = u32;
}
///`read()` method returns [hr3::R](R) reader structure
impl crate::Readable for HR3_SPEC {
    type Reader = R;
}
///`reset()` method sets HR3 to value 0
impl crate::Resettable for HR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
