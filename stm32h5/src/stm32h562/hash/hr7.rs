///Register `HR7` reader
pub struct R(crate::R<HR7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HR7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HR7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HR7_SPEC>) -> Self {
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
///For information about available fields see [hr7](index.html) module
pub struct HR7_SPEC;
impl crate::RegisterSpec for HR7_SPEC {
    type Ux = u32;
}
///`read()` method returns [hr7::R](R) reader structure
impl crate::Readable for HR7_SPEC {
    type Reader = R;
}
///`reset()` method sets HR7 to value 0
impl crate::Resettable for HR7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
