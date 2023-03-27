///Register `M4FAR` reader
pub struct R(crate::R<M4FAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<M4FAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<M4FAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<M4FAR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `FDATAH` reader - Failing data high (64-bit memory)
pub type FDATAH_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - Failing data high (64-bit memory)
    #[inline(always)]
    pub fn fdatah(&self) -> FDATAH_R {
        FDATAH_R::new(self.bits)
    }
}
///RAMECC monitor x failing address register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [m4far](index.html) module
pub struct M4FAR_SPEC;
impl crate::RegisterSpec for M4FAR_SPEC {
    type Ux = u32;
}
///`read()` method returns [m4far::R](R) reader structure
impl crate::Readable for M4FAR_SPEC {
    type Reader = R;
}
///`reset()` method sets M4FAR to value 0
impl crate::Resettable for M4FAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
