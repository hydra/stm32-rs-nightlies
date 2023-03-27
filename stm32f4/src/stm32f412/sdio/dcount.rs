///Register `DCOUNT` reader
pub struct R(crate::R<DCOUNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCOUNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCOUNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCOUNT_SPEC>) -> Self {
        R(reader)
    }
}
///Field `DATACOUNT` reader - Data count value
pub type DATACOUNT_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:24 - Data count value
    #[inline(always)]
    pub fn datacount(&self) -> DATACOUNT_R {
        DATACOUNT_R::new(self.bits & 0x01ff_ffff)
    }
}
///data counter register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dcount](index.html) module
pub struct DCOUNT_SPEC;
impl crate::RegisterSpec for DCOUNT_SPEC {
    type Ux = u32;
}
///`read()` method returns [dcount::R](R) reader structure
impl crate::Readable for DCOUNT_SPEC {
    type Reader = R;
}
///`reset()` method sets DCOUNT to value 0
impl crate::Resettable for DCOUNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
