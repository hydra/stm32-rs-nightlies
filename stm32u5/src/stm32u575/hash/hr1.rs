///Register `HR1` reader
pub struct R(crate::R<HR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HR1_SPEC>) -> Self {
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
///digest register 1
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hr1](index.html) module
pub struct HR1_SPEC;
impl crate::RegisterSpec for HR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [hr1::R](R) reader structure
impl crate::Readable for HR1_SPEC {
    type Reader = R;
}
///`reset()` method sets HR1 to value 0
impl crate::Resettable for HR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
