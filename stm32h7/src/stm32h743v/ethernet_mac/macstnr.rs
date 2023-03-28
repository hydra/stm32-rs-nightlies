///Register `MACSTNR` reader
pub struct R(crate::R<MACSTNR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACSTNR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACSTNR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACSTNR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `TSSS` reader - Timestamp Sub-seconds
pub type TSSS_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:30 - Timestamp Sub-seconds
    #[inline(always)]
    pub fn tsss(&self) -> TSSS_R {
        TSSS_R::new(self.bits & 0x7fff_ffff)
    }
}
///System time nanoseconds register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [macstnr](index.html) module
pub struct MACSTNR_SPEC;
impl crate::RegisterSpec for MACSTNR_SPEC {
    type Ux = u32;
}
///`read()` method returns [macstnr::R](R) reader structure
impl crate::Readable for MACSTNR_SPEC {
    type Reader = R;
}
///`reset()` method sets MACSTNR to value 0
impl crate::Resettable for MACSTNR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
