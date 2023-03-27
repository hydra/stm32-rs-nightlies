///Register `HR15` reader
pub struct R(crate::R<HR15_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HR15_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HR15_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HR15_SPEC>) -> Self {
        R(reader)
    }
}
///Field `Hx` reader - Hash data x Refer to introduction.
pub type HX_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - Hash data x Refer to introduction.
    #[inline(always)]
    pub fn hx(&self) -> HX_R {
        HX_R::new(self.bits)
    }
}
///HASH digest register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hr15](index.html) module
pub struct HR15_SPEC;
impl crate::RegisterSpec for HR15_SPEC {
    type Ux = u32;
}
///`read()` method returns [hr15::R](R) reader structure
impl crate::Readable for HR15_SPEC {
    type Reader = R;
}
///`reset()` method sets HR15 to value 0
impl crate::Resettable for HR15_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
