///Register `HR9` reader
pub struct R(crate::R<HR9_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HR9_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HR9_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HR9_SPEC>) -> Self {
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
///For information about available fields see [hr9](index.html) module
pub struct HR9_SPEC;
impl crate::RegisterSpec for HR9_SPEC {
    type Ux = u32;
}
///`read()` method returns [hr9::R](R) reader structure
impl crate::Readable for HR9_SPEC {
    type Reader = R;
}
///`reset()` method sets HR9 to value 0
impl crate::Resettable for HR9_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
