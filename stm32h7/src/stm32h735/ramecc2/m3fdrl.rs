///Register `M3FDRL` reader
pub struct R(crate::R<M3FDRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<M3FDRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<M3FDRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<M3FDRL_SPEC>) -> Self {
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
///RAMECC monitor x failing data low register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [m3fdrl](index.html) module
pub struct M3FDRL_SPEC;
impl crate::RegisterSpec for M3FDRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [m3fdrl::R](R) reader structure
impl crate::Readable for M3FDRL_SPEC {
    type Reader = R;
}
///`reset()` method sets M3FDRL to value 0
impl crate::Resettable for M3FDRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
