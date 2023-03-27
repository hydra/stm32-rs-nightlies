///Register `HRA1` reader
pub struct R(crate::R<HRA1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HRA1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HRA1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HRA1_SPEC>) -> Self {
        R(reader)
    }
}
///Field `H1` reader - Hash data x Refer to Section 24.7.4: HASH digest registers introduction.
pub type H1_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - Hash data x Refer to Section 24.7.4: HASH digest registers introduction.
    #[inline(always)]
    pub fn h1(&self) -> H1_R {
        H1_R::new(self.bits)
    }
}
///HASH aliased digest register 1
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hra1](index.html) module
pub struct HRA1_SPEC;
impl crate::RegisterSpec for HRA1_SPEC {
    type Ux = u32;
}
///`read()` method returns [hra1::R](R) reader structure
impl crate::Readable for HRA1_SPEC {
    type Reader = R;
}
///`reset()` method sets HRA1 to value 0
impl crate::Resettable for HRA1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
