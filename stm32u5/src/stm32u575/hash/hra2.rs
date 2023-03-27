///Register `HRA2` reader
pub struct R(crate::R<HRA2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HRA2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HRA2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HRA2_SPEC>) -> Self {
        R(reader)
    }
}
///Field `H2` reader - H2
pub type H2_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - H2
    #[inline(always)]
    pub fn h2(&self) -> H2_R {
        H2_R::new(self.bits)
    }
}
///HASH aliased digest register 2
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hra2](index.html) module
pub struct HRA2_SPEC;
impl crate::RegisterSpec for HRA2_SPEC {
    type Ux = u32;
}
///`read()` method returns [hra2::R](R) reader structure
impl crate::Readable for HRA2_SPEC {
    type Reader = R;
}
///`reset()` method sets HRA2 to value 0
impl crate::Resettable for HRA2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
