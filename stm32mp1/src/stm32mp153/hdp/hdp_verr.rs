///Register `HDP_VERR` reader
pub struct R(crate::R<HDP_VERR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HDP_VERR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HDP_VERR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HDP_VERR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `MINREV` reader - MINREV
pub type MINREV_R = crate::FieldReader<u8, u8>;
///Field `MAJREV` reader - MAJREV
pub type MAJREV_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:3 - MINREV
    #[inline(always)]
    pub fn minrev(&self) -> MINREV_R {
        MINREV_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - MAJREV
    #[inline(always)]
    pub fn majrev(&self) -> MAJREV_R {
        MAJREV_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
///HDP version register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hdp_verr](index.html) module
pub struct HDP_VERR_SPEC;
impl crate::RegisterSpec for HDP_VERR_SPEC {
    type Ux = u32;
}
///`read()` method returns [hdp_verr::R](R) reader structure
impl crate::Readable for HDP_VERR_SPEC {
    type Reader = R;
}
///`reset()` method sets HDP_VERR to value 0x10
impl crate::Resettable for HDP_VERR_SPEC {
    const RESET_VALUE: Self::Ux = 0x10;
}
