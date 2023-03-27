///Register `HR0` reader
pub struct R(crate::R<HR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HR0_SPEC>) -> Self {
        R(reader)
    }
}
///Field `H0` reader - H0
pub type H0_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - H0
    #[inline(always)]
    pub fn h0(&self) -> H0_R {
        H0_R::new(self.bits)
    }
}
///HASH digest register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hr0](index.html) module
pub struct HR0_SPEC;
impl crate::RegisterSpec for HR0_SPEC {
    type Ux = u32;
}
///`read()` method returns [hr0::R](R) reader structure
impl crate::Readable for HR0_SPEC {
    type Reader = R;
}
///`reset()` method sets HR0 to value 0
impl crate::Resettable for HR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
