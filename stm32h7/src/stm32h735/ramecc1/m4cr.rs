///Register `M4CR` reader
pub struct R(crate::R<M4CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<M4CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<M4CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<M4CR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `FDATAL` reader - Failing data low
pub type FDATAL_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - Failing data low
    #[inline(always)]
    pub fn fdatal(&self) -> FDATAL_R {
        FDATAL_R::new(self.bits)
    }
}
///RAMECC monitor x configuration register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [m4cr](index.html) module
pub struct M4CR_SPEC;
impl crate::RegisterSpec for M4CR_SPEC {
    type Ux = u32;
}
///`read()` method returns [m4cr::R](R) reader structure
impl crate::Readable for M4CR_SPEC {
    type Reader = R;
}
///`reset()` method sets M4CR to value 0
impl crate::Resettable for M4CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
