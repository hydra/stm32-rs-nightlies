///Register `M1FDRH` reader
pub struct R(crate::R<M1FDRH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<M1FDRH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<M1FDRH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<M1FDRH_SPEC>) -> Self {
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
///RAMECC monitor x failing data high register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [m1fdrh](index.html) module
pub struct M1FDRH_SPEC;
impl crate::RegisterSpec for M1FDRH_SPEC {
    type Ux = u32;
}
///`read()` method returns [m1fdrh::R](R) reader structure
impl crate::Readable for M1FDRH_SPEC {
    type Reader = R;
}
///`reset()` method sets M1FDRH to value 0
impl crate::Resettable for M1FDRH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
