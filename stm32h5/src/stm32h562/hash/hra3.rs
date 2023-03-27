///Register `HRA3` reader
pub struct R(crate::R<HRA3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HRA3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HRA3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HRA3_SPEC>) -> Self {
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
///HASH aliased digest register 3
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hra3](index.html) module
pub struct HRA3_SPEC;
impl crate::RegisterSpec for HRA3_SPEC {
    type Ux = u32;
}
///`read()` method returns [hra3::R](R) reader structure
impl crate::Readable for HRA3_SPEC {
    type Reader = R;
}
///`reset()` method sets HRA3 to value 0
impl crate::Resettable for HRA3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
