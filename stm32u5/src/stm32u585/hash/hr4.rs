///Register `HR4` reader
pub struct R(crate::R<HR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HR4_SPEC>) -> Self {
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
///digest register 4
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hr4](index.html) module
pub struct HR4_SPEC;
impl crate::RegisterSpec for HR4_SPEC {
    type Ux = u32;
}
///`read()` method returns [hr4::R](R) reader structure
impl crate::Readable for HR4_SPEC {
    type Reader = R;
}
///`reset()` method sets HR4 to value 0
impl crate::Resettable for HR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
