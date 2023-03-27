///Register `HRA4` reader
pub struct R(crate::R<HRA4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HRA4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HRA4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HRA4_SPEC>) -> Self {
        R(reader)
    }
}
///Field `H4` reader - Hash data x Refer to Section 24.7.4: HASH digest registers introduction.
pub type H4_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - Hash data x Refer to Section 24.7.4: HASH digest registers introduction.
    #[inline(always)]
    pub fn h4(&self) -> H4_R {
        H4_R::new(self.bits)
    }
}
///HASH aliased digest register 4
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hra4](index.html) module
pub struct HRA4_SPEC;
impl crate::RegisterSpec for HRA4_SPEC {
    type Ux = u32;
}
///`read()` method returns [hra4::R](R) reader structure
impl crate::Readable for HRA4_SPEC {
    type Reader = R;
}
///`reset()` method sets HRA4 to value 0
impl crate::Resettable for HRA4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
